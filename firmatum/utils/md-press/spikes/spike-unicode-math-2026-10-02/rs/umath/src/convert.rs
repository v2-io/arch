//! Stage 3: render spans, guard, and write. Port of `render_span`, `tex_ok`,
//! `dollar_hazard`, `normalize_paren_math`, `symbols_in`, `only_house_labels`,
//! `convert` (v3) / `convert_once` + `convert` + `_spans_vs` (v4).

use crate::consts::*;
use crate::lex::{self, K, Tok, find, protected_ranges, slice, starts_with};
use crate::spans::*;
use crate::term::*;
use crate::uni::*;
use crate::{Error, Span, Ver};

fn render_span(us: &[Unit], toks: &[Tok], a: usize, b: usize) -> Result<Option<String>, Error> {
    let Some(_deep) = crate::Deep::enter() else { return Ok(None) };
    let mut parts: Vec<String> = Vec::new();
    let mut skip_to_tok: usize = 0; // python: -1; `u.a < skip` false for all u when 0
    let mut k = a;
    while k < b {
        let u = &us[k];
        let kk = k;
        k += 1;
        if u.a < skip_to_tok || u.kind == K::Ws {
            continue;
        }
        if u.kind == K::Term {
            let t = u.tm();
            if t.m == M::Word
                && let Some(last) = parts.last_mut()
                && last.starts_with(r"\text{")
                && last.ends_with('}')
                && kk > a
                && us[kk - 1].kind == K::Ws
            {
                last.pop();
                last.push(' ');
                last.push_str(&u.text);
                last.push('}');
                continue;
            }
            parts.push(t.latex.clone());
        } else if u.kind == K::Op
            && u.is("√")
            && kk + 1 < b
            && us[kk + 1].kind == K::Open
            && us[kk + 1].is("(")
            && !toks[us[kk + 1].a].sp
        {
            let mut d = 0i64;
            let mut e = None;
            for e2 in kk + 1..b {
                d += (us[e2].kind == K::Open) as i64;
                d -= (us[e2].kind == K::Close) as i64;
                if d == 0 {
                    e = Some(e2);
                    break;
                }
            }
            let Some(e) = e else { return Ok(None) };
            let Some(inner) = render_span(us, toks, kk + 2, e)? else { return Ok(None) };
            parts.push(format!("\\sqrt{{{inner}}}"));
            skip_to_tok = us[e].b;
            continue;
        } else if u.kind == K::Op && u.is("√") && !(kk + 1 < b && us[kk + 1].kind == K::Term) {
            return Ok(None);
        } else if u.kind == K::Op && u.is("√") {
            let mut e = kk + 2;
            if e < b && us[e].kind == K::Open && !toks[us[e].a].sp {
                let mut d = 0i64;
                for e2 in e..b {
                    d += (us[e2].kind == K::Open) as i64;
                    d -= (us[e2].kind == K::Close) as i64;
                    if d == 0 {
                        e = e2 + 1;
                        break;
                    }
                }
            }
            let Some(inner) = render_span(us, toks, kk + 1, e)? else { return Ok(None) };
            let inner = if inner.starts_with('(') && inner.ends_with(')') && e == kk + 2 {
                let v: Vec<char> = inner.chars().collect();
                if v.len() >= 2 { v[1..v.len() - 1].iter().collect() } else { String::new() }
            } else {
                inner
            };
            parts.push(format!("\\sqrt{{{inner}}}"));
            skip_to_tok = us[e - 1].b;
            continue;
        } else {
            let piece: String = match u.kind {
                K::Op => op_latex(&u.text),
                K::Hyph => if u.cls == "range" { r"\text{–}".into() } else { "-".into() },
                K::Dash if u.cls == "range" => r"\text{–}".into(),
                K::Punct => u.text.clone(),
                K::Open => open_latex(&u.text).unwrap().into(),
                K::Close => close_latex(&u.text).unwrap().into(),
                K::Bar => r"\mid".into(),
                K::Dbar => r"\Vert".into(),
                K::Ellip => r"\ldots".into(),
                K::Star => r"\ast".into(),
                K::Tilde => r"\sim".into(),
                K::Other if u.is("%") => r"\%".into(),
                K::Other if op_str(&u.text).is_some() => op_str(&u.text).unwrap().0.into(),
                K::Caret | K::Us | K::Sub | K::Sup => {
                    let tk = &toks[u.a];
                    if matches!(u.kind, K::Sub | K::Sup) {
                        let ch = subsup_chars(&tk.text);
                        let Some(last) = parts.last_mut() else { return Err(Error::IndexError("render_span: script with no base")) };
                        last.push_str(if u.kind == K::Sub { "_" } else { "^" });
                        last.push_str(&brace(&ch));
                    } else {
                        let r = parse_script_arg(toks, u.b);
                        let (Some((r0, r1, _)), Some(last)) = (r, parts.last_mut()) else { return Ok(None) };
                        last.push_str(if u.kind == K::Us { "_" } else { "^" });
                        last.push_str(&brace(&r1));
                        skip_to_tok = r0;
                    }
                    continue;
                }
                K::Esc if u.cls == "|" => r"\vert".into(),
                _ => return Ok(None),
            };
            parts.push(piece);
        }
    }
    Ok(Some(join_latex(&parts)))
}

// ------------------------------------------------------------ tex_ok

const ARG_CMDS: &[(&str, usize)] = &[
    (r"\sqrt", 1), (r"\frac", 2), (r"\text", 1), (r"\mathrm", 1), (r"\mathcal", 1), (r"\mathbb", 1), (r"\mathbf", 1),
    (r"\mathfrak", 1), (r"\mathsf", 1), (r"\mathtt", 1), (r"\boldsymbol", 1), (r"\operatorname", 1), (r"\hat", 1),
    (r"\bar", 1), (r"\tilde", 1), (r"\dot", 1), (r"\ddot", 1), (r"\vec", 1), (r"\breve", 1), (r"\check", 1),
    (r"\acute", 1), (r"\grave", 1),
];

fn arg_cmd(t: &str) -> Option<usize> {
    ARG_CMDS.iter().find(|(k, _)| *k == t).map(|(_, n)| *n)
}

/// re.findall(r'\\[A-Za-z]+|\\.|[{}_^]|\s+|.', lat)
fn tex_tokens(lat: &str) -> Vec<String> {
    let s: Vec<char> = lat.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < s.len() {
        let c = s[i];
        if c == '\\' && i + 1 < s.len() && s[i + 1].is_ascii_alphabetic() {
            let mut j = i + 1;
            while j < s.len() && s[j].is_ascii_alphabetic() {
                j += 1;
            }
            out.push(slice(&s, i, j));
            i = j;
        } else if c == '\\' && i + 1 < s.len() && s[i + 1] != '\n' {
            out.push(slice(&s, i, i + 2));
            i += 2;
        } else if matches!(c, '{' | '}' | '_' | '^') {
            out.push(c.to_string());
            i += 1;
        } else if is_space(c) {
            let mut j = i;
            while j < s.len() && is_space(s[j]) {
                j += 1;
            }
            out.push(slice(&s, i, j));
            i = j;
        } else if c != '\n' {
            out.push(c.to_string());
            i += 1;
        } else {
            i += 1; // unreachable: '\n' is \s
        }
    }
    out
}

fn tok_isspace(t: &str) -> bool {
    !t.is_empty() && t.chars().all(is_space)
}

pub fn tex_ok(lat: &str) -> Result<bool, Error> {
    let v: Vec<char> = lat.chars().collect();
    for (i, &c) in v.iter().enumerate() {
        if matches!(c, '&' | '#' | '%') && !(i > 0 && v[i - 1] == '\\') {
            return Ok(false);
        }
    }
    let toks = tex_tokens(lat);
    let mut depth = 0i64;
    for t in &toks {
        depth += (t == "{") as i64;
        depth -= (t == "}") as i64;
        if depth < 0 {
            return Ok(false);
        }
    }
    if depth != 0 {
        return Ok(false);
    }
    fn arg_end(toks: &[String], mut i: usize) -> Option<usize> {
    let _deep = crate::Deep::enter()?;
        while i < toks.len() && tok_isspace(&toks[i]) {
            i += 1;
        }
        if i >= toks.len() || matches!(toks[i].as_str(), "}" | "_" | "^") {
            return None;
        }
        if toks[i] == "{" {
            let mut d = 0i64;
            for j in i..toks.len() {
                d += (toks[j] == "{") as i64;
                d -= (toks[j] == "}") as i64;
                if d == 0 {
                    return Some(j + 1);
                }
            }
            return None;
        }
        if let Some(na) = arg_cmd(&toks[i]) {
            let mut e = i + 1;
            for _ in 0..na {
                e = arg_end(toks, e)?;
            }
            return Some(e);
        }
        Some(i + 1)
    }
    // per brace level: (sub seen, sup seen) on the current atom
    let mut stack: Vec<(bool, bool)> = vec![(false, false)];
    let mut i = 0;
    while i < toks.len() {
        let t = toks[i].as_str();
        if let Some(na) = arg_cmd(t) {
            let mut e = i + 1;
            for _ in 0..na {
                match arg_end(&toks, e) {
                    Some(x) => e = x,
                    None => return Ok(false),
                }
            }
            *stack.last_mut().unwrap() = (false, false);
            i = e;
            continue;
        }
        if t == "_" || t == "^" {
            let top = stack.last_mut().unwrap();
            let seen = if t == "_" { &mut top.0 } else { &mut top.1 };
            if *seen {
                return Ok(false);
            }
            *seen = true;
            match arg_end(&toks, i + 1) {
                Some(e) => i = e,
                None => return Ok(false),
            }
            continue;
        }
        if t == "'" {
            i += 1;
            continue;
        }
        if t == "{" {
            stack.push((false, false));
            i += 1;
            continue;
        }
        if t == "}" {
            stack.pop();
            let Some(top) = stack.last_mut() else { return Err(Error::IndexError("tex_ok: stack underflow")) };
            *top = (false, false);
            i += 1;
            continue;
        }
        if !tok_isspace(t) {
            *stack.last_mut().unwrap() = (false, false);
        }
        i += 1;
    }
    Ok(true)
}

// ------------------------------------------------------------ guards

fn strip_chars<'a>(s: &'a [char]) -> &'a [char] {
    let mut a = 0;
    let mut b = s.len();
    while a < b && is_space(s[a]) {
        a += 1;
    }
    while b > a && is_space(s[b - 1]) {
        b -= 1;
    }
    &s[a..b]
}

fn contains(s: &[char], pat: &str) -> bool {
    let p: Vec<char> = pat.chars().collect();
    find(s, &p, 0).is_some()
}

fn dollar_hazard(s: &[char], ver: Ver) -> Result<bool, Error> {
    if ver >= Ver::V4 {
        let st = strip_chars(s);
        let hit = if starts_with(st, 0, "$$") {
            let mid: &[char] = if st.len() > 4 { &st[2..st.len() - 2] } else { &[] };
            contains(mid, "$$")
        } else {
            contains(s, "$$")
        };
        if hit {
            return Ok(true);
        }
    }
    for (a, b, k) in protected_ranges(s) {
        if k != lex::PKind::Math {
            continue;
        }
        let d = if starts_with(s, a, "$$") { 2 } else { 1 };
        let inner: Option<&[char]> =
            if b - a >= 2 * d && s[b - d..b].iter().all(|&c| c == '$') { Some(&s[a + d..b - d]) } else { None };
        let Some(inner) = inner else { return Ok(true) };
        if inner.is_empty() || is_space(inner[0]) || is_space(inner[inner.len() - 1]) {
            return Ok(true);
        }
        if d == 1
            && (inner.len() > 200
                || contains(inner, " | ")
                || contains(inner, "&&")
                || contains(inner, "](")
                || contains(inner, "⟩ ⟨"))
        {
            return Ok(true);
        }
        if ver >= Ver::V4 && !tex_ok(&inner.iter().collect::<String>())? {
            return Ok(true);
        }
    }
    Ok(false)
}

/// (?<![\\$])\$\d
fn currency(s: &[char]) -> bool {
    (0..s.len()).any(|i| {
        s[i] == '$' && i + 1 < s.len() && is_decimal(s[i + 1]) && !(i > 0 && (s[i - 1] == '\\' || s[i - 1] == '$'))
    })
}

/// <script|</script>|function\s*\(|=>\s*\{|\b(?:var|const) \w+\s*=[^;]*;
fn codeish(s: &[char]) -> bool {
    let n = s.len();
    for i in 0..n {
        if starts_with(s, i, "<script") || starts_with(s, i, "</script>") {
            return true;
        }
        if starts_with(s, i, "function") {
            let mut j = i + 8;
            while j < n && is_space(s[j]) {
                j += 1;
            }
            if j < n && s[j] == '(' {
                return true;
            }
        }
        if starts_with(s, i, "=>") {
            let mut j = i + 2;
            while j < n && is_space(s[j]) {
                j += 1;
            }
            if j < n && s[j] == '{' {
                return true;
            }
        }
        for kw in ["var ", "const "] {
            if starts_with(s, i, kw) && (i == 0 || !is_word(s[i - 1])) {
                let mut j = i + kw.chars().count();
                let w0 = j;
                while j < n && is_word(s[j]) {
                    j += 1;
                }
                if j == w0 {
                    continue;
                }
                while j < n && is_space(s[j]) {
                    j += 1;
                }
                if j < n && s[j] == '=' && s[j + 1..].contains(&';') {
                    return true;
                }
            }
        }
    }
    false
}

const MP_GLYPHS: &str = "‖→↦≤≥≠≈≡·×∈∉∞±∂∇∑∏√⊂⊃⊆⊇∪∩∅¬𝒯𝒪𝒜𝒞𝓔𝓜";

fn interior_is_math(s: &[char], ver: Ver) -> Result<bool, Error> {
    let t = strip_chars(s);
    if ver >= Ver::V4 && !tex_ok(&t.iter().collect::<String>())? {
        return Ok(false);
    }
    if (0..s.len()).any(|i| s[i] == '\\' && i + 1 < s.len() && s[i + 1].is_ascii_alphabetic()) {
        return Ok(true);
    }
    let glyph = |c: char| match ver {
        Ver::V3 => op(c).is_some() || greek(c).is_some(),
        Ver::V4 | Ver::V5 | Ver::V6 => MP_GLYPHS.contains(c) || greek(c).is_some(),
    };
    if s.iter().any(|&c| glyph(c)) {
        return Ok(true);
    }
    if s.contains(&'_') || s.contains(&'^') {
        return Ok(true);
    }
    Ok(!t.is_empty()
        && t.len() <= 3
        && t.iter().all(|c| c.is_ascii())
        && t.iter().all(|&c| is_alnum(c))
        && t.iter().any(|&c| is_alpha(c)))
}

fn normalize_paren_math(text: &[char], ver: Ver) -> Result<Vec<char>, Error> {
    let prot = protected_ranges(text);
    let inr = |p: usize| prot.iter().any(|&(a, b, _)| a <= p && p < b);
    let mut out = Vec::with_capacity(text.len());
    let mut i = 0;
    while i < text.len() {
        if starts_with(text, i, "\\(")
            && !inr(i)
            && let Some(rel) = find(text, &['\\', ')'], i + 2)
        {
            let interior = &text[i + 2..rel];
            if !(i..rel + 2).any(inr)
                && !interior.contains(&'`')
                && !interior.contains(&'$')
                && interior_is_math(interior, ver)?
            {
                out.push('$');
                out.extend_from_slice(strip_chars(interior));
                out.push('$');
                i = rel + 2;
                continue;
            }
        }
        out.push(text[i]);
        i += 1;
    }
    Ok(out)
}

/// SINGLE_LETTER_IN_TEX = (?<![\\A-Za-z_^{])([B-HJ-Zb-z])(?![A-Za-z])
fn single_letters_in_tex(lat: &str, out: &mut Vec<char>) {
    let v: Vec<char> = lat.chars().collect();
    for i in 0..v.len() {
        let c = v[i];
        if !(c.is_ascii_alphabetic() && !matches!(c, 'A' | 'I' | 'a')) {
            continue;
        }
        if i > 0 && (v[i - 1] == '\\' || v[i - 1].is_ascii_alphabetic() || matches!(v[i - 1], '_' | '^' | '{')) {
            continue;
        }
        if i + 1 < v.len() && v[i + 1].is_ascii_alphabetic() {
            continue;
        }
        if !out.contains(&c) {
            out.push(c);
        }
    }
}

fn symbols_in(us: &[Unit], toks: &[Tok], spans: &[Option<(usize, usize)>]) -> Vec<char> {
    let mut out: Vec<char> = Vec::new();
    for &(a, b) in spans.iter().flatten() {
        for u in &us[a..b] {
            if u.kind == K::Term
                && u.tm().base == Base::Word
                && toks[u.a].clen() == 1
                && matches!(u.tm().m, M::Letter | M::Strong)
            {
                let c = toks[u.a].text.chars().next().unwrap();
                if !out.contains(&c) {
                    out.push(c);
                }
            }
        }
    }
    for u in us {
        if u.kind == K::Term && u.tm().base == Base::Mspan {
            single_letters_in_tex(&u.tm().latex, &mut out);
        }
    }
    out
}

fn only_house_labels(region: &[char]) -> bool {
    let mut parts: Vec<String> = Vec::new();
    let mut cur = String::new();
    for &c in region {
        if is_space(c) || c == '/' || c == ',' {
            if !cur.is_empty() {
                parts.push(std::mem::take(&mut cur));
            }
        } else {
            cur.push(c);
        }
    }
    if !cur.is_empty() {
        parts.push(cur);
    }
    !parts.is_empty()
        && parts.iter().all(|p| house_label(p) || p == "W₂")
        && parts.iter().any(|p| house_label(p))
}

/// FLATTENED = [A-Za-z][α-ωΑ-Ω][A-Za-z]
fn flattened(r: &[char]) -> bool {
    r.windows(3).any(|w| {
        w[0].is_ascii_alphabetic()
            && (('α'..='ω').contains(&w[1]) || ('Α'..='Ω').contains(&w[1]))
            && w[2].is_ascii_alphabetic()
    })
}

fn unmatched_open(s: &[char], i: usize) -> bool {
    let o = s[i];
    let c = match o {
        '(' => ')',
        '[' => ']',
        _ => '}',
    };
    let mut d = 0i64;
    for &ch in &s[i..] {
        d += (ch == o) as i64;
        d -= (ch == c) as i64;
        if d == 0 {
            return false;
        }
    }
    true
}

/// Has at least one char from \[A-Za-z∞^_] after removing `\cmd` words and `\text{–}`.
fn has_mathy_residue(lat: &str) -> bool {
    let r = lat.replace(r"\text{–}", "");
    let v: Vec<char> = r.chars().collect();
    let mut i = 0;
    while i < v.len() {
        if v[i] == '\\' && i + 1 < v.len() && v[i + 1].is_ascii_alphabetic() {
            i += 1;
            while i < v.len() && v[i].is_ascii_alphabetic() {
                i += 1;
            }
            continue;
        }
        if v[i].is_ascii_alphabetic() || matches!(v[i], '∞' | '^' | '_') {
            return true;
        }
        i += 1;
    }
    false
}

const NAMED: &[&str] = &[
    "infty", "alpha", "beta", "gamma", "delta", "epsilon", "varepsilon", "zeta", "eta", "theta", "iota", "kappa",
    "lambda", "mu", "nu", "xi", "pi", "rho", "sigma", "tau", "upsilon", "phi", "chi", "psi", "omega", "Gamma",
    "Delta", "Theta", "Lambda", "Xi", "Pi", "Sigma", "Phi", "Psi", "Omega", "math", "partial", "nabla", "sum",
    "prod", "int", "ell", "emptyset", "sqrt", "times", "cdot",
];

fn has_named(lat: &str) -> bool {
    lat.match_indices('\\').any(|(p, _)| NAMED.iter().any(|w| lat[p + 1..].starts_with(w)))
}

// ------------------------------------------------------------ convert

/// v6: every `\command` in an emitted span is one the converter can emit
/// (generated KNOWN_COMMANDS) or appears literally in the source text.
fn commands_known(lat: &str, source: &[char]) -> bool {
    let v: Vec<char> = lat.chars().collect();
    let mut i = 0;
    while i < v.len() {
        if v[i] == '\\' && i + 1 < v.len() && v[i + 1].is_ascii_alphabetic() {
            let mut j = i + 1;
            while j < v.len() && v[j].is_ascii_alphabetic() {
                j += 1;
            }
            let cmd = &v[i..j];
            let name: String = cmd.iter().collect();
            if !crate::tables::KNOWN_COMMANDS.contains(&name.as_str()) && find(source, cmd, 0).is_none() {
                return false;
            }
            i = j;
        } else {
            i += 1;
        }
    }
    true
}

/// v5: unmatched backticks, or a backtick inside a `$…$` range.
fn code_math_interleave(s: &[char]) -> bool {
    let mut covered = vec![false; s.len()];
    for (a, b, k) in protected_ranges(s) {
        if k == lex::PKind::Math && s[a..b].contains(&'`') {
            return true;
        }
        for c in &mut covered[a..b] {
            *c = true;
        }
    }
    s.iter().enumerate().any(|(i, &c)| c == '`' && !covered[i])
}

/// v5: positions of the `[` and `]` delimiting link text in `[text](dest)`.
fn link_bracket_positions(s: &[char]) -> Vec<usize> {
    let mut pos = Vec::new();
    for j in 0..s.len().saturating_sub(1) {
        if s[j] == ']' && s[j + 1] == '(' {
            pos.push(j);
            let mut depth = 0i64;
            for i in (0..=j).rev() {
                if s[i] == ']' {
                    depth += 1;
                } else if s[i] == '[' {
                    depth -= 1;
                    if depth == 0 {
                        pos.push(i);
                        break;
                    }
                }
            }
        }
    }
    pos
}

/// v3 `convert` / v4 `convert_once`.
pub fn convert_once(s: &[char], ver: Ver) -> Result<(Vec<char>, Vec<Span>), Error> {
    let Some(_deep) = crate::Deep::enter() else { return Ok((s.to_vec(), vec![])) };
    if s.iter().any(|&c| ('\u{2500}'..='\u{257f}').contains(&c)) {
        return Ok((s.to_vec(), vec![]));
    }
    if ver >= Ver::V4 && (currency(s) || dollar_hazard(s, ver)? || (ver >= Ver::V5 && code_math_interleave(s))) {
        return Ok((s.to_vec(), vec![]));
    }
    let norm = normalize_paren_math(s, ver)?;
    if norm != s {
        return convert_once(&norm, ver);
    }
    if currency(s) || dollar_hazard(s, ver)? {
        return Ok((s.to_vec(), vec![]));
    }
    let count = |c: char| s.iter().filter(|&&x| x == c).count();
    if codeish(s) || (count(';') >= 3 && count('{') + count('}') + count('=') >= 6) {
        return Ok((s.to_vec(), vec![]));
    }
    let v4 = ver >= Ver::V4;
    let linkb = if ver >= Ver::V5 { link_bracket_positions(s) } else { Vec::new() };
    let mut toks = lex::lex(s, v4)?;
    lex::mark_emphasis(&mut toks);
    let us = units_of(&toks);
    let (mut us, mut spans) = find_spans(us, &toks, &[]);
    let syms = symbols_in(&us, &toks, &spans);
    if !syms.is_empty() {
        toks = lex::lex(s, v4)?;
        lex::mark_emphasis(&mut toks);
        let us2 = units_of(&toks);
        (us, spans) = find_spans(us2, &toks, &syms);
    }
    let n = s.len();
    let mut out: Vec<char> = Vec::with_capacity(n + 16);
    let mut pos = 0;
    let mut info = Vec::new();
    for &(a, b) in spans.iter().flatten() {
        if (a..b).all(|k| {
            matches!(us[k].kind, K::Ws | K::Punct | K::Open | K::Close)
                || (us[k].kind == K::Term && us[k].tm().base == Base::Mspan)
        }) {
            continue;
        }
        let mut lat = render_span(&us, &toks, a, b)?;
        if let Some(l) = &lat
            && !has_mathy_residue(l)
            && !has_named(l)
        {
            continue;
        }
        let start = toks[us[a].a].a;
        let end = toks[us[b - 1].b - 1].b;
        let region = &s[start..end];
        if only_house_labels(region) {
            lat = None;
        }
        if linkb.iter().any(|&q| start <= q && q < end) {
            lat = None; // `[≠](…)`: a span may not swallow link-text brackets (v5)
        }
        if flattened(region) {
            lat = None;
        }
        let rest_blank = s[end..].iter().all(|c| matches!(c, ' ' | '.' | ',' | ';'));
        if start > 0 && "([{".contains(s[start - 1]) && unmatched_open(s, start - 1) && rest_blank {
            lat = None;
        }
        let quote_after = if ver >= Ver::V6 {
            end < n && "\"”`".contains(s[end]) // v6: a closing quote must exist
        } else {
            end >= n || "\"”`".contains(s[end]) // v3-v5: '' in '"”`' is True (bug 3)
        };
        if start > 0 && "\"“`".contains(s[start - 1]) && quote_after {
            lat = None;
        }
        if (end < n && matches!(s[end], '^' | '_')) || (start > 0 && matches!(s[start - 1], '^' | '_')) {
            lat = None;
        }
        if (end < n && matches!(s[end], '}' | '{')) || (start > 0 && matches!(s[start - 1], '{' | '}')) {
            lat = None;
        }
        let conf = (a..b).filter(|&k| us[k].kind == K::Term).map(|k| us[k].tm().conf).fold(1.0f64, f64::min);
        if let Some(l) = &lat
            && (!tex_ok(l)? || (ver >= Ver::V6 && !commands_known(l, s)))
        {
            lat = None;
        }
        let Some(l) = lat else {
            info.push(Span { start, end, latex: None, conf });
            continue;
        };
        out.extend_from_slice(&s[pos..start]);
        out.push('$');
        out.extend(l.chars());
        out.push('$');
        pos = end;
        info.push(Span { start, end, latex: Some(l), conf });
    }
    out.extend_from_slice(&s[pos..]);
    Ok((out, info))
}

/// v4/v5 `convert`: iterate to a fixed point (at most 4 more passes), refuse
/// anything that is not "input with regions replaced".
pub fn convert_fixpoint(s: &[char], ver: Ver) -> Result<(Vec<char>, Vec<Span>), Error> {
    let (mut out, info) = convert_once(s, ver)?;
    if out == s {
        return Ok((out, info));
    }
    if spans_vs(s, &out).is_none() {
        return Ok((s.to_vec(), vec![]));
    }
    for _ in 0..4 {
        let (out2, _) = convert_once(&out, ver)?;
        if out2 == out {
            return Ok(match spans_vs(s, &out) {
                Some(sp) => (out, sp),
                None => (s.to_vec(), vec![]),
            });
        }
        out = out2;
    }
    Ok((s.to_vec(), vec![]))
}

fn spans_vs(orig: &[char], out: &[char]) -> Option<Vec<Span>> {
    let mut prose: Vec<Vec<char>> = Vec::new();
    let mut maths: Vec<Vec<char>> = Vec::new();
    let (mut cur, mut mcur): (Vec<char>, Vec<char>) = (Vec::new(), Vec::new());
    let (mut inm, mut esc) = (false, false);
    let mut i = 0;
    while i < out.len() {
        let c = out[i];
        if c == '`' && !inm {
            let j = find(out, &['`'], i + 1).unwrap_or(out.len() - 1);
            cur.extend_from_slice(&out[i..j + 1]);
            i = j + 1;
            continue;
        }
        if c == '$' && !esc {
            if !inm {
                prose.push(std::mem::take(&mut cur));
            } else {
                maths.push(std::mem::take(&mut mcur));
            }
            inm = !inm;
            i += 1;
            continue;
        }
        esc = c == '\\' && !esc;
        if inm { mcur.push(c) } else { cur.push(c) }
        i += 1;
    }
    if inm {
        return None;
    }
    prose.push(cur);
    if prose.len() - 1 != maths.len() {
        return None;
    }
    if !orig.starts_with(&prose[0]) {
        return None;
    }
    let mut res = Vec::new();
    let mut p = prose[0].len();
    let n = prose.len() - 1;
    for k in 1..=n {
        let seg = &prose[k];
        let q = p + 1;
        let f;
        if k == n {
            if orig.len() < seg.len() {
                return None;
            }
            let ff = orig.len() - seg.len();
            if ff < q || orig[ff..] != seg[..] {
                return None;
            }
            f = ff;
        } else {
            f = find(orig, seg, q)?;
        }
        // orig[p:f] != '$' + maths + '$'   (python slicing: empty when f < p)
        let region: &[char] = if f > p { &orig[p..f] } else { &[] };
        let m = &maths[k - 1];
        let same = region.len() == m.len() + 2 && region[0] == '$' && region[region.len() - 1] == '$' && region[1..region.len() - 1] == m[..];
        if !same {
            res.push(Span { start: p, end: f, latex: Some(m.iter().collect()), conf: 1.0 });
        }
        p = f + seg.len();
    }
    Some(res)
}
