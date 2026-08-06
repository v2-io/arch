use std::io::Read;
use std::path::PathBuf;

fn usage(code: i32) -> ! {
    let text = r#"md-press — canonicalize markdown to house standards

Named files are EDITED IN PLACE. Use --check for a dry run.

usage: md-press [--math[=MODEL]] <file.md>...    edit those files in place
       md-press --check <file.md>...             dry run; write nothing
       md-press [--math[=MODEL]] -               read stdin, write stdout

  --explain         Debug: print every consulted break with its calibrated
                    probability and named feature values on stderr.
  --check           Dry run. Print the path of each file that would change,
                    change nothing on disk, and exit 1 if there were any.
  --force           Format even files excluded by a .md-pressignore.
  --allow-udon      Format .udon files too. Off by default, and the default
                    is the recommendation — see FOREIGN LANGUAGES below.
                    Deliberately NOT covered by --force, so that overriding
                    verbatim exclusions does not also disable this.
  --math[=MODEL]    Additionally promote Unicode/bare math to $LaTeX$ using
                    a local ollama model (default llama3.2:3b). The model
                    proposes only where each expression starts and stops;
                    house rules are then applied deterministically, and any
                    proposal failing verification is reported rather than
                    written. Also fixes blank lines around $$ display math.
  -h, --help        Show this.

To preview the actual edits to one file, use stdin mode and your own diff:

    md-press - < FILE.md | diff FILE.md -

STDIN MODE IS UNGUARDED, which matters most in exactly that preview use. There
is no filename to read, so neither a .md-pressignore exclusion nor the .udon
guard can be consulted; and the render-equality gate is not run at all, because
stdin writes to stdout and there is no write to refuse. So the pipeline above
answers "what would the engine do to these bytes", not "what would md-press write
to this file" — a file that file mode would skip comes back fully reformatted
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
wikilinks, HTML, and line breaks that carry meaning are left alone.

WHY UNWRAPPING IS SAFE: the source text changes, but the rendered document
must not. Before writing any file, md-press re-parses its own output and
compares the rendered result against the original. If they differ at all —
which would mean a bug in md-press — that file is left exactly as it was and
the problem is reported. Running md-press again on its output changes nothing.

WHY --math IS DIFFERENT: promoting math is *meant* to change rendering (a
literal "η" becomes a typeset symbol), so the rule above cannot apply to it
and does not. Its guarantee is separate, per line, and narrower: prose
outside the math is untouched word for word, and nothing may appear inside a
new $...$ span that does not trace back to the original line. A line failing
either check is left byte-identical and reported, so the model can never
quietly reword your prose or invent mathematics.

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

fn is_known_flag(a: &str) -> bool {
    a == "--check" || a == "--force" || a == "--allow-udon" || a == "--no-classify" || a == "--explain" || a == "--math" || a.starts_with("--math=")
}

fn trunc(s: &str) -> String {
    let t: String = s.chars().take(70).collect();
    if t.len() < s.len() { format!("{t}…") } else { t }
}

/// Apply the math pass (deterministic \(…\) normalization + LLM promotion +
/// $$ blank lines) to already-unwrapped text, promoting only at the prose
/// sites the parse identified — whole paragraph/heading lines, or table
/// cells individually. Code, frontmatter, and HTML have no sites and are
/// structurally out of reach. Returns the new text and flags.
fn math_pass(input: &str, sites: &[md_press::MathSite], model: &str) -> (String, Vec<String>) {
    use md_press::MathSite;
    use md_press::math::{MathOutcome, promote_text};
    let mut flags = Vec::new();
    let mut out_lines: Vec<String> = Vec::new();
    for (i, line) in input.lines().enumerate() {
        let mut flag = |ctx: String, why: String, det_kept: bool| {
            let kept = if det_kept { " (kept the deterministic \\(…\\) → $…$ normalization)" } else { "" };
            flags.push(format!("line {}{}: {}{}", i + 1, ctx, why, kept));
        };
        let site = sites.get(i).cloned().unwrap_or(MathSite::None);
        let new_line = match site {
            MathSite::None => line.to_string(),
            MathSite::Whole => match promote_text(model, line) {
                MathOutcome::Unchanged => line.to_string(),
                MathOutcome::Converted(l) => l,
                MathOutcome::Flagged(why, det) => {
                    flag(String::new(), why, det.is_some());
                    det.unwrap_or_else(|| line.to_string())
                }
            },
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
                    let lead = &cell[..cell.len() - cell.trim_start().len()];
                    let trail = &cell[cell.trim_end().len()..];
                    let interior = cell.trim();
                    let converted = match promote_text(model, interior) {
                        MathOutcome::Unchanged => interior.to_string(),
                        MathOutcome::Converted(c) => c,
                        MathOutcome::Flagged(why, det) => {
                            flag(format!(" cell {}", n + 1), why, det.is_some());
                            det.unwrap_or_else(|| interior.to_string())
                        }
                    };
                    let converted = format!("{lead}{converted}{trail}");
                    // a proposal may never mint a new cell boundary
                    if converted.matches('|').count() != cell.matches('|').count() {
                        flag(
                            format!(" cell {}", n + 1),
                            "proposal changed table delimiters; left unchanged".into(),
                            false,
                        );
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
        out_lines.push(new_line);
    }
    let mut s = out_lines.join("\n");
    if input.ends_with('\n') {
        s.push('\n');
    }
    (md_press::math::fix_display_math_blanks(&s), flags)
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
    let math_model: Option<String> = args.iter().find_map(|a| {
        if a == "--math" {
            Some("llama3.2:3b".to_string())
        } else {
            a.strip_prefix("--math=").map(str::to_string)
        }
    });
    let force = args.iter().any(|a| a == "--force");
    let allow_udon = args.iter().any(|a| a == "--allow-udon");
    let no_classify = args.iter().any(|a| a == "--no-classify");
    let explain = args.iter().any(|a| a == "--explain");
    let files: Vec<&String> = args
        .iter()
        .filter(|a| {
            a.as_str() != "--check"
                && a.as_str() != "--force"
                && a.as_str() != "--allow-udon"
                && a.as_str() != "--no-classify"
                && a.as_str() != "--explain"
                && a.as_str() != "--math"
                && !a.starts_with("--math=")
        })
        .collect();
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

    let print_notes = |ctx: &str, notes: &[md_press::Note]| {
        let kept: Vec<_> = notes.iter().filter(|n| n.kept).collect();
        let joined: Vec<_> = notes.iter().filter(|n| !n.kept).collect();
        if !kept.is_empty() {
            eprintln!("md-press: {ctx}: wasn't sure about the following but KEPT the line break (and marked it). Manually concatenate if that was wrong:");
            for n in &kept {
                eprintln!("    (p={:.2}) {} \\n {}", n.p, trunc(&n.line_a), trunc(&n.line_b));
            }
        }
        if !joined.is_empty() {
            eprintln!("md-press: {ctx}: wasn't sure about the following and JOINED the lines. Manually re-separate and append two trailing spaces to make the break permanent:");
            for n in &joined {
                eprintln!("    (p={:.2}) {} \\n {}", n.p, trunc(&n.line_a), trunc(&n.line_b));
            }
        }
    };

    if files.len() == 1 && files[0] == "-" {
        let mut input = String::new();
        std::io::stdin()
            .read_to_string(&mut input)
            .expect("read stdin");
        let r = run(&input);
        let mut out = r.output;
        print_notes("(stdin)", &r.notes);
        if let Some(model) = &math_model {
            let (o, flags) = math_pass(&out, &r.math_sites, model);
            out = o;
            for f in flags {
                eprintln!("md-press: (stdin): {f}");
            }
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
            eprintln!(
                "md-press: {}: skipped, .{lang} is not markdown — its line breaks carry meaning this tool has no model of, so the render-equality gate cannot defend it (--allow-udon overrides)",
                path.display()
            );
            continue;
        }
        if !force
            && let Some(rule_file) = excluder.excluded(&path)
        {
            eprintln!(
                "md-press: {}: skipped, excluded by {} (--force overrides)",
                path.display(),
                rule_file.display()
            );
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
        let (unwrapped, gate_variant, notes) = (r.output, r.gate_output, r.notes);
        // Built-in safety gate on the unwrap stage: render-equality before
        // any write — compared against the gate variant, because classifier
        // marker insertion deliberately changes rendering (its narrower
        // guarantee: only add "  " at an existing break, or join at a soft
        // break). The math pass likewise runs after with its own gates.
        if gate_variant != input
            && md_press::render_fingerprint(&input) != md_press::render_fingerprint(&gate_variant)
        {
            eprintln!(
                "md-press: {}: SKIPPED — result would change rendered document (bug or unsupported construct; please report)",
                path.display()
            );
            errors = true;
            continue;
        }
        if !check {
            print_notes(&path.display().to_string(), &notes);
        }
        let output = if let Some(model) = &math_model {
            let (o, flags) = math_pass(&unwrapped, &r.math_sites, model);
            for f in flags {
                eprintln!("md-press: {}: {}", path.display(), f);
            }
            o
        } else {
            unwrapped
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
    std::process::exit(if errors {
        2
    } else if check && would_change {
        1
    } else {
        0
    });
}
