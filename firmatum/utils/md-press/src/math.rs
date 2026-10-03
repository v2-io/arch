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
    '√', '⊂', '⊃', '⊆', '⊇', '∪', '∩', '∅', '¬', '𝒯', '𝒪', '𝒜', '𝒞', '𝓔', '𝓜',
];

/// Script-capital glyphs and the ASCII letter each denotes — so a proposal's
/// `\mathcal{O}` token-checks against an original that wrote `𝒪`.
const SCRIPT_LETTERS: &[(char, char)] =
    &[('𝒯', 'T'), ('𝒪', 'O'), ('𝒜', 'A'), ('𝒞', 'C'), ('𝓔', 'E'), ('𝓜', 'M')];

const GREEK_LATEX: &[(char, &str)] = &[
    ('α', "\\alpha"),
    ('β', "\\beta"),
    ('γ', "\\gamma"),
    ('δ', "\\delta"),
    ('ε', "\\varepsilon"),
    ('η', "\\eta"),
    ('θ', "\\theta"),
    ('ι', "\\iota"),
    ('κ', "\\kappa"),
    ('λ', "\\lambda"),
    ('μ', "\\mu"),
    ('ν', "\\nu"),
    ('π', "\\pi"),
    ('ρ', "\\rho"),
    ('ξ', "\\xi"),
    ('σ', "\\sigma"),
    ('τ', "\\tau"),
    ('υ', "\\upsilon"),
    ('φ', "\\phi"),
    ('χ', "\\chi"),
    ('ψ', "\\psi"),
    ('ω', "\\omega"),
    ('ζ', "\\zeta"),
    ('Γ', "\\Gamma"),
    ('Θ', "\\Theta"),
    ('Λ', "\\Lambda"),
    ('Ξ', "\\Xi"),
    ('Σ', "\\Sigma"),
    ('Φ', "\\Phi"),
    ('Ψ', "\\Psi"),
    ('Ω', "\\Omega"),
    ('Π', "\\Pi"),
    ('Δ', "\\Delta"),
];

fn is_greek(c: char) -> bool {
    GREEK_LATEX.iter().any(|&(g, _)| g == c)
}

/// Glyphs this estate also uses as ordinary prose punctuation: `→` for
/// "becomes / leads to", `·` as a list separator, `×` for "times faster",
/// `≈`/`≤`/`±` for approximate quantities. Over the estate's 12k tracked
/// markdown files (2026-10-02), lines whose *only* trigger was `→` and/or
/// `·` were two thirds of all would-be model calls, and sampled ones were
/// nearly all prose. Such a glyph is math only in an operator role — when an
/// operand beside it looks like math (`η → 1`, `M_t · K`, `H ≤ 5`); between
/// words, numbers, links, or code it is punctuation, does not trigger the
/// model, and may not be converted (FEEDBACK-2026-08-12 §2).
const WEAK_GLYPHS: &[char] = &['→', '·', '×', '≈', '≤', '≥', '≠', '±', '≡', '√'];

fn is_weak(c: char) -> bool {
    WEAK_GLYPHS.contains(&c)
}

/// SI micro / ohm units written with a Greek letter (`31μs`, `2 μm`, `5 kΩ`)
/// are prose quantities, not variables.
fn greek_is_unit(chars: &[char], i: usize) -> bool {
    let c = chars[i];
    let next = chars.get(i + 1).copied();
    let prev = if i > 0 { Some(chars[i - 1]) } else { None };
    let after_word = chars.get(i + 2).is_some_and(|c| c.is_alphanumeric());
    match c {
        'μ' => next.is_some_and(|n| "smgLlVAFWJKH".contains(n)) && !after_word,
        'Ω' => prev.is_some_and(|p| "kMG".contains(p)),
        _ => false,
    }
}

/// Byte ranges of `s` that are not prose and must never be read or rewritten
/// by the math pass: inline code, existing `$…$` / `$$…$$` math, link
/// destinations `](…)`, wikilinks `[[…]]`, autolinks and HTML tags `<…>`, and
/// bare URLs. (Motivating incident: `\(1\)` inside a scraped link destination
/// became `$1$` and silently broke the URL — FEEDBACK-2026-08-12 §1. The
/// unwrap stage's parse sites the *line*; this is the inline-level mask
/// inside it.)
pub fn protected_ranges(s: &str) -> Vec<(usize, usize)> {
    let b = s.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < b.len() {
        let start = i;
        match b[i] {
            b'\\' => {
                // escaped char (incl. \$ and \`) is literal prose; it may be
                // multibyte (`\×` occurs in the estate)
                i += 1 + s[i + 1..].chars().next().map_or(0, char::len_utf8);
                continue;
            }
            b'`' => {
                let n = b[i..].iter().take_while(|&&c| c == b'`').count();
                let fence = &s[i..i + n];
                if let Some(rel) = s[i + n..].find(fence) {
                    let end = i + n + rel + n;
                    out.push((start, end));
                    i = end;
                } else {
                    i += n; // unmatched backticks are literal
                }
                continue;
            }
            b'$' => {
                let n = if b.get(i + 1) == Some(&b'$') { 2 } else { 1 };
                let delim = &s[i..i + n];
                let mut j = i + n;
                let mut end = None;
                while j < b.len() {
                    if b[j] == b'\\' {
                        j += 2;
                        continue;
                    }
                    if b[j..].starts_with(delim.as_bytes()) {
                        end = Some(j + n);
                        break;
                    }
                    j += 1;
                }
                // an unclosed `$` protects the rest of the line: a stray
                // price or shell variable is not ours to reinterpret
                let end = end.unwrap_or(b.len());
                out.push((start, end));
                i = end;
                continue;
            }
            b']' if b.get(i + 1) == Some(&b'(') => {
                let mut depth = 0usize;
                let mut j = i + 1;
                while j < b.len() {
                    match b[j] {
                        b'\\' => j += 1,
                        b'(' => depth += 1,
                        b')' => {
                            depth -= 1;
                            if depth == 0 {
                                break;
                            }
                        }
                        _ => {}
                    }
                    j += 1;
                }
                let end = (j + 1).min(b.len());
                out.push((i + 1, end));
                i = end;
                continue;
            }
            b'[' if b.get(i + 1) == Some(&b'[') => {
                if let Some(rel) = s[i..].find("]]") {
                    out.push((start, i + rel + 2));
                    i += rel + 2;
                    continue;
                }
            }
            b'<' => {
                if let Some(rel) = s[i..].find('>') {
                    let inner = &s[i + 1..i + rel];
                    let tagish = inner
                        .chars()
                        .next()
                        .is_some_and(|c| c.is_ascii_alphabetic() || c == '/' || c == '!');
                    // autolink / bare tag (`<https://…>`, `<br/>`, `</div>`)
                    // or a tag with attributes (`<a href="…">`); a math
                    // comparison like `< 3 and x >` has a space first and
                    // no `=`-attribute shape, so it stays prose
                    let simple = !inner.contains(char::is_whitespace);
                    let with_attrs = inner.contains('=')
                        && inner.split_whitespace().next().is_some_and(|t| {
                            t.chars().all(|c| c.is_ascii_alphanumeric() || c == '/')
                        });
                    if tagish && (simple || with_attrs) {
                        out.push((start, i + rel + 1));
                        i += rel + 1;
                        continue;
                    }
                }
            }
            b'h' | b'w' if s[i..].starts_with("http://")
                || s[i..].starts_with("https://")
                || s[i..].starts_with("www.") =>
            {
                let end = s[i..].find(char::is_whitespace).map_or(b.len(), |r| i + r);
                out.push((start, end));
                i = end;
                continue;
            }
            _ => {}
        }
        i += s[i..].chars().next().map_or(1, char::len_utf8);
    }
    out
}

fn in_ranges(ranges: &[(usize, usize)], pos: usize) -> bool {
    ranges.iter().any(|&(a, b)| pos >= a && pos < b)
}

/// `s` with every protected byte range blanked to spaces (same length in
/// chars is not preserved; byte positions are not needed by callers).
fn masked(s: &str) -> String {
    let ranges = protected_ranges(s);
    let mut out = String::with_capacity(s.len());
    for (i, c) in s.char_indices() {
        out.push(if in_ranges(&ranges, i) { ' ' } else { c });
    }
    out
}

/// Does a whitespace-delimited token read as a mathematical operand? Single
/// letters (`x`, `η`, `R`, with primes/stars/sub- or superscripts), anything
/// carrying `_`/`^` or an isolated Greek letter, function application
/// (`f(x)`, `ρ(A)`), or a strong math glyph. Words, numbers, labels like
/// `E1`, links and code are not.
fn mathish_token(tok: &str) -> bool {
    let t = tok.trim_matches(|c: char| ",;:.!?\"'“”‘’)([]{}*".contains(c));
    if t.is_empty() {
        return false;
    }
    let chars: Vec<char> = t.chars().collect();
    // sub/superscript syntax on a single-symbol base (`M_t`, `c_min`,
    // `η_t`, `x^2`, `\Sigma_t`) — but not snake_case identifiers
    // (`if_else_directive`, `inline_after_interp`), which session logs and
    // code-ish prose carry in bulk
    if let Some(k) = t.find(['_', '^']) {
        let base = &t[..k];
        let base = base.strip_prefix('\\').filter(|b| b.chars().all(|c| c.is_ascii_alphabetic())).map_or(base, |_| "x");
        let mut bc = base.chars();
        let one = bc.next().is_some_and(|c| c.is_alphabetic() || c.is_ascii_digit()) && bc.next().is_none();
        if one {
            return true;
        }
    }
    if chars.iter().any(|&c| MATH_GLYPHS.contains(&c) && !is_weak(c)) {
        return true;
    }
    for (i, &c) in chars.iter().enumerate() {
        if is_greek(c) && !greek_is_unit(&chars, i) {
            let in_word = (i > 0 && chars[i - 1].is_alphabetic() && !chars[i - 1].is_ascii())
                || chars.get(i + 1).is_some_and(|n| n.is_alphabetic() && !n.is_ascii());
            if !in_word {
                return true;
            }
        }
    }
    // a single letter, optionally primed/starred (x, x', η*), or a single
    // letter applied to an argument (f(x), R(t) — the trailing `)` was
    // trimmed above)
    if !chars[0].is_alphabetic() {
        return false;
    }
    let rest = &chars[1..];
    rest.iter().all(|&c| "'′*".contains(c)) || rest.first() == Some(&'(')
}

/// Role of the weak glyph at byte `pos` of `s`: true = operator (math).
fn weak_is_operator(s: &str, pos: usize, prot: &[(usize, usize)]) -> bool {
    let g = s[pos..].chars().next().unwrap();
    let before = &s[..pos];
    let after = &s[pos + g.len_utf8()..];
    // nearest operand on each side; an adjacent protected math span counts
    // as math, any other protected span (code, link) as non-math
    let left_end = before.trim_end_matches([' ', '\t']).len();
    let right_start = pos + g.len_utf8() + (after.len() - after.trim_start_matches([' ', '\t']).len());
    let side_is_math_span = |p: usize| {
        prot.iter()
            .any(|&(a, b)| (p >= a && p < b) && s[a..].starts_with('$'))
    };
    let left_tok = before[..left_end]
        .rsplit(char::is_whitespace)
        .next()
        .unwrap_or("");
    let right_tok = s[right_start..].split(char::is_whitespace).next().unwrap_or("");
    // glued glyph runs (`2×2×2`, `E1→E10`): judge the immediate neighbours
    let left_tok = left_tok.rsplit(|c| is_weak(c)).next().unwrap_or("");
    let right_tok = right_tok.split(|c| is_weak(c)).next().unwrap_or("");
    // An operator needs math on one side and nothing prose-like on the
    // other: `x → ∞`, `u→+1`, `n × k`, `H ≤ 5`. A word or code span on
    // either side makes it punctuation even beside math — "higher $\gamma$
    // → fewer partitions", "**A** outline only · **B** full tour".
    let left_math = left_end > 0 && side_is_math_span(left_end - 1);
    let right_math = right_start < s.len() && side_is_math_span(right_start);
    let (l, r) = (operand_class(left_tok, left_math), operand_class(right_tok, right_math));
    (l == Operand::Math || r == Operand::Math)
        && l != Operand::Prose
        && r != Operand::Prose
}

#[derive(PartialEq)]
enum Operand {
    Math,
    /// a number, or nothing at all (start/end of the text)
    Neutral,
    /// a word, code span, link, label — anything prose
    Prose,
}

fn operand_class(tok: &str, is_math_span: bool) -> Operand {
    if is_math_span || mathish_token(tok) {
        return Operand::Math;
    }
    let t = tok.trim_matches(|c: char| ",;:.!?\"'“”‘’)([]{}".contains(c));
    if t.is_empty()
        || t.chars().all(|c| c.is_ascii_digit() || ".,%+-−~∼".contains(c))
            && t.chars().any(|c| c.is_ascii_digit())
    {
        return Operand::Neutral;
    }
    Operand::Prose
}

/// Byte positions of weak glyphs in prose role (outside protected ranges).
fn prose_weak_positions(s: &str, prot: &[(usize, usize)]) -> Vec<usize> {
    s.char_indices()
        .filter(|&(i, c)| is_weak(c) && !in_ranges(prot, i) && !weak_is_operator(s, i, prot))
        .map(|(i, _)| i)
        .collect()
}

/// Does this text carry unpromoted math? (The gate for the LLM pass —
/// most lines answer no and never touch the model.) Triggers: a strong math
/// glyph, an isolated Greek letter that is not a unit, or a weak glyph in an
/// operator role — all outside protected ranges.
pub fn needs_math_pass(line: &str) -> bool {
    let prot = protected_ranges(line);
    let m = masked(line);
    let chars: Vec<char> = m.chars().collect();
    for (i, &c) in chars.iter().enumerate() {
        if MATH_GLYPHS.contains(&c) && !is_weak(c) {
            return true;
        }
        if is_greek(c) && !greek_is_unit(&chars, i) {
            let prev_greekish = i > 0 && chars[i - 1].is_alphabetic() && !chars[i - 1].is_ascii();
            let next_greekish =
                i + 1 < chars.len() && chars[i + 1].is_alphabetic() && !chars[i + 1].is_ascii();
            if !prev_greekish && !next_greekish {
                return true;
            }
        }
    }
    line.char_indices()
        .any(|(i, c)| is_weak(c) && !in_ranges(&prot, i) && weak_is_operator(line, i, &prot))
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

/// The default model, used when `--math` names none.
pub const DEFAULT_MODEL: &str = "llama3.2:3b";

/// A local model handle with a circuit breaker. The first failure that is
/// about the *server or model* rather than one line (ollama not running,
/// model not pulled, an error object instead of a completion) marks the
/// model down for the rest of the run, so a missing model costs one message
/// instead of one per line — the run proceeds unwrap-only and says so once.
pub struct Model {
    name: String,
    down: std::cell::RefCell<Option<String>>,
    /// test seam: answer proposals from a function instead of ollama
    fake: Option<Box<dyn Fn(&str) -> String>>,
}

impl Model {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), down: std::cell::RefCell::new(None), fake: None }
    }
    /// A model whose "proposals" come from `f` — for testing the gates
    /// against known-bad proposals without a live model.
    #[doc(hidden)]
    pub fn fake(f: impl Fn(&str) -> String + 'static) -> Self {
        Self { name: "fake".into(), down: std::cell::RefCell::new(None), fake: Some(Box::new(f)) }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Why the model was marked unavailable, if it was.
    pub fn down_reason(&self) -> Option<String> {
        self.down.borrow().clone()
    }
    fn propose(&self, text: &str) -> Result<String, ProposeError> {
        if let Some(r) = self.down.borrow().as_ref() {
            return Err(ProposeError::Unavailable(r.clone()));
        }
        if let Some(f) = &self.fake {
            return Ok(f(text));
        }
        let cached = cache_path(&self.name, text);
        if let Some(p) = &cached
            && let Ok(hit) = std::fs::read_to_string(p)
        {
            return Ok(hit);
        }
        let r = propose(&self.name, text);
        match &r {
            Err(ProposeError::Unavailable(why)) => *self.down.borrow_mut() = Some(why.clone()),
            Ok(proposal) => {
                if let Some(p) = &cached
                    && let Some(dir) = p.parent()
                    && std::fs::create_dir_all(dir).is_ok()
                {
                    let _ = std::fs::write(p, proposal); // a cache, not a record
                }
            }
            Err(ProposeError::Line(_)) => {}
        }
        r
    }
}

/// Where a model's proposal for `text` is cached: keyed by model name,
/// prompt file contents, and the exact text, so a new model or an edited
/// prompt misses cleanly. Only raw proposals are cached — every gate still
/// runs on every use, so a gate change takes effect immediately. The point
/// is that re-running md-press (or `--check`) over files it has already seen
/// costs no model time and gives the same answer, including for passages
/// whose proposals were refused. `MD_PRESS_CACHE=off` disables it; deleting
/// the directory is always safe.
fn cache_path(model: &str, text: &str) -> Option<std::path::PathBuf> {
    if std::env::var("MD_PRESS_CACHE").is_ok_and(|v| matches!(v.as_str(), "off" | "0" | "no")) {
        return None;
    }
    let base = std::env::var_os("XDG_CACHE_HOME")
        .map(std::path::PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| std::path::PathBuf::from(h).join(".cache")))?;
    // FNV-1a, 128-bit: stable across Rust versions (unlike DefaultHasher)
    let mut h: u128 = 0x6c62272e07bb014262b821756295c58d;
    for part in [model, "\0", &instructions(), "\0", text] {
        for &byte in part.as_bytes() {
            h ^= byte as u128;
            h = h.wrapping_mul(0x0000000001000000000000000000013B);
        }
    }
    let hex = format!("{h:032x}");
    Some(base.join("md-press/math").join(&hex[..2]).join(&hex[2..]))
}

enum ProposeError {
    /// the server or model is not usable — stop asking for this run
    Unavailable(String),
    /// this one proposal went wrong
    Line(String),
}

/// Ask a local ollama model for a conversion proposal for one line, via the
/// HTTP API (the CLI emits terminal redraw sequences even when piped).
fn propose(model: &str, line: &str) -> Result<String, ProposeError> {
    use ProposeError::{Line, Unavailable};
    let prompt = format!("{}Input: {}\nOutput:", instructions(), line);
    let body = serde_json::json!({
        "model": model,
        "prompt": prompt,
        "stream": false,
        // only the first line is used: stop there rather than let the model
        // run on inventing further Input/Output pairs, and cap the length
        // at what a faithful conversion of this line could need
        "options": {
            "temperature": 0.0,
            "stop": ["\n"],
            "num_predict": 64 + line.len() / 2
        }
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
        .map_err(|e| Unavailable(format!("could not run curl ({e})")))?;
    if out.stdout.is_empty() {
        return Err(Unavailable(
            "ollama is not answering at localhost:11434 (is `ollama serve` running?)".into(),
        ));
    }
    let v: serde_json::Value = serde_json::from_slice(&out.stdout)
        .map_err(|e| Unavailable(format!("unreadable ollama response ({e})")))?;
    if let Some(err) = v["error"].as_str() {
        let hint = if err.contains("not found") {
            format!(" — try `ollama pull {model}`")
        } else {
            String::new()
        };
        return Err(Unavailable(format!("ollama: {err}{hint}")));
    }
    let text = v["response"]
        .as_str()
        .ok_or_else(|| Unavailable(format!("ollama returned no completion: {v}")))?;
    // first nonempty line of the completion is the converted line
    text.lines()
        .find(|l| !l.trim().is_empty())
        .map(|l| l.trim().to_string())
        .ok_or_else(|| Line("empty model output".into()))
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
    t = t.replace(">>", "\\gg ").replace("<<", "\\ll ");
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
    // The split applies to the whole command name, never inside a real
    // longer command: `\infty` was being "fixed" to `\in fty` (likewise
    // `\int`, `\top`, `\cdots`, `\leqslant`, `\gtrsim`).
    const GLUED: [&str; 9] = ["lt", "gt", "leq", "geq", "vert", "Vert", "to", "in", "cdot"];
    const LONGER: &[&str] = &[
        "infty", "int", "iint", "iiint", "inf", "injlim", "intercal", "top", "cdots", "cdotp",
        "leqq", "leqslant", "geqq", "geqslant", "ltimes", "gtrless", "gtrsim", "gtrapprox",
        "gtreqless", "Vvert", "vertical",
    ];
    let mut r = String::with_capacity(t.len() + 4);
    let mut chars = t.char_indices().peekable();
    while let Some((i, c)) = chars.next() {
        r.push(c);
        if c != '\\' {
            continue;
        }
        let name: String = t[i + 1..].chars().take_while(|c| c.is_ascii_alphabetic()).collect();
        for _ in 0..name.chars().count() {
            chars.next();
        }
        match GLUED.iter().find(|g| name.starts_with(**g) && name.len() > g.len()) {
            Some(g) if !LONGER.contains(&name.as_str()) => {
                r.push_str(g);
                r.push(' ');
                r.push_str(&name[g.len()..]);
            }
            _ => r.push_str(&name),
        }
    }
    t = r;
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
        let mut it = s.chars().peekable();
        while let Some(c) = it.next() {
            // a subscript/superscript word (`ρ_base`, `α_min`) is math, not
            // a prose word the proposal must keep outside its spans
            if c == '_' || c == '^' {
                out.push(' ');
                if it.peek() == Some(&'{') {
                    while it.next().is_some_and(|n| n != '}') {}
                } else {
                    while it.peek().is_some_and(|n| n.is_alphanumeric()) {
                        it.next();
                    }
                }
                continue;
            }
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
    if binding_words(&skeleton_original(original)) != binding_words(&skeleton_proposal(proposal)) {
        return false;
    }
    // One-directional punctuation gate: the proposal may not introduce
    // punctuation the original lacked (observed: the model turned "¬agency"
    // into "$\lnot$-agency", minting a hyphen the word-token comparison
    // cannot see). Losing punctuation into a math span is fine; gaining any
    // outside one is not.
    fn punct_counts(s: &str) -> std::collections::HashMap<char, usize> {
        let mut m = std::collections::HashMap::new();
        let mut in_math = false;
        for c in s.chars() {
            if c == '$' {
                in_math = !in_math;
                continue;
            }
            if !in_math
                && !c.is_alphanumeric()
                && !c.is_whitespace()
                && !MATH_GLYPHS.contains(&c)
                && !is_greek(c)
            {
                *m.entry(c).or_insert(0) += 1;
            }
        }
        m
    }
    let orig_p = punct_counts(original);
    punct_counts(proposal)
        .iter()
        .all(|(c, n)| orig_p.get(c).is_some_and(|o| o >= n))
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
        ("\\gg", &['≫', '>']),
        ("\\ll", &['≪', '<']),
        ("\\to", &['→']),
        ("\\rightarrow", &['→']),
        ("\\mapsto", &['↦']),
        ("\\cdot", &['·', '*']),
        ("\\times", &['×']),
        ("\\in", &['∈']),
        ("\\notin", &['∉']),
        ("\\subset", &['⊂']),
        ("\\supset", &['⊃']),
        ("\\subseteq", &['⊆', '⊂']),
        ("\\supseteq", &['⊇', '⊃']),
        ("\\cup", &['∪']),
        ("\\cap", &['∩']),
        ("\\emptyset", &['∅']),
        ("\\varnothing", &['∅']),
        ("\\neg", &['¬']),
        ("\\lnot", &['¬']),
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
    // commands that carry structure, decoration, or a named operator, not
    // content of their own (their arguments are still token-checked)
    let structural = [
        "\\mathcal", "\\mathbb", "\\mathbf", "\\mathrm", "\\mathit", "\\mathsf", "\\boldsymbol",
        "\\text", "\\operatorname", "\\frac", "\\dfrac", "\\tfrac", "\\left", "\\right", "\\big",
        "\\Big", "\\prime", "\\hat", "\\bar", "\\tilde", "\\dot", "\\ddot", "\\vec", "\\ldots",
        "\\cdots", "\\dots", "\\quad", "\\qquad", "\\max", "\\min", "\\sup", "\\inf", "\\lim",
        "\\log", "\\exp", "\\ln", "\\det", "\\arg", "\\Pr", "\\limsup", "\\liminf",
    ];
    // LaTeX spellings the Greek table doesn't use but a model may
    let greek_alias: &[(&str, char)] =
        &[("\\epsilon", 'ε'), ("\\epsilon", 'ϵ'), ("\\varepsilon", 'ϵ'), ("\\varphi", 'φ'), ("\\phi", 'ϕ'), ("\\vartheta", 'θ'), ("\\varrho", 'ρ'), ("\\varsigma", 'σ')];

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
            let aliased = greek_alias.iter().any(|&(c, g)| c == cmd && original.contains(g));
            let ok = if aliased {
                true
            } else if let Some(&(g, _)) = known_greek {
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
    // "much greater/less than" may not weaken: `ν_M >> ν_Σ` came back as
    // `\nu_M \gt \nu_\Sigma` (2026-10-02), which every other check passed
    for (strong, cmd, glyph) in [(">>", "\\gg", '≫'), ("<<", "\\ll", '≪')] {
        let want = original.matches(strong).count() + original.matches(glyph).count();
        let have = proposal.matches(strong).count()
            + proposal.matches(glyph).count()
            + proposal.matches(cmd).count();
        if want != have {
            return false;
        }
    }
    no_cmds
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|t| !t.is_empty())
        .all(|t| {
            original.contains(t)
                || (t.len() == 1
                    && SCRIPT_LETTERS.iter().any(|&(g, a)| {
                        t == a.to_string() && original.contains(g)
                    }))
        })
}

/// Deterministic `\(...\)` → `$...$` normalization (house delimiters),
/// applied only when the interior itself indicates math — a LaTeX command
/// or a math glyph / Greek letter. A plain parenthetical that happens to be
/// written `\(...\)` is left alone, as is anything inside code or existing
/// `$` spans.
pub fn normalize_paren_math(text: &str) -> String {
    fn interior_is_math(s: &str) -> bool {
        let has_cmd = s
            .as_bytes()
            .windows(2)
            .any(|w| w[0] == b'\\' && w[1].is_ascii_alphabetic());
        if has_cmd || s.chars().any(|c| MATH_GLYPHS.contains(&c) || is_greek(c)) {
            return true;
        }
        // subscript/superscript syntax is math even without commands (M_t)
        if s.contains('_') || s.contains('^') {
            return true;
        }
        // a short space-free identifier (h, a, M2) is a variable, not prose
        // — nobody writes a one-word parenthetical with \( \) delimiters.
        // It must contain a letter: scrapers escape literal parentheses, and
        // `\(1\)` in a file name is a number in parentheses (it became `$1$`
        // inside a link destination, FEEDBACK-2026-08-12 §1).
        let t = s.trim();
        !t.is_empty()
            && t.len() <= 3
            && t.chars().all(|c| c.is_ascii_alphanumeric())
            && t.chars().any(|c| c.is_ascii_alphabetic())
    }
    let prot = protected_ranges(text);
    let mut out = String::with_capacity(text.len());
    let mut i = 0;
    while i < text.len() {
        let rest = &text[i..];
        let c = rest.chars().next().unwrap();
        if rest.starts_with("\\(")
            && !in_ranges(&prot, i)
            && let Some(rel) = rest[2..].find("\\)")
        {
            let interior = &rest[2..2 + rel];
            let crosses_protected = (i..i + 2 + rel + 2).any(|p| in_ranges(&prot, p));
            if !crosses_protected
                && !interior.contains('`')
                && !interior.contains('$')
                && interior_is_math(interior)
            {
                out.push('$');
                out.push_str(interior.trim());
                out.push('$');
                i += 2 + rel + 2;
                continue;
            }
        }
        out.push(c);
        i += c.len_utf8();
    }
    out
}

/// Where the candidate differs from the original, as byte ranges of the
/// original — or `None` if the candidate is *not* "the original with some
/// regions replaced by `$…$` spans". Every character of the candidate outside
/// its math spans must be the original's own bytes, in order: whitespace,
/// list markers, emphasis, and punctuation included. This is the gate that
/// was missing when the model turned `- **Label**: …` into `-Label: …`
/// (list marker and bold gone) and every word-level gate passed it.
pub fn edit_regions(original: &str, candidate: &str) -> Option<Vec<(usize, usize)>> {
    edit_pairs(original, candidate).map(|v| v.into_iter().map(|(r, _)| r).collect())
}

/// `edit_regions`, each paired with the content of the `$…$` span that
/// replaced it (spans and regions alternate, so the k-th region is the k-th
/// span).
fn edit_pairs(original: &str, candidate: &str) -> Option<Vec<((usize, usize), String)>> {
    let mut prose: Vec<String> = Vec::new();
    let mut maths: Vec<String> = Vec::new();
    let mut cur = String::new();
    let mut mcur = String::new();
    let mut in_math = false;
    let mut escaped = false;
    for c in candidate.chars() {
        if c == '$' && !escaped {
            if !in_math {
                prose.push(std::mem::take(&mut cur));
            } else {
                maths.push(std::mem::take(&mut mcur));
            }
            in_math = !in_math;
            continue;
        }
        escaped = c == '\\' && !escaped;
        if !in_math {
            cur.push(c);
        } else {
            mcur.push(c);
        }
    }
    if in_math {
        return None;
    }
    prose.push(cur);
    let n = prose.len() - 1;
    if n == 0 {
        return (original == candidate).then(Vec::new);
    }
    if !original.starts_with(prose[0].as_str()) {
        return None;
    }
    let mut p = prose[0].len();
    let mut regions = Vec::with_capacity(n);
    for (k, seg) in prose.iter().enumerate().skip(1) {
        // each replaced region is non-empty
        let q = p + original.get(p..)?.chars().next()?.len_utf8();
        let start = if k == n {
            let s = original.len().checked_sub(seg.len())?;
            (s >= q && original.is_char_boundary(s) && original[s..] == **seg).then_some(s)?
        } else {
            q + original.get(q..)?.find(seg.as_str())?
        };
        regions.push(((p, start), maths[k - 1].clone()));
        p = start + seg.len();
    }
    Some(regions)
}

/// The alignment gate: edits confined to replaced regions that contain no
/// protected text (code, links, URLs — existing math may be absorbed into a
/// new span), no prose-role weak glyph, and no markdown emphasis or code
/// delimiters; and every pre-existing math span's content survives.
pub fn edits_confined(original: &str, candidate: &str) -> bool {
    let Some(pairs) = edit_pairs(original, candidate) else {
        return false;
    };
    let prot = protected_ranges(original);
    let prose_weak = prose_weak_positions(original, &prot);
    for ((a, b), span) in &pairs {
        let (a, b) = (*a, *b);
        let r = &original[a..b];
        if !operands_survive(r, span) {
            // observed (llama3.2, 2026-10-02): `ρ/R` → `\rho/\rho`,
            // `𝒜` → `\mathcal{S}`, `||δ||` → `\delta`, `Σ_t.` → `\Sigma_t`
            return false;
        }
        if r.contains("**") || r.contains('`') {
            return false;
        }
        if !region_reads_as_math(r) {
            // observed: "PID + non-positive-real plant" → "PID $+$ non-…"
            return false;
        }
        if prose_weak.iter().any(|&p| p >= a && p < b) {
            return false;
        }
        for &(pa, pb) in &prot {
            let overlaps = pa < b && pb > a;
            let is_math = original[pa..].starts_with('$');
            if overlaps && (!is_math || pa < a || pb > b) {
                return false;
            }
        }
    }
    // pre-existing math keeps its content (it may be merged into a wider span)
    prot.iter().all(|&(a, b)| {
        let span = &original[a..b];
        if !span.starts_with('$') {
            return true;
        }
        let inner = span.trim_matches('$');
        candidate.contains(inner)
    })
}

/// Does everything the replaced text says survive into the span that
/// replaced it? Every ASCII letter-run and digit-run (`R`, `min`, `2`), every
/// Greek letter (as its command, a known alias, or itself), every script
/// capital (as its letter), norm and absolute-value bars, and a trailing full
/// stop or comma. The content-consistency gate only asks that a span invent
/// nothing *anywhere in the line*, which let a model swap one variable for
/// another that happened to appear elsewhere.
fn operands_survive(region: &str, span: &str) -> bool {
    let stripped: String = {
        let mut out = String::new();
        let mut it = span.chars().peekable();
        while let Some(c) = it.next() {
            if c == '\\' {
                while it.peek().is_some_and(|n| n.is_ascii_alphabetic()) {
                    it.next();
                }
                out.push(' ');
            } else {
                out.push(c);
            }
        }
        out
    };
    // existing $…$ inside the region: its content is checked elsewhere
    let r: String = {
        let prot = protected_ranges(region);
        region.char_indices().map(|(i, c)| if in_ranges(&prot, i) { ' ' } else { c }).collect()
    };
    let mut runs: Vec<String> = Vec::new();
    let mut cur = String::new();
    let mut kind = 0u8;
    for c in r.chars().chain(std::iter::once(' ')) {
        let k = if c.is_ascii_alphabetic() { 1 } else if c.is_ascii_digit() { 2 } else { 0 };
        if k != kind && !cur.is_empty() {
            runs.push(std::mem::take(&mut cur));
        }
        if k != 0 {
            cur.push(c);
        }
        kind = k;
    }
    for t in &runs {
        if !stripped.contains(t.as_str()) && !span.contains(&format!("\\{t}")) {
            return false;
        }
    }
    for g in r.chars().filter(|&c| is_greek(c)) {
        let cmd_ok = GREEK_LATEX.iter().any(|&(c, l)| c == g && span.contains(l))
            || [("ε", "\\epsilon"), ("φ", "\\varphi"), ("θ", "\\vartheta"), ("ρ", "\\varrho"), ("σ", "\\varsigma")]
                .iter()
                .any(|&(c, l)| c.starts_with(g) && span.contains(l));
        if !cmd_ok && !span.contains(g) {
            return false;
        }
    }
    for &(g, letter) in SCRIPT_LETTERS {
        if r.contains(g) && !stripped.contains(letter) && !span.contains(g) {
            return false;
        }
    }
    let norm_in = r.contains('‖') || r.contains("||");
    let norm_out = ["\\lVert", "\\rVert", "\\Vert", "\\|", "||", "‖"].iter().any(|n| span.contains(n));
    if norm_in && !norm_out {
        return false;
    }
    if !norm_in && r.contains('|') && !(span.contains('|') || span.contains("vert") || span.contains("\\mid")) {
        return false;
    }
    // every `*` survives, as `*` or `\ast`: a star on a symbol (`η*`) is
    // math, but a single-`*` emphasis marker deleted along the way is not
    // (2026-10-02 verification: two llama3.2 outputs dropped one and passed,
    // because only `**` was checked)
    let stars = r.matches('*').count();
    if stars > 0 && span.matches('*').count() + span.matches("\\ast").count() < stars {
        return false;
    }
    let tail = r.trim_end().chars().last();
    if let Some(p) = tail.filter(|c| ".,;".contains(*c)) {
        if !span.trim_end().ends_with(p) {
            return false;
        }
    }
    true
}

/// Does a replaced region of the original read as mathematics on its own?
/// A trigger (strong glyph, Greek, operator-role weak glyph), LaTeX, an
/// existing span, or a run of math-looking operands, numbers, and operator
/// symbols with at least one operand or relation in it. A lone prose `+`
/// or a bracketed word does not.
fn region_reads_as_math(r: &str) -> bool {
    if needs_math_pass(r) || r.contains('\\') || r.contains('$') {
        return true;
    }
    let toks: Vec<&str> = r.split_whitespace().collect();
    let symbolic = |t: &str| t.chars().all(|c| "+-−*/=<>()[]|,.'′!^_{}0123456789%".contains(c));
    let all_ok = toks.iter().all(|t| mathish_token(t) || symbolic(t));
    let anchor = toks.iter().any(|t| mathish_token(t)) || r.contains(['=', '<', '>']);
    !toks.is_empty() && all_ok && anchor
}

/// Outcome of the math pass on one prose chunk.
pub enum MathOutcome {
    Unchanged,
    Converted(String),
    /// The model stage failed its gates. `reason` says why; the text carries
    /// any deterministic `\(...\)` normalization that already held (None
    /// when nothing safe survived — the chunk stays byte-identical).
    Flagged(String, Option<String>),
    /// The model is unavailable this run; carries any deterministic
    /// normalization that held, like `Flagged`.
    Unavailable(Option<String>),
}

/// Full pipeline on one prose chunk (the caller hands us parser-delimited
/// prose with its markdown prefix already removed): deterministic `\(...\)`
/// normalization, then the model pass under its gates against the
/// normalized base. Only the trimmed core goes to the model; surrounding
/// whitespace is reattached verbatim.
pub fn promote_text(model: &Model, text: &str) -> MathOutcome {
    let base = normalize_paren_math(text);
    let det = if base != text { Some(base.clone()) } else { None };
    let settle = |reason: String| MathOutcome::Flagged(reason, det.clone());
    if !needs_math_pass(&base) {
        return match det {
            Some(b) => MathOutcome::Converted(b),
            None => MathOutcome::Unchanged,
        };
    }
    let core = base.trim_matches([' ', '\t']);
    let lead = &base[..base.len() - base.trim_start_matches([' ', '\t']).len()];
    let trail = &base[base.trim_end_matches([' ', '\t']).len()..];
    match model.propose(core) {
        Err(ProposeError::Unavailable(_)) => MathOutcome::Unavailable(det),
        Err(ProposeError::Line(e)) => settle(format!("model error: {e}")),
        Ok(raw) => {
            let candidate = postprocess(&raw);
            if candidate == core {
                match det {
                    Some(b) => MathOutcome::Converted(b),
                    None => MathOutcome::Unchanged,
                }
            } else if !preserves_prose(core, &candidate) {
                settle(format!("proposal altered prose: «{candidate}»"))
            } else if !math_content_consistent(core, &candidate) {
                settle(format!("proposal invented math content: «{candidate}»"))
            } else if !edits_confined(core, &candidate) {
                settle(format!("proposal changed text outside the math it promoted: «{candidate}»"))
            } else if needs_math_pass(&candidate) {
                settle(format!("residual unpromoted math after conversion: «{candidate}»"))
            } else {
                MathOutcome::Converted(format!("{lead}{candidate}{trail}"))
            }
        }
    }
}

/// Back-compat name for the whole-line case.
pub fn promote_line(model: &Model, line: &str) -> MathOutcome {
    promote_text(model, line)
}

/// Split a prose line into (markdown prefix, body, trailing whitespace /
/// hard-break marker). The prefix — indentation, `>` markers, list markers,
/// task boxes, heading `#`s, footnote/link-definition labels — is structure,
/// and the model must never see it: a model that drops the space after `-`
/// unmakes a list item, and one that drops indentation un-nests it (both
/// observed 2026-10-02). The trailing `  ` / `\` is a hard break.
pub fn split_structure(s: &str) -> (&str, &str, &str) {
    let mut i = 0;
    loop {
        i = s.len() - s[i..].trim_start_matches([' ', '\t']).len();
        let r = &s[i..];
        if r.starts_with('>') {
            i += 1;
            continue;
        }
        let ws_after = |k: usize| r[k..].starts_with([' ', '\t']);
        if r.starts_with(['-', '*', '+']) && ws_after(1) {
            i += 1;
            continue;
        }
        let d = r.bytes().take_while(u8::is_ascii_digit).count();
        if (1..=9).contains(&d) && r[d..].starts_with(['.', ')']) && ws_after(d + 1) {
            i += d + 1;
            continue;
        }
        if (r.starts_with("[ ]") || r.starts_with("[x]") || r.starts_with("[X]")) && ws_after(3) {
            i += 3;
            continue;
        }
        let h = r.bytes().take_while(|&c| c == b'#').count();
        if (1..=6).contains(&h) && (ws_after(h) || r.len() == h) {
            i += h;
            continue;
        }
        if r.starts_with('[')
            && let Some(c) = r.find("]:")
            && (r.starts_with("[^") || !r[1..c].contains(char::is_whitespace))
        {
            i += c + 2;
            continue;
        }
        break;
    }
    let (pre, rest) = s.split_at(i);
    let mut body_end = rest.trim_end_matches([' ', '\t']).len();
    if body_end == rest.len() && rest.ends_with('\\') && !rest.ends_with("\\\\") {
        body_end -= 1;
    }
    (pre, &rest[..body_end], &rest[body_end..])
}

/// Split prose at prose-role weak glyphs: `(text, is_separator)` pieces whose
/// concatenation is the input. A math expression never spans a prose
/// separator, so each piece can be promoted (and gated) on its own — the
/// model never sees, and so can never convert, the house `→` / `·` between
/// phrases, and one stubborn expression no longer blocks the rest of a line.
pub fn split_at_prose_separators(s: &str) -> Vec<(&str, bool)> {
    let prot = protected_ranges(s);
    let mut out = Vec::new();
    let mut cur = 0;
    for p in prose_weak_positions(s, &prot) {
        if p < cur {
            continue;
        }
        let g = s[p..].chars().next().unwrap().len_utf8();
        let left = s[..p].trim_end_matches([' ', '\t']).len().max(cur);
        let after = &s[p + g..];
        let right = p + g + (after.len() - after.trim_start_matches([' ', '\t']).len());
        if left > cur {
            out.push((&s[cur..left], false));
        }
        out.push((&s[left..right], true));
        cur = right;
    }
    if cur < s.len() {
        out.push((&s[cur..], false));
    }
    out
}

/// Split prose at sentence boundaries (`. ` / `? ` / `! ` followed by a
/// capital or an opening mark, outside protected ranges): `(text,
/// is_separator)` pieces whose concatenation is the input. No expression
/// spans a sentence end, and the model has to re-emit everything it is
/// shown, so a 1,000-character unwrapped paragraph with one `η` in it costs
/// one sentence of generation instead of the paragraph.
pub fn split_sentences(s: &str) -> Vec<(&str, bool)> {
    let prot = protected_ranges(s);
    let b = s.as_bytes();
    let mut out = Vec::new();
    let mut cur = 0;
    let mut i = 0;
    while i + 2 < b.len() {
        if matches!(b[i], b'.' | b'?' | b'!') && b[i + 1] == b' ' && !in_ranges(&prot, i) {
            let mut j = i + 1;
            while j < b.len() && b[j] == b' ' {
                j += 1;
            }
            let next = s[j..].chars().next();
            if next.is_some_and(|c| c.is_uppercase() || "*_([\"'“`".contains(c)) {
                out.push((&s[cur..=i], false));
                out.push((&s[i + 1..j], true));
                cur = j;
                i = j;
                continue;
            }
        }
        i += 1;
    }
    if cur < s.len() {
        out.push((&s[cur..], false));
    }
    out
}

/// Result of promoting one prose site (a whole line or one table cell).
pub struct SiteResult {
    pub text: String,
    /// per-chunk reasons a model proposal was refused (text left as written)
    pub flags: Vec<String>,
    /// the model was unavailable for at least one chunk that wanted it
    pub skipped: bool,
}

/// The math pass for one prose site: structure split off and reattached
/// verbatim, body split at prose separators, each piece promoted under the
/// gates independently.
pub fn promote_site(model: &Model, text: &str) -> SiteResult {
    let (pre, body, suf) = split_structure(text);
    let mut out = String::with_capacity(text.len() + 16);
    out.push_str(pre);
    let mut flags = Vec::new();
    let mut skipped = false;
    let pieces = split_at_prose_separators(body).into_iter().flat_map(|(p, sep)| {
        if sep { vec![(p, true)] } else { split_sentences(p) }
    });
    for (piece, is_sep) in pieces {
        if is_sep {
            out.push_str(piece);
            continue;
        }
        match promote_text(model, piece) {
            MathOutcome::Unchanged => out.push_str(piece),
            MathOutcome::Converted(c) => out.push_str(&c),
            MathOutcome::Flagged(why, det) => {
                flags.push(why);
                out.push_str(det.as_deref().unwrap_or(piece));
            }
            MathOutcome::Unavailable(det) => {
                skipped = true;
                out.push_str(det.as_deref().unwrap_or(piece));
            }
        }
    }
    out.push_str(suf);
    SiteResult { text: out, flags, skipped }
}

/// Is this (trimmed) line blank, allowing for blockquote markers?
fn blankish(l: &str) -> bool {
    l.trim_matches(|c| c == ' ' || c == '\t' || c == '>').is_empty()
}

/// The blank line to insert beside `l` without leaving its container: a
/// blockquoted `$$` needs `>` (a truly empty line would end the quote).
fn container_blank(l: &str) -> String {
    let lead: String = l.chars().take_while(|&c| c == ' ' || c == '\t' || c == '>').collect();
    match lead.rfind('>') {
        Some(k) => lead[..=k].to_string(),
        None => String::new(),
    }
}

/// Deterministic R19 fix: `$$` delimiters that already sit on their own
/// lines get blank lines before/after (GitHub requires this to render
/// display math). Applied outside code fences only.
pub fn fix_display_math_blanks(input: &str) -> String {
    fix_display_math_blanks_at(input, None)
}

/// As `fix_display_math_blanks`, but only lines the parse marked as prose
/// sites can be delimiters — `$$` inside indented code, HTML, or
/// frontmatter is never touched.
pub fn fix_display_math_blanks_at(input: &str, sites: Option<&[crate::MathSite]>) -> String {
    // lines keep their own endings (LF/CRLF/none): rebuilding with `lines()`
    // + `join` dropped a trailing blank line per run and flattened CRLF
    let raw: Vec<&str> = input.split_inclusive('\n').collect();
    let body = |r: &str| -> String { r.trim_end_matches(['\n', '\r']).to_string() };
    let eol = |r: &str| -> String { r[r.trim_end_matches(['\n', '\r']).len()..].to_string() };
    let lines: Vec<String> = raw.iter().map(|r| body(r)).collect();
    let mut out: Vec<String> = Vec::with_capacity(raw.len());
    let mut in_code = false;
    let mut in_display = false;
    for (i, l) in lines.iter().enumerate() {
        let nl = { let e = eol(raw[i]); if e.is_empty() { "\n".to_string() } else { e } };
        let t = l.trim();
        let t_noquote = t.trim_start_matches(['>', ' ']);
        if t_noquote.starts_with("```") || t_noquote.starts_with("~~~") {
            in_code = !in_code;
            out.push(raw[i].to_string());
            continue;
        }
        let is_site = sites.is_none_or(|s| s.get(i).is_some_and(|x| *x != crate::MathSite::None));
        let one_line_block = t_noquote.len() > 4 && t_noquote.starts_with("$$") && t_noquote.ends_with("$$");
        if !in_code && is_site && !in_display && (t_noquote == "$$" || one_line_block) {
            // opening delimiter (or a complete one-line $$…$$ block)
            if !out.last().is_none_or(|p| blankish(p.trim_end_matches(['\n', '\r']))) {
                out.push(container_blank(l) + &nl);
            }
            out.push(raw[i].to_string());
            if t_noquote == "$$" {
                in_display = true;
            } else if lines.get(i + 1).is_some_and(|n| !blankish(n)) {
                out.push(container_blank(l) + &nl);
            }
            continue;
        }
        if !in_code && is_site && in_display && t_noquote == "$$" {
            // closing delimiter
            in_display = false;
            out.push(raw[i].to_string());
            if lines.get(i + 1).is_some_and(|n| !blankish(n)) {
                out.push(container_blank(l) + &nl);
            }
            continue;
        }
        out.push(raw[i].to_string());
    }
    out.concat()
}
