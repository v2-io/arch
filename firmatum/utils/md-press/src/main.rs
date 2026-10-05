use md_press::BreakAction;
use md_press::plan::{Detail, Item, Plan, Tally, plan_file};
use std::collections::HashMap;
use std::io::Read;
use std::path::PathBuf;

fn usage(code: i32) -> ! {
    let text = r#"md-press — canonicalize markdown to house standards

By default md-press only REPORTS: what it would change in each file, and a
short code for every decision, so any one of them can be flipped. Nothing is
written unless you pass --fix.

usage: md-press [options] <file.md>...          report; write nothing
       md-press --fix [options] <file.md>...    edit those files in place
       md-press --fix --toggle CODES <file.md>  edit, with those decisions flipped
       md-press [options] -                     report on stdin
       md-press --fix [options] -               stdin to stdout, edited

  --check           The default; accepted for scripts written before it was.
  --fix             Write the changes. Without --toggle, every decision is
                    taken at its default — the same plan the report shows.
  --toggle CODES    Flip the decisions with these codes (comma- or
                    space-separated; repeatable). With --fix the flipped plan
                    is written; without it, the report shows the flipped plan.
  --no-math         Skip the math pass (see MATH below), e.g. for a fully
                    deterministic check, or where ollama is not installed.
  --math[=MODEL]    The math pass is on by default; this flag names a
                    different local model (default llama3.2:3b), or turns
                    the pass back on over MD_PRESS_MATH=off.
  -q, --quiet       Report: list only the files that would change (exit code
                    as usual). --fix: errors only.
  -v, --verbose     Report: also list each line break inside joined
                    paragraphs. Both modes: list what the summaries count.
  --no-log          Don't add this run to the local usage log (see LOG).
  --force           Format even files excluded by a .md-pressignore.
  --allow-udon      Format .udon files too. Off by default, and the default
                    is the recommendation — see FOREIGN LANGUAGES below.
                    Deliberately NOT covered by --force, so that overriding
                    verbatim exclusions does not also disable this.
  --explain         Debug: print every consulted break with its calibrated
                    probability and named feature values on stderr.
  -V, --version     Print the version (0.2.0: report by default, --fix, codes).
  -h, --help        Show this.

THE REPORT. Per file that would change: first what md-press is confident
about — paragraphs to join (one code per paragraph), breaks to keep and
mark, math to convert, blank lines to add around $$ — then the decisions it
is unsure about, each with the alternative a flip gives. Flipping a join
keeps that break and ends the line with two spaces, so it renders as a line
break and is never asked about again; flipping a keep joins the lines.
Either way the file itself carries the decision. Flipping a math change
puts the original text back, and that one is NOT remembered: the next run
proposes it again. If you flip the same thing more than once and it looks
like a bug rather than a judgment call, please report it (see the end of
the report).

CODES are derived from the file's exact contents, its path, this build of
md-press, and the options that change the plan. They stop matching as soon
as any of those change — a stale or mistyped code is an error, and nothing
is written. So a code always means exactly what the report showed.

Environment: MD_PRESS_MATH=off (or 0/no) disables the math pass for every
call that doesn't pass --math; MD_PRESS_MATH=MODEL picks the model. Flags
win over the environment. Model proposals are cached under
$XDG_CACHE_HOME/md-press/math (default ~/.cache), keyed by model, prompt, and
text, so re-runs and reports on text already seen are instant and
repeatable; every safety check still re-runs on each use. MD_PRESS_CACHE=off
disables it; deleting the directory is always safe.

LOG: each run that reports or writes changes appends what it decided — the
unsure decisions with their context, which codes were flipped, and counts —
to $XDG_DATA_HOME/md-press/usage.jsonl (default ~/.local/share). It stays on
this machine and exists to improve md-press: flips are labels for retraining
the break classifier and for finding math bugs. --no-log or MD_PRESS_LOG=off
skips it; deleting the file is always safe.

STDIN IS UNGUARDED. There is no filename to read, so neither a
.md-pressignore exclusion nor the .udon guard can be consulted (the
render-equality gate below does run). So `md-press --fix - < FILE` answers
"what would the engine do to these bytes", not "what would md-press write
to this file" — a file that file mode would skip comes back reformatted. To
see what would happen to a file, report on it by name.

EXCLUSIONS: a .md-pressignore file (gitignore syntax) at or above a file marks
it as not-for-formatting, and is honoured even when the file is named
explicitly — because the realistic accident is an agent running
`md-press --fix $(find . -name '*.md')` over verbatim material. Use it for raw
transcripts, provenanced copies, and frozen archaeology: reformatting those
is render-equivalent and still wrong, and no automatic check can tell,
because nothing about the rendered document changed.

FOREIGN LANGUAGES: .udon files are skipped by default, even when named
explicitly, because UDON is not markdown and the safety gate below cannot
tell. Three reasons, each reproduced rather than argued (see
UDON-ASSESSMENT-2026-07-29.md): UDON's text law makes the newline literal
text content rather than collapsible whitespace, so joining prose lines edits
the value; a bare attribute value runs to end of line, so a join can swallow
every following :key into one attribute holding garbage; and !:lang: verbatim
blocks are invisible to this tool's parser and get flattened. All three
survive the render check, because a mangled UDON line renders as ordinary
markdown text. --allow-udon proceeds anyway; --force deliberately does not, so
overriding verbatim exclusions cannot quietly disable this too.

WHAT IT CHANGES: each prose paragraph that is split across several lines
becomes one long line — including paragraphs inside list items, blockquotes,
and footnotes. Tables, code (fenced and inline), YAML frontmatter, math,
wikilinks, HTML, and line breaks that carry meaning are left alone; so are
display-math ($$) lines and lines that look like table rows. Where a line
break might be deliberate, a trained classifier decides, with the
probability that the break was meant shown in the report.

WHY UNWRAPPING IS SAFE: the source text changes, but the rendered document
must not. Before writing any file, md-press re-parses its own output and
compares the rendered result against the original (setting aside the
two-space marks it adds on purpose). If they differ at all — which would
mean a bug in md-press — that file is left exactly as it was and the problem
is reported. Running md-press again on its output changes nothing.

MATH: Unicode/bare math in prose (η, ‖δ‖ ≤ R, M_t) is promoted to $LaTeX$
using a local ollama model, and blank lines around $$ display math are
fixed. The model proposes only where each expression starts and stops;
house rules are applied deterministically afterward. Glyphs the house also
uses as punctuation (→ · × ≈ ≤ ≥ ≠ ± ≡ √) count as math only beside a
math-looking operand: "asf → logos" and "a · b · c" separators are prose.
If ollama or the model is missing, the run says so once and continues
unwrap-only.

WHY MATH IS GOVERNED DIFFERENTLY: promoting math is *meant* to change
rendering (a literal "η" becomes a typeset symbol), so the render rule above
cannot apply to it and does not. Its guarantee is separate and narrower: the
model sees only prose — never list markers, quote markers, indentation, code,
links, or existing math — and a proposal is kept only if every character
outside its new $...$ spans is the original's own, in order, and nothing
inside a new span is invented. Anything else is left byte-identical (and
counted in a summary; -v lists them), so the model can never quietly reword
prose, restructure markdown, or invent mathematics. It is not strictly
deterministic: the model runs at temperature 0, but a different model or
version can propose differently.

Exit codes: 0 = nothing would change (report), or done (--fix)
            1 = the report found files that would change
            2 = an error, a bad code, or a file left untouched by the
                safety check"#;
    if code == 0 {
        println!("{text}");
    } else {
        // on an error, just the usage lines: the full text buries the error
        let short: String = text.lines().skip_while(|l| !l.starts_with("usage:")).take_while(|l| !l.is_empty()).collect::<Vec<_>>().join("\n");
        eprintln!("{short}\n(md-press --help for everything)");
    }
    std::process::exit(code)
}

const FLAGS: &[&str] = &[
    "--check", "--fix", "--force", "--allow-udon", "--no-classify", "--explain", "--math",
    "--no-math", "--no-log", "-q", "--quiet", "-v", "--verbose",
];

struct Opts {
    fix: bool,
    model: Option<String>,
    force: bool,
    allow_udon: bool,
    no_classify: bool,
    explain: bool,
    quiet: bool,
    verbose: bool,
    log: bool,
    toggles: Vec<String>,
    files: Vec<String>,
}

fn parse_args(args: &[String]) -> Opts {
    let mut toggles = Vec::new();
    let mut files = Vec::new();
    let mut flags: Vec<&str> = Vec::new();
    let mut math_flag: Option<Option<String>> = None;
    let mut i = 0;
    while i < args.len() {
        let a = args[i].as_str();
        if a == "--toggle" || a == "-t" {
            let Some(v) = args.get(i + 1) else {
                eprintln!("md-press: {a} needs codes");
                usage(2);
            };
            toggles.extend(split_codes(v));
            i += 2;
            continue;
        }
        if let Some(v) = a.strip_prefix("--toggle=") {
            toggles.extend(split_codes(v));
        } else if a == "--no-math" {
            math_flag = Some(None);
        } else if a == "--math" {
            math_flag = Some(Some(md_press::math::DEFAULT_MODEL.to_string()));
        } else if let Some(m) = a.strip_prefix("--math=") {
            math_flag = Some(Some(m.to_string()));
        } else if FLAGS.contains(&a) {
            flags.push(a);
        } else if a.starts_with('-') && a != "-" {
            eprintln!("md-press: unknown option '{a}'");
            usage(2);
        } else {
            files.push(a.to_string());
        }
        i += 1;
    }
    let mut seen = std::collections::HashSet::new();
    toggles.retain(|c| seen.insert(c.clone()));
    let has = |f: &str| flags.contains(&f);
    if has("--check") && has("--fix") {
        eprintln!("md-press: --check and --fix contradict each other (--check is the default)");
        std::process::exit(2);
    }
    let model = match math_flag {
        Some(choice) => choice,
        None => match std::env::var("MD_PRESS_MATH") {
            Ok(v) if off(&v) => None,
            Ok(v) if !v.trim().is_empty() && !matches!(v.to_ascii_lowercase().as_str(), "on" | "1" | "yes" | "true") => {
                Some(v.trim().to_string())
            }
            _ => Some(md_press::math::DEFAULT_MODEL.to_string()),
        },
    };
    let log = !has("--no-log") && !std::env::var("MD_PRESS_LOG").is_ok_and(|v| off(&v));
    Opts {
        fix: has("--fix"),
        model,
        force: has("--force"),
        allow_udon: has("--allow-udon"),
        no_classify: has("--no-classify"),
        explain: has("--explain"),
        quiet: has("-q") || has("--quiet"),
        verbose: has("-v") || has("--verbose"),
        log,
        toggles,
        files,
    }
}

fn off(v: &str) -> bool {
    matches!(v.to_ascii_lowercase().as_str(), "off" | "0" | "no" | "false")
}

/// Codes as typed or pasted: separated by commas or spaces, brackets
/// optional, case-insensitive, with Crockford's look-alikes folded.
fn split_codes(v: &str) -> Vec<String> {
    v.split(|c: char| c == ',' || c.is_whitespace())
        .map(|c| {
            c.trim_matches(|c| c == '[' || c == ']')
                .to_ascii_lowercase()
                .chars()
                .map(|c| match c {
                    'i' | 'l' => '1',
                    'o' => '0',
                    c => c,
                })
                .collect::<String>()
        })
        .filter(|c| !c.is_empty())
        .collect()
}

// ---------------------------------------------------------------- run identity

/// What one run's plans are a function of besides the file: this build of
/// md-press (the executable's own bytes, which include the classifier) and
/// the options that change a plan.
fn run_identity(opts: &Opts) -> String {
    let exe = std::env::current_exe().ok().and_then(|p| std::fs::read(p).ok()).unwrap_or_default();
    let math = match &opts.model {
        Some(m) => format!("math={m};prompt={:032x}", md_press::fnv128(&[md_press::math::instructions().as_bytes()])),
        None => "math=off".into(),
    };
    format!("build={:032x};{math};classify={}", md_press::fnv128(&[&exe]), !opts.no_classify)
}

// ---------------------------------------------------------------- report

fn tail(s: &str, n: usize) -> String {
    let c: Vec<char> = s.chars().collect();
    if c.len() <= n { s.to_string() } else { format!("…{}", c[c.len() - n..].iter().collect::<String>()) }
}

fn head(s: &str, n: usize) -> String {
    let c: Vec<char> = s.chars().collect();
    if c.len() <= n { s.to_string() } else { format!("{}…", c[..n].iter().collect::<String>()) }
}

fn plural(n: usize, one: &str, many: &str) -> String {
    format!("{n} {}", if n == 1 { one } else { many })
}

fn describe(it: &Item) -> String {
    let lead = if it.flipped() { "FLIPPED: " } else { "" };
    match &it.detail {
        Detail::Para { first, last, joins, flipped } => {
            let (a, b) = (&joins[0].1, &joins[joins.len() - 1].2);
            if *flipped {
                format!(
                    "L{first}-{last}  {lead}keep and mark {} (not joined): \"{}\" … \"{}\"",
                    plural(joins.len(), "break", "breaks"),
                    head(a, 36),
                    tail(b, 36)
                )
            } else {
                format!("L{first}-{last}  join {}: \"{}\" … \"{}\"", plural(joins.len(), "break", "breaks"), head(a, 36), tail(b, 36))
            }
        }
        Detail::Break { item } => {
            let what = match item.action {
                BreakAction::Join => "join",
                BreakAction::Mark => "keep + mark",
            };
            let p = item.p.map(|p| format!("p {p:.2}")).unwrap_or_else(|| "by the parse".into());
            let alt = if it.flipped() {
                format!("{p}; default: {}", if item.default == BreakAction::Join { "join" } else { "keep + mark" })
            } else if it.sure {
                p
            } else {
                format!("{p}; flip: {}", if item.action == BreakAction::Join { "keep + mark" } else { "join" })
            };
            format!("L{}  {lead}{what} ({alt}): \"{}\" ⏎ \"{}\"", it.line, tail(&item.line_a, 40), head(&item.line_b, 40))
        }
        Detail::Math { before, removed, added, after, applied } => {
            let verb = if *applied { "math" } else { "math, left as written" };
            format!("L{}  {lead}{verb}: {before}«{removed}» → «{added}»{after}", it.line)
        }
        Detail::Blank { after, delim, applied } => format!(
            "L{}  {lead}{} blank line {} {}",
            it.line,
            if *applied { "add" } else { "no" },
            if *after { "after" } else { "before" },
            head(delim, 40)
        ),
    }
}

fn report(plan: &Plan, verbose: bool) {
    let mut joins = 0;
    let mut marks = 0;
    let mut maths = 0;
    let mut blanks = 0;
    for it in &plan.items {
        match &it.detail {
            Detail::Para { joins: j, flipped, .. } => {
                if *flipped {
                    marks += j.len();
                } else {
                    joins += j.len();
                }
            }
            Detail::Break { item } => match item.action {
                BreakAction::Join => joins += 1,
                BreakAction::Mark => marks += 1,
            },
            Detail::Math { applied, .. } => maths += *applied as usize,
            Detail::Blank { applied, .. } => blanks += *applied as usize,
        }
    }
    let mut parts = Vec::new();
    if joins > 0 {
        parts.push(format!("{} joined", plural(joins, "line break", "line breaks")));
    }
    if marks > 0 {
        parts.push(format!("{} kept and marked", plural(marks, "break", "breaks")));
    }
    if maths > 0 {
        parts.push(format!("{} converted to math", plural(maths, "passage", "passages")));
    }
    if blanks > 0 {
        parts.push(format!("{} added around $$", plural(blanks, "blank line", "blank lines")));
    }
    let changes = plan.output != plan.input;
    println!(
        "{}: {}",
        plan.ctx,
        if changes { format!("would change — {}", parts.join(", ")) } else { "no change with these flips".into() }
    );
    let sure: Vec<&Item> = plan.items.iter().filter(|i| i.sure).collect();
    let unsure: Vec<&Item> = plan.items.iter().filter(|i| !i.sure).collect();
    if !sure.is_empty() {
        println!("  confident:");
        for it in sure {
            println!("    [{}] {}", it.code, describe(it));
            if verbose && let Detail::Para { joins, .. } = &it.detail {
                for (l, a, b) in joins {
                    println!("              L{l}  \"{}\" ⏎ \"{}\"", tail(a, 40), head(b, 40));
                }
            }
        }
    }
    if !unsure.is_empty() {
        println!("  unsure:");
        for it in unsure {
            println!("    [{}] {}", it.code, describe(it));
        }
    }
}

const FOOTER: &str = "\
Each [code] flips one decision; a paragraph's confident joins flip together.
  md-press --fix FILE…                     write the plan as shown
  md-press --fix --toggle CODE,CODE FILE…  write it with those decisions flipped
  md-press --toggle CODE FILE…             show the flipped plan first
Codes match only this exact file content and md-press build. A flipped join
keeps the break and marks it (two trailing spaces, so it renders as a line
break), and the file remembers it. A flipped math change comes back on the
next run. If you flip the same thing twice and it looks like a bug rather
than a judgment call, please describe it in a dated FEEDBACK-*.md in";

fn print_summaries(tally: &Tally, model: Option<&md_press::math::Model>, fix: bool, verbose: bool) {
    if let Some(m) = model
        && tally.math_skipped
    {
        let why = m.down_reason().unwrap_or_else(|| "model unavailable".into());
        eprintln!(
            "md-press: math pass skipped — {why}. Unwrapping ran as usual{}; --no-math silences this.",
            if fix { "" } else { ", so this report does not cover math" }
        );
    }
    if !tally.math_refused.is_empty() && (verbose || fix) {
        if verbose {
            eprintln!("md-press: math passages left as written (the model's proposal failed a check):");
            for r in &tally.math_refused {
                eprintln!("    {r}");
            }
        } else {
            eprintln!(
                "md-press: math: {} in {} left as written — the model's proposals failed a check (-v lists them).",
                plural(tally.math_refused.len(), "passage", "passages"),
                plural(tally.math_files.len(), "file", "files")
            );
        }
    }
    let skipped = tally.excluded.len() + tally.foreign.len();
    if skipped > 0 {
        if verbose || skipped <= 3 {
            for e in &tally.excluded {
                eprintln!("md-press: {e} (--force overrides)");
            }
            for f in &tally.foreign {
                eprintln!("md-press: {f} (--allow-udon overrides)");
            }
        } else {
            let mut parts = Vec::new();
            if !tally.excluded.is_empty() {
                parts.push(format!("{} by .md-pressignore (--force overrides)", tally.excluded.len()));
            }
            if !tally.foreign.is_empty() {
                parts.push(format!("{} .udon (--allow-udon overrides)", tally.foreign.len()));
            }
            eprintln!("md-press: skipped {}: {}; -v lists them.", plural(skipped, "file", "files"), parts.join(", "));
        }
    }
}

// ---------------------------------------------------------------- log

fn log_path() -> Option<PathBuf> {
    let base = std::env::var_os("XDG_DATA_HOME")
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/share")))?;
    Some(base.join("md-press/usage.jsonl"))
}

/// One line per file that had decisions. What a later reader needs to use
/// it as training data: every decision that wasn't a confident join, with
/// its context, default, outcome, and whether it was flipped; counts for
/// the confident joins. Failure to log never fails the run.
fn log_run(plans: &[Plan], run_id: &str, fix: bool, given: &[String]) {
    let Some(path) = log_path() else { return };
    let ts = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    let mut lines = String::new();
    for p in plans.iter().filter(|p| !p.items.is_empty()) {
        let mut items = Vec::new();
        let mut para_joins = 0usize;
        let mut para_flips = 0usize;
        for it in &p.items {
            let flipped = it.flipped();
            let v = match &it.detail {
                Detail::Para { joins, flipped: f, .. } => {
                    para_joins += joins.len();
                    para_flips += *f as usize;
                    if !*f {
                        continue;
                    }
                    serde_json::json!({"kind": "para", "line": it.line, "joins": joins.iter().map(|(l, a, b)| serde_json::json!([l, a, b])).collect::<Vec<_>>(), "flipped": true})
                }
                Detail::Break { item } => serde_json::json!({
                    "kind": "break", "line": it.line, "p": item.p, "sure": it.sure,
                    "default": format!("{:?}", item.default), "final": format!("{:?}", item.action),
                    "flipped": flipped, "a": item.line_a, "b": item.line_b,
                    "features": item.features.iter().map(|(n, v)| (n.to_string(), serde_json::json!(v))).collect::<serde_json::Map<_, _>>(),
                }),
                Detail::Math { before, removed, added, after, applied } => serde_json::json!({
                    "kind": "math", "line": it.line, "before": before, "removed": removed,
                    "added": added, "after": after, "flipped": !applied,
                }),
                Detail::Blank { after, delim, applied } => serde_json::json!({
                    "kind": "blank", "line": it.line, "after": after, "delim": delim, "flipped": !applied,
                }),
            };
            items.push(v);
        }
        let rec = serde_json::json!({
            "ts": ts,
            "mode": if fix { "fix" } else { "report" },
            "run": run_id,
            "file": p.path.as_ref().map(|x| std::fs::canonicalize(x).unwrap_or_else(|_| x.clone()).display().to_string()).unwrap_or_else(|| p.ctx.clone()),
            "sha": format!("{:032x}", md_press::fnv128(&[p.input.as_bytes()])),
            "toggles_given": !given.is_empty(),
            "toggles_used": p.matched,
            "para_joins": para_joins,
            "para_flips": para_flips,
            "items": items,
        });
        lines.push_str(&rec.to_string());
        lines.push('\n');
    }
    if lines.is_empty() {
        return;
    }
    if let Some(dir) = path.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    use std::io::Write;
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(&path) {
        let _ = f.write_all(lines.as_bytes());
    }
}

// ---------------------------------------------------------------- main

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        usage(2);
    }
    if args.iter().any(|a| a == "--help" || a == "-h") {
        usage(0);
    }
    if args.iter().any(|a| a == "--version" || a == "-V") {
        println!("md-press {}", env!("CARGO_PKG_VERSION"));
        return;
    }
    let opts = parse_args(&args);
    if opts.files.is_empty() {
        eprintln!("md-press: no files named");
        usage(2);
    }
    let model = opts.model.clone().map(md_press::math::Model::new);
    let clf = if opts.no_classify {
        None
    } else {
        match md_press::classify::Classifier::load() {
            Ok(c) => Some(c),
            Err(e) => {
                eprintln!("md-press: classifier unavailable ({e}); running deterministic-only");
                None
            }
        }
    };
    let run_id = run_identity(&opts);
    let mut tally = Tally::default();
    let mut plans: Vec<Plan> = Vec::new();
    let mut errors = false;
    let stdin = opts.files.len() == 1 && opts.files[0] == "-";

    if stdin {
        let mut input = String::new();
        std::io::stdin().read_to_string(&mut input).expect("read stdin");
        match plan_file("(stdin)", "-", input, &run_id, clf.as_ref(), model.as_ref(), &opts.toggles, opts.explain, &mut tally) {
            Ok(p) => plans.push(p),
            Err(e) => {
                eprintln!("md-press: {e}");
                std::process::exit(2);
            }
        }
    } else {
        let mut excluder = md_press::exclude::Excluder::new();
        for f in &opts.files {
            let path = PathBuf::from(f);
            if !opts.allow_udon
                && let Some(lang) = md_press::exclude::foreign_language(&path)
            {
                tally.foreign.push(format!(
                    "{}: skipped, .{lang} is not markdown — its line breaks carry meaning this tool has no model of",
                    path.display()
                ));
                continue;
            }
            if !opts.force
                && let Some(rule_file) = excluder.excluded(&path)
            {
                tally.excluded.push(format!("{}: skipped, excluded by {}", path.display(), rule_file.display()));
                continue;
            }
            let input = match std::fs::read_to_string(&path) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("md-press: {}: {}", path.display(), e);
                    errors = true;
                    continue;
                }
            };
            let key = std::fs::canonicalize(&path).unwrap_or_else(|_| path.clone()).display().to_string();
            let ctx = path.display().to_string();
            match plan_file(&ctx, &key, input, &run_id, clf.as_ref(), model.as_ref(), &opts.toggles, opts.explain, &mut tally) {
                Ok(mut p) => {
                    p.path = Some(path);
                    plans.push(p);
                }
                Err(e) => {
                    eprintln!("md-press: {e}");
                    errors = true;
                }
            }
        }
    }

    // every given code must name exactly one decision, or nothing is written
    let mut uses: HashMap<&str, usize> = HashMap::new();
    for p in &plans {
        for m in &p.matched {
            *uses.entry(m.as_str()).or_default() += 1;
        }
    }
    let unknown: Vec<&String> = opts.toggles.iter().filter(|c| !uses.contains_key(c.as_str())).collect();
    let ambiguous: Vec<&String> = opts.toggles.iter().filter(|c| uses.get(c.as_str()).is_some_and(|&n| n > 1)).collect();
    if !unknown.is_empty() || !ambiguous.is_empty() {
        for c in unknown {
            eprintln!(
                "md-press: code '{c}' matches nothing in {} — mistyped, or the file, md-press, or the options changed since the report; run the report again",
                if stdin { "the input" } else { "the files named" }
            );
        }
        for c in ambiguous {
            eprintln!("md-press: code '{c}' matches more than one decision; use the longer code the report shows for it, or name one file");
        }
        eprintln!("md-press: nothing written");
        std::process::exit(2);
    }

    let mut would_change = false;
    if !opts.fix {
        let mut any = false;
        for p in &plans {
            if p.output == p.input && p.items.iter().all(|i| !i.flipped()) {
                continue;
            }
            would_change |= p.output != p.input;
            if opts.quiet {
                if p.output != p.input {
                    println!("{}", p.ctx);
                }
                continue;
            }
            if any {
                println!();
            }
            any = true;
            report(p, opts.verbose);
        }
        if any && !opts.quiet {
            println!();
            println!("{FOOTER}\n{}/", env!("CARGO_MANIFEST_DIR"));
        }
    } else if stdin {
        print!("{}", plans[0].output);
    } else {
        let mut wrote = 0;
        let mut unsure = 0;
        for p in &plans {
            if p.output == p.input {
                continue;
            }
            unsure += p.items.iter().filter(|i| !i.sure && !i.flipped()).count();
            let path = p.path.as_ref().unwrap();
            match std::fs::write(path, &p.output) {
                Ok(()) => wrote += 1,
                Err(e) => {
                    eprintln!("md-press: {}: {}", path.display(), e);
                    errors = true;
                }
            }
        }
        if !opts.quiet && wrote > 0 {
            let mut msg = format!("md-press: wrote {}", plural(wrote, "file", "files"));
            if !opts.toggles.is_empty() {
                msg.push_str(&format!(" with {} flipped", plural(opts.toggles.len(), "decision", "decisions")));
            } else if unsure > 0 {
                msg.push_str(&format!(
                    "; {} taken at the default (run without --fix first to review them)",
                    plural(unsure, "unsure decision", "unsure decisions")
                ));
            }
            eprintln!("{msg}.");
        }
    }
    if opts.log {
        log_run(&plans, &run_id, opts.fix, &opts.toggles);
    }
    print_summaries(&tally, model.as_ref(), opts.fix, opts.verbose);
    std::process::exit(if errors {
        2
    } else if !opts.fix && would_change {
        1
    } else {
        0
    });
}
