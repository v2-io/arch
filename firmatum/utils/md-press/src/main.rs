use std::io::Read;
use std::path::PathBuf;

fn usage(code: i32) -> ! {
    let text = r#"md-press — canonicalize markdown to house standards

Named files are EDITED IN PLACE. Use --check for a dry run.

usage: md-press [options] <file.md>...    edit those files in place
       md-press --check <file.md>...      dry run; write nothing
       md-press [options] -               read stdin, write stdout

  --check           Dry run. Print the path of each file that would change,
                    change nothing on disk, and exit 1 if there were any.
                    With stdin (`--check -`) nothing is printed to stdout;
                    the exit code answers.
  --no-math         Skip the math pass (see MATH below), e.g. for a fully
                    deterministic check, or where ollama is not installed.
  --math[=MODEL]    The math pass is on by default; this flag names a
                    different local model (default llama3.2:3b), or turns
                    the pass back on over MD_PRESS_MATH=off.
  -q, --quiet       Don't list the uncertain line breaks (see BREAKS below);
                    errors and one-line summaries still print.
  -v, --verbose     List what the summaries count: each passage the math
                    pass left as written and why, and each skipped file.
  --force           Format even files excluded by a .md-pressignore.
  --allow-udon      Format .udon files too. Off by default, and the default
                    is the recommendation — see FOREIGN LANGUAGES below.
                    Deliberately NOT covered by --force, so that overriding
                    verbatim exclusions does not also disable this.
  --explain         Debug: print every consulted break with its calibrated
                    probability and named feature values on stderr.
  -h, --help        Show this.

Environment: MD_PRESS_MATH=off (or 0/no) disables the math pass for every
call that doesn't pass --math; MD_PRESS_MATH=MODEL picks the model. Flags
win over the environment. Model proposals are cached under
$XDG_CACHE_HOME/md-press/math (default ~/.cache), keyed by model, prompt, and
text, so re-runs and checks of text already seen are instant and repeatable;
every safety check still re-runs on each use. MD_PRESS_CACHE=off disables
it; deleting the directory is always safe.

To preview the actual edits to one file, use stdin mode and your own diff:

    md-press - < FILE.md | diff FILE.md -

STDIN MODE IS UNGUARDED, which matters most in exactly that preview use. There
is no filename to read, so neither a .md-pressignore exclusion nor the .udon
guard can be consulted; and when writing to stdout the render-equality gate is
not run, because there is no write to refuse. So the pipeline above answers
"what would the engine do to these bytes", not "what would md-press write to
this file" — a file that file mode would skip comes back fully reformatted
through stdin. When the question is whether a file would actually change, ask
it directly with --check FILE, which runs the guards and the gate.

EXCLUSIONS: a .md-pressignore file (gitignore syntax) at or above a file marks
it as not-for-formatting, and is honoured even when the file is named
explicitly — because the realistic accident is an agent running
`md-press $(find . -name '*.md')` over verbatim material. Use it for raw
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
overriding verbatim exclusions cannot quietly disable this too. Like the
exclusions above, this guard is filename-keyed and therefore absent in stdin
mode — see STDIN MODE IS UNGUARDED.

WHAT IT CHANGES: each prose paragraph that is split across several lines
becomes one long line — including paragraphs inside list items, blockquotes,
and footnotes. Tables, code (fenced and inline), YAML frontmatter, math,
wikilinks, HTML, and line breaks that carry meaning are left alone; so are
display-math ($$) lines and lines that look like table rows.

BREAKS: where a line break might be deliberate, a trained classifier decides.
Confident calls are silent. Uncertain ones are listed per file, with the
probability that the break was meant: "joined" lines can be re-separated by
hand (end the first with two spaces to make the break permanent); "kept"
breaks were marked with two trailing spaces, which you can delete and join.

WHY UNWRAPPING IS SAFE: the source text changes, but the rendered document
must not. Before writing any file, md-press re-parses its own output and
compares the rendered result against the original. If they differ at all —
which would mean a bug in md-press — that file is left exactly as it was and
the problem is reported. Running md-press again on its output changes nothing.

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

Exit codes: 0 = done (files written, or nothing needed changing)
            1 = --check found files that would change
            2 = an error, or a file was left untouched by the safety check"#;
    if code == 0 {
        println!("{text}");
    } else {
        eprintln!("{text}");
    }
    std::process::exit(code)
}

const FLAGS: &[&str] = &[
    "--check", "--force", "--allow-udon", "--no-classify", "--explain", "--math", "--no-math",
    "-q", "--quiet", "-v", "--verbose",
];

fn is_known_flag(a: &str) -> bool {
    FLAGS.contains(&a) || a.starts_with("--math=")
}

fn trunc(s: &str) -> String {
    let t: String = s.chars().take(70).collect();
    if t.len() < s.len() { format!("{t}…") } else { t }
}

/// Which model the math pass uses, if any: flags, then MD_PRESS_MATH, then
/// the default (on).
fn math_model(args: &[String]) -> Option<String> {
    let flag = args.iter().rev().find_map(|a| match a.as_str() {
        "--no-math" => Some(None),
        "--math" => Some(Some(md_press::math::DEFAULT_MODEL.to_string())),
        _ => a.strip_prefix("--math=").map(|m| Some(m.to_string())),
    });
    if let Some(choice) = flag {
        return choice;
    }
    match std::env::var("MD_PRESS_MATH") {
        Ok(v) if matches!(v.to_ascii_lowercase().as_str(), "off" | "0" | "no" | "false") => None,
        Ok(v) if !v.trim().is_empty() && !matches!(v.to_ascii_lowercase().as_str(), "on" | "1" | "yes" | "true") => {
            Some(v.trim().to_string())
        }
        _ => Some(md_press::math::DEFAULT_MODEL.to_string()),
    }
}

/// Run-wide tallies behind the one-line summaries.
#[derive(Default)]
struct Tally {
    math_refused: Vec<String>, // "path: line N[ cell M]: reason"
    math_files: std::collections::BTreeSet<String>,
    math_skipped: bool,
    excluded: Vec<String>,
    foreign: Vec<String>,
    notes_printed: bool,
}

/// Apply the math pass (deterministic \(…\) normalization + LLM promotion +
/// $$ blank lines) to already-unwrapped text, promoting only at the prose
/// sites the parse identified — whole paragraph/heading lines, or table
/// cells individually. Code, frontmatter, and HTML have no sites and are
/// structurally out of reach, and so are the interiors of multi-line $$
/// display blocks.
fn math_pass(
    input: &str,
    sites: &[md_press::MathSite],
    model: &md_press::math::Model,
    ctx: &str,
    tally: &mut Tally,
) -> String {
    use md_press::MathSite;
    use md_press::math::promote_site;
    let mut out_lines: Vec<String> = Vec::new();
    let mut in_display = false;
    // carry each line's own ending (LF or CRLF, or none on a final line)
    // through untouched: `str::lines` drops `\r` and the final empty line
    for (i, raw) in input.split_inclusive('\n').enumerate() {
        let (line, eol) = match raw.strip_suffix("\r\n") {
            Some(l) => (l, "\r\n"),
            None => raw.strip_suffix('\n').map_or((raw, ""), |l| (l, "\n")),
        };
        let mut refused = |cell: Option<usize>, why: &str| {
            let at = match cell {
                Some(n) => format!("line {} cell {}", i + 1, n),
                None => format!("line {}", i + 1),
            };
            tally.math_refused.push(format!("{ctx}: {at}: {why}"));
            tally.math_files.insert(ctx.to_string());
        };
        let site = sites.get(i).cloned().unwrap_or(MathSite::None);
        // track multi-line $$ blocks: their interior lines are paragraph
        // text to the parser but LaTeX to the reader
        let t = line.trim_start_matches([' ', '\t', '>']).trim_end();
        let delims = t.matches("$$").count();
        let inside = in_display;
        if site != MathSite::None {
            if in_display {
                in_display = delims % 2 == 0;
            } else if t.starts_with("$$") && delims % 2 == 1 {
                in_display = true;
            }
        } else {
            in_display = false;
        }
        let new_line = match site {
            MathSite::None => line.to_string(),
            _ if inside => line.to_string(),
            MathSite::Whole => {
                let r = promote_site(model, line);
                tally.math_skipped |= r.skipped;
                for f in &r.flags {
                    refused(None, f);
                }
                r.text
            }
            MathSite::Cells(ranges) => {
                let mut s = String::with_capacity(line.len());
                let mut cursor = 0;
                for (n, &(a, b)) in ranges.iter().enumerate() {
                    if a < cursor || b > line.len() || a > b {
                        continue; // defensive: malformed range, leave bytes as-is
                    }
                    s.push_str(&line[cursor..a]);
                    let cell = &line[a..b];
                    // promote the trimmed interior; the cell's own padding is
                    // table formatting and must survive untouched
                    if cell.trim().is_empty() {
                        // an empty cell is all padding; lead and trail would
                        // both be the whole cell and double it every run
                        s.push_str(cell);
                        cursor = b;
                        continue;
                    }
                    let lead = &cell[..cell.len() - cell.trim_start().len()];
                    let trail = &cell[cell.trim_end().len()..];
                    let r = promote_site(model, cell.trim());
                    tally.math_skipped |= r.skipped;
                    for f in &r.flags {
                        refused(Some(n + 1), f);
                    }
                    let converted = format!("{lead}{}{trail}", r.text);
                    // a proposal may never mint a new cell boundary
                    if converted.matches('|').count() != cell.matches('|').count() {
                        refused(Some(n + 1), "proposal changed table delimiters");
                        s.push_str(cell);
                    } else {
                        s.push_str(&converted);
                    }
                    cursor = b;
                }
                s.push_str(&line[cursor..]);
                s
            }
        };
        out_lines.push(new_line + eol);
    }
    md_press::math::fix_display_math_blanks_at(&out_lines.concat(), Some(sites))
}

fn print_notes(ctx: &str, notes: &[md_press::Note], tally: &mut Tally) {
    if notes.is_empty() {
        return;
    }
    tally.notes_printed = true;
    let n = notes.len();
    eprintln!(
        "md-press: {ctx}: {n} uncertain line break{}:",
        if n == 1 { "" } else { "s" }
    );
    for note in notes {
        eprintln!(
            "    {} p={:.2}  {} \\n {}",
            if note.kept { "kept  " } else { "joined" },
            note.p,
            trunc(&note.line_a),
            trunc(&note.line_b)
        );
    }
}

fn plural(n: usize, one: &str, many: &str) -> String {
    format!("{n} {}", if n == 1 { one } else { many })
}

fn print_summaries(tally: &Tally, model: Option<&md_press::math::Model>, check: bool, verbose: bool) {
    if tally.notes_printed {
        eprintln!(
            "md-press: p is how likely the break was meant. Undo a join: re-split and end the first line with two spaces. Undo a keep: delete its trailing two spaces and join."
        );
    }
    if let Some(m) = model
        && tally.math_skipped
    {
        let why = m.down_reason().unwrap_or_else(|| "model unavailable".into());
        eprintln!(
            "md-press: math pass skipped — {why}. Unwrapping ran as usual{}; --no-math silences this.",
            if check { ", so --check did not cover math" } else { "" }
        );
    }
    // In --check the refusals change nothing about the answer (those lines
    // are left as written either way), so they are listed only on request.
    if !tally.math_refused.is_empty() && (verbose || !check) {
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
            eprintln!(
                "md-press: skipped {}: {}; -v lists them.",
                plural(skipped, "file", "files"),
                parts.join(", ")
            );
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        usage(2);
    }
    if args.iter().any(|a| a == "--help" || a == "-h") {
        usage(0);
    }
    if let Some(bad) = args
        .iter()
        .find(|a| a.starts_with('-') && a.as_str() != "-" && !is_known_flag(a))
    {
        eprintln!("md-press: unknown option '{bad}'");
        usage(2);
    }
    let check = args.iter().any(|a| a == "--check");
    let model = math_model(&args).map(md_press::math::Model::new);
    let force = args.iter().any(|a| a == "--force");
    let allow_udon = args.iter().any(|a| a == "--allow-udon");
    let no_classify = args.iter().any(|a| a == "--no-classify");
    let explain = args.iter().any(|a| a == "--explain");
    let quiet = args.iter().any(|a| a == "-q" || a == "--quiet");
    let verbose = args.iter().any(|a| a == "-v" || a == "--verbose");
    let files: Vec<&String> = args.iter().filter(|a| !is_known_flag(a)).collect();
    let clf = if no_classify {
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

    let run = |input: &str| -> md_press::Classified {
        match &clf {
            Some(c) => md_press::format_classified(input, c, explain),
            None => md_press::format_plain(input),
        }
    };
    let mut tally = Tally::default();
    let render_ok = |input: &str, gate_variant: &str| {
        gate_variant == input
            || md_press::render_fingerprint(input) == md_press::render_fingerprint(gate_variant)
    };

    if files.len() == 1 && files[0] == "-" {
        let mut input = String::new();
        std::io::stdin()
            .read_to_string(&mut input)
            .expect("read stdin");
        let r = run(&input);
        if check {
            // a real check, not a formatted copy: guards still can't apply
            // (no filename), but the render gate can, and stdout stays empty
            if !render_ok(&input, &r.gate_output) {
                eprintln!("md-press: (stdin): would be SKIPPED — result would change rendered document (bug or unsupported construct; please report)");
                std::process::exit(2);
            }
        } else if !quiet {
            print_notes("(stdin)", &r.notes, &mut tally);
        }
        let mut out = r.output;
        if let Some(m) = &model {
            out = math_pass(&out, &r.math_sites, m, "(stdin)", &mut tally);
        }
        print_summaries(&tally, model.as_ref(), check, verbose);
        if check {
            std::process::exit(if out != input { 1 } else { 0 });
        }
        print!("{out}");
        return;
    }

    let mut would_change = false;
    let mut errors = false;
    let mut excluder = md_press::exclude::Excluder::new();
    for f in files {
        let path = PathBuf::from(f);
        if !allow_udon
            && let Some(lang) = md_press::exclude::foreign_language(&path)
        {
            tally.foreign.push(format!(
                "{}: skipped, .{lang} is not markdown — its line breaks carry meaning this tool has no model of",
                path.display()
            ));
            continue;
        }
        if !force
            && let Some(rule_file) = excluder.excluded(&path)
        {
            tally.excluded.push(format!(
                "{}: skipped, excluded by {}",
                path.display(),
                rule_file.display()
            ));
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
        let r = run(&input);
        // Built-in safety gate on the unwrap stage: render-equality before
        // any write — compared against the gate variant, because classifier
        // marker insertion deliberately changes rendering (its narrower
        // guarantee: only add "  " at an existing break, or join at a soft
        // break). The math pass likewise runs after with its own gates.
        if !render_ok(&input, &r.gate_output) {
            eprintln!(
                "md-press: {}: SKIPPED — result would change rendered document (bug or unsupported construct; please report)",
                path.display()
            );
            errors = true;
            continue;
        }
        let ctx = path.display().to_string();
        if !check && !quiet {
            print_notes(&ctx, &r.notes, &mut tally);
        }
        let output = match &model {
            Some(m) => math_pass(&r.output, &r.math_sites, m, &ctx, &mut tally),
            None => r.output,
        };
        if output == input {
            continue;
        }
        would_change = true;
        if check {
            println!("{}", path.display());
        } else if let Err(e) = std::fs::write(&path, &output) {
            eprintln!("md-press: {}: {}", path.display(), e);
            errors = true;
        }
    }
    print_summaries(&tally, model.as_ref(), check, verbose);
    std::process::exit(if errors {
        2
    } else if check && would_change {
        1
    } else {
        0
    });
}
