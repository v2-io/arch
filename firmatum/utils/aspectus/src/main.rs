use std::env;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use std::collections::BTreeMap;

use aspectus::color::Mode as ColorMode;
use aspectus::config::{
    CALLER_FLAG, DEFAULTS_TOML, boolish, format_val, old_columns_note, render_show, resolve,
};

/// One source: every accepted flag/verb is named here and printed in help.
const COMMANDS: &[(&str, &str)] = &[
    ("help, -h, --help", "this page (includes the version line)"),
    (
        "version, -v, --version",
        "one line: name + semver; +sha[.dirty] if not a tagged release, \
         and the build's UTC time",
    ),
    (
        "config",
        "show layers, what won, the effective [layout] and maps",
    ),
    (
        "config defaults",
        "print the embedded defaults.toml (stdout, exit 0)",
    ),
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
        "line budget for the whole look (default 80; 0 = no limit). \
         A non-empty root's honest floor is 3 lines — stamp, the root's \
         facts/census, path — plus a config-drift line when anything \
         differs from the built-in defaults; 1 and 2 overshoot rather \
         than omit",
    ),
    (
        "--walk N",
        "stat and expand at most N names; names past the bound still \
         count in censuses, and a cut dir says [walk bound] \
         (default 10000; 0 = no bound)",
    ),
    (
        "--explain-budget",
        "how lines and the walk were spent, on stderr (header lines — \
         stamp, config drift, root facts, column headings — charge --lines)",
    ),
    (
        "--show-all",
        "list furniture names (.git, target/, …) as ordinary children",
    ),
    (
        "--inspect KIND",
        "list furniture of one kind as children (git, build, …); repeatable. \
         A kind with nothing of that kind in this look says so on stderr",
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
    (
        "--format text|json",
        "the same look, serialized instead of drawn: sizes in bytes, \
         times iso-8601 UTC, every glyph-mark a field (denied, walk_bound, \
         censuses, truncated). Text stays default",
    ),
];

fn help_page() -> String {
    let mut out = format!(
        "{ver}\n\
         \n\
         usage: aspectus [PATH ...]\n\
                aspectus help\n\
                aspectus version\n\
                aspectus config\n\
                aspectus config defaults\n\
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
         Several paths are one look, not several: the locus becomes their\n\
         common ancestor, the named paths are the picture, and --depth\n\
         counts from each of them (so --depth 4 on four sibling volumes\n\
         means N generations under each; the ancestor chain spends\n\
         none). Their unselected siblings do not vanish -- each connective\n\
         level keeps them as one typed remainder line. Shell brace\n\
         expansion hands us plain paths, so `dir/{{a,b,c}}` just works. A\n\
         path that does not exist is confessed on stderr and dropped; the\n\
         rest of the ask is still a look.\n\
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
         (children and grandchildren), then it exits. The header is the\n\
         UTC time of the look, then every effective setting that differs\n\
         from the built-in defaults with its source (depth = 3 (user-home)\n\
         · --lines 200 (flag)) — absent when nothing differs — then the\n\
         root's own facts when it has any (heat, [git: ...], [has: ...]),\n\
         then the bare absolute root directly above its children.\n\
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
           aspectus config defaults\n\
           aspectus\n\
           aspectus PATH\n\
           aspectus --depth 3 --lines 120 PATH\n\
           aspectus --lines 200 --depth 4 asf/{01-aat-core,02-tst-core}\n\
                                       (several paths: one look of their\n\
                                        common ancestor, depth from each)\n\
           aspectus --walk 500 PATH    (huge tree: expand less, count all)\n\
           aspectus --inspect git PATH (see inside .git as ordinary children;\n\
                                        a submodule's gitlink .git instead\n\
                                        names its gitdir)\n\
           aspectus --format json PATH (the same look, as data)\n\
         \n\
         Non-binary files show a line count in a 12-cell field (binary\n\
         shows none, never 0); type is stat \u{2192} magic \u{2192} shebang \u{2192}\n\
         suffix (the `[kinds]` table; legacy key `kinds`: `SUFFIX:text|binary`,\n\
         `!SUFFIX` to drop) \u{2192} sniff. Census buckets stay by suffix unless\n\
         `format.census = minor|major`. An unexpanded directory\n\
         carries a census of what it held — [dir\u{d7}3 \u{2248}120f \u{b7} md\u{d7}31] — with\n\
         subdirectories as containers whose deep file-count (mass) leads,\n\
         and the subtree's text lines in the `lines` column (\u{2248}  61.2K /\n\
         ~   5.0M / \u{2265} 434.0K): a glance calibrates how much has not been\n\
         seen. Below 10,000 the value is exact (`1\u{b7}099.`); at and above,\n\
         three significant digits and a scale letter. \u{2248} is an exact count\n\
         grouped for the eye; ~ is this walk's estimate; \u{2265} marks a floor;\n\
         a single concealed name shows the name. The look reads file\n\
         content only up to a budget (config `reads`, bytes); past it, deep\n\
         line totals are estimated from sizes and marked ~ (this walk's\n\
         estimate, not a property of the directory) and per-file counts\n\
         are omitted \u{2014} the glance stays fast, and says how it degraded.\n\
         \n\
         Inside a git repo each visible line carries the aliveness cluster\n\
         `score \u{b7} age` under the `heat \u{b7} age` heading: the score is\n\
         commit-decay heat (git-heat's model, half-life 7 commits via\n\
         config `heat.half-life`) on a 0\u{2013}~2 scale \u{2014} not a size \u{2014}\n\
         counting in commits; the age is the mtime delta, counting in\n\
         wall-clock. Two clocks, one glance-stop. The score decays in\n\
         commits behind *that repo's* HEAD, so it is comparable within a\n\
         repo, not across repos -- at a multi-repo root a dormant repo's\n\
         concentrated last commits can outscore a busy one; the paired\n\
         age (and the recency sort) is the cross-repo signal. A line\n\
         git knows but does not score still carries its age in the\n\
         cluster (` \u{b7} 6m ago`) -- the score absent, never faked.\n\
         Outside git, no heat is claimed. `--sort heat` orders by it;\n\
         config `recency-source = git` makes the default recency sort use\n\
         git last-touch where known. The same log pass carries the sha\n\
         facts: columns.initial-sha / columns.latest-sha = on add the\n\
         commit that introduced a file and the one that last touched it\n\
         (format short / h~n / full; H~N counts commits behind HEAD);\n\
         absent outside git or past the log window, never guessed.\n\
         \n\
         Symlinked directories are followed and recursed like real ones\n\
         (facts are the target's; `-> target` says how it got here). A\n\
         cycle prints [cycle] instead of hanging. The walk stays on the\n\
         starting filesystem; a mount point shows [other fs] and stops\n\
         there unless --no-one-fs.\n\
         \n\
         Inside a git work tree, gitignored contents stay out of the look\n\
         and out of every aggregate -- the repo already declared them not\n\
         the project -- while presence still shows: an ignored directory\n\
         keeps its line, dimmed on a TTY, unexpanded and unweighed; ignored\n\
         files appear only as a typed remainder (ignored\u{d7}3). The rules\n\
         are git's own -- nested .gitignore files, negations, info/exclude,\n\
         the global core.excludesFile -- and a tracked file matching an\n\
         ignore pattern lists normally (tracked beats ignored, as in git).\n\
         Furniture fates apply first; --show-all restores ignored\n\
         contents, marks kept. Outside a repo a .gitignore is just a file.\n\
         \n\
         Git status sits in one cell to the left of the tree, blank when\n\
         clean, absent entirely when the look contains no repo. Worktree\n\
         wins when index and worktree differ. --sort git is not built.\n\
           \u{2298}  gitignored     M  modified     A  added\n\
           \u{2047}  untracked      R  renamed      U  unmerged\n\
           D  deleted        C  copied       T  typechange\n\
         \n\
         A real sequence of names collapses to its pattern:\n\
         output-[001-047].bak  (44 files) -- one line, exact count; a\n\
         count below the span means gaps. Only genuine series fuse: at\n\
         least 5 members (config globify.min), exactly one varying\n\
         digit run, uniform zero-padding, all files or all dirs;\n\
         important files stay listed by name; collapsed dirs are never\n\
         expanded. globify = off (or --show-all) restores every name.\n\
         \n\
         A directory line can borrow the title its README gives it\n\
         (config readme-title = on; off by default): the first heading\n\
         of the first important-files match, quoted after the name --\n\
         truthful or silent, from a bounded head-peek, never a\n\
         placeholder, and never repeating the folder name.\n\
         \n\
         Well-known names are furniture: state on their parent line, not\n\
         children of the look. A git work tree does not list .git — the\n\
         directory line says what git is here ([git: remote<…> br<main>\n\
         @sha dirty<N>], local facts only, no network); build debris\n\
         (target/, __pycache__, …) folds into the [has: …] spot — a\n\
         claim about contents, exactly what the evidence supports.\n\
         Hidden names are not counted as children; the has-spot is what\n\
         says they are here. The map is glob-based and extendable from config\n\
         (the `[furniture]` table in `aspectus config defaults`; legacy key\n\
         `furniture`: `PATTERN[:KIND[:hide|omit|mark]]`, comma-separated;\n\
         `!PATTERN` or `\"PATTERN\" = \"!\"` drops a default row).\n\
         \n\
         The look never lies by omission: an unexpanded directory carries a\n\
         census, a walk-bound cut keeps the full name count and says\n\
         [walk bound], a count that hides an unreadable place is marked \u{2265},\n\
         and a directory it could not read says [denied] rather than\n\
         printing as empty.\n\
         \n\
         Quiet facts appear only when they surprise. Size speaks on a\n\
         magnitude outlier among its siblings; mtime when recent (within a\n\
         day); permissions when odd for its level (special bits like setuid\n\
         always speak); owner when neither you nor the level's majority;\n\
         the file-kind word when a file's countable class differs from its\n\
         level's plurality (a PDF among the .md says doc; a .toml does not).\n\
         Usual is silent: 644 among 644s,\n\
         owner-you, an old mtime print nothing. Sibling norms (size,\n\
         perms, owner, kind) come from the full level, so --lines cannot\n\
         flicker them; mtime alone is an absolute window -- recent vs\n\
         now, not vs siblings -- so a freshly-made tree speaks it on\n\
         every line. One dial scales the\n\
         statistical thresholds: config quiet.sensitivity (default 1.0;\n\
         higher = harder to surprise), per-fact quiet.sensitivity.size /\n\
         .mtime. Convention laws (setuid, root-owner) never scale.\n\
         \n\
         Important files (config `important = [\"README*\", …]` or the\n\
         legacy comma-separated globs; default README*, AGENTS.md,\n\
         CLAUDE.md; !PATTERN drops one) survive a tight --lines ahead of\n\
         plain files — dirs, then important, then the rest, sort-key order\n\
         within each tier. They claim no glyph and no position: with budget\n\
         to spare the look is unchanged.\n\
         \n\
         --format json emits the same look as one JSON document: same\n\
         walk, same budget, same censuses and marks -- never deeper or\n\
         wider for being machine-shaped. Sizes are bytes, times iso-8601\n\
         UTC, censuses objects, denied/walk_bound booleans, a top-level\n\
         `truncated` says whether any cut or denial occurred anywhere\n\
         (exit stays 0), and `config_drift` is an array of\n\
         {key, value, source} when anything differs from the built-in\n\
         defaults (omitted when nothing does). Quiet governs only the\n\
         drawn look: JSON carries the underlying facts (mode, uid,\n\
         mtime, ...) either way. Refusals in machine mode are JSON on\n\
         stderr.\n\
         \n\
         A dimmed headings line sits under the root path, right-aligned\n\
         over the fact columns it names (`lines   heat \u{b7} age`), so the\n\
         numbers below it are never bare. It appears only when fact\n\
         columns do, and costs one header line of the budget. mtime cells\n\
         default to the relative form (`2.2h ago`) \u{2014} one time register\n\
         with the heat cluster's age; config format.mtime = iso-8601 or\n\
         epoch restores the absolute spellings (JSON always iso-8601).\n\
         \n\
         Every look ends with a feedback footer on stderr (stdout stays\n\
         data, so pipes and `jq` never see it): this tool is critical\n\
         but new and unproven -- submit feedback, anomalies, issues, or\n\
         confusion (with the command and cwd) to the bottom of\n\
         arch/firmatum/utils/aspectus/inbox.md.\n\
         \n\
         Facts beyond the defaults (size, mtime, ...) have no flags of\n\
         their own; ask through config on the caller stack. Membership is\n\
         `[layout]` in `aspectus config defaults` — a fact in a position\n\
         list is shown, a fact in `quiet` speaks only on surprise, a fact\n\
         in no list renders nothing. `columns.size = on` still works for\n\
         this release (stderr names `[layout]`). format.mtime = epoch,\n\
         sort = -size (env: ASPECTUS_COLUMNS_SIZE, ...). Column values\n\
         align at computed tab-stops -- a function of the tree, never\n\
         terminal width, so two looks of the same tree stay diffable.\n\
         `aspectus config` lists every fact with its state, ask, format,\n\
         the effective `[layout]`, and the furniture/kinds/important maps.\n",
    );
    out
}

fn version_line() -> String {
    let ver = env!("CARGO_PKG_VERSION");
    let mut line = match option_env!("ASPECTUS_GIT_SHA") {
        // `sha` may carry a `.dirty` suffix — uncommitted state at build
        // time (a stale install was invisible without it, 2026-08-14).
        Some(sha) if !sha.is_empty() => format!("aspectus {ver}+{sha}"),
        _ => format!("aspectus {ver}"),
    };
    if let Some(epoch) = option_env!("ASPECTUS_BUILD_EPOCH")
        && let Ok(secs) = epoch.parse::<u64>()
        && secs > 0
    {
        let t = std::time::UNIX_EPOCH + std::time::Duration::from_secs(secs);
        line.push_str(&format!(" (built {})", aspectus::overview::stamp_utc(t)));
    }
    line
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
    /// `aspectus config defaults` — dump the embedded file, no layers.
    dump_defaults: bool,
}

struct ShowArgs {
    /// One path is the locus; several are a focus set over their common
    /// ancestor (design/focus.md §Multiple paths).
    paths: Vec<PathBuf>,
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
    let mut paths: Vec<String> = Vec::new();
    let mut help = false;
    let mut version = false;
    let mut config_cmd = false;
    let mut dump_defaults = false;
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
            paths.push(a);
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
                let v = args.next().ok_or_else(|| {
                    Refusal::Usage("--sort needs a key (recency, name, size)".into())
                })?;
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
            "--format" => {
                let v = args
                    .next()
                    .ok_or_else(|| Refusal::Usage("--format needs text or json".into()))?;
                parse_format(&v)?;
                flag_vals.insert("format".into(), v);
            }
            s if s.starts_with("--format=") => {
                let v = &s[9..];
                parse_format(v)?;
                flag_vals.insert("format".into(), v.to_string());
            }
            "--explain-budget" => explain = true,
            "--show-all" => show_all = true,
            "--inspect" => {
                let v = args.next().ok_or_else(|| {
                    Refusal::Usage("--inspect needs a kind (git, build, …)".into())
                })?;
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
            s => {
                if config_cmd && s == "defaults" && !dump_defaults && paths.is_empty() {
                    dump_defaults = true;
                } else {
                    paths.push(s.to_string());
                }
            }
        }
    }

    if help {
        return Ok(Cmd::Help);
    }
    if version {
        return Ok(Cmd::Version);
    }
    // An explicitly named config file that does not exist is a refusal,
    // not a silent `absent` layer — the caller pointed at something.
    if let Some(p) = &user_home_override
        && !p.exists()
    {
        return Err(Refusal::Usage(format!(
            "--config file not found: {}",
            p.display()
        )));
    }
    if config_cmd {
        return Ok(Cmd::Config(ConfigArgs {
            user_home_override,
            caller,
            flag_vals,
            dump_defaults,
        }));
    }

    // Arity is the whole rule: none = here, one = the locus (and the only
    // place a bare word could still be a mistyped verb), several = a focus
    // set (shell brace expansion hands us plain paths).
    let paths: Vec<PathBuf> = match paths.len() {
        0 => vec![PathBuf::from(".")],
        1 if !path_is_literal => vec![classify_positional(paths.remove(0))?],
        _ => paths.into_iter().map(PathBuf::from).collect(),
    };

    Ok(Cmd::Show(ShowArgs {
        paths,
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

fn parse_format(s: &str) -> Result<(), Refusal> {
    match s {
        "text" | "json" => Ok(()),
        _ => Err(Refusal::Usage(format!(
            "--format is text or json (got {s}; udon later, csv/yaml/tsv refused)"
        ))),
    }
}

fn classify_positional(token: String) -> Result<PathBuf, Refusal> {
    let p = Path::new(&token);
    if p.exists() || token.contains('/') || token.starts_with('.') {
        return Ok(PathBuf::from(token));
    }
    Err(Refusal::UnknownVerb(token))
}

/// Was json asked for on the surfaces we can read before (or despite) a
/// parse failure? Argv and env only — a config-file `format = json` still
/// gets the text refusal (recorded limitation, impl/json.md).
fn machine_mode() -> bool {
    let argv_json = {
        let mut args = env::args().skip(1).peekable();
        let mut hit = false;
        while let Some(a) = args.next() {
            if a == "--" {
                break;
            }
            if a == "--format=json"
                || (a == "--format" && args.peek().map(String::as_str) == Some("json"))
            {
                hit = true;
            }
        }
        hit
    };
    argv_json || env::var("ASPECTUS_FORMAT").as_deref() == Ok("json")
}

fn main() -> ExitCode {
    match parse_args(env::args().skip(1)) {
        Err(r) => {
            if machine_mode() {
                let _ = write!(
                    io::stderr(),
                    "{}",
                    aspectus::json::refusal(r.class(), r.token(), None)
                );
            } else {
                r.write_stderr();
            }
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
            if c.dump_defaults {
                print!("{DEFAULTS_TOML}");
                return ExitCode::SUCCESS;
            }
            let res = resolve(
                c.user_home_override.as_deref(),
                c.caller.as_deref(),
                c.flag_vals,
            );
            if let Some(note) = old_columns_note(&res) {
                let _ = writeln!(io::stderr(), "{note}");
            }
            print!("{}", render_show(&res));
            print!("{}", aspectus::facts::inventory(&res));
            feedback_footer();
            ExitCode::SUCCESS
        }
        Ok(Cmd::Show(args)) => match show(args) {
            Ok(()) => ExitCode::SUCCESS,
            Err(err) => {
                if machine_mode() {
                    let (class, tok) = match &err {
                        ShowErr::NotFound(p) => ("not found", p.as_str()),
                        ShowErr::Other(e) => ("error", e.as_str()),
                    };
                    let _ = write!(
                        io::stderr(),
                        "{}",
                        aspectus::json::refusal(class, tok, None)
                    );
                } else {
                    match &err {
                        ShowErr::NotFound(p) => {
                            let _ = writeln!(io::stderr(), "aspectus: not found {p}");
                        }
                        ShowErr::Other(e) => {
                            let _ = writeln!(io::stderr(), "aspectus: {e}");
                        }
                    }
                }
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
    let locus = aspectus::overview::resolve_locus(&args.paths[0]);
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
    // Several positional paths are a focus set over their common ancestor
    // (design/focus.md §Multiple paths). Bad paths are confessed and
    // dropped — the rest of the ask is still a serviceable look; only when
    // nothing is left is there no place to look at all.
    let mut focus = None;
    let mut abs = aspectus::overview::absolute_root(&locus).map_err(|e| map_io(&locus, e))?;
    if args.paths.len() > 1 {
        let mut sel = Vec::new();
        let mut missing = Vec::new();
        for p in &args.paths {
            let p = aspectus::overview::resolve_locus(p);
            match aspectus::overview::absolute_root(&p) {
                Ok(a) if a.exists() => sel.push(a),
                _ => missing.push(p.display().to_string()),
            }
        }
        if !missing.is_empty() {
            let _ = writeln!(
                io::stderr(),
                "aspectus: focus path not found ({}): {}",
                missing.len(),
                missing.join(", ")
            );
        }
        if sel.is_empty() {
            return Err(ShowErr::NotFound(args.paths[0].display().to_string()));
        }
        sel.sort();
        sel.dedup();
        let nested = aspectus::focus::drop_nested(&mut sel);
        if !nested.is_empty() {
            let _ = writeln!(
                io::stderr(),
                "aspectus: focus path already inside another ({}): {} — depth counts from the outer one",
                nested.len(),
                nested
                    .iter()
                    .map(|p| p.display().to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        if sel.len() > 1 {
            abs = aspectus::focus::common_ancestor(&sel);
            focus = Some(aspectus::focus::Focus { sel, depth });
        } else {
            // One survivor: today's behavior exactly, at that path.
            abs = sel.remove(0);
        }
    }
    let (order, cols) = resolve_look(&cfg)?;
    if let Some(note) = old_columns_note(&cfg) {
        let _ = writeln!(io::stderr(), "{note}");
    }
    let map = aspectus::furniture::Map::from_sourced(&cfg.furniture);
    // An unknown --inspect kind used to exit 0 silently — "the flags I
    // would use to debug the tool lie by being fine" (grok, 2026-08-14).
    // Refuse by name, with the menu.
    for k in &args.inspect {
        if !map.known_kinds().iter().any(|known| known == k) {
            return Err(ShowErr::Other(format!(
                "not a furniture kind this map knows: {k}\n  kinds: {}\n  next: aspectus help",
                map.known_kinds().join(", ")
            )));
        }
    }
    let view = aspectus::furniture::View {
        show_all: args.show_all,
        inspect: args.inspect,
    };
    let kinds = aspectus::filetype::Map::from_sourced(&cfg.kinds);
    let census_grain = match format_val(&cfg.won, "format.census") {
        None | Some("suffix") => aspectus::filetype::CensusGrain::Suffix,
        Some("minor") => aspectus::filetype::CensusGrain::Minor,
        Some("major") => aspectus::filetype::CensusGrain::Major,
        Some(v) => {
            return Err(ShowErr::Other(format!(
                "format.census is suffix, minor, or major (got {v})"
            )));
        }
    };
    let one_fs = match cfg.won.get("one-fs").map(|(v, _)| v.as_str()) {
        None => true,
        Some(v) => {
            boolish(v).ok_or_else(|| ShowErr::Other(format!("one-fs is on or off (got {v})")))?
        }
    };
    let read_budget: u64 = cfg
        .won
        .get("reads")
        .and_then(|(v, _)| v.parse().ok())
        .unwrap_or(64 * 1024 * 1024);
    let mut walk = aspectus::n_level::WalkBudget::new(walk_bound);
    // The important set is built pre-walk: readme-title borrows this exact
    // set (design/readme-title.md — one definition, the rows cannot drift).
    let important = aspectus::important::Set::from_sourced(&cfg.important);
    let title_on = match cfg.won.get("readme-title").map(|(v, _)| v.as_str()) {
        None => false,
        Some(v) => boolish(v)
            .ok_or_else(|| ShowErr::Other(format!("readme-title is on or off (got {v})")))?,
    };
    let mut ctx = aspectus::n_level::LookCtx::new(
        &map,
        &view,
        &kinds,
        cols.line_fmt == aspectus::columns::LineFmt::NonBlank,
        one_fs,
        read_budget,
    );
    ctx.census_grain = census_grain;
    if title_on {
        ctx.titles = Some(&important);
    }
    let mut tree = match &focus {
        Some(f) => aspectus::n_level::gather_focus(&abs, f, &mut walk, &mut ctx)
            .map_err(|e| map_io(&abs, e))?,
        None => aspectus::n_level::gather(&abs, depth, &mut walk, &mut ctx)
            .map_err(|e| map_io(&locus, e))?,
    };
    // Post-gather parallel phases (hardening 2026-08-14): deep mass over
    // cutoff subtrees, then git facets (porcelain per repo).
    aspectus::n_level::deep_phase(&mut tree, &abs, &mut ctx);
    // After the deep phase, so a folded sibling arrives at the remainder
    // carrying its real mass (`dir×9 ≈1.4Kf`) rather than a bare count —
    // the difference between compressing the context and cutting it.
    if focus.is_some() {
        aspectus::focus::fold_asides(&mut tree);
    }
    aspectus::n_level::hidden_phase(&mut tree, &abs);
    aspectus::git::annotate(&mut tree, &abs);
    if cols.heat
        || cols.intro_sha
        || cols.latest_sha
        || order.key == aspectus::sort::Key::Heat
        || order.recency_git
    {
        let half_life: f64 = cfg
            .won
            .get("heat.half-life")
            .and_then(|(v, _)| v.parse().ok())
            .unwrap_or(aspectus::heat::DEFAULT_HALF_LIFE);
        aspectus::heat::annotate(&mut tree, &abs, half_life);
    }
    // Importance (survival weight) and quiet (cold surprise) annotate the
    // pre-budget tree: sibling norms over the full statted level, so
    // `--lines` cannot flicker either decision.
    important.annotate(&mut tree);
    // Globify (design/globify.md): after importance (its exemption) and
    // the fact annotates the collapsed line aggregates, before quiet — the
    // one listee joins its level's sibling norms — and before the budget
    // spends: one series, one line.
    let globify_on = match cfg.won.get("globify").map(|(v, _)| v.as_str()) {
        None => true,
        Some(v) => {
            boolish(v).ok_or_else(|| ShowErr::Other(format!("globify is on or off (got {v})")))?
        }
    };
    if globify_on && !args.show_all {
        let min: usize = cfg
            .won
            .get("globify.min")
            .and_then(|(v, _)| v.parse().ok())
            .unwrap_or(5);
        aspectus::globify::apply(&mut tree, min.max(2));
    }
    let now = std::time::SystemTime::now();
    let now_secs = now
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    let dial = |k: &str| cfg.won.get(k).and_then(|(v, _)| v.parse::<f64>().ok());
    let dials = aspectus::quiet::Dials {
        base: dial("quiet.sensitivity").unwrap_or(1.0),
        size: dial("quiet.sensitivity.size"),
        mtime: dial("quiet.sensitivity.mtime"),
    };
    let kind_ask = match cfg.won.get("columns.filekind").map(|(v, _)| v.as_str()) {
        None | Some("quiet") => aspectus::quiet::KindAsk::Quiet,
        Some("on") => aspectus::quiet::KindAsk::On,
        Some("off") => aspectus::quiet::KindAsk::Off,
        Some(v) => {
            return Err(ShowErr::Other(format!(
                "columns.filekind is on, off, or quiet (got {v})"
            )));
        }
    };
    aspectus::quiet::annotate(
        &mut tree,
        &dials,
        &aspectus::quiet::Caller::detect(),
        kind_ask,
        now_secs,
    );
    let lines: usize = cfg
        .won
        .get("lines")
        .and_then(|(v, _)| v.parse().ok())
        .unwrap_or(80);
    let mut why = Vec::new();
    if let Some(f) = &focus {
        why.push(format!(
            "focus: {} selected paths, depth {depth} from each; \
             root is their common ancestor {}; unselected siblings on the \
             connective levels folded to one typed remainder each",
            f.sel.len(),
            abs.display()
        ));
    }
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
    // The header is stamp, config-drift (when anything differs from the
    // built-in defaults), the root's facts when it has any, then the bare
    // root path (overview invariants). The tree keeps the rest; the root
    // path line is counted inside its budget.
    let mut cols = cols;
    cols.now = now_secs;
    let drift = aspectus::config::drift(&cfg, args.caller.as_deref());
    let drift_text = aspectus::config::drift_line(&drift);
    let header = 1
        + usize::from(!drift_text.is_empty())
        + usize::from(!aspectus::columns::root_facts_line(&tree, &cols).is_empty());
    let tree_budget = if lines == 0 {
        0
    } else {
        lines.saturating_sub(header).max(1)
    };
    // The headings line costs a header line only when it actually renders,
    // which depends on what survives the budget — so allocate, look, and
    // when headings landed, re-allocate one line tighter (the tighter tree
    // either still earns its headings line, total exactly --lines, or
    // loses every column and comes in one under — never over).
    let mut why_alloc = Vec::new();
    let pre = tree.clone();
    aspectus::n_level::apply_budget(&mut tree, tree_budget, &order, &mut why_alloc);
    let mut headed = aspectus::columns::headings_expected(&tree, &cols);
    if lines > 0 && headed && tree_budget > 1 {
        let tighter = tree_budget - 1;
        let mut t2 = pre;
        why_alloc.clear();
        aspectus::n_level::apply_budget(&mut t2, tighter, &order, &mut why_alloc);
        tree = t2;
        headed = aspectus::columns::headings_expected(&tree, &cols);
    }
    if lines > 0 {
        why.push(format!(
            "header: {} line(s) (stamp, config drift, root facts, column headings as rendered) \
             cost of --lines {lines} (root line inside the tree's share)",
            header + usize::from(headed)
        ));
    }
    why.append(&mut why_alloc);
    why.push("footer: feedback solicitation rides on stderr, outside --lines".into());
    aspectus::sort::apply(&mut tree, &order);
    if let Some(f) = &focus {
        let lost = aspectus::focus::unlisted(&tree, &abs, &f.sel);
        if !lost.is_empty() {
            let _ = writeln!(
                io::stderr(),
                "aspectus: --lines {lines} could not give every focus path a line ({}): {} — \
                 they are inside the remainder censuses; a larger --lines lists them",
                lost.len(),
                lost.iter()
                    .map(|p| p.display().to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
    }
    if args.explain {
        for line in &why {
            let _ = writeln!(io::stderr(), "{line}");
        }
    }
    // A no-op --inspect: the flag asked to look inside a kind that this
    // look does not have. Silence was the one confession that didn't
    // confess (hallway 2026-08-22). Exit unchanged — the look succeeded.
    for k in &view.inspect {
        if !aspectus::n_level::tree_claims_kind(&tree, k) {
            let _ = writeln!(
                io::stderr(),
                "aspectus: --inspect {k}: nothing of that kind in this look"
            );
        }
    }
    let root = abs.to_string_lossy();
    let stamp = aspectus::overview::stamp_utc(now);
    let machine = match cfg.won.get("format").map(|(v, _)| v.as_str()) {
        None | Some("text") => false,
        Some("json") => true,
        Some(v) => {
            return Err(ShowErr::Other(format!(
                "format is text or json (got {v}; udon later, csv/yaml/tsv refused)"
            )));
        }
    };
    if machine {
        print!("{}", aspectus::json::render(&root, &stamp, &tree, &drift));
        feedback_footer();
        return Ok(());
    }
    print!(
        "{}",
        aspectus::columns::render(
            &root,
            &tree,
            args.color.active(),
            &stamp,
            &cols,
            &drift_text
        )
    );
    feedback_footer();
    Ok(())
}

/// The steward's feedback footer (verbatim, overview.rs), on **stderr**:
/// it is the tool speaking about itself, not the look of the place, so
/// stdout stays data (Joseph, 2026-08-22; until then it rode stdout as a
/// dimmed last line and as a JSON `feedback` field). Harnesses merge the
/// streams, so agents still see it; pipes and `jq` no longer do.
fn feedback_footer() {
    let _ = writeln!(io::stderr(), "\n{}", aspectus::overview::FEEDBACK_FOOTER);
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
        None => false,
        Some(v) => boolish(v)
            .ok_or_else(|| ShowErr::Other(format!("dotfiles-first is on or off (got {v})")))?,
    };
    let recency_git = match cfg.won.get("recency-source").map(|(v, _)| v.as_str()) {
        None | Some("mtime") => false,
        Some("git") => true,
        Some(v) => {
            return Err(ShowErr::Other(format!(
                "recency-source is mtime or git (got {v})"
            )));
        }
    };
    let order = Order {
        key,
        rev,
        dotfiles_first,
        recency_git,
    };

    let col_state = |key: &str| -> Result<aspectus::columns::State, ShowErr> {
        match cfg.won.get(key).map(|(v, _)| v.as_str()) {
            None => Ok(aspectus::columns::State::Quiet),
            Some(v) => aspectus::columns::State::parse(v)
                .ok_or_else(|| ShowErr::Other(format!("{key} is on, off, or quiet (got {v})"))),
        }
    };
    let on_off = |key: &str| -> Result<bool, ShowErr> {
        match cfg.won.get(key).map(|(v, _)| v.as_str()) {
            None | Some("off") => Ok(false),
            Some("on") => Ok(true),
            Some(v) => Err(ShowErr::Other(format!("{key} is on or off (got {v})"))),
        }
    };
    let intro_sha = on_off("columns.initial-sha")?;
    let latest_sha = on_off("columns.latest-sha")?;
    let size_state = col_state("columns.size")?;
    let mtime_state = col_state("columns.mtime")?;
    let lc_state = col_state("columns.line-count")?;
    let heat_state = col_state("columns.heat")?;
    let perms = col_state("columns.permissions")?;
    let owner = col_state("columns.owner")?;
    // An explicitly asked sort key implies its column (the order is a
    // claim; the evidence belongs on the line) — unless the caller said
    // off. The recency *default* does not: position already carries the
    // signal, the value stays quiet (design/sort.md, design/columns.md).
    // A copied defaults.toml restating `sort = "recency"` keeps the
    // defaults layer as source (equal values don't promote), so it does
    // not lift either.
    use aspectus::columns::State;
    let explicit_sort = sort_layer != "defaults";
    let implied =
        |k: sort::Key, state: State| explicit_sort && order.key == k && state != State::Off;
    let lift = |k: sort::Key, state: State| {
        if implied(k, state) { State::On } else { state }
    };
    let size = lift(sort::Key::Size, size_state);
    let mtime = lift(sort::Key::Mtime, mtime_state);
    let line_count = lc_state == State::On || implied(sort::Key::LineCount, lc_state);
    let heat = heat_state == State::On || implied(sort::Key::Heat, heat_state);

    let size_fmt = match format_val(&cfg.won, "format.size") {
        None | Some("human") => SizeFmt::Human,
        Some("bytes") => SizeFmt::Bytes,
        Some(v) => {
            return Err(ShowErr::Other(format!(
                "format.size is human or bytes (got {v}; log not built yet)"
            )));
        }
    };
    let mtime_fmt = match format_val(&cfg.won, "format.mtime") {
        None | Some("relative") => TimeFmt::Relative,
        Some("iso-8601") => TimeFmt::Iso8601,
        Some("epoch") => TimeFmt::Epoch,
        // Unbuilt spellings (design/phenom-format.md; starred in the
        // shipped file 2026-08-23). An unbuilt default must not refuse
        // the look — relative is the built spelling. An explicit ask of
        // a truly unknown word still refuses.
        Some("signa") | Some("rfc-3339") | Some("pattern") => TimeFmt::Relative,
        Some(v) => {
            return Err(ShowErr::Other(format!(
                "format.mtime is relative, iso-8601, or epoch (got {v}; rfc-3339, pattern, signa not built yet)"
            )));
        }
    };
    let line_fmt = match format_val(&cfg.won, "format.line-count") {
        None | Some("physical") => LineFmt::Physical,
        Some("non-blank") => LineFmt::NonBlank,
        Some(v) => {
            return Err(ShowErr::Other(format!(
                "format.line-count is physical or non-blank (got {v}; signa not built yet)"
            )));
        }
    };
    let sha_fmt = |key: &str| -> Result<aspectus::columns::ShaFmt, ShowErr> {
        use aspectus::columns::ShaFmt;
        match cfg.won.get(key).map(|(v, _)| v.as_str()) {
            None | Some("short") => Ok(ShaFmt::Short),
            Some("h~n") | Some("H~N") => Ok(ShaFmt::HN),
            Some("full") => Ok(ShaFmt::Full),
            Some(v) => Err(ShowErr::Other(format!(
                "{key} is short, h~n, or full (got {v})"
            ))),
        }
    };
    let intro_fmt = sha_fmt("format.initial-sha")?;
    let latest_fmt = sha_fmt("format.latest-sha")?;
    let owner_fmt = match format_val(&cfg.won, "format.owner") {
        None | Some("name") => aspectus::columns::OwnerFmt::Name,
        Some("id") => aspectus::columns::OwnerFmt::Id,
        Some(v) => {
            return Err(ShowErr::Other(format!(
                "format.owner is name or id (got {v})"
            )));
        }
    };
    Ok((
        order,
        Cols {
            line_count,
            size,
            mtime,
            perms,
            owner,
            heat,
            intro_sha,
            latest_sha,
            size_fmt,
            mtime_fmt,
            line_fmt,
            owner_fmt,
            intro_fmt,
            latest_fmt,
            now: 0,
            far_left: cfg
                .layout
                .get("layout.far-left")
                .map(|(v, _)| v.clone())
                .unwrap_or_default(),
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
