use std::env;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use aspectus::budget::{allocate_shares, explain};
use aspectus::config::{render_show, resolve, CALLER_FLAG};
use aspectus::render::render_text;
use aspectus::walk::{walk, WalkOptions};

/// One source: every accepted flag/verb is named here and printed in help.
const COMMANDS: &[(&str, &str)] = &[
    ("help, -h, --help", "this page (includes the version line)"),
    (
        "version, -v, --version",
        "one line: name + semver; +sha if not a tagged release",
    ),
    ("config", "show which config layers were consulted and what won"),
];

const OPTIONS: &[(&str, &str)] = &[
    ("--", "end of flags"),
    ("--config PATH", "use this file as user-home for this run"),
    ("--caller KEY", "agent-type for configuration selection"),
    ("--lines N", "line budget including the root (default 80)"),
    (
        "--visit N",
        "max directory entries to process (default 400)",
    ),
    ("--explain-budget", "shares and why, on stderr"),
    (
        "--show-all",
        "show .git, target/, and other otherwise hidden areas",
    ),
    (
        "--inspect KIND",
        "show KIND directories and files",
    ),
    ("--no-one-fs", "follow mounts (default is one filesystem)"),
    (
        "--color=auto|always|never",
        "color only if stdout is a TTY (auto)",
    ),
];

fn help_page() -> String {
    let mut out = format!(
        "{ver}\n\
         \n\
         usage: aspectus [PATH]\n\
                aspectus help\n\
                aspectus version\n\
                aspectus config\n\
                aspectus [-h|--help]\n\
                aspectus [-v|--version]\n\
         \n\
         aspectus is the faculty of looking at a locus: the look itself, and\n\
         the command that produces it. You run it on a locus. The path\n\
         argument is how the command is given a place.\n\
         \n\
         A locus is the location of action — project, sandbox, channel,\n\
         environment, machine. This tool does not implement a locus. It looks\n\
         at one. Today that look is of the filesystem face of the place (the\n\
         tree under the path).\n\
         \n\
         An aspecta is one look: the seen-things from one running of aspectus\n\
         on one locus. Two prints of the same path are two aspecta. They may\n\
         differ. It is the picture, not a summary essay about the place, and\n\
         not the place itself.\n\
         \n\
         It is not carta (who the place is). It is not conspectus (what a mind\n\
         is shown). It is not percepta (ongoing status and health). aspectus\n\
         is how the place looks right now.\n\
         \n\
         Default: the place and its immediate children, then it exits.\n\
         \n\
         Commands:\n",
        ver = version_line()
    );
    for (name, desc) in COMMANDS {
        out.push_str(&format!("  {name:<28}{desc}\n"));
    }
    out.push_str("\nOptions:\n");
    for (name, desc) in OPTIONS {
        out.push_str(&format!("  {name:<28}{desc}\n"));
    }
    out.push_str(
        "\n\
         Examples:\n\
           aspectus help\n\
           aspectus version\n\
           aspectus config\n\
           aspectus\n\
           aspectus PATH\n",
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
    Config(ConfigArgs),
    Show(ShowArgs),
}

struct ConfigArgs {
    user_home_override: Option<PathBuf>,
    caller: Option<String>,
    flag_lines: Option<u32>,
}

struct ShowArgs {
    path: PathBuf,
    lines: usize,
    visit: usize,
    explain: bool,
    inspect: Option<String>,
    one_fs: bool,
    #[allow(dead_code)]
    user_home_override: Option<PathBuf>,
    #[allow(dead_code)]
    caller: Option<String>,
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
    let mut config_cmd = false;
    let mut user_home_override = None;
    let mut caller = None;
    let mut lines_set = false;
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
            "config" => config_cmd = true,
            "--config" => {
                let v = args
                    .next()
                    .ok_or_else(|| Refusal::Usage("--config needs a path".into()))?;
                user_home_override = Some(PathBuf::from(v));
            }
            s if s.starts_with("--config=") => {
                user_home_override = Some(PathBuf::from(&s[9..]));
            }
            s if s == CALLER_FLAG => {
                let v = args
                    .next()
                    .ok_or_else(|| Refusal::Usage("--caller needs a key".into()))?;
                caller = Some(v);
            }
            s if s.starts_with("--caller=") => {
                caller = Some(s[9..].to_string());
            }
            "--explain-budget" => explain = true,
            "--show-all" => inspect = Some("*".into()),
            "--no-one-fs" => one_fs = false,
            "--inspect" => {
                let v = args
                    .next()
                    .ok_or_else(|| Refusal::Usage("--inspect needs a KIND".into()))?;
                if v.starts_with('-') {
                    return Err(Refusal::Usage("--inspect needs a KIND".into()));
                }
                inspect = Some(v);
            }
            "--lines" => {
                let v = args
                    .next()
                    .ok_or_else(|| Refusal::Usage("--lines needs a number".into()))?;
                lines = v
                    .parse()
                    .map_err(|_| Refusal::Usage(format!("bad --lines {v}")))?;
                lines_set = true;
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
                lines_set = true;
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
            "--color" => {
                if let Some(n) = args.peek() {
                    if matches!(n.as_str(), "auto" | "always" | "never") {
                        let _ = args.next();
                    }
                }
            }
            s if s.starts_with("--color=") => {}
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
    if config_cmd {
        return Ok(Cmd::Config(ConfigArgs {
            user_home_override,
            caller,
            flag_lines: if lines_set {
                Some(lines as u32)
            } else {
                None
            },
        }));
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
        user_home_override,
        caller,
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
        Ok(Cmd::Config(c)) => {
            let res = resolve(
                c.user_home_override.as_deref(),
                c.caller.as_deref(),
                c.flag_lines,
            );
            print!("{}", render_show(&res));
            ExitCode::SUCCESS
        }
        Ok(Cmd::Show(args)) => match show(args) {
            Ok(()) => ExitCode::SUCCESS,
            Err(ShowErr::NotFound(p)) => {
                let _ = writeln!(io::stderr(), "aspectus: not found {p}");
                ExitCode::from(2)
            }
            Err(ShowErr::Other(e)) => {
                let _ = writeln!(io::stderr(), "aspectus: {e}");
                ExitCode::from(2)
            }
        },
    }
}

enum ShowErr {
    NotFound(String),
    Other(String),
}

fn show(args: ShowArgs) -> Result<(), ShowErr> {
    let deep = args.inspect.is_some() || args.explain;
    if deep {
        let opts = WalkOptions {
            visit_budget: args.visit,
            one_filesystem: args.one_fs,
            inspect: args.inspect,
        };
        let result = walk(&args.path, opts).map_err(|e| map_io(&args.path, e))?;
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
        return Ok(());
    }

    let locus = aspectus::two_level::resolve_locus(&args.path);
    let (name, kids) = aspectus::two_level::list(&locus).map_err(|e| map_io(&locus, e))?;
    print!("{}", aspectus::two_level::render(&name, &kids));
    Ok(())
}

fn map_io(path: &Path, e: std::io::Error) -> ShowErr {
    if e.kind() == std::io::ErrorKind::NotFound {
        ShowErr::NotFound(path.display().to_string())
    } else {
        ShowErr::Other(format!("{}: {e}", path.display()))
    }
}
