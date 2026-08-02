//! Unicode/bare-math promotion pass (PLAN Phase 3, model-assisted).
//!
//! Pipeline per line: cheap deterministic detector (does this line carry
//! unpromoted math at all?) → local-LLM proposal (ollama, prompt file with
//! accumulating edge-case instructions) → deterministic post-processing
//! (the FORMAT.md cross-renderer rules) → prose-preservation verification.
//! A proposal that fails any gate is rejected and the line is flagged
//! instead — the failure mode is "tells you," never a bad write.

use std::process::{Command, Stdio};

/// Unicode glyphs that signal math when found outside code/math spans.
/// Greek singletons live in GREEK_LATEX; this is the operator/symbol set.
const MATH_GLYPHS: &[char] = &[
    '‖', '→', '↦', '≤', '≥', '≠', '≈', '≡', '·', '×', '∈', '∉', '∞', '±', '∂', '∇', '∑', '∏',
    '√', '⊂', '⊃', '⊆', '⊇', '∪', '∩', '𝒯', '𝒪', '𝒜', '𝒞', '𝓔', '𝓜',
];

const GREEK_LATEX: &[(char, &str)] = &[
    ('α', "\\alpha"),
    ('β', "\\beta"),
    ('γ', "\\gamma"),
    ('δ', "\\delta"),
    ('ε', "\\varepsilon"),
    ('η', "\\eta"),
    ('θ', "\\theta"),
    ('λ', "\\lambda"),
    ('μ', "\\mu"),
    ('ν', "\\nu"),
    ('π', "\\pi"),
    ('ρ', "\\rho"),
    ('σ', "\\sigma"),
    ('τ', "\\tau"),
    ('φ', "\\phi"),
    ('ω', "\\omega"),
    ('Σ', "\\Sigma"),
    ('Ω', "\\Omega"),
    ('Π', "\\Pi"),
    ('Δ', "\\Delta"),
];

fn is_greek(c: char) -> bool {
    GREEK_LATEX.iter().any(|&(g, _)| g == c)
}

/// Mask code spans and existing math spans so the detector ignores them.
fn masked(line: &str) -> String {
    let mut out = String::with_capacity(line.len());
    let mut in_code = false;
    let mut in_math = false;
    for c in line.chars() {
        match c {
            '`' if !in_math => {
                in_code = !in_code;
                out.push(' ');
            }
            '$' if !in_code => {
                in_math = !in_math;
                out.push(' ');
            }
            _ if in_code || in_math => out.push(' '),
            _ => out.push(c),
        }
    }
    out
}

/// Does this line carry unpromoted math? (The gate for the LLM pass —
/// most lines answer no and never touch the model.)
pub fn needs_math_pass(line: &str) -> bool {
    let m = masked(line);
    if m.chars().any(|c| MATH_GLYPHS.contains(&c)) {
        return true;
    }
    // isolated Greek letter used as a variable (not inside a Greek word)
    let chars: Vec<char> = m.chars().collect();
    for (i, &c) in chars.iter().enumerate() {
        if is_greek(c) {
            let prev_greekish = i > 0 && chars[i - 1].is_alphabetic() && !chars[i - 1].is_ascii();
            let next_greekish =
                i + 1 < chars.len() && chars[i + 1].is_alphabetic() && !chars[i + 1].is_ascii();
            if !prev_greekish && !next_greekish {
                return true;
            }
        }
    }
    false
}

/// The default accumulating instruction file, compiled in; an on-disk copy
/// next to the binary's project (prompts/unicode-math.txt) overrides it so
/// edge cases can be added without a rebuild. It is `.txt` deliberately:
/// its line structure is data (one record per line), so a markdown
/// formatter — including this one — must not treat it as prose.
pub fn instructions() -> String {
    let compiled = include_str!("../prompts/unicode-math.txt");
    let disk = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("prompts/unicode-math.txt");
    std::fs::read_to_string(disk).unwrap_or_else(|_| compiled.to_string())
}

/// Ask a local ollama model for a conversion proposal for one line, via the
/// HTTP API (the CLI emits terminal redraw sequences even when piped).
pub fn propose(model: &str, line: &str) -> Result<String, String> {
    let prompt = format!("{}Input: {}\nOutput:", instructions(), line);
    let body = serde_json::json!({
        "model": model,
        "prompt": prompt,
        "stream": false,
        "options": { "temperature": 0.0 }
    })
    .to_string();
    let out = Command::new("curl")
        .args([
            "-s",
            "--max-time",
            "120",
            "http://localhost:11434/api/generate",
            "-d",
            &body,
        ])
        .stdin(Stdio::null())
        .output()
        .map_err(|e| format!("curl: {e}"))?;
    let v: serde_json::Value = serde_json::from_slice(&out.stdout)
        .map_err(|e| format!("ollama response parse: {e}"))?;
    let text = v["response"]
        .as_str()
        .ok_or_else(|| format!("no response field: {v}"))?;
    // first nonempty line of the completion is the converted line
    text.lines()
        .find(|l| !l.trim().is_empty())
        .map(|l| l.trim().to_string())
        .ok_or_else(|| "empty model output".into())
}

/// Deterministic post-processing: the FORMAT.md cross-renderer rules applied
/// inside the proposal's math spans (the model needn't know the house
/// standard; these normalize whatever it produced).
pub fn postprocess(line: &str) -> String {
    let mut out = String::with_capacity(line.len());
    let mut in_math = false;
    let mut in_code = false;
    let mut span = String::new();
    for c in line.chars() {
        match c {
            '`' if !in_math => {
                in_code = !in_code;
                out.push(c);
            }
            '$' if !in_code => {
                if in_math {
                    out.push_str(&fix_math_span(&span));
                    span.clear();
                }
                in_math = !in_math;
                out.push('$');
            }
            _ if in_math => span.push(c),
            _ => out.push(c),
        }
    }
    if in_math {
        // unbalanced $ — emit rest untouched; verification will reject
        out.push_str(&span);
    }
    out
}

fn fix_math_span(s: &str) -> String {
    let mut t = s.trim().to_string(); // R22: no spaces just inside $
    // R24: raw angles
    t = t.replace("\\lt", "\u{0}LT\u{0}").replace("\\gt", "\u{0}GT\u{0}");
    t = t.replace('<', "\\lt ").replace('>', "\\gt ");
    t = t.replace("\u{0}LT\u{0}", "\\lt").replace("\u{0}GT\u{0}", "\\gt");
    // R25: bare * → \ast
    let mut fixed = String::new();
    let mut prev = '\0';
    for c in t.chars() {
        if c == '*' && prev != '\\' {
            fixed.push_str("\\ast");
        } else {
            fixed.push(c);
        }
        prev = c;
    }
    t = fixed;
    // glued commands (the scorecard rule): \lt/\gt/\vert/\Vert/… + letter
    for cmd in ["\\lt", "\\gt", "\\leq", "\\geq", "\\vert", "\\Vert", "\\to", "\\in", "\\cdot"] {
        let mut r = String::new();
        let mut rest = t.as_str();
        while let Some(pos) = rest.find(cmd) {
            let after = pos + cmd.len();
            r.push_str(&rest[..after]);
            let next = rest[after..].chars().next();
            if next.is_some_and(|c| c.is_ascii_alphabetic()) {
                r.push(' ');
            }
            rest = &rest[after..];
        }
        r.push_str(rest);
        t = r;
    }
    // collapse doubled spaces the fixes may have introduced
    while t.contains("  ") {
        t = t.replace("  ", " ");
    }
    t.trim().to_string()
}

/// Prose-preservation verification: outside math spans, the proposal must be
/// the original minus its (now-converted) math tokens. Both sides are
/// reduced to their prose skeletons (math spans / math glyphs / Greek
/// removed, whitespace collapsed) and must match exactly.
pub fn preserves_prose(original: &str, proposal: &str) -> bool {
    // balanced $ count is a precondition
    if proposal.chars().filter(|&c| c == '$').count() % 2 != 0 {
        return false;
    }
    fn skeleton_original(s: &str) -> String {
        let mut out = String::new();
        for c in s.chars() {
            if MATH_GLYPHS.contains(&c) || is_greek(c) {
                out.push(' ');
            } else if matches!(c, '_' | '^' | '{' | '}' | '=' | '|' | '<' | '>' | '\\' | '*') {
                out.push(' ');
            } else {
                out.push(c);
            }
        }
        collapse(&out)
    }
    fn skeleton_proposal(s: &str) -> String {
        let mut out = String::new();
        let mut in_math = false;
        for c in s.chars() {
            match c {
                '$' => {
                    in_math = !in_math;
                    out.push(' ');
                }
                _ if in_math => out.push(' '),
                _ => out.push(c),
            }
        }
        skeleton_original(&out) // same char-class scrub for fairness
    }
    fn collapse(s: &str) -> String {
        s.split_whitespace().collect::<Vec<_>>().join(" ")
    }
    // Compare only multi-letter word tokens, in order: punctuation adjacency
    // legitimately shifts when spans form, and single-letter tokens (K, R, t)
    // may be absorbed into math spans as variables.
    fn binding_words(s: &str) -> Vec<String> {
        s.split(|c: char| !c.is_alphabetic())
            .filter(|w| w.len() > 1)
            .map(str::to_string)
            .collect()
    }
    binding_words(&skeleton_original(original)) == binding_words(&skeleton_proposal(proposal))
}

/// Math-content consistency gate: nothing inside the proposal's `$…$`
/// spans may be invented. Every LaTeX command must be either purely
/// structural or map back to a glyph present in the original; every
/// alphanumeric token must appear in the original line. (Motivating
/// incident: llama3.2 copied `\to M_{t+1}` out of a few-shot example into
/// a line that had neither — invisible to the prose gate.)
pub fn math_content_consistent(original: &str, proposal: &str) -> bool {
    // command → source glyphs, one of which must be present in the original
    let glyph_cmds: &[(&str, &[char])] = &[
        ("\\leq", &['≤', '<']),
        ("\\geq", &['≥', '>']),
        ("\\lt", &['<', '≤']),
        ("\\gt", &['>', '≥']),
        ("\\neq", &['≠']),
        ("\\to", &['→']),
        ("\\rightarrow", &['→']),
        ("\\mapsto", &['↦']),
        ("\\cdot", &['·', '*']),
        ("\\times", &['×']),
        ("\\in", &['∈']),
        ("\\infty", &['∞']),
        ("\\ast", &['*', '∗']),
        ("\\approx", &['≈']),
        ("\\equiv", &['≡']),
        ("\\partial", &['∂']),
        ("\\nabla", &['∇']),
        ("\\sum", &['∑']),
        ("\\prod", &['∏']),
        ("\\sqrt", &['√']),
        ("\\pm", &['±']),
        ("\\lVert", &['‖', '|']),
        ("\\rVert", &['‖', '|']),
        ("\\Vert", &['‖', '|']),
        ("\\lvert", &['|']),
        ("\\rvert", &['|']),
        ("\\vert", &['|']),
        ("\\mid", &['|']),
    ];
    let structural = ["\\mathcal", "\\mathbb", "\\mathbf", "\\mathrm", "\\text", "\\operatorname"];

    // collect span contents
    let mut spans = String::new();
    let mut in_math = false;
    for c in proposal.chars() {
        match c {
            '$' => {
                in_math = !in_math;
                spans.push(' '); // separator so adjacent spans can't merge tokens
            }
            _ if in_math => spans.push(c),
            _ => {}
        }
    }
    // check commands
    let mut rest = spans.as_str();
    while let Some(pos) = rest.find('\\') {
        rest = &rest[pos..];
        let cmd: String = rest
            .char_indices()
            .take_while(|&(i, c)| i == 0 || c.is_ascii_alphabetic())
            .map(|(_, c)| c)
            .collect();
        if cmd.len() > 1 {
            let known_greek = GREEK_LATEX.iter().find(|&&(_, l)| l == cmd);
            let known_glyph = glyph_cmds.iter().find(|&&(c, _)| c == cmd);
            let ok = if let Some(&(g, _)) = known_greek {
                original.contains(g)
            } else if let Some(&(_, glyphs)) = known_glyph {
                glyphs.iter().any(|&g| original.contains(g))
            } else {
                structural.contains(&cmd.as_str())
            };
            if !ok {
                return false;
            }
        }
        rest = &rest[cmd.len().max(1)..];
    }
    // check alphanumeric tokens (variable names, subscripts, numbers):
    // strip commands first so e.g. \mathcal's letters don't count
    let mut no_cmds = String::new();
    let mut chars = spans.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\\' {
            while chars.peek().is_some_and(|n| n.is_ascii_alphabetic()) {
                chars.next();
            }
        } else {
            no_cmds.push(c);
        }
    }
    no_cmds
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|t| !t.is_empty())
        .all(|t| original.contains(t))
}

/// Outcome of the math pass on one line.
pub enum MathOutcome {
    Unchanged,
    Converted(String),
    Flagged(String), // reason
}

/// Run the full per-line pipeline against a model.
pub fn promote_line(model: &str, line: &str) -> MathOutcome {
    if !needs_math_pass(line) {
        return MathOutcome::Unchanged;
    }
    match propose(model, line) {
        Err(e) => MathOutcome::Flagged(format!("model error: {e}")),
        Ok(raw) => {
            let candidate = postprocess(&raw);
            if candidate == line {
                MathOutcome::Unchanged
            } else if !preserves_prose(line, &candidate) {
                MathOutcome::Flagged("proposal altered prose; left unchanged".into())
            } else if !math_content_consistent(line, &candidate) {
                MathOutcome::Flagged("proposal invented math content; left unchanged".into())
            } else if needs_math_pass(&candidate) {
                MathOutcome::Flagged("residual unpromoted math after conversion".into())
            } else {
                MathOutcome::Converted(candidate)
            }
        }
    }
}

/// Deterministic R19 fix: `$$` delimiters that already sit on their own
/// lines get blank lines before/after (GitHub requires this to render
/// display math). Applied outside code fences only.
pub fn fix_display_math_blanks(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let mut out: Vec<String> = Vec::with_capacity(lines.len());
    let mut in_code = false;
    let mut in_display = false;
    for (i, l) in lines.iter().enumerate() {
        let t = l.trim();
        if t.starts_with("```") {
            in_code = !in_code;
            out.push(l.to_string());
            continue;
        }
        let one_line_block = t.len() > 4 && t.starts_with("$$") && t.ends_with("$$");
        if !in_code && !in_display && (t == "$$" || one_line_block) {
            // opening delimiter (or a complete one-line $$…$$ block)
            if !out.last().is_none_or(|p| p.trim().is_empty()) {
                out.push(String::new());
            }
            out.push(l.to_string());
            if t == "$$" {
                in_display = true;
            } else if lines.get(i + 1).is_some_and(|n| !n.trim().is_empty()) {
                out.push(String::new());
            }
            continue;
        }
        if !in_code && in_display && t == "$$" {
            // closing delimiter
            in_display = false;
            out.push(l.to_string());
            if lines.get(i + 1).is_some_and(|n| !n.trim().is_empty()) {
                out.push(String::new());
            }
            continue;
        }
        out.push(l.to_string());
    }
    let mut s = out.join("\n");
    if input.ends_with('\n') && !s.ends_with('\n') {
        s.push('\n');
    }
    s
}
