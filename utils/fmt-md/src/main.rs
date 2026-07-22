use std::io::Read;
use std::path::PathBuf;

fn usage(code: i32) -> ! {
    let text = r#"fmt-md — canonicalize markdown to house standards

Named files are EDITED IN PLACE. Use --check for a dry run.

usage: fmt-md [--math[=MODEL]] <file.md>...    edit those files in place
       fmt-md --check <file.md>...             dry run; write nothing
       fmt-md [--math[=MODEL]] -               read stdin, write stdout

  --check           Dry run. Print the path of each file that would change,
                    change nothing on disk, and exit 1 if there were any.
  --force           Format even files excluded by a .fmt-mdignore.
  --math[=MODEL]    Additionally promote Unicode/bare math to $LaTeX$ using
                    a local ollama model (default llama3.2:3b). The model
                    proposes only where each expression starts and stops;
                    house rules are then applied deterministically, and any
                    proposal failing verification is reported rather than
                    written. Also fixes blank lines around $$ display math.
  -h, --help        Show this.

To preview the actual edits to one file, use stdin mode and your own diff:

    fmt-md - < FILE.md | diff FILE.md -

EXCLUSIONS: a .fmt-mdignore file (gitignore syntax) at or above a file marks
it as not-for-formatting, and is honoured even when the file is named
explicitly — because the realistic accident is an agent running
`fmt-md $(find . -name '*.md')` over verbatim material. Use it for raw
transcripts, provenanced copies, and frozen archaeology: reformatting those
is render-equivalent and still wrong, and no automatic check can tell,
because nothing about the rendered document changed.

WHAT IT CHANGES: each prose paragraph that is split across several lines
becomes one long line — including paragraphs inside list items, blockquotes,
and footnotes. Tables, code (fenced and inline), YAML frontmatter, math,
wikilinks, HTML, and line breaks that carry meaning are left alone.

WHY UNWRAPPING IS SAFE: the source text changes, but the rendered document
must not. Before writing anything, fmt-md re-parses its own output and
compares the rendered result against the original. If they differ at all —
which would mean a bug in fmt-md — that file is left exactly as it was and
the problem is reported. Running fmt-md again on its output changes nothing.

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
    a == "--check" || a == "--force" || a == "--no-classify" || a == "--math" || a.starts_with("--math=")
}

fn trunc(s: &str) -> String {
    let t: String = s.chars().take(70).collect();
    if t.len() < s.len() { format!("{t}…") } else { t }
}

/// Apply the math pass (LLM promotion per line + deterministic $$ blank
/// lines) to already-unwrapped text. Returns the new text and flags.
fn math_pass(input: &str, model: &str) -> (String, Vec<String>) {
    let mut flags = Vec::new();
    let mut out_lines: Vec<String> = Vec::new();
    let mut in_code = false;
    for (i, line) in input.lines().enumerate() {
        if line.trim_start().starts_with("```") {
            in_code = !in_code;
            out_lines.push(line.to_string());
            continue;
        }
        if in_code {
            out_lines.push(line.to_string());
            continue;
        }
        match fmt_md::math::promote_line(model, line) {
            fmt_md::math::MathOutcome::Unchanged => out_lines.push(line.to_string()),
            fmt_md::math::MathOutcome::Converted(l) => out_lines.push(l),
            fmt_md::math::MathOutcome::Flagged(why) => {
                flags.push(format!("line {}: {}", i + 1, why));
                out_lines.push(line.to_string());
            }
        }
    }
    let mut s = out_lines.join("\n");
    if input.ends_with('\n') {
        s.push('\n');
    }
    (fmt_md::math::fix_display_math_blanks(&s), flags)
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
        eprintln!("fmt-md: unknown option '{bad}'");
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
    let no_classify = args.iter().any(|a| a == "--no-classify");
    let files: Vec<&String> = args
        .iter()
        .filter(|a| {
            a.as_str() != "--check"
                && a.as_str() != "--force"
                && a.as_str() != "--no-classify"
                && a.as_str() != "--math"
                && !a.starts_with("--math=")
        })
        .collect();
    let clf = if no_classify {
        None
    } else {
        match fmt_md::classify::Classifier::load() {
            Ok(c) => Some(c),
            Err(e) => {
                eprintln!("fmt-md: classifier unavailable ({e}); running deterministic-only");
                None
            }
        }
    };

    let run = |input: &str| -> (String, String, Vec<fmt_md::Note>) {
        match &clf {
            Some(c) => {
                let r = fmt_md::format_classified(input, c);
                (r.output, r.gate_output, r.notes)
            }
            None => {
                let o = fmt_md::format(input);
                (o.clone(), o, Vec::new())
            }
        }
    };

    let print_notes = |ctx: &str, notes: &[fmt_md::Note]| {
        let kept: Vec<_> = notes.iter().filter(|n| n.kept).collect();
        let joined: Vec<_> = notes.iter().filter(|n| !n.kept).collect();
        if !kept.is_empty() {
            eprintln!("fmt-md: {ctx}: wasn't sure about the following but KEPT the line break (and marked it). Manually concatenate if that was wrong:");
            for n in &kept {
                eprintln!("    (p={:.2}) {} \\n {}", n.p, trunc(&n.line_a), trunc(&n.line_b));
            }
        }
        if !joined.is_empty() {
            eprintln!("fmt-md: {ctx}: wasn't sure about the following and JOINED the lines. Manually re-separate and append two trailing spaces to make the break permanent:");
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
        let (mut out, _gate, notes) = run(&input);
        print_notes("(stdin)", &notes);
        if let Some(model) = &math_model {
            let (o, flags) = math_pass(&out, model);
            out = o;
            for f in flags {
                eprintln!("fmt-md: (stdin): {f}");
            }
        }
        print!("{out}");
        return;
    }

    let mut would_change = false;
    let mut errors = false;
    let mut excluder = fmt_md::exclude::Excluder::new();
    for f in files {
        let path = PathBuf::from(f);
        if !force {
            if let Some(rule_file) = excluder.excluded(&path) {
                eprintln!(
                    "fmt-md: {}: skipped, excluded by {} (--force overrides)",
                    path.display(),
                    rule_file.display()
                );
                continue;
            }
        }
        let input = match std::fs::read_to_string(&path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("fmt-md: {}: {}", path.display(), e);
                errors = true;
                continue;
            }
        };
        let (unwrapped, gate_variant, notes) = run(&input);
        // Built-in safety gate on the unwrap stage: render-equality before
        // any write — compared against the gate variant, because classifier
        // marker insertion deliberately changes rendering (its narrower
        // guarantee: only add "  " at an existing break, or join at a soft
        // break). The math pass likewise runs after with its own gates.
        if gate_variant != input
            && fmt_md::render_fingerprint(&input) != fmt_md::render_fingerprint(&gate_variant)
        {
            eprintln!(
                "fmt-md: {}: SKIPPED — result would change rendered document (bug or unsupported construct; please report)",
                path.display()
            );
            errors = true;
            continue;
        }
        if !check {
            print_notes(&path.display().to_string(), &notes);
        }
        let output = if let Some(model) = &math_model {
            let (o, flags) = math_pass(&unwrapped, model);
            for f in flags {
                eprintln!("fmt-md: {}: {}", path.display(), f);
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
            eprintln!("fmt-md: {}: {}", path.display(), e);
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
