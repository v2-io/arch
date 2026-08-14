use std::env;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use std::collections::BTreeMap;

use aspectus::color::Mode as ColorMode;
use aspectus::config::{render_show, resolve, CALLER_FLAG};

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
    (
        "--color=auto|always|never",
        "color directories when stdout is a TTY (auto)",
    ),
    (
        "--depth N",
        "generations below the root (default 2; 0 = no limit)",
    ),
    (
        "--lines N",
        "line budget for the whole look (default 80; 0 = no limit)",
    ),
    (
        "--walk N",
        "stat and expand at most N names; names past the bound still \
         count in censuses, and a cut dir says [walk bound] \
         (default 10000; 0 = no bound)",
    ),
    ("--explain-budget", "how lines and the walk were spent, on stderr"),
    (
        "--show-all",
        "list furniture names (.git, target/, …) as ordinary children",
    ),
    (
        "--inspect KIND",
        "list furniture of one kind as children (git, build, …); repeatable",
    ),
    (
        "--sort KEY",
        "display order: recency (default; mtime, newest first), name, \
         size, line-count, heat; -KEY reverses. Unbuilt lattice keys \
         are refused by name",
    ),
    (
        "--dotfiles-first",
        "group dot-names before the rest (dirs still first)",
    ),
    (
        "--no-one-fs",
        "follow mount points; the default stays on the starting \
         filesystem and marks a mount [other fs]",
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
         Default: the absolute path of the place, two generations down\n\
         (children and grandchildren), then it exits. The header is two\n\
         lines: the UTC time of the look, then the absolute root.\n\
         \n\
         Children are ordered by recency (newest first, directories\n\
         first) so calling again shows what moved, at the top. Under a\n\
         tight --lines the recently-changed also survive. --sort name\n\
         restores alphabetical.\n\
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
           aspectus PATH\n\
           aspectus --depth 3 --lines 120 PATH\n\
           aspectus --walk 500 PATH    (huge tree: expand less, count all)\n\
           aspectus --inspect git PATH (see inside .git as ordinary children)\n\
         \n\
         Non-binary files show a line count (binary shows none, never 0);\n\
         kind comes from a suffix-map extendable via config key `kinds`\n\
         (`SUFFIX:text|binary`, `!SUFFIX` to drop). An unexpanded directory\n\
         carries a census of what it held — [dir×3 \u{2248}120f \u{b7} md\u{d7}31] — with\n\
         subdirectories as containers whose deep file-count (mass) leads,\n\
         and the subtree's text lines after it (\u{2248}61k lines): a glance\n\
         calibrates how much has not been seen. \u{2265} marks a floor; a\n\
         single concealed name shows the name. The look reads file content\n\
         only up to a budget (config `reads`, bytes); past it, deep line\n\
         totals are estimated (\u{2248}) and per-file counts omitted \u{2014} the\n\
         glance stays fast, and says how it degraded.\n\
         \n\
         Inside a git repo each visible line carries the aliveness cluster\n\
         `score \u{b7} age` (commit-decay heat, git-heat's model, half-life 7\n\
         commits via config `heat.half-life`; age is the mtime delta).\n\
         Outside git, no heat is claimed. `--sort heat` orders by it;\n\
         config `recency-source = git` makes the default recency sort use\n\
         git last-touch where known.\n\
         \n\
         Symlinked directories are followed and recursed like real ones\n\
         (facts are the target's; `-> target` says how it got here). A\n\
         cycle prints [cycle] instead of hanging. The walk stays on the\n\
         starting filesystem; a mount point shows [other fs] and stops\n\
         there unless --no-one-fs.\n\
         \n\
         Well-known names are furniture: state on their parent line, not\n\
         children of the look. A git work tree does not list .git — the\n\
         directory line says what git is here ([git: remote<…> br<main>\n\
         @sha dirty<N>], local facts only, no network); build debris\n\
         (target/, __pycache__, …) folds into the [kind: …] spot. Hidden\n\
         names are not counted as children; the kind spot is what says\n\
         they are here. The map is glob-based and extendable from config\n\
         (key `furniture`: `PATTERN[:KIND[:hide|omit|mark]]`, comma-\n\
         separated; `!PATTERN` drops a default row).\n\
         \n\
         The look never lies by omission: an unexpanded directory carries a\n\
         census, a walk-bound cut keeps the full name count and says\n\
         [walk bound], a count that hides an unreadable place is marked \u{2265},\n\
         and a directory it could not read says [denied] rather than\n\
         printing as empty.\n\
         \n\
         Facts beyond the defaults (size, mtime, ...) have no flags of\n\
         their own; ask through config on the caller stack, e.g.\n\
         columns.size = on, format.mtime = epoch, sort = -size (env:\n\
         ASPECTUS_COLUMNS_SIZE, ...). Column values align at computed\n\
         tab-stops -- a function of the tree, never terminal width, so\n\
         two looks of the same tree stay diffable. `aspectus config`\n\
         lists every fact with its state, ask, and format.\n",
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
    flag_vals: BTreeMap<String, String>,
}

struct ShowArgs {
    path: PathBuf,
    color: ColorMode,
    user_home_override: Option<PathBuf>,
    caller: Option<String>,
    flag_vals: BTreeMap<String, String>,
    explain: bool,
    show_all: bool,
    inspect: Vec<String>,
}

enum Refusal {
    UnknownOption(String),
    UnknownVerb(String),
    Usage(String),
    /// `--size` etc. — a real lattice fact, refused a flag; the message
    /// names the config path (design/columns.md subfeature 3).
    FactAsk(String, String),
}

impl Refusal {
    fn class(&self) -> &'static str {
        match self {
            Refusal::UnknownOption(_) => "unknown option",
            Refusal::UnknownVerb(_) => "unknown verb",
            Refusal::Usage(_) => "usage",
            Refusal::FactAsk(_, _) => "no flag for fact",
        }
    }

    fn token(&self) -> &str {
        match self {
            Refusal::UnknownOption(t)
            | Refusal::UnknownVerb(t)
            | Refusal::Usage(t)
            | Refusal::FactAsk(t, _) => t,
        }
    }

    fn write_stderr(&self) {
        match self {
            Refusal::FactAsk(t, ask) => {
                let _ = writeln!(
                    io::stderr(),
                    "aspectus: {} {t}\n  {ask}\n  next: aspectus help",
                    self.class()
                );
            }
            _ => {
                let _ = writeln!(
                    io::stderr(),
                    "aspectus: {} {}\n  next: aspectus help",
                    self.class(),
                    self.token()
                );
            }
        }
    }
}

fn parse_args(argv: impl IntoIterator<Item = String>) -> Result<Cmd, Refusal> {
    let mut path = None;
    let mut help = false;
    let mut version = false;
    let mut config_cmd = false;
    let mut user_home_override = None;
    let mut caller = None;
    let mut color = ColorMode::Auto;
    let mut flag_vals = BTreeMap::new();
    let mut explain = false;
    let mut show_all = false;
    let mut inspect = Vec::new();
    let mut end_flags = false;
    // After `--` everything is a path — never re-read as a verb.
    let mut path_is_literal = false;
    let mut args = argv.into_iter().peekable();

    while let Some(a) = args.next() {
        if end_flags {
            take_positional(&mut path, a)?;
            path_is_literal = true;
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
            "--color" => {
                let v = args
                    .next()
                    .ok_or_else(|| Refusal::Usage("--color needs auto, always, or never".into()))?;
                color = ColorMode::parse(&v).ok_or_else(|| {
                    Refusal::Usage(format!("--color needs auto, always, or never (got {v})"))
                })?;
            }
            s if s.starts_with("--color=") => {
                let v = &s[8..];
                color = ColorMode::parse(v).ok_or_else(|| {
                    Refusal::Usage(format!("--color needs auto, always, or never (got {v})"))
                })?;
            }
            "--depth" => {
                let v = args
                    .next()
                    .ok_or_else(|| Refusal::Usage("--depth needs a number".into()))?;
                parse_depth(&v)?;
                flag_vals.insert("depth".into(), v);
            }
            s if s.starts_with("--depth=") => {
                let v = &s[8..];
                parse_depth(v)?;
                flag_vals.insert("depth".into(), v.to_string());
            }
            "--lines" => {
                let v = args
                    .next()
                    .ok_or_else(|| Refusal::Usage("--lines needs a number".into()))?;
                parse_lines(&v)?;
                flag_vals.insert("lines".into(), v);
            }
            s if s.starts_with("--lines=") => {
                let v = &s[8..];
                parse_lines(v)?;
                flag_vals.insert("lines".into(), v.to_string());
            }
            "--walk" => {
                let v = args
                    .next()
                    .ok_or_else(|| Refusal::Usage("--walk needs a number".into()))?;
                parse_walk(&v)?;
                flag_vals.insert("walk".into(), v);
            }
            s if s.starts_with("--walk=") => {
                let v = &s[7..];
                parse_walk(v)?;
                flag_vals.insert("walk".into(), v.to_string());
            }
            "--sort" => {
                let v = args
                    .next()
                    .ok_or_else(|| Refusal::Usage("--sort needs a key (recency, name, size)".into()))?;
                flag_vals.insert("sort".into(), v);
            }
            s if s.starts_with("--sort=") => {
                flag_vals.insert("sort".into(), s[7..].to_string());
            }
            "--dotfiles-first" => {
                flag_vals.insert("dotfiles-first".into(), "on".into());
            }
            "--no-one-fs" => {
                flag_vals.insert("one-fs".into(), "off".into());
            }
            "--explain-budget" => explain = true,
            "--show-all" => show_all = true,
            "--inspect" => {
                let v = args
                    .next()
                    .ok_or_else(|| Refusal::Usage("--inspect needs a kind (git, build, …)".into()))?;
                inspect.push(v);
            }
            s if s.starts_with("--inspect=") => {
                inspect.push(s[10..].to_string());
            }
            "--" => end_flags = true,
            s if s.starts_with('-') => {
                if let Some(ask) = aspectus::facts::flag_refusal(s) {
                    return Err(Refusal::FactAsk(s.to_string(), ask));
                }
                return Err(Refusal::UnknownOption(s.to_string()));
            }
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
            flag_vals,
        }));
    }

    let path = match path {
        None => PathBuf::from("."),
        Some(p) if path_is_literal => PathBuf::from(p),
        Some(p) => classify_positional(p)?,
    };

    Ok(Cmd::Show(ShowArgs {
        path,
        color,
        user_home_override,
        caller,
        flag_vals,
        explain,
        show_all,
        inspect,
    }))
}

fn parse_lines(s: &str) -> Result<u32, Refusal> {
    s.parse::<u32>()
        .map_err(|_| Refusal::Usage(format!("--lines needs a number (got {s})")))
}

fn parse_depth(s: &str) -> Result<u32, Refusal> {
    s.parse::<u32>()
        .map_err(|_| Refusal::Usage(format!("--depth needs a number (got {s})")))
}

fn parse_walk(s: &str) -> Result<u64, Refusal> {
    s.parse::<u64>()
        .map_err(|_| Refusal::Usage(format!("--walk needs a number (got {s})")))
}

fn take_positional(path: &mut Option<String>, token: String) -> Result<(), Refusal> {
    if path.is_some() {
        return Err(Refusal::Usage("only one PATH".into()));
    }
    *path = Some(token);
    Ok(())
}

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
                c.flag_vals,
            );
            print!("{}", render_show(&res));
            print!("{}", aspectus::facts::inventory(&res));
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
    let locus = aspectus::overview::resolve_locus(&args.path);
    let cfg = resolve(
        args.user_home_override.as_deref(),
        args.caller.as_deref(),
        args.flag_vals,
    );
    let depth = cfg
        .won
        .get("depth")
        .and_then(|(v, _)| v.parse().ok())
        .unwrap_or(2);
    let walk_bound: u64 = cfg
        .won
        .get("walk")
        .and_then(|(v, _)| v.parse().ok())
        .unwrap_or(10_000);
    let abs = aspectus::overview::absolute_root(&locus).map_err(|e| map_io(&locus, e))?;
    let (order, cols) = resolve_look(&cfg)?;
    let map = match cfg.won.get("furniture") {
        Some((rules, _)) => aspectus::furniture::Map::with_config(rules),
        None => aspectus::furniture::Map::shipped(),
    };
    let view = aspectus::furniture::View {
        show_all: args.show_all,
        inspect: args.inspect,
    };
    let kinds = match cfg.won.get("kinds") {
        Some((rules, _)) => aspectus::kind::Map::with_config(rules),
        None => aspectus::kind::Map::shipped(),
    };
    let one_fs = match cfg.won.get("one-fs").map(|(v, _)| v.as_str()) {
        None | Some("on") => true,
        Some("off") => false,
        Some(v) => return Err(ShowErr::Other(format!("one-fs is on or off (got {v})"))),
    };
    let read_budget: u64 = cfg
        .won
        .get("reads")
        .and_then(|(v, _)| v.parse().ok())
        .unwrap_or(64 * 1024 * 1024);
    let mut walk = aspectus::n_level::WalkBudget::new(walk_bound);
    let mut ctx = aspectus::n_level::LookCtx::new(
        &map,
        &view,
        &kinds,
        cols.line_fmt == aspectus::columns::LineFmt::NonBlank,
        one_fs,
        read_budget,
    );
    let mut tree = aspectus::n_level::gather(&abs, depth, &mut walk, &mut ctx)
        .map_err(|e| map_io(&locus, e))?;
    if cols.heat || order.key == aspectus::sort::Key::Heat || order.recency_git {
        let half_life: f64 = cfg
            .won
            .get("heat.half-life")
            .and_then(|(v, _)| v.parse().ok())
            .unwrap_or(aspectus::heat::DEFAULT_HALF_LIFE);
        aspectus::heat::annotate(&mut tree, &abs, half_life);
    }
    let lines: usize = cfg
        .won
        .get("lines")
        .and_then(|(v, _)| v.parse().ok())
        .unwrap_or(80);
    let mut why = Vec::new();
    if walk.furniture_hidden > 0 {
        why.push(format!(
            "furniture: {} names as parent-line state, not children",
            walk.furniture_hidden
        ));
    }
    if walk.tripped {
        why.push(format!(
            "walk: bound {walk_bound} reached; some dirs not expanded, marked [walk bound]"
        ));
    }
    // The header is two lines (stamp, then root — overview invariants);
    // the tree keeps the rest, the root line counted inside its budget.
    let tree_budget = if lines == 0 { 0 } else { (lines - 1).max(1) };
    if lines > 0 {
        why.push(format!(
            "header: stamp line costs 1 of --lines {lines}; {tree_budget} left (root line included)"
        ));
    }
    aspectus::n_level::apply_budget(&mut tree, tree_budget, &order, &mut why);
    aspectus::sort::apply(&mut tree, &order);
    if args.explain {
        for line in &why {
            let _ = writeln!(io::stderr(), "{line}");
        }
    }
    let root = abs.to_string_lossy();
    let now = std::time::SystemTime::now();
    let stamp = aspectus::overview::stamp_utc(now);
    let mut cols = cols;
    cols.now = now
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    print!(
        "{}",
        aspectus::columns::render(&root, &tree, args.color.active(), &stamp, &cols)
    );
    Ok(())
}

/// Selection and order from the resolved caller stack (design/columns.md,
/// design/sort.md). Refusals name the class and the menu.
fn resolve_look(
    cfg: &aspectus::config::Resolved,
) -> Result<(aspectus::sort::Order, aspectus::columns::Cols), ShowErr> {
    use aspectus::columns::{Cols, LineFmt, SizeFmt, TimeFmt};
    use aspectus::sort::{self, KeyErr, Order};

    let (sort_val, sort_layer) = cfg
        .won
        .get("sort")
        .map(|(v, l)| (v.as_str(), *l))
        .unwrap_or(("recency", "defaults"));
    let (key, rev) = sort::parse_key(sort_val).map_err(|e| {
        ShowErr::Other(match e {
            KeyErr::Unbuilt(k) => format!(
                "sort key not built yet: {k}\n  built keys: {}\n  next: aspectus help",
                sort::BUILT.join(", ")
            ),
            KeyErr::Unknown(k) => format!(
                "not a sortable fact: {k}\n  built keys: {}\n  next: aspectus help",
                sort::BUILT.join(", ")
            ),
        })
    })?;
    let dotfiles_first = match cfg.won.get("dotfiles-first").map(|(v, _)| v.as_str()) {
        None | Some("off") => false,
        Some("on") => true,
        Some(v) => {
            return Err(ShowErr::Other(format!(
                "dotfiles-first is on or off (got {v})"
            )))
        }
    };
    let recency_git = match cfg.won.get("recency-source").map(|(v, _)| v.as_str()) {
        None | Some("mtime") => false,
        Some("git") => true,
        Some(v) => {
            return Err(ShowErr::Other(format!(
                "recency-source is mtime or git (got {v})"
            )))
        }
    };
    let order = Order {
        key,
        rev,
        dotfiles_first,
        recency_git,
    };

    let col_state = |key: &str| -> Result<&str, ShowErr> {
        match cfg.won.get(key).map(|(v, _)| v.as_str()) {
            None => Ok("quiet"),
            Some(v @ ("on" | "off" | "quiet")) => Ok(v),
            Some(v) => Err(ShowErr::Other(format!(
                "{key} is on, off, or quiet (got {v})"
            ))),
        }
    };
    let size_state = col_state("columns.size")?;
    let mtime_state = col_state("columns.mtime")?;
    let lc_state = col_state("columns.line-count")?;
    let heat_state = col_state("columns.heat")?;
    // An explicitly asked sort key implies its column (the order is a
    // claim; the evidence belongs on the line) — unless the caller said
    // off. The recency *default* does not: position already carries the
    // signal, the value stays quiet (design/sort.md, design/columns.md).
    let explicit_sort = sort_layer != "defaults";
    let implied = |k: sort::Key, state: &str| {
        explicit_sort && order.key == k && state != "off"
    };
    let size = size_state == "on" || implied(sort::Key::Size, size_state);
    let mtime = mtime_state == "on" || implied(sort::Key::Mtime, mtime_state);
    let line_count = lc_state == "on" || implied(sort::Key::LineCount, lc_state);
    let heat = heat_state == "on" || implied(sort::Key::Heat, heat_state);

    let size_fmt = match cfg.won.get("format.size").map(|(v, _)| v.as_str()) {
        None | Some("human") => SizeFmt::Human,
        Some("bytes") => SizeFmt::Bytes,
        Some(v) => {
            return Err(ShowErr::Other(format!(
                "format.size is human or bytes (got {v}; log not built yet)"
            )))
        }
    };
    let mtime_fmt = match cfg.won.get("format.mtime").map(|(v, _)| v.as_str()) {
        None | Some("iso-8601") => TimeFmt::Iso8601,
        Some("epoch") => TimeFmt::Epoch,
        Some(v) => {
            return Err(ShowErr::Other(format!(
                "format.mtime is iso-8601 or epoch (got {v}; rfc-3339, pattern, signa not built yet)"
            )))
        }
    };
    let line_fmt = match cfg.won.get("format.line-count").map(|(v, _)| v.as_str()) {
        None | Some("physical") => LineFmt::Physical,
        Some("non-blank") => LineFmt::NonBlank,
        Some(v) => {
            return Err(ShowErr::Other(format!(
                "format.line-count is physical or non-blank (got {v}; signa not built yet)"
            )))
        }
    };
    Ok((
        order,
        Cols {
            line_count,
            size,
            mtime,
            heat,
            size_fmt,
            mtime_fmt,
            line_fmt,
            now: 0,
        },
    ))
}

fn map_io(path: &Path, e: std::io::Error) -> ShowErr {
    if e.kind() == std::io::ErrorKind::NotFound {
        ShowErr::NotFound(path.display().to_string())
    } else {
        ShowErr::Other(format!("{}: {e}", path.display()))
    }
}
