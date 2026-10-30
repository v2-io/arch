//! Stage 1: text -> fine tokens. Port of `protected_ranges`, `lex`,
//! `_precomposed_math`, `mark_emphasis`. All offsets are in chars (Python
//! code points), never bytes.

use crate::consts::*;
use crate::uni::*;
use crate::Error;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum K {
    Mspan,
    Prot,
    Ws,
    Esc,
    Star,
    Emph,
    Us,
    Caret,
    Word,
    Num,
    Greek,
    Gword,
    Comb,
    Malpha,
    Sub,
    Sup,
    Prime,
    Dbar,
    Bar,
    Open,
    Close,
    Hyph,
    Op,
    Punct,
    Ellip,
    Dash,
    Tilde,
    Other,
    /// unit-level only
    Term,
}

#[derive(Clone, Debug)]
pub struct Tok {
    pub kind: K,
    pub text: String,
    pub a: usize,
    pub b: usize,
    /// operator class / word class / escaped char / protected kind
    pub cls: String,
    pub sp: bool,
}

impl Tok {
    fn new(kind: K, text: String, a: usize, b: usize, cls: &str) -> Tok {
        Tok { kind, text, a, b, cls: cls.to_string(), sp: false }
    }
    #[inline]
    pub fn is(&self, s: &str) -> bool {
        self.text == s
    }
    /// number of chars in text (Python len)
    #[inline]
    pub fn clen(&self) -> usize {
        self.text.chars().count()
    }
}

pub fn slice(s: &[char], a: usize, b: usize) -> String {
    s[a..b.min(s.len())].iter().collect()
}

pub fn starts_with(s: &[char], i: usize, pat: &str) -> bool {
    let mut k = i;
    for p in pat.chars() {
        if k >= s.len() || s[k] != p {
            return false;
        }
        k += 1;
    }
    true
}

/// `s.find(pat, from)` on chars.
pub fn find(s: &[char], pat: &[char], from: usize) -> Option<usize> {
    if pat.is_empty() {
        return if from <= s.len() { Some(from) } else { None };
    }
    if pat.len() > s.len() {
        return None;
    }
    (from..=s.len() - pat.len()).find(|&i| s[i..i + pat.len()] == *pat)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PKind {
    Code,
    Math,
    Link,
    Wiki,
    Html,
    Url,
}

impl PKind {
    pub fn name(self) -> &'static str {
        match self {
            PKind::Code => "code",
            PKind::Math => "math",
            PKind::Link => "link",
            PKind::Wiki => "wiki",
            PKind::Html => "html",
            PKind::Url => "url",
        }
    }
}

/// Port of the reference's `protected_ranges` (itself a port of md-press's,
/// with small differences — see PORT.md).
pub fn protected_ranges(s: &[char]) -> Vec<(usize, usize, PKind)> {
    let n = s.len();
    let mut out = Vec::new();
    let mut i = 0;
    while i < n {
        let c = s[i];
        if c == '\\' {
            i += 2;
            continue;
        }
        if c == '`' {
            let mut k = i;
            while k < n && s[k] == '`' {
                k += 1;
            }
            let fence = &s[i..k];
            if let Some(j) = find(s, fence, k) {
                out.push((i, j + fence.len(), PKind::Code));
                i = j + fence.len();
            } else {
                i = k;
            }
            continue;
        }
        if c == '$' {
            let d: &[char] = if starts_with(s, i, "$$") { &['$', '$'] } else { &['$'] };
            let mut j = i + d.len();
            let mut end = None;
            while j < n {
                if s[j] == '\\' {
                    j += 2;
                    continue;
                }
                if s[j..].starts_with(d) {
                    end = Some(j + d.len());
                    break;
                }
                j += 1;
            }
            let end = end.unwrap_or(n);
            out.push((i, end, PKind::Math));
            i = end;
            continue;
        }
        if c == ']' && i + 1 < n && s[i + 1] == '(' {
            let mut depth: i64 = 0;
            let mut j = i + 1;
            while j < n {
                if s[j] == '\\' {
                    j += 2;
                    continue;
                }
                if s[j] == '(' {
                    depth += 1;
                } else if s[j] == ')' {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                j += 1;
            }
            let end = (j + 1).min(n);
            out.push((i + 1, end, PKind::Link));
            i = end;
            continue;
        }
        if starts_with(s, i, "[[")
            && let Some(j) = find(s, &[']', ']'], i)
        {
            out.push((i, j + 2, PKind::Wiki));
            i = j + 2;
            continue;
        }
        if c == '<'
            && let Some(j) = find(s, &['>'], i)
        {
            let inner = &s[i + 1..j];
            let tagish = inner.first().is_none_or(|&f| f.is_ascii() && (is_alpha(f) || f == '/' || f == '!'));
            let simple = !inner.iter().any(|&x| is_space(x));
            // '=' in inner and re.match(r'^[A-Za-z0-9/]+\s', inner + ' ')
            let with_attrs = inner.contains(&'=') && {
                let r = inner.iter().take_while(|&&x| x.is_ascii_alphanumeric() || x == '/').count();
                r > 0 && inner.get(r).is_none_or(|&x| is_space(x))
            };
            if tagish && (simple || with_attrs) && !inner.is_empty() {
                out.push((i, j + 1, PKind::Html));
                i = j + 1;
                continue;
            }
        }
        if starts_with(s, i, "http://") || starts_with(s, i, "https://") || starts_with(s, i, "www.") {
            let end = (i..n).find(|&k| is_space(s[k])).unwrap_or(n);
            out.push((i, end, PKind::Url));
            i = end;
            continue;
        }
        i += 1;
    }
    out
}

/// SNAKE = `[A-Za-z0-9]+(?:_[A-Za-z0-9]+){2,}\b`, anchored at i. Backtracking
/// can never rescue a failed `\b` here (every earlier end is followed by an
/// ASCII alnum or `_`), so the greedy end is the only candidate.
pub fn snake_match(s: &[char], i: usize) -> Option<usize> {
    let n = s.len();
    let mut j = i;
    while j < n && s[j].is_ascii_alphanumeric() {
        j += 1;
    }
    if j == i {
        return None;
    }
    let mut groups = 0;
    while j + 1 < n && s[j] == '_' && s[j + 1].is_ascii_alphanumeric() {
        j += 1;
        while j < n && s[j].is_ascii_alphanumeric() {
            j += 1;
        }
        groups += 1;
    }
    if groups < 2 || (j < n && is_word(s[j])) {
        return None;
    }
    Some(j)
}

fn is_greek_script(c: char) -> bool {
    let o = c as u32;
    ((0x0370..=0x03FF).contains(&o) || (0x1F00..=0x1FFF).contains(&o)) && is_alpha(c)
}

fn greek_numeral_after(s: &[char], j: usize) -> bool {
    j < s.len() && matches!(s[j], '\u{0374}' | '\u{0375}' | '\u{0384}' | '\u{02b9}')
}

fn precomposed_math(s: &[char], i: usize) -> bool {
    if precomposed(s[i]).is_none() {
        return false;
    }
    let before = if i > 0 { s[i - 1] } else { ' ' };
    let after = if i + 1 < s.len() { s[i + 1] } else { ' ' };
    if is_alpha(before) || is_alpha(after) {
        return false;
    }
    if matches!(after, '_' | '^' | '′' | '\'') {
        return true;
    }
    // re.match(r'\s*(=|:=|≤|≥|<|>|≈|→|∈)', s[i + 1:])
    let mut k = i + 1;
    while k < s.len() && is_space(s[k]) {
        k += 1;
    }
    k < s.len() && ("=≤≥<>≈→∈".contains(s[k]) || starts_with(s, k, ":="))
}

const PUNCT: &str = ",;:.?!";
const DASHES: &str = "—–";

/// `v4`: a two-char ASCII op does not start where its second char begins a
/// protected range (umath_v4 change).
pub fn lex(s: &[char], v4: bool) -> Result<Vec<Tok>, Error> {
    let n = s.len();
    let prot = protected_ranges(s);
    let mut pmap: Vec<Option<(usize, PKind)>> = vec![None; n];
    for (a, b, k) in prot {
        pmap[a] = Some((b, k));
    }
    let mut toks: Vec<Tok> = Vec::new();
    let mut i = 0;
    while i < n {
        if let Some((b, k)) = pmap[i] {
            let kind = if k == PKind::Math { K::Mspan } else { K::Prot };
            toks.push(Tok::new(kind, slice(s, i, b), i, b, k.name()));
            i = b;
            continue;
        }
        let c = s[i];
        if is_space(c) {
            let mut j = i;
            while j < n && is_space(s[j]) && pmap[j].is_none() {
                j += 1;
            }
            toks.push(Tok::new(K::Ws, slice(s, i, j), i, j, ""));
            i = j;
            continue;
        }
        if c == '\\' && i + 1 < n {
            toks.push(Tok::new(K::Esc, slice(s, i, i + 2), i, i + 2, &s[i + 1].to_string()));
            i += 2;
            continue;
        }
        if c == '*' || (c == '_' && (i == 0 || is_space(s[i - 1]) || "([{\"“‘'*~>".contains(s[i - 1]))) {
            let mut j = i;
            while j < n && s[j] == c {
                j += 1;
            }
            let kind = if c == '*' && j - i == 1 { K::Star } else { K::Emph };
            toks.push(Tok::new(kind, slice(s, i, j), i, j, ""));
            i = j;
            continue;
        }
        if c == '_' {
            toks.push(Tok::new(K::Us, "_".into(), i, i + 1, ""));
            i += 1;
            continue;
        }
        if c == '^' {
            toks.push(Tok::new(K::Caret, "^".into(), i, i + 1, ""));
            i += 1;
            continue;
        }
        let boundary = i == 0 || !(is_alnum(s[i - 1]) || s[i - 1] == '_');
        if c.is_ascii_digit()
            && boundary
            && let Some(e) = snake_match(s, i)
        {
            toks.push(Tok::new(K::Word, slice(s, i, e), i, e, "ident"));
            i = e;
            continue;
        }
        if c.is_ascii_digit() {
            // \d+(?:[.,]\d+)*
            let mut j = i;
            while j < n && is_decimal(s[j]) {
                j += 1;
            }
            while j + 1 < n && (s[j] == '.' || s[j] == ',') && is_decimal(s[j + 1]) {
                j += 1;
                while j < n && is_decimal(s[j]) {
                    j += 1;
                }
            }
            toks.push(Tok::new(K::Num, slice(s, i, j), i, j, ""));
            i = j;
            continue;
        }
        if c.is_ascii_alphanumeric()
            && boundary
            && let Some(e) = snake_match(s, i)
        {
            toks.push(Tok::new(K::Word, slice(s, i, e), i, e, "ident"));
            i = e;
            continue;
        }
        if c.is_ascii_alphabetic() && toks.last().is_some_and(|t| t.kind == K::Num && t.b == i) {
            let mut j = i;
            while j < n && s[j].is_ascii_alphabetic() {
                j += 1;
            }
            toks.push(Tok::new(K::Word, slice(s, i, j), i, j, "unit"));
            i = j;
            continue;
        }
        if c.is_ascii_alphabetic() {
            // [A-Za-z]+(?:[0-9]+[A-Za-z]*)*
            let mut j = i;
            while j < n && s[j].is_ascii_alphabetic() {
                j += 1;
            }
            while j < n && s[j].is_ascii_digit() {
                while j < n && s[j].is_ascii_digit() {
                    j += 1;
                }
                while j < n && s[j].is_ascii_alphabetic() {
                    j += 1;
                }
            }
            let w = slice(s, i, j);
            if j - i == 1 && c.is_ascii_uppercase() && j < n && s[j] == '∞' {
                toks.push(Tok::new(K::Word, slice(s, i, j + 1), i, j + 1, "label"));
                i = j + 1;
                continue;
            }
            let cls = if w.chars().any(|x| x.is_ascii_digit()) { "label" } else { "" };
            toks.push(Tok::new(K::Word, w, i, j, cls));
            i = j;
            continue;
        }
        if is_greek_script(c) {
            let mut j = i;
            while j < n && (is_greek_script(s[j]) || (is_mn(s[j]) && combining(s[j]).is_none())) {
                j += 1;
            }
            if j - i == 1 && greek_numeral_after(s, j) {
                toks.push(Tok::new(K::Gword, slice(s, i, j + 1), i, j + 1, ""));
                i = j + 1;
                continue;
            }
            if j - i == 1
                && greek(c).is_some()
                && j < n
                && s[j].is_ascii_digit()
                && (i == 0 || !is_alnum(s[i - 1]))
            {
                let mut e = j;
                while e < n && is_decimal(s[e]) {
                    e += 1;
                }
                while e < n && s[e].is_ascii_alphabetic() {
                    e += 1;
                }
                toks.push(Tok::new(K::Gword, slice(s, i, e), i, e, ""));
                i = e;
                continue;
            }
            if j - i == 1 && c == 'μ' && j < n && UNITS_AFTER_MU.contains(s[j]) && !(j + 1 < n && is_alpha(s[j + 1])) {
                toks.push(Tok::new(K::Word, slice(s, i, j + 1), i, j + 1, "unit"));
                j += 1;
            } else if j - i == 1 && c == 'Ω' && i > 0 && "kMGm".contains(s[i - 1]) && (i < 2 || !is_alpha(s[i - 2])) {
                toks.push(Tok::new(K::Word, c.to_string(), i, j, "unit"));
            } else if j - i == 1 && greek(c).is_some() {
                toks.push(Tok::new(K::Greek, c.to_string(), i, j, ""));
            } else if j - i == 1 && greek_numeral_after(s, j) {
                toks.push(Tok::new(K::Gword, slice(s, i, j + 1), i, j + 1, ""));
                j += 1;
            } else if s[i..j].iter().all(|&ch| greek(ch).is_some()) && j - i <= 3 && !greek_numeral_after(s, j) {
                for k in i..j {
                    toks.push(Tok::new(K::Greek, s[k].to_string(), k, k + 1, ""));
                }
            } else {
                toks.push(Tok::new(K::Gword, slice(s, i, j), i, j, ""));
            }
            i = j;
            continue;
        }
        if is_alpha(c) && !c.is_ascii() && precomposed_math(s, i) {
            let (base, mark) = precomposed(c).unwrap();
            toks.push(Tok::new(K::Word, base.to_string(), i, i + 1, ""));
            toks.push(Tok::new(K::Comb, mark.to_string(), i, i + 1, ""));
            i += 1;
            continue;
        }
        if is_alpha(c) && !c.is_ascii() {
            if mathalpha_raises(c) && crate::ver() < crate::Ver::V6 {
                return Err(Error::MathalphaKeyError(c));
            }
            if mathalpha_latex(c).is_none() && subsup_of(c).is_none() {
                // [^\W\d_]+
                let mut j = i;
                while j < n && is_alnum(s[j]) && !is_decimal(s[j]) {
                    j += 1;
                }
                toks.push(Tok::new(K::Word, slice(s, i, j), i, j, "foreign"));
                i = j;
                continue;
            }
        }
        if mathalpha_latex(c).is_some() {
            toks.push(Tok::new(K::Malpha, c.to_string(), i, i + 1, ""));
            i += 1;
            continue;
        }
        if let Some((sup, _)) = subsup_of(c) {
            let mut j = i;
            while j < n && subsup_of(s[j]).is_some_and(|(x, _)| x == sup) {
                j += 1;
            }
            toks.push(Tok::new(if sup { K::Sup } else { K::Sub }, slice(s, i, j), i, j, ""));
            i = j;
            continue;
        }
        let one = move |k: K, cls: &str| Tok::new(k, c.to_string(), i, i + 1, cls);
        if combining(c).is_some() {
            toks.push(one(K::Comb, ""));
        } else if "'′″‴".contains(c) {
            toks.push(one(K::Prime, ""));
        } else if c == '‖' {
            toks.push(one(K::Dbar, ""));
        } else if c == '|' {
            if starts_with(s, i, "||") {
                toks.push(Tok::new(K::Dbar, "||".into(), i, i + 2, ""));
                i += 2;
                continue;
            }
            toks.push(one(K::Bar, ""));
        } else if open_latex(&c.to_string()).is_some() {
            toks.push(one(K::Open, ""));
        } else if close_latex(&c.to_string()).is_some() {
            toks.push(one(K::Close, ""));
        } else if c == '-' {
            toks.push(one(K::Hyph, ""));
        } else if let Some((two, _)) =
            COMPOUNDS.iter().find(|(two, _)| starts_with(s, i, two) && !(v4 && i + 1 < n && pmap[i + 1].is_some()))
        {
            toks.push(Tok::new(K::Op, two.to_string(), i, i + 2, "rel"));
            i += 2;
            continue;
        } else if let Some((_, cls)) = op(c) {
            toks.push(one(K::Op, cls.name()));
        } else if PUNCT.contains(c) {
            toks.push(one(K::Punct, ""));
        } else if c == '…' {
            toks.push(one(K::Ellip, ""));
        } else if DASHES.contains(c) {
            toks.push(one(K::Dash, ""));
        } else if c == '~' {
            toks.push(one(K::Tilde, ""));
        } else {
            toks.push(one(K::Other, ""));
        }
        i += 1;
    }
    for k in 1..toks.len() {
        toks[k].sp = toks[k - 1].kind == K::Ws;
    }
    Ok(toks)
}

/// Single `*` tokens that pair as markdown emphasis become Emph.
pub fn mark_emphasis(toks: &mut [Tok]) {
    let stars: Vec<usize> = (0..toks.len()).filter(|&k| toks[k].kind == K::Star).collect();
    let mut opener: Option<usize> = None;
    for k in stars {
        let prev_ws = k == 0 || matches!(toks[k - 1].kind, K::Ws | K::Open | K::Punct | K::Dash | K::Emph);
        let next_ok = k + 1 < toks.len() && toks[k + 1].kind != K::Ws;
        match opener {
            None => {
                if prev_ws && next_ok {
                    opener = Some(k);
                }
            }
            Some(o) => {
                if !prev_ws {
                    toks[o].kind = K::Emph;
                    toks[k].kind = K::Emph;
                    opener = None;
                }
            }
        }
    }
}
