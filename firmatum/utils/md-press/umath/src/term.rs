//! Stage 2a: operand terms and the sub/superscript grammar. Port of
//! `parse_script_arg`, `parse_term`, `render_tokens`, `join_latex`, `brace`,
//! `tex_word_sub`, `_add_sup`, and the reference's trailing-script regexes.

use crate::consts::*;
use crate::lex::{K, Tok};
use crate::uni::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Base {
    Word,
    Greek,
    Malpha,
    Num,
    Mspan,
    Norm,
    Abs,
    Group,
    Large,
}

/// "mathness"
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum M {
    Strong,
    Letter,
    Num,
    Func,
    Label,
    Word,
    Ident,
    Group,
    Broken,
}

#[derive(Clone, Debug)]
pub struct Term {
    pub b: usize,
    pub base: Base,
    pub m: M,
    pub latex: String,
    pub conf: f64,
}

// ------------------------------------------------------------ regex ports

/// Python `$`: end of string, or just before a final `\n`.
#[inline]
fn end_ok(s: &[char], e: usize) -> bool {
    e == s.len() || (e + 1 == s.len() && s[e] == '\n')
}

#[derive(Clone, Copy, PartialEq)]
enum BraceAlt {
    None,
    /// `\{[^{}]*\}`
    Flat,
    /// `\{(?:[^{}]|\{[^{}]*\})*\}`
    Nested,
}

#[derive(Clone, Copy, PartialEq)]
enum Single {
    /// `.`
    Dot,
    /// `[^{}\\]`
    NotBraceBs,
    /// `[a-z]`
    LowerAz,
}

/// End of the brace group starting at s[i] == '{' (exclusive), per mode.
fn brace_end(s: &[char], i: usize, mode: BraceAlt) -> Option<usize> {
    if s.get(i) != Some(&'{') {
        return None;
    }
    let mut j = i + 1;
    while j < s.len() {
        match s[j] {
            '}' => return Some(j + 1),
            '{' => {
                if mode != BraceAlt::Nested {
                    return None;
                }
                let mut k = j + 1;
                while k < s.len() && s[k] != '{' && s[k] != '}' {
                    k += 1;
                }
                if k < s.len() && s[k] == '}' {
                    j = k + 1;
                } else {
                    return None;
                }
            }
            _ => j += 1,
        }
    }
    None
}

/// `re.search(marker + '(' + alts + ')$', s)`: leftmost marker whose
/// remainder is exactly one alternative (tried in order). Returns (marker
/// position, group 1) in chars.
fn trail_script(s: &[char], marker: char, bmode: BraceAlt, single: Single) -> Option<(usize, String)> {
    for p in 0..s.len() {
        if s[p] != marker {
            continue;
        }
        let g = p + 1;
        if bmode != BraceAlt::None
            && let Some(e) = brace_end(s, g, bmode)
            && end_ok(s, e)
        {
            return Some((p, s[g..e].iter().collect()));
        }
        if g < s.len() && s[g] == '\\' {
            let mut e = g + 1;
            while e < s.len() && s[e].is_ascii_alphabetic() {
                e += 1;
            }
            if e > g + 1 && end_ok(s, e) {
                return Some((p, s[g..e].iter().collect()));
            }
        }
        if g < s.len() {
            let c = s[g];
            let ok = match single {
                Single::Dot => c != '\n',
                Single::NotBraceBs => c != '{' && c != '}' && c != '\\',
                Single::LowerAz => c.is_ascii_lowercase(),
            };
            if ok && end_ok(s, g + 1) {
                return Some((p, c.to_string()));
            }
        }
    }
    None
}

fn chars(s: &str) -> Vec<char> {
    s.chars().collect()
}

/// `re.search(r'\^(\{[^{}]*\}|\\[A-Za-z]+|.)$', lat)`
pub fn has_trailing_sup(lat: &str) -> bool {
    trail_script(&chars(lat), '^', BraceAlt::Flat, Single::Dot).is_some()
}

fn strip_braces(g: &str) -> String {
    if g.starts_with('{') {
        let v = chars(g);
        v[1..v.len() - 1].iter().collect()
    } else {
        g.to_string()
    }
}

fn prefix(s: &[char], p: usize) -> String {
    s[..p].iter().collect()
}

pub fn add_sup(lat: &str, extra: &str) -> String {
    let s = chars(lat);
    if let Some((p, g)) = trail_script(&s, '^', BraceAlt::Nested, Single::NotBraceBs)
        && !lat.ends_with('\'')
    {
        // v3-v5: the separator expression is '' on both branches (bug 2);
        // v6: a space between a trailing \command and a letter (`\ast T`)
        let inner = strip_braces(&g);
        let sep = if crate::ver() >= crate::Ver::V6
            && trailing_cmd(&inner)
            && extra.chars().next().is_some_and(is_alpha)
        {
            " "
        } else {
            ""
        };
        return format!("{}^{{{}{}{}}}", prefix(&s, p), inner, sep, extra);
    }
    format!("{lat}^{extra}")
}

const BRACE_FUNCS: [&str; 13] =
    ["min", "max", "sup", "inf", "lim", "log", "ln", "exp", "det", "arg", "Pr", "limsup", "liminf"];

pub fn brace(x: &str) -> String {
    let x = py_strip(x);
    if let Some(r) = x.strip_prefix('\\')
        && BRACE_FUNCS.contains(&r)
    {
        return format!("{{{x}}}");
    }
    let mut it = x.chars();
    if matches!((it.next(), it.next()), (Some(_), None)) {
        return x.to_string();
    }
    if let Some(r) = x.strip_prefix('\\')
        && !r.is_empty()
        && r.chars().all(|c| c.is_ascii_alphabetic())
    {
        return x.to_string();
    }
    format!("{{{x}}}")
}

pub fn tex_word_sub(w: &str) -> String {
    if matches!(w, "max" | "min" | "sup" | "inf" | "lim") {
        return func(w).unwrap().to_string();
    }
    let v = chars(w);
    // SIMPLE_SUBWORD: ^(?:[A-Za-z]|[ijklmnpqrstuvxyzab]{2})$   (no '\n' in words)
    let simple = (v.len() == 1 && v[0].is_ascii_alphabetic())
        || (v.len() == 2 && v.iter().all(|c| "ijklmnpqrstuvxyzab".contains(*c)));
    // ^[A-Za-z]\d*$
    let letter_digits = !v.is_empty() && v[0].is_ascii_alphabetic() && v[1..].iter().all(|&c| is_decimal(c));
    if simple || letter_digits {
        return w.to_string();
    }
    format!("\\text{{{w}}}")
}

// ------------------------------------------------------------ token helpers

#[inline]
pub fn glued(toks: &[Tok], k: usize) -> bool {
    k < toks.len() && !toks[k].sp && toks[k].kind != K::Ws
}

fn match_close(toks: &[Tok], k: usize) -> Option<usize> {
    let o = toks[k].text.as_str();
    let c = pair(o)?;
    let mut depth = 0i64;
    for j in k..toks.len() {
        if toks[j].kind == K::Open && toks[j].text == o {
            depth += 1;
        } else if toks[j].kind == K::Close && toks[j].text == c {
            depth -= 1;
            if depth == 0 {
                return Some(j);
            }
        }
    }
    None
}

pub fn subsup_chars(text: &str) -> String {
    text.chars().map(|c| subsup_of(c).unwrap().1).collect()
}

// ------------------------------------------------------------ grammar

/// Argument after `_` / `^` at token k (glued): (end, latex, conf).
pub fn parse_script_arg(toks: &[Tok], k: usize) -> Option<(usize, String, f64)> {
    let _deep = crate::Deep::enter()?;
    if k >= toks.len() || toks[k].sp || toks[k].kind == K::Ws {
        return None;
    }
    let t = &toks[k];
    if t.kind == K::Open && matches!(t.text.as_str(), "(" | "{" | "[") {
        let j = match_close(toks, k)?;
        let inner = &toks[k + 1..j];
        let lat = render_tokens(inner)?;
        if t.is("{") {
            return Some((j + 1, lat, 1.0));
        }
        if t.is("[") {
            return Some((j + 1, format!("[{lat}]"), 0.9));
        }
        if matches!(toks[k - 1].kind, K::Us | K::Emph) || inner.iter().any(|x| matches!(x.kind, K::Op | K::Hyph)) {
            return Some((j + 1, lat, 0.8));
        }
        return Some((j + 1, format!("({lat})"), 0.9));
    }
    if t.kind == K::Word {
        let lat = tex_word_sub(&t.text);
        if glued(toks, k + 1)
            && toks[k + 1].kind == K::Punct
            && toks[k + 1].is(",")
            && glued(toks, k + 2)
            && matches!(toks[k + 2].kind, K::Word | K::Num)
            && toks[k + 2].clen() == 1
            && (k + 3 >= toks.len()
                || !matches!(
                    toks[k + 3].kind,
                    K::Word | K::Num | K::Us | K::Caret | K::Sub | K::Sup | K::Open | K::Emph
                ))
        {
            return Some((k + 3, format!("{},{}", lat, toks[k + 2].text), 0.9));
        }
        return Some((k + 1, lat, 1.0));
    }
    if t.kind == K::Num {
        return Some((k + 1, t.text.clone(), 1.0));
    }
    if t.kind == K::Greek {
        return Some((k + 1, greek(t.text.chars().next().unwrap()).unwrap().to_string(), 1.0));
    }
    if t.kind == K::Malpha {
        return Some((k + 1, mathalpha_latex(t.text.chars().next().unwrap()).unwrap().to_string(), 1.0));
    }
    if ((t.kind == K::Op && matches!(t.text.as_str(), "+" | "−")) || t.kind == K::Hyph)
        && glued(toks, k + 1)
        && toks[k + 1].kind == K::Open
        && toks[k + 1].is("(")
    {
        return Some((k + 1, if t.is("+") { "+".into() } else { "-".into() }, 0.9));
    }
    if (t.kind == K::Op && matches!(t.text.as_str(), "<" | "≤" | ">" | "≥" | "−" | "+")) || t.kind == K::Hyph {
        if let Some((e, lat, c)) = parse_script_arg(toks, k + 1) {
            let sym = match t.text.as_str() {
                "<" => r"\lt ",
                "≤" => r"\leq ",
                ">" => r"\gt ",
                "≥" => r"\geq ",
                "−" => "-",
                "+" => "+",
                _ => "-",
            };
            return Some((e, format!("{sym}{lat}"), c * 0.9));
        }
    }
    if t.kind == K::Star {
        return Some((k + 1, r"\ast".into(), 1.0));
    }
    if t.kind == K::Op
        && let Some((l, OpCls::Ord)) = op_str(&t.text)
    {
        return Some((k + 1, l.into(), 1.0));
    }
    if matches!(t.kind, K::Op | K::Hyph) && matches!(t.text.as_str(), "+" | "−" | "-") {
        return Some((k + 1, if t.is("+") { "+".into() } else { "-".into() }, 0.9));
    }
    None
}

/// Greedy operand term at token k: base + glued decorations.
pub fn parse_term(toks: &[Tok], k0: usize, allow_group: bool) -> Option<Term> {
    let _deep = crate::Deep::enter()?;
    if k0 >= toks.len() {
        return None;
    }
    let t = &toks[k0];
    let mut k = k0;
    let mut conf = 1.0f64;
    let (base, mut lat, mut mness);
    if t.kind == K::Word && t.cls != "foreign" {
        let w = t.text.as_str();
        if t.cls == "label" || t.cls == "unit" {
            (base, lat, mness) = (Base::Word, format!("\\mathrm{{{w}}}"), M::Label);
        } else if t.cls == "ident" {
            (base, lat, mness) = (Base::Word, format!("\\text{{{w}}}"), M::Ident);
        } else if let Some(f) = func(w) {
            (base, lat, mness) = (Base::Word, f.to_string(), M::Func);
        } else if t.clen() == 1 {
            (base, lat, mness) = (Base::Word, w.to_string(), M::Letter);
        } else {
            (base, lat, mness) = (Base::Word, format!("\\text{{{w}}}"), M::Word);
        }
    } else if t.kind == K::Greek {
        (base, lat, mness) = (Base::Greek, greek(t.text.chars().next().unwrap()).unwrap().to_string(), M::Strong);
    } else if t.kind == K::Malpha {
        (base, lat, mness) =
            (Base::Malpha, mathalpha_latex(t.text.chars().next().unwrap()).unwrap().to_string(), M::Strong);
    } else if t.kind == K::Num {
        (base, lat, mness) = (Base::Num, t.text.clone(), M::Num);
    } else if t.kind == K::Mspan {
        return Some(Term {
            b: k0 + 1,
            base: Base::Mspan,
            m: M::Strong,
            latex: t.text.trim_matches('$').to_string(),
            conf: 1.0,
        });
    } else if matches!(t.kind, K::Dbar | K::Bar) || (t.kind == K::Esc && t.cls == "|") {
        let mut j = None;
        for jj in k0 + 1..toks.len().min(k0 + 24) {
            if toks[jj].kind == t.kind && toks[jj].text == t.text {
                j = Some(jj);
                break;
            }
            if toks[jj].kind == K::Ws
                && (t.kind == K::Bar
                    || (jj + 1 < toks.len() && toks[jj + 1].kind == K::Word && toks[jj + 1].clen() > 3))
            {
                break;
            }
        }
        let j = j?;
        if j == k0 + 1 {
            return None;
        }
        if !toks[k0 + 1..j]
            .iter()
            .any(|x| matches!(x.kind, K::Greek | K::Malpha | K::Num | K::Mspan) || (x.kind == K::Word && x.clen() == 1))
        {
            return None;
        }
        let inner = render_tokens(&toks[k0 + 1..j])?;
        if t.kind == K::Dbar {
            (base, lat) = (Base::Norm, format!("\\lVert {inner}\\rVert"));
        } else {
            (base, lat) = (Base::Abs, format!("\\lvert {inner}\\rvert"));
        }
        mness = M::Strong;
        k = j;
    } else if t.kind == K::Op
        && is_large(&t.text)
        && k0 + 1 < toks.len()
        && glued(toks, k0 + 1)
        && matches!(toks[k0 + 1].kind, K::Us | K::Caret | K::Sub | K::Sup)
    {
        (base, lat, mness) = (Base::Large, op_str(&t.text).unwrap().0.to_string(), M::Strong);
    } else if t.kind == K::Open && allow_group && (t.is("(") || t.is("[")) {
        let j = match_close(toks, k0)?;
        let inner = render_tokens(&toks[k0 + 1..j])?;
        (base, lat, mness) = (Base::Group, format!("{}{}{}", t.text, inner, toks[j].text), M::Group);
        k = j;
    } else {
        return None;
    }
    let mut end = k + 1;
    let mut has_script = false;
    let single = matches!(base, Base::Greek | Base::Malpha | Base::Norm | Base::Abs | Base::Group | Base::Large)
        || (base == Base::Word && t.clen() == 1);
    while end < toks.len() && glued(toks, end) {
        let d = &toks[end];
        if d.kind == K::Comb {
            lat = format!("{}{{{}}}", combining(d.text.chars().next().unwrap()).unwrap(), lat);
            has_script = true;
            end += 1;
        } else if d.kind == K::Prime {
            if d.is("'") && glued(toks, end + 1) && toks[end + 1].kind == K::Word {
                break;
            }
            lat.push_str(match d.text.as_str() {
                "'" | "′" => "'",
                "″" => "''",
                _ => "'''",
            });
            has_script = true;
            end += 1;
        } else if matches!(d.kind, K::Sub | K::Sup) {
            let mut ch = subsup_chars(&d.text);
            // re.sub(r'(?<=\d)x(?=\d)', r'\\times ', chars)   (0₃ₓ₃)
            {
                let v = chars(&ch);
                let mut o = String::new();
                for (i, &c) in v.iter().enumerate() {
                    if c == 'x' && i > 0 && is_decimal(v[i - 1]) && i + 1 < v.len() && is_decimal(v[i + 1]) {
                        o.push_str(r"\times ");
                    } else {
                        o.push(c);
                    }
                }
                ch = o;
            }
            let sup = d.kind == K::Sup;
            if sup && ch == "T" {
                lat = if has_trailing_sup(&lat) { add_sup(&lat, r"\top") } else { format!("{lat}^\\top") };
            } else if sup && has_trailing_sup(&lat) {
                lat = add_sup(&lat, &ch);
            } else if sup
                && (ch == "+" || ch == "-")
                && let Some((p, g)) = trail_script(&chars(&lat), '_', BraceAlt::None, Single::LowerAz)
            {
                // M_τ⁺ = M_{τ^+}
                lat = format!("{}_{{{}^{}}}", prefix(&chars(&lat), p), strip_braces(&g), ch);
            } else if !sup
                && let Some((p, g)) = trail_script(&chars(&lat), '_', BraceAlt::Flat, Single::NotBraceBs)
            {
                // κ_W₁
                lat = format!("{}_{{{}_{}}}", prefix(&chars(&lat), p), strip_braces(&g), brace(&ch));
            } else {
                lat = format!("{}{}{}", lat, if sup { "^" } else { "_" }, brace(&ch));
            }
            has_script = true;
            end += 1;
        } else if matches!(d.kind, K::Us | K::Caret) || (d.kind == K::Emph && d.is("_")) {
            let Some((e2, arg, c2)) = parse_script_arg(toks, end + 1) else { break };
            if !d.is("_") {
                lat = if has_trailing_sup(&lat) { add_sup(&lat, &arg) } else { format!("{}^{}", lat, brace(&arg)) };
            } else if let Some((p, g)) = trail_script(&chars(&lat), '_', BraceAlt::Nested, Single::NotBraceBs) {
                // κ_W_1, σ_source_phrase
                lat = format!("{}_{{{}_{}}}", prefix(&chars(&lat), p), strip_braces(&g), brace(&arg));
            } else {
                lat = format!("{}_{}", lat, brace(&arg));
            }
            conf = conf.min(c2);
            has_script = true;
            end = e2;
        } else if (d.kind == K::Star || (d.kind == K::Esc && d.cls == "*"))
            && (matches!(base, Base::Greek | Base::Malpha) || single)
        {
            let nxt = toks.get(end + 1);
            let ok = match nxt {
                None => true,
                Some(x) => {
                    matches!(
                        x.kind,
                        K::Ws | K::Punct | K::Close | K::Op | K::Hyph | K::Dash | K::Emph | K::Us | K::Sub | K::Caret
                    ) || x.kind == K::Star
                        || (x.kind == K::Open && x.is("("))
                }
            };
            if ok {
                lat = add_sup(&lat, r"\ast");
                has_script = true;
                end += 1;
            } else {
                break;
            }
        } else if d.kind == K::Op && d.is("†") {
            lat.push_str(r"^\dagger");
            end += 1;
        } else {
            break;
        }
    }
    if base == Base::Word && has_script {
        let only_primes = (k + 1..end).all(|x| toks[x].kind == K::Prime);
        if t.cls == "label" {
            mness = M::Label;
        } else if single && only_primes {
            mness = M::Letter;
        } else if single && py_isupper(&t.text) && (k + 1..end).all(|x| matches!(toks[x].kind, K::Star | K::Esc)) {
            mness = M::Letter;
        } else if single || func(&t.text).is_some() {
            mness = if func(&t.text).is_none() { M::Strong } else { M::Func };
        } else {
            mness = M::Ident;
        }
    }
    if base == Base::Num && has_script {
        mness = if lat.contains('^') { M::Strong } else { M::Num };
    }
    if end + 1 < toks.len()
        && glued(toks, end)
        && toks[end].kind == K::Punct
        && toks[end].is(".")
        && glued(toks, end + 1)
        && toks[end + 1].kind == K::Word
        && py_islower(&toks[end + 1].text)
    {
        mness = M::Broken;
    }
    if end < toks.len() && glued(toks, end) && matches!(toks[end].kind, K::Us | K::Caret) {
        mness = M::Broken;
    }
    Some(Term { b: end, base, m: mness, latex: lat, conf })
}

/// Render a token run that is entirely math (inside braces / bars / groups).
pub fn render_tokens(ts: &[Tok]) -> Option<String> {
    let _deep = crate::Deep::enter()?;
    let mut out: Vec<String> = Vec::new();
    let mut k = 0;
    while k < ts.len() {
        let t = &ts[k];
        if t.kind == K::Ws {
            k += 1;
            continue;
        }
        let term = parse_term(ts, k, true);
        if let Some(tm) = &term {
            if !matches!(tm.m, M::Word | M::Ident | M::Broken) {
                out.push(tm.latex.clone());
                k = tm.b;
                continue;
            }
            if tm.m == M::Word {
                let mut w = ts[k].text.clone();
                k = tm.b;
                while k + 1 < ts.len() && ts[k].kind == K::Hyph && !ts[k].sp && ts[k + 1].kind == K::Word && !ts[k + 1].sp
                {
                    w.push('-');
                    w.push_str(&ts[k + 1].text);
                    k += 2;
                }
                out.push(format!("\\text{{{w}}}"));
                continue;
            }
        }
        if t.kind == K::Word && t.cls == "ident" {
            out.push(format!("\\text{{{}}}", t.text.replace('_', "-")));
            k += 1;
            continue;
        }
        let piece: String = match t.kind {
            K::Op => match op_str(&t.text) {
                Some((l, _)) => l.into(),
                None => compound_latex(&t.text).unwrap().into(),
            },
            K::Hyph => "-".into(),
            K::Punct if matches!(t.text.as_str(), "," | ";" | ":") => t.text.clone(),
            K::Open => open_latex(&t.text).unwrap().into(),
            K::Close => close_latex(&t.text).unwrap().into(),
            K::Bar => r"\mid".into(),
            K::Dbar => r"\Vert".into(),
            K::Ellip => r"\ldots".into(),
            K::Star => r"\ast".into(),
            K::Tilde => r"\sim".into(),
            K::Other if t.is("%") => r"\%".into(),
            K::Esc if t.cls == "|" => r"\vert".into(),
            _ => return None,
        };
        out.push(piece);
        k += 1;
    }
    Some(join_latex(&out))
}

thread_local! {
    static REL: Vec<&'static str> = rel_latex();
}

fn spaced(p: &str) -> bool {
    REL.with(|r| r.contains(&p))
}

fn spaced_tail(s: &str) -> bool {
    REL.with(|rel| {
        if rel.iter().any(|r| s == *r || (s.ends_with(r) && s[..s.len() - r.len()].ends_with(' '))) {
            return true;
        }
        rel.iter().any(|r| {
            r.starts_with('\\')
                && s.ends_with(r)
                && (s.len() == r.len() || !is_alpha(s[..s.len() - r.len()].chars().next_back().unwrap()))
        })
    })
}

fn trailing_cmd(s: &str) -> bool {
    // re.search(r'\\[A-Za-z]+$', s)
    let t = s.strip_suffix('\n').unwrap_or(s);
    let letters = t.chars().rev().take_while(|c| c.is_ascii_alphabetic()).count();
    letters > 0 && t[..t.len() - letters].ends_with('\\')
}

pub fn join_latex<S: AsRef<str>>(parts: &[S]) -> String {
    let mut s = String::new();
    for p in parts {
        let p = p.as_ref();
        if p.is_empty() {
            continue;
        }
        if !s.is_empty() && trailing_cmd(&s) && p.chars().next().is_some_and(|c| c.is_ascii_alphanumeric()) {
            s.push(' ');
        } else if (!s.is_empty() && spaced(p)) || (!s.is_empty() && spaced_tail(&s)) {
            s.push(' ');
        }
        s.push_str(p);
    }
    s
}
