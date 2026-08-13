use std::env;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::ExitCode;

use aspectus::budget::{allocate_shares, explain};
use aspectus::render::render_text;
use aspectus::walk::{walk, WalkOptions};

fn usage() -> &'static str {
    "aspectus — the look of a locus (print-and-quit)

A budgeted snapshot of a filesystem workspace. Well-known furniture
(.git, target/, .obsidian/, …) is parent state, not a child listing.
Siblings share a line budget so the whole place still has a shape.

  aspectus [PATH]             snapshot this locus (cwd if omitted)
  aspectus --lines N          line budget including the root (default 80)
  aspectus --visit N          max directory entries to process (default 400)
  aspectus --explain-budget   shares and why, on stderr
  aspectus --raw              open absorbed names (.git, target/, …)
  aspectus --inspect [KIND]   open absorbed names, or only KIND (git, build, …)
  aspectus -x                 stay on one filesystem (default)
  aspectus --no-one-fs        follow mounts
  aspectus -h | --help        this text
  aspectus --version

DANGEROUS: --raw / --inspect descend into absorbed furniture (.git/objects,
target/, …). The default picture already says the parent is a git/rust/…
locus; you do not need --raw to understand the tree.

Stdout is the picture. Diagnostics go to stderr.
"
}

struct Args {
    path: PathBuf,
    lines: usize,
    visit: usize,
    explain: bool,
    inspect: Option<String>,
    one_fs: bool,
    help: bool,
    version: bool,
}

fn parse_args(argv: impl IntoIterator<Item = String>) -> Result<Args, String> {
    let mut path = None;
    let mut lines = 80usize;
    let mut visit = 400usize;
    let mut explain = false;
    let mut inspect = None;
    let mut one_fs = true;
    let mut help = false;
    let mut version = false;
    let mut args = argv.into_iter().peekable();
    while let Some(a) = args.next() {
        match a.as_str() {
            "-h" | "--help" => help = true,
            "--version" => version = true,
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
                let v = args.next().ok_or("--lines needs a number")?;
                lines = v.parse().map_err(|_| format!("bad --lines {v}"))?;
            }
            "--visit" => {
                let v = args.next().ok_or("--visit needs a number")?;
                visit = v.parse().map_err(|_| format!("bad --visit {v}"))?;
            }
            s if s.starts_with("--lines=") => {
                let v = &s[8..];
                lines = v.parse().map_err(|_| format!("bad --lines {v}"))?;
            }
            s if s.starts_with("--visit=") => {
                let v = &s[8..];
                visit = v.parse().map_err(|_| format!("bad --visit {v}"))?;
            }
            s if s.starts_with("--inspect=") => {
                inspect = Some(s[10..].to_string());
            }
            "--" => {
                if let Some(p) = args.next() {
                    path = Some(PathBuf::from(p));
                }
            }
            s if s.starts_with('-') => return Err(format!("unknown flag {s}")),
            s => {
                if path.is_some() {
                    return Err("only one PATH".into());
                }
                path = Some(PathBuf::from(s));
            }
        }
    }
    Ok(Args {
        path: path.unwrap_or_else(|| PathBuf::from(".")),
        lines,
        visit,
        explain,
        inspect,
        one_fs,
        help,
        version,
    })
}

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(e) => {
            let _ = writeln!(io::stderr(), "aspectus: {e}");
            ExitCode::from(2)
        }
    }
}

fn run() -> Result<ExitCode, String> {
    let args = parse_args(env::args().skip(1))?;
    if args.help {
        print!("{}", usage());
        return Ok(ExitCode::SUCCESS);
    }
    if args.version {
        println!("aspectus {}", env!("CARGO_PKG_VERSION"));
        return Ok(ExitCode::SUCCESS);
    }

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
    Ok(ExitCode::SUCCESS)
}
