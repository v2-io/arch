use std::env;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use aspectus::budget::{allocate_shares, explain};
use aspectus::render::render_text;
use aspectus::walk::{walk, WalkOptions};

/// One source: every accepted flag/verb is named here and printed in help.
const VERBS_AND_FLAGS: &[(&str, &str)] = &[
    ("help, -h, --help", "this page"),
    (
        "version, -v, --version",
        "name + semver; +sha if not a tagged release",
    ),
    ("--", "end of flags"),
    ("--lines N", "line budget including the root (default 80)"),
    (
        "--visit N",
        "max directory entries to process (default 400)",
    ),
    ("--explain-budget", "shares and why, on stderr"),
    ("--raw", "open absorbed names (.git, target/, …)"),
    (
        "--inspect [KIND]",
        "open absorbed names, or only KIND (git, build, …)",
    ),
    ("-x", "stay on one filesystem (default)"),
    ("--no-one-fs", "follow mounts"),
];

fn help_page() -> String {
    let mut out = String::from(
        "aspectus — the look of a locus\n\
         \n\
         aspectus is the faculty of looking at a place. One look is an aspecta.\n\
         \n\
         usage: aspectus help\n\
                aspectus version\n\
                aspectus [-h|--help]\n\
                aspectus [-v|--version]\n\
         \n",
    );
    for (name, desc) in VERBS_AND_FLAGS {
        out.push_str(&format!("  {name:<24}{desc}\n"));
    }
    out.push_str(
        "\n\
         Examples:\n\
           aspectus help\n\
           aspectus version\n",
    );
    out
}

fn version_line() -> String {
    let ver = env!("CARGO_PKG_VERSION");
    match option_env!("ASPECTUS_GIT_SHA") {
        Some(sha) if !sha.is_empty() => format!("aspectus {ver}+{sha}"),
        _ => format!("aspectus {ver}"),
    }
}

enum Cmd {
    Help,
    Version,
    Show(ShowArgs),
}

struct ShowArgs {
    path: PathBuf,
    lines: usize,
    visit: usize,
    explain: bool,
    inspect: Option<String>,
    one_fs: bool,
}

enum Refusal {
    UnknownOption(String),
    UnknownVerb(String),
    Usage(String),
}

impl Refusal {
    fn class(&self) -> &'static str {
        match self {
            Refusal::UnknownOption(_) => "unknown option",
            Refusal::UnknownVerb(_) => "unknown verb",
            Refusal::Usage(_) => "usage",
        }
    }

    fn token(&self) -> &str {
        match self {
            Refusal::UnknownOption(t) | Refusal::UnknownVerb(t) | Refusal::Usage(t) => t,
        }
    }

    fn write_stderr(&self) {
        let _ = writeln!(
            io::stderr(),
            "aspectus: {} {}\n  next: aspectus help",
            self.class(),
            self.token()
        );
    }
}

fn parse_args(argv: impl IntoIterator<Item = String>) -> Result<Cmd, Refusal> {
    let mut path = None;
    let mut lines = 80usize;
    let mut visit = 400usize;
    let mut explain = false;
    let mut inspect = None;
    let mut one_fs = true;
    let mut help = false;
    let mut version = false;
    let mut end_flags = false;
    let mut args = argv.into_iter().peekable();

    while let Some(a) = args.next() {
        if end_flags {
            take_positional(&mut path, a)?;
            continue;
        }
        match a.as_str() {
            "help" | "-h" | "--help" => help = true,
            "version" | "-v" | "--version" => version = true,
            "--explain-budget" => explain = true,
            "--raw" => inspect = Some("*".into()),
            "-x" => one_fs = true,
            "--no-one-fs" => one_fs = false,
            "--inspect" => {
                if let Some(n) = args.peek() {
                    if !n.starts_with('-') {
                        inspect = Some(args.next().unwrap());
                        continue;
                    }
                }
                inspect = Some("*".into());
            }
            "--lines" => {
                let v = args
                    .next()
                    .ok_or_else(|| Refusal::Usage("--lines needs a number".into()))?;
                lines = v
                    .parse()
                    .map_err(|_| Refusal::Usage(format!("bad --lines {v}")))?;
            }
            "--visit" => {
                let v = args
                    .next()
                    .ok_or_else(|| Refusal::Usage("--visit needs a number".into()))?;
                visit = v
                    .parse()
                    .map_err(|_| Refusal::Usage(format!("bad --visit {v}")))?;
            }
            s if s.starts_with("--lines=") => {
                let v = &s[8..];
                lines = v
                    .parse()
                    .map_err(|_| Refusal::Usage(format!("bad --lines {v}")))?;
            }
            s if s.starts_with("--visit=") => {
                let v = &s[8..];
                visit = v
                    .parse()
                    .map_err(|_| Refusal::Usage(format!("bad --visit {v}")))?;
            }
            s if s.starts_with("--inspect=") => {
                inspect = Some(s[10..].to_string());
            }
            "--" => end_flags = true,
            s if s.starts_with('-') => return Err(Refusal::UnknownOption(s.to_string())),
            s => take_positional(&mut path, s.to_string())?,
        }
    }

    if help {
        return Ok(Cmd::Help);
    }
    if version {
        return Ok(Cmd::Version);
    }

    let path = match path {
        None => PathBuf::from("."),
        Some(p) => classify_positional(p)?,
    };

    Ok(Cmd::Show(ShowArgs {
        path,
        lines,
        visit,
        explain,
        inspect,
        one_fs,
    }))
}

fn take_positional(path: &mut Option<String>, token: String) -> Result<(), Refusal> {
    if path.is_some() {
        return Err(Refusal::Usage("only one PATH".into()));
    }
    *path = Some(token);
    Ok(())
}

/// After flags: reserved verbs already handled. A token that is not an
/// existing path and does not look like a path is an unknown verb.
fn classify_positional(token: String) -> Result<PathBuf, Refusal> {
    let p = Path::new(&token);
    if p.exists() || token.contains('/') || token.starts_with('.') {
        return Ok(PathBuf::from(token));
    }
    Err(Refusal::UnknownVerb(token))
}

fn main() -> ExitCode {
    match parse_args(env::args().skip(1)) {
        Err(r) => {
            r.write_stderr();
            ExitCode::from(2)
        }
        Ok(Cmd::Help) => {
            print!("{}", help_page());
            ExitCode::SUCCESS
        }
        Ok(Cmd::Version) => {
            println!("{}", version_line());
            ExitCode::SUCCESS
        }
        Ok(Cmd::Show(args)) => match show(args) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                let _ = writeln!(io::stderr(), "aspectus: {e}");
                ExitCode::from(2)
            }
        },
    }
}

fn show(args: ShowArgs) -> Result<(), String> {
    let opts = WalkOptions {
        visit_budget: args.visit,
        one_filesystem: args.one_fs,
        inspect: args.inspect,
    };
    let result = walk(&args.path, opts).map_err(|e| format!("{}: {e}", args.path.display()))?;

    if args.explain {
        let alloc = allocate_shares(&result.aspecta.node.children, args.lines);
        let _ = write!(
            io::stderr(),
            "{}",
            explain(&result.aspecta.node.children, &alloc, args.lines)
        );
    }

    let picture = render_text(&result.aspecta.node, args.lines);
    print!("{picture}");
    Ok(())
}
