//! Stage 2b: units and span finding. Port of `units_of`, `find_spans`
//! (with its closures as methods), and `trim_span`.

use crate::consts::*;
use crate::lex::{K, Tok};
use crate::term::*;
use crate::uni::*;

#[derive(Clone, Debug)]
pub struct Unit {
    pub kind: K,
    pub a: usize,
    pub b: usize,
    pub term: Option<Term>,
    pub text: String,
    pub cls: String,
}

impl Unit {
    #[inline]
    pub fn tm(&self) -> &Term {
        self.term.as_ref().unwrap()
    }
    /// kind == term and mathness in set
    #[inline]
    pub fn mis(&self, ms: &[M]) -> bool {
        self.kind == K::Term && ms.contains(&self.tm().m)
    }
    #[inline]
    pub fn is(&self, s: &str) -> bool {
        self.text == s
    }
}

pub fn units_of(toks: &[Tok]) -> Vec<Unit> {
    let mut us = Vec::new();
    let mut k = 0;
    while k < toks.len() {
        let t = &toks[k];
        if let Some(term) = parse_term(toks, k, false)
            && term.b > k
        {
            let b = term.b;
            let text: String = toks[k..b].iter().map(|x| x.text.as_str()).collect();
            us.push(Unit { kind: K::Term, a: k, b, term: Some(term), text, cls: String::new() });
            k = b;
            continue;
        }
        let mut kind = t.kind;
        if matches!(kind, K::Greek | K::Malpha | K::Word | K::Num | K::Mspan) {
            kind = K::Other;
        }
        if kind == K::Op && matches!(op_str(&t.text), Some((_, OpCls::Post | OpCls::Ord))) {
            kind = K::Other;
        }
        us.push(Unit { kind, a: k, b: k + 1, term: None, text: t.text.clone(), cls: t.cls.clone() });
        k += 1;
    }
    us
}

pub fn op_latex(text: &str) -> String {
    if let Some((l, _)) = op_str(text) {
        return l.into();
    }
    compound_latex(text).map(|x| x.to_string()).unwrap_or_else(|| text.to_string())
}

fn is_strong_op(u: &Unit) -> bool {
    u.kind == K::Op && (is_strong_op_char(&u.text) || matches!(u.text.as_str(), "<=" | ">=" | "!=" | ">>" | "<<"))
}

const OPERAND_OK: &[M] = &[M::Strong, M::Letter, M::Num, M::Func, M::Group];
const OPERAND_OK_FUNC: &[M] = &[M::Strong, M::Letter, M::Num, M::Func, M::Group];

struct FS<'a> {
    us: Vec<Unit>,
    toks: &'a [Tok],
    n: usize,
    /// Bracket partners, precomputed in one stack pass per bracket type. For
    /// a single type, the reference's scan ("first close where the depth of
    /// this type returns to 0") is exactly the stack partner; precomputing
    /// keeps a line of unmatched `(` linear instead of quadratic.
    close_of: Vec<Option<usize>>,
    open_of: Vec<Option<usize>>,
}

impl<'a> FS<'a> {
    fn ttext(&self, u: usize) -> &str {
        &self.toks[self.us[u].a].text
    }
    fn tsp(&self, u: usize) -> bool {
        self.toks[self.us[u].a].sp
    }

    /// v7: an operator name glued after a hyphen and not applied/scripted.
    fn hyphen_compound_func(&self, j2: usize) -> bool {
        let u2 = &self.us[j2];
        if !(u2.kind == K::Term && u2.tm().m == M::Func && u2.tm().base == Base::Word) {
            return false;
        }
        if self.toks[u2.a].sp {
            return false;
        }
        if let Some(nx) = self.toks.get(u2.b)
            && matches!(nx.kind, K::Open | K::Us | K::Caret | K::Sub | K::Sup)
            && !nx.sp
        {
            return false;
        }
        true
    }

    fn label_number(&self, j: usize) -> bool {
        let mut k = j as isize - 1;
        while k >= 0 && self.us[k as usize].kind == K::Ws {
            k -= 1;
        }
        k >= 0 && self.us[k as usize].mis(&[M::Word, M::Label])
    }

    /// index of next non-ws unit in direction d, and whether ws was skipped
    fn nxt(&self, i: usize, d: isize) -> (Option<usize>, bool) {
        let n = self.n as isize;
        let mut j = i as isize + d;
        let mut sk = false;
        while 0 <= j && j < n && self.us[j as usize].kind == K::Ws {
            j += d;
            sk = true;
        }
        (if 0 <= j && j < n { Some(j as usize) } else { None }, sk)
    }

    fn operand_right(&mut self, j: Option<usize>) -> Option<usize> {
    let _deep = crate::Deep::enter()?;
        let j = j?;
        let n = self.n;
        {
            let u = &self.us[j];
            if u.mis(&[M::Letter]) && matches!(self.ttext(j), "a" | "I") && u.b - u.a == 1 {
                let (k3, _) = self.nxt(j, 1);
                if let Some(k3) = k3
                    && self.us[k3].mis(&[M::Word, M::Label, M::Ident])
                {
                    return None;
                }
            }
            if u.mis(&[M::Num])
                && j + 1 < n
                && self.us[j + 1].kind == K::Term
                && self.toks[self.us[j + 1].a].cls == "unit"
                && !self.toks[self.us[j + 1].a].sp
            {
                return None;
            }
            if u.mis(OPERAND_OK) {
                return Some(self.absorb_application(j + 1));
            }
        }
        if self.us[j].mis(&[M::Word]) && is_differential(&self.us[j].text) && j > 0 && {
            let p = &self.us[j - 1];
            p.kind == K::Op && p.is("/")
        } && !self.tsp(j)
        {
            let t = self.us[j].text.clone();
            self.us[j].term.as_mut().unwrap().latex = t; // a differential: d‖δ‖/dt
            return Some(j + 1);
        }
        if self.us[j].kind == K::Term && self.us[j].tm().base == Base::Large {
            let (k2, _) = self.nxt(j, 1);
            let r = self.operand_right(k2);
            return Some(r.unwrap_or(j + 1)); // (r is never 0)
        }
        let (ukind, ucls, utext) = (self.us[j].kind, self.us[j].cls.clone(), self.us[j].text.clone());
        if (ukind == K::Op && ucls == "pre")
            || (ukind == K::Op && matches!(utext.as_str(), "−" | "-" | "+" | "±" | "¬"))
            || ukind == K::Hyph
        {
            let mut k2 = Some(j + 1);
            if j + 1 < n && self.us[j + 1].kind == K::Ws && is_large(&utext) {
                k2 = self.nxt(j, 1).0;
            }
            if let Some(k2) = k2
                && k2 < n
                && self.us[k2].kind != K::Ws
                && let Some(r) = self.operand_right(Some(k2))
            {
                return Some(r);
            }
            if ucls == "pre" {
                return Some(j + 1);
            }
            return None;
        }
        if ukind == K::Open
            && matches!(utext.as_str(), "(" | "[" | "⟨" | "{")
            && let Some(c) = self.group_close(j)
            && self.group_ok(j, c, 0)
        {
            return Some(self.absorb_script(c + 1));
        }
        if ukind == K::Other && utext == "∞" {
            return Some(j + 1);
        }
        None
    }

    fn operand_left(&self, j: Option<usize>) -> Option<usize> {
        let j = j?;
        let u = &self.us[j];
        if u.mis(OPERAND_OK) {
            return Some(j);
        }
        if u.kind == K::Close
            && let Some(o) = self.group_open(j)
            && self.group_ok(o, j, 0)
        {
            if o > 0 && self.us[o - 1].kind == K::Term && !self.tsp(o) && self.us[o - 1].mis(OPERAND_OK_FUNC) {
                return Some(o - 1);
            }
            return Some(o);
        }
        if u.kind == K::Other && u.is("∞") {
            return Some(j);
        }
        None
    }

    fn group_close(&self, j: usize) -> Option<usize> {
        if self.us[j].kind == K::Open {
            return self.close_of[j];
        }
        self.group_close_scan(j)
    }

    fn group_open(&self, j: usize) -> Option<usize> {
        if self.us[j].kind == K::Close {
            return self.open_of[j];
        }
        self.group_open_scan(j)
    }

    fn group_close_scan(&self, j: usize) -> Option<usize> {
        let o = self.us[j].text.as_str();
        let c = pair(o); // None never matches a close (PAIR.get)
        let mut depth = 0i64;
        for k in j..self.n {
            let u = &self.us[k];
            if u.kind == K::Open && u.text == o {
                depth += 1;
            } else if u.kind == K::Close && Some(u.text.as_str()) == c {
                depth -= 1;
                if depth == 0 {
                    return Some(k);
                }
            }
        }
        None
    }

    fn group_open_scan(&self, j: usize) -> Option<usize> {
        let c = self.us[j].text.as_str();
        let o = pair_rev(c);
        let mut depth = 0i64;
        for k in (0..=j).rev() {
            let u = &self.us[k];
            if u.kind == K::Close && u.text == c {
                depth += 1;
            } else if u.kind == K::Open && Some(u.text.as_str()) == o {
                depth -= 1;
                if depth == 0 {
                    return Some(k);
                }
            }
        }
        None
    }

    fn group_ok(&self, o: usize, c: usize, mut words: i32) -> bool {
        if c < o + 2 {
            return false;
        }
        if self.us[o].is("⟨") && (c - o > 12 || self.toks[self.us[c].a].a - self.toks[self.us[o].a].a > 40) {
            return false;
        }
        for k in o + 1..c {
            let u = &self.us[k];
            match u.kind {
                K::Ws => {}
                K::Term => {
                    let m = u.tm().m;
                    if matches!(m, M::Strong | M::Letter | M::Num | M::Func | M::Group) {
                    } else if m == M::Word && u.text.chars().count() <= 3 && py_isupper(&u.text) {
                    } else if m == M::Word && words > 0 && u.text.chars().count() <= 16 {
                        words -= 1;
                    } else {
                        return false;
                    }
                }
                K::Op | K::Hyph => {}
                K::Punct if matches!(u.text.as_str(), "," | ";" | ":") => {}
                K::Open | K::Close => {}
                K::Ellip => {}
                K::Other if matches!(u.text.as_str(), "∞" | "%" | "|") => {}
                K::Bar | K::Star => {}
                _ => return false,
            }
        }
        true
    }

    fn unit_like(&self, k: usize) -> bool {
        let u = &self.us[k];
        let tk = &self.toks[u.a];
        if !(u.tm().base == Base::Word && tk.clen() == 1 && tk.text.is_ascii()) {
            return false;
        }
        if !(u.a + 1..u.b).all(|x| matches!(self.toks[x].kind, K::Sup | K::Sub)) || u.b == u.a + 1 {
            return false;
        }
        if !(u.a + 1..u.b).all(|x| subsup_chars(&self.toks[x].text).chars().all(|ch| is_digit(ch) || ch == '-')) {
            return false;
        }
        let (jl, sk) = self.nxt(k, -1);
        let Some(jl) = jl else { return false };
        if sk && self.us[jl].mis(&[M::Num]) {
            return true;
        }
        if !sk && self.us[jl].kind == K::Op && self.us[jl].is("/") && jl >= 1 {
            let j2 = jl - 1;
            let u2 = &self.us[j2];
            if u2.kind == K::Term
                && (matches!(u2.tm().m, M::Word | M::Label)
                    || (u2.tm().m == M::Letter
                        && self.nxt(j2, -1).0.is_some_and(|q| self.us[q].mis(&[M::Num]))))
            {
                return true;
            }
        }
        false
    }

    fn plausible_variable(&self, k: usize, known: bool) -> bool {
        let n = self.n;
        let toks = self.toks;
        let us = &self.us;
        let l = toks[us[k].a].text.as_str();
        if matches!(l, "a" | "A" | "I") {
            return false;
        }
        let lupper = py_isupper(l);
        let t0 = &toks[us[k].a];
        let prev_t = if us[k].a > 0 { Some(&toks[us[k].a - 1]) } else { None };
        let next_t = toks.get(us[k].b);
        if let Some(p) = prev_t
            && !t0.sp
            && (!matches!(p.kind, K::Open | K::Punct | K::Dash | K::Emph | K::Op | K::Bar | K::Dbar | K::Star)
                || (p.kind == K::Punct && !matches!(p.text.as_str(), "," | ";" | ":")))
        {
            return false;
        }
        if let Some(nt) = next_t
            && nt.kind == K::Punct
            && nt.is(".")
            && us[k].b + 1 < toks.len()
            && (matches!(toks[us[k].b + 1].kind, K::Punct | K::Ellip | K::Word)
                || (toks[us[k].b + 1].kind == K::Ws && us[k].b + 2 < toks.len() && toks[us[k].b + 2].kind == K::Num))
        {
            return false;
        }
        if let Some(nt) = next_t
            && !matches!(
                nt.kind,
                K::Ws | K::Close | K::Punct | K::Prime | K::Emph | K::Dash | K::Op | K::Bar | K::Dbar | K::Open | K::Star
            )
            && !(nt.kind == K::Other && nt.is("’"))
        {
            return false;
        }
        if k + 1 < n && us[k + 1].kind == K::Punct && us[k + 1].is(".") && k + 2 < n && us[k + 2].kind == K::Term {
            return false;
        }
        if k > 0 && us[k - 1].kind == K::Open && k + 1 < n && us[k + 1].kind == K::Close {
            return false;
        }
        let (jl, _) = self.nxt(k, -1);
        if !known
            && let Some(jl) = jl
            && us[jl].mis(&[M::Word, M::Label])
            && first_upper(&us[jl].text)
            && lupper
        {
            return false;
        }
        if let Some(jl) = jl
            && us[jl].kind == K::Term
            && LABEL_NOUNS.contains(&us[jl].text.to_lowercase().as_str())
            && lupper
        {
            return false;
        }
        if lupper
            && let Some(nt) = next_t
            && nt.kind == K::Punct
            && nt.is(".")
        {
            let (jp, _) = self.nxt(k, -1);
            if let Some(jp) = jp
                && ((us[jp].kind == K::Punct && matches!(us[jp].text.as_str(), "," | "&"))
                    || (us[jp].mis(&[M::Letter])
                        && us[jp].b < toks.len()
                        && toks[us[jp].b].kind == K::Punct
                        && toks[us[jp].b].is(".")))
            {
                return false;
            }
        }
        if let Some(jl) = jl
            && us[jl].kind == K::Open
            && jl > 0
        {
            let (j2, _) = self.nxt(jl, -1);
            if let Some(j2) = j2
                && us[j2].mis(&[M::Word, M::Label])
                && first_upper(&us[j2].text)
                && lupper
            {
                return false;
            }
        }
        true
    }

    fn is_binop(u: &Unit) -> bool {
        (u.kind == K::Op
            && (match op_str(&u.text) {
                Some((_, c)) => matches!(c, OpCls::Rel | OpCls::Arrow | OpCls::Bin),
                None => true, // OPS.get(..., ('', 'rel'))
            } || matches!(u.text.as_str(), "<=" | ">=" | "!=" | ">>" | "<<" | ":=")))
            || u.kind == K::Hyph
    }

    fn attachable_rel(u: &Unit) -> bool {
        u.kind == K::Op
            && matches!(
                u.text.as_str(),
                "<" | ">" | "≤" | "≥" | "≠" | "∈" | "∉" | "⊂" | "⊃" | "⊆" | "⊇" | "≪" | "≫" | "<=" | ">=" | "≡" | "∝"
            )
    }

    fn span_has_relation(&self, a: usize, b: usize) -> bool {
        (a..b).any(|k| {
            let u = &self.us[k];
            u.kind == K::Op && (matches!(op_str(&u.text), Some((_, OpCls::Rel))) || matches!(u.text.as_str(), "<=" | ">="))
        })
    }

    fn juxtaposable(&self, i: usize) -> bool {
        let u = &self.us[i];
        if u.kind != K::Term {
            return false;
        }
        if matches!(u.tm().m, M::Strong | M::Group) {
            return true;
        }
        if u.tm().m != M::Letter || matches!(self.ttext(i), "a" | "I" | "A") {
            return false;
        }
        u.a == 0 || matches!(self.toks[u.a - 1].kind, K::Ws | K::Open)
    }

    fn span_has_op(&self, a: usize, b: usize) -> bool {
        (a..b).any(|k| self.us[k].kind == K::Op && Self::is_binop(&self.us[k]))
    }

    fn complex_term(u: &Unit) -> bool {
        let t = u.tm();
        t.m == M::Strong && (matches!(t.base, Base::Norm | Base::Abs | Base::Group) || t.latex.contains(['_', '^']))
    }

    fn simple_tuple(&self, o: usize, c: usize) -> bool {
        let mut items: Vec<Vec<usize>> = vec![vec![]];
        for k in o + 1..c {
            let u = &self.us[k];
            if u.kind == K::Ws {
                continue;
            }
            if u.kind == K::Punct && u.is(",") {
                items.push(vec![]);
            } else {
                items.last_mut().unwrap().push(k);
            }
        }
        if items.len() < 2 || items.iter().any(|it| it.len() != 1) {
            return false;
        }
        if items.iter().any(|it| !self.us[it[0]].mis(&[M::Strong, M::Letter, M::Num])) {
            return false;
        }
        items.iter().any(|it| self.us[it[0]].tm().m == M::Strong)
    }

    fn absorb_application(&self, e: usize) -> usize {
        if e < self.n && self.us[e].kind == K::Open && matches!(self.us[e].text.as_str(), "(" | "[") && !self.tsp(e) {
            let c = self.group_close(e);
            let w = if e > 0 && self.us[e - 1].mis(&[M::Strong]) { 3 } else { 0 };
            if let Some(c) = c
                && self.group_ok(e, c, w)
            {
                return self.absorb_script(c + 1);
            }
        }
        e
    }

    fn absorb_script(&self, e: usize) -> usize {
        let n = self.n;
        if e < n && matches!(self.us[e].kind, K::Sub | K::Sup) && !self.tsp(e) {
            return e + 1;
        }
        if e + 1 < n
            && matches!(self.us[e].kind, K::Caret | K::Us)
            && !self.tsp(e)
            && let Some((r0, _, _)) = parse_script_arg(self.toks, self.us[e + 1].a)
        {
            let mut k = e + 1;
            while k < n && self.us[k].b <= r0 {
                k += 1;
            }
            return k;
        }
        e
    }
}

fn is_differential(t: &str) -> bool {
    // re.match(r'^d[a-z]$', u.text)
    let v: Vec<char> = t.chars().collect();
    (v.len() == 2 || (v.len() == 3 && v[2] == '\n')) && v[0] == 'd' && v[1].is_ascii_lowercase()
}

fn first_upper(s: &str) -> bool {
    s.chars().next().is_some_and(|c| py_isupper(&c.to_string()))
}

/// find_spans: returns (units after any in-place mutation, trimmed spans).
pub fn find_spans(us: Vec<Unit>, toks: &[Tok], symbols: &[char]) -> (Vec<Unit>, Vec<Option<(usize, usize)>>) {
    let n = us.len();
    let mut f = FS {
        us,
        toks,
        n,
        close_of: Vec::new(),
        open_of: Vec::new(),
    };
    (f.close_of, f.open_of) = bracket_partners(&f.us);
    let mut is_anchor = vec![false; n];
    for i in 0..n {
        let u = &f.us[i];
        if u.mis(&[M::Strong]) {
            is_anchor[i] = true;
            if i >= 2
                && f.us[i - 1].kind == K::Hyph
                && !toks[f.us[i - 1].a].sp
                && !toks[u.a].sp
                && f.us[i - 2].mis(&[M::Label])
            {
                is_anchor[i] = false;
            }
        } else if is_strong_op(u) || (u.kind == K::Op && u.cls == "pre") {
            is_anchor[i] = true;
        }
    }
    let mut inspan = vec![false; n];

    for k in 0..n {
        if is_anchor[k] && f.us[k].kind == K::Term && f.unit_like(k) {
            is_anchor[k] = false;
            continue;
        }
        let (ukind, utext) = (f.us[k].kind, f.us[k].text.clone());
        if ukind == K::Open && utext == "(" {
            if let Some(c) = f.group_close(k)
                && f.simple_tuple(k, c)
            {
                is_anchor[k] = true;
            }
        } else if ukind == K::Other && utext == "∞" {
            is_anchor[k] = true;
        } else if f.us[k].mis(&[M::Letter]) {
            let l = f.ttext(k).to_string();
            if matches!(l.as_str(), "O" | "o") && k + 1 < n && f.us[k + 1].kind == K::Open && f.us[k + 1].is("(") && !f.tsp(k + 1) {
                if let Some(c) = f.group_close(k + 1)
                    && f.group_ok(k + 1, c, 0)
                {
                    is_anchor[k] = true;
                }
            } else if single_char(&l).is_some_and(|c| symbols.contains(&c)) && f.plausible_variable(k, true) {
                is_anchor[k] = true;
            }
        } else if ukind == K::Op && matches!(utext.as_str(), "=" | "<" | ">" | "<=" | ">=" | "≈" | "≤" | "≥" | "≠") {
            let (jl, _) = f.nxt(k, -1);
            let (jr, _) = f.nxt(k, 1);
            if let (Some(jl), Some(jr)) = (jl, jr) {
                let var = |x: usize| {
                    f.us[x].mis(&[M::Letter]) && !matches!(f.us[x].text.as_str(), "a" | "A" | "I") && f.plausible_variable(x, false)
                };
                let val = |x: usize| f.us[x].mis(&[M::Num, M::Letter, M::Strong]) || (f.us[x].kind == K::Other && f.us[x].is("∞"));
                if (var(jl) && val(jr)) || (var(jr) && val(jl) && utext != "=") {
                    is_anchor[k] = true;
                }
            }
        }
    }

    let mut spans: Vec<(usize, usize)> = Vec::new();
    let mut i = 0;
    'main: while i < n {
        if !is_anchor[i] || inspan[i] {
            i += 1;
            continue;
        }
        let (mut a, mut b) = (i, i + 1);
        if f.us[i].kind == K::Term {
            b = f.absorb_application(b);
        } else if f.us[i].kind == K::Open {
            b = f.absorb_script(f.group_close(i).unwrap() + 1);
        } else if f.us[i].kind == K::Op {
            if f.us[i].cls == "pre" {
                if i + 1 < n
                    && f.us[i + 1].kind != K::Ws
                    && let Some(e) = f.operand_right(Some(i + 1))
                {
                    b = e;
                }
            } else {
                let (jr, _) = f.nxt(i, 1);
                let e = f.operand_right(jr);
                let (jl, _) = f.nxt(i, -1);
                let st = f.operand_left(jl);
                let Some(e) = e else {
                    i += 1;
                    continue;
                };
                b = e;
                if let Some(st) = st
                    && !(jl.is_some_and(|jl| f.us[jl].mis(&[M::Num]) && f.label_number(jl)))
                {
                    a = st;
                }
            }
        }
        loop {
            let (a0, b0) = (a, b);
            // ---- grow right
            loop {
                let (j, sk) = f.nxt(b - 1, 1);
                let Some(j) = j else { break };
                let glued_ = !sk;
                let last = b - 1;
                if FS::is_binop(&f.us[j]) {
                    if f.us[j].kind == K::Hyph && glued_ {
                        let j2 = j + 1;
                        if f.us[last].mis(&[M::Num]) && j2 < n && f.us[j2].mis(&[M::Num]) {
                            if f.span_has_relation(a, b) {
                                f.us[j].cls = "range".into();
                                b = j2 + 1;
                                continue;
                            }
                            break;
                        }
                        if j2 < n && f.us[j2].mis(&[M::Word, M::Ident, M::Label]) {
                            break;
                        }
                        if crate::ver() >= crate::Ver::V7 && j2 < n && f.hyphen_compound_func(j2) {
                            break; // v7: `$n$-dim`, `γ-sign`: a word hyphen, not a minus
                        }
                        if j2 < n && f.us[j2].kind == K::Ws {
                            break;
                        }
                        if j2 + 1 < n && f.us[j2].mis(&[M::Letter]) && f.us[j2 + 1].kind == K::Punct && f.us[j2 + 1].is(".") {
                            break;
                        }
                    }
                    let (k2, sk2) = f.nxt(j, 1);
                    if f.us[j].kind == K::Hyph && (glued_ != !sk2) {
                        break;
                    }
                    if f.us[last].kind == K::Term
                        && f.us[last].tm().base == Base::Mspan
                        && k2.is_some_and(|k2| f.us[k2].kind == K::Term && f.us[k2].tm().base == Base::Mspan)
                    {
                        break;
                    }
                    let e = f.operand_right(k2);
                    let Some(e) = e else {
                        if FS::attachable_rel(&f.us[j]) && k2.is_some_and(|k2| f.us[k2].mis(&[M::Word])) {
                            b = j + 1;
                        }
                        break;
                    };
                    b = e;
                    continue;
                }
                if f.us[j].kind == K::Dash
                    && glued_
                    && f.us[last].mis(&[M::Num])
                    && f.span_has_relation(a, b)
                    && j + 1 < n
                    && f.us[j + 1].mis(&[M::Num])
                {
                    f.us[j].cls = "range".into();
                    b = j + 2;
                    continue;
                }
                if glued_ && f.us[j].mis(&[M::Strong, M::Letter, M::Num]) && f.us[last].kind == K::Term {
                    if toks[f.us[last].b - 1].kind == K::Sup
                        && f.us[j].tm().base == Base::Word
                        && first_upper(&f.us[j].text)
                    {
                        break;
                    }
                    if f.us[j].tm().base == Base::Mspan || f.us[last].tm().base == Base::Mspan {
                        break;
                    }
                    b = f.absorb_application(j + 1);
                    continue;
                }
                if glued_
                    && f.us[j].mis(&[M::Word])
                    && f.us[last].kind == K::Term
                    && f.us[last].tm().base == Base::Greek
                    && f.us[j].text.chars().count() <= 3
                    && py_isupper(&f.us[j].text)
                {
                    b = j + 1;
                    continue;
                }
                if glued_ && f.us[j].kind == K::Punct && f.us[j].is(",") {
                    let k2 = j + 1;
                    if k2 < n && f.us[k2].mis(&[M::Strong]) && !f.tsp(k2) {
                        b = f.absorb_application(k2 + 1);
                        continue;
                    }
                }
                if f.us[j].mis(&[M::Func]) && f.us[last].kind == K::Term {
                    let (k2, _) = f.nxt(j, 1);
                    if f.operand_right(k2).is_some() {
                        b = f.operand_right(k2).unwrap();
                        continue;
                    }
                }
                if sk
                    && f.us[j].kind == K::Term
                    && FS::complex_term(&f.us[j])
                    && f.us[last].kind == K::Term
                    && FS::complex_term(&f.us[last])
                {
                    b = f.absorb_application(j + 1);
                    continue;
                }
                if sk && f.juxtaposable(j) && f.juxtaposable(last) && f.span_has_op(a, b) {
                    let (k3, _) = f.nxt(j, 1);
                    if let Some(k3) = k3
                        && FS::is_binop(&f.us[k3])
                        && f.us[k3].kind != K::Hyph
                    {
                        b = f.absorb_application(j + 1);
                        continue;
                    }
                }
                break;
            }
            // ---- grow left
            loop {
                let (j, sk) = f.nxt(a, -1);
                let Some(j) = j else { break };
                let glued_ = !sk;
                let first = a;
                if FS::is_binop(&f.us[j]) {
                    if f.us[j].kind == K::Hyph && glued_ {
                        let j0 = j as isize - 1;
                        if j0 >= 0 && f.us[j0 as usize].mis(&[M::Word, M::Ident, M::Label]) {
                            break;
                        }
                        if crate::ver() >= crate::Ver::V7
                            && j0 >= 0
                            && f.us[j0 as usize].mis(&[M::Func])
                            && !(j0 > 0 && matches!(f.us[j0 as usize - 1].kind, K::Op | K::Open))
                        {
                            break; // v7: `log-det/λ`, `arg-max`: an operator-name compound
                        }
                        if j0 >= 1
                            && f.us[j0 as usize].mis(&[M::Letter])
                            && f.us[j0 as usize - 1].kind == K::Hyph
                            && !f.tsp(j0 as usize - 1)
                        {
                            break;
                        }
                        if j0 < 0 || matches!(f.us[j0 as usize].kind, K::Ws | K::Open) {
                            a = j;
                            break;
                        }
                    }
                    let (k2, sk2) = f.nxt(j, -1);
                    if f.us[j].kind == K::Hyph && (glued_ != !sk2) {
                        break;
                    }
                    if f.us[first].kind == K::Term
                        && f.us[first].tm().base == Base::Mspan
                        && k2.is_some_and(|k2| f.us[k2].kind == K::Term && f.us[k2].tm().base == Base::Mspan)
                    {
                        break;
                    }
                    let mut s = f.operand_left(k2);
                    if s.is_some()
                        && let Some(k2v) = k2
                        && f.us[k2v].mis(&[M::Num])
                        && f.label_number(k2v)
                        && !matches!(f.us[j].text.as_str(), "=" | "<" | ">" | "≤" | "≥")
                    {
                        s = None;
                    }
                    let Some(s) = s else {
                        if glued_ && matches!(f.us[j].text.as_str(), "−" | "¬" | "±" | "-") && (k2.is_none() || sk2) {
                            a = j;
                        } else if FS::attachable_rel(&f.us[j]) && k2.is_some_and(|k2| f.us[k2].mis(&[M::Word])) {
                            a = j;
                        }
                        break;
                    };
                    a = s;
                    continue;
                }
                if f.us[j].kind == K::Op && (f.us[j].cls == "pre" || matches!(f.us[j].text.as_str(), "↑" | "↓")) && glued_ {
                    a = j;
                    continue;
                }
                let fu = &f.us[first];
                if glued_
                    && f.us[j].mis(&[M::Strong, M::Letter, M::Num])
                    && (fu.kind == K::Term || (fu.kind == K::Op && fu.cls == "pre"))
                    && !(f.us[j].tm().base == Base::Mspan || (fu.kind == K::Term && fu.tm().base == Base::Mspan))
                {
                    a = j;
                    continue;
                }
                if glued_ && f.us[j].mis(OPERAND_OK_FUNC) && fu.kind == K::Open && !f.tsp(first) {
                    a = j;
                    continue;
                }
                if glued_ && f.us[j].kind == K::Open {
                    let c = f.group_close(j);
                    if let Some(c) = c
                        && f.group_ok(j, c, 0)
                        && j > 0
                        && f.us[j - 1].kind == K::Term
                        && !f.tsp(j)
                        && f.us[j - 1].mis(OPERAND_OK_FUNC)
                    {
                        a = j - 1;
                        b = b.max(f.absorb_script(c + 1));
                        continue;
                    }
                    if let Some(c) = c
                        && c >= b
                        && f.group_ok(j, c, 0)
                    {
                        let lft = if j > 0 { Some(j - 1) } else { None };
                        let rgt = if c + 1 < n { Some(c + 1) } else { None };
                        let att_l = lft.is_some_and(|l| !f.tsp(j) && matches!(f.us[l].kind, K::Op | K::Hyph));
                        let att_r = rgt.is_some_and(|r| {
                            !f.tsp(r) && matches!(f.us[r].kind, K::Caret | K::Sup | K::Sub | K::Us | K::Op)
                        });
                        if att_l || att_r {
                            a = j;
                            b = b.max(f.absorb_script(c + 1));
                            continue;
                        }
                    }
                    if let Some(c) = c
                        && c + 1 >= b
                        && f.simple_tuple(j, c)
                    {
                        a = j;
                        b = b.max(c + 1);
                        continue;
                    }
                }
                if glued_ && f.us[j].kind == K::Punct && f.us[j].is(",") && j >= 1 && f.us[j - 1].mis(&[M::Strong]) {
                    a = j - 1;
                    continue;
                }
                if f.us[j].mis(&[M::Func]) {
                    a = j;
                    continue;
                }
                if sk
                    && f.us[j].kind == K::Term
                    && FS::complex_term(&f.us[j])
                    && f.us[first].kind == K::Term
                    && FS::complex_term(&f.us[first])
                {
                    a = j;
                    continue;
                }
                if sk
                    && f.juxtaposable(j)
                    && f.juxtaposable(first)
                    && f.span_has_op(a, b)
                    && (f.us[j].tm().m == M::Strong || f.us[first].tm().m == M::Strong)
                {
                    a = j;
                    continue;
                }
                break;
            }
            if (a, b) == (a0, b0) {
                break;
            }
        }
        if b - a == 1 && f.us[a].kind == K::Op {
            let (j, _) = f.nxt(a, 1);
            if f.operand_right(j).is_none() {
                i += 1;
                continue 'main;
            }
            b = f.operand_right(j).unwrap();
        }
        for k in a..b {
            inspan[k] = true;
        }
        spans.push((a, b));
        i = b;
    }
    spans.sort();
    let mut merged: Vec<(usize, usize)> = Vec::new();
    for s in spans {
        if let Some(m) = merged.last_mut()
            && s.0 <= m.1
        {
            m.1 = m.1.max(s.1);
        } else {
            merged.push(s);
        }
    }
    let out = merged.into_iter().map(|(a, b)| trim_span(&f.us, a, b)).collect();
    (f.us, out)
}

fn bracket_partners(us: &[Unit]) -> (Vec<Option<usize>>, Vec<Option<usize>>) {
    let n = us.len();
    let (mut close_of, mut open_of) = (vec![None; n], vec![None; n]);
    let mut stacks: Vec<(&str, Vec<usize>)> = Vec::new();
    for k in 0..n {
        let u = &us[k];
        if u.kind == K::Open {
            match stacks.iter_mut().find(|(o, _)| *o == u.text) {
                Some((_, st)) => st.push(k),
                None => stacks.push((u.text.as_str(), vec![k])),
            }
        } else if u.kind == K::Close
            && let Some(o) = pair_rev(&u.text)
            && let Some((_, st)) = stacks.iter_mut().find(|(x, _)| *x == o)
            && let Some(op) = st.pop()
        {
            close_of[op] = Some(k);
        }
    }
    // the backward scan from a close is the mirror image
    let mut stacks: Vec<(&str, Vec<usize>)> = Vec::new();
    for k in (0..n).rev() {
        let u = &us[k];
        if u.kind == K::Close {
            match stacks.iter_mut().find(|(c, _)| *c == u.text) {
                Some((_, st)) => st.push(k),
                None => stacks.push((u.text.as_str(), vec![k])),
            }
        } else if u.kind == K::Open
            && let Some(c) = pair(&u.text)
            && let Some((_, st)) = stacks.iter_mut().find(|(x, _)| *x == c)
            && let Some(cl) = st.pop()
        {
            open_of[cl] = Some(k);
        }
    }
    (close_of, open_of)
}

fn single_char(s: &str) -> Option<char> {
    let mut it = s.chars();
    match (it.next(), it.next()) {
        (Some(c), None) => Some(c),
        _ => None,
    }
}

pub fn trim_span(us: &[Unit], mut a: usize, mut b: usize) -> Option<(usize, usize)> {
    while a < b
        && (matches!(us[a].kind, K::Ws | K::Punct | K::Emph | K::Dash | K::Close)
            || (us[a].kind == K::Op && matches!(us[a].cls.as_str(), "rel" | "arrow")))
    {
        a += 1;
    }
    while b > a
        && (matches!(us[b - 1].kind, K::Ws | K::Punct | K::Emph | K::Dash | K::Open | K::Hyph)
            || (us[b - 1].kind == K::Op && matches!(us[b - 1].cls.as_str(), "rel" | "arrow" | "bin")))
    {
        b -= 1;
    }
    let bal = |a: usize, b: usize| {
        let mut d = 0i64;
        for k in a..b {
            if us[k].kind == K::Open {
                d += 1;
            } else if us[k].kind == K::Close {
                d -= 1;
                if d < 0 {
                    return false;
                }
            }
        }
        d == 0
    };
    while a < b && !bal(a, b) {
        if us[a].kind == K::Open {
            a += 1;
        } else if us[b - 1].kind == K::Close {
            b -= 1;
        } else {
            return None;
        }
    }
    if a >= b {
        return None;
    }
    Some((a, b))
}
