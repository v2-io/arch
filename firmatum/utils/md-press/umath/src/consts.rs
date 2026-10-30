//! Symbol tables, verbatim from the reference (umath_v3.py top of file).

pub fn greek(c: char) -> Option<&'static str> {
    Some(match c {
        'α' => r"\alpha", 'β' => r"\beta", 'γ' => r"\gamma", 'δ' => r"\delta", 'ε' => r"\varepsilon",
        'ϵ' => r"\epsilon", 'ζ' => r"\zeta", 'η' => r"\eta", 'θ' => r"\theta", 'ϑ' => r"\vartheta",
        'ι' => r"\iota", 'κ' => r"\kappa", 'λ' => r"\lambda", 'μ' => r"\mu", 'ν' => r"\nu", 'ξ' => r"\xi",
        'π' => r"\pi", 'ϖ' => r"\varpi", 'ρ' => r"\rho", 'ϱ' => r"\varrho", 'σ' => r"\sigma", 'ς' => r"\varsigma",
        'τ' => r"\tau", 'υ' => r"\upsilon", 'φ' => r"\phi", 'ϕ' => r"\phi", 'χ' => r"\chi", 'ψ' => r"\psi",
        'ω' => r"\omega", 'Γ' => r"\Gamma", 'Δ' => r"\Delta", 'Θ' => r"\Theta", 'Λ' => r"\Lambda",
        'Ξ' => r"\Xi", 'Π' => r"\Pi", 'Σ' => r"\Sigma", 'Υ' => r"\Upsilon", 'Φ' => r"\Phi", 'Ψ' => r"\Psi",
        'Ω' => r"\Omega", 'ο' => "o",
        'Α' => "A", 'Β' => "B", 'Ε' => "E", 'Ζ' => "Z", 'Η' => "H", 'Ι' => "I", 'Κ' => "K", 'Μ' => "M",
        'Ν' => "N", 'Ο' => "O", 'Ρ' => "P", 'Τ' => "T", 'Χ' => "X",
        'ϰ' => r"\varkappa",
        _ => return None,
    })
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OpCls {
    Rel,
    Arrow,
    Bin,
    Pre,
    Ord,
    Post,
}

impl OpCls {
    pub fn name(self) -> &'static str {
        match self {
            OpCls::Rel => "rel",
            OpCls::Arrow => "arrow",
            OpCls::Bin => "bin",
            OpCls::Pre => "pre",
            OpCls::Ord => "ord",
            OpCls::Post => "post",
        }
    }
}

pub fn op(c: char) -> Option<(&'static str, OpCls)> {
    use OpCls::*;
    Some(match c {
        '=' => ("=", Rel), '<' => (r"\lt", Rel), '>' => (r"\gt", Rel), '≤' => (r"\leq", Rel),
        '≥' => (r"\geq", Rel), '≠' => (r"\neq", Rel), '≈' => (r"\approx", Rel), '≡' => (r"\equiv", Rel),
        '∝' => (r"\propto", Rel), '∼' => (r"\sim", Rel), '≪' => (r"\ll", Rel), '≫' => (r"\gg", Rel),
        '∈' => (r"\in", Rel), '∉' => (r"\notin", Rel), '⊂' => (r"\subset", Rel), '⊃' => (r"\supset", Rel),
        '⊆' => (r"\subseteq", Rel), '⊇' => (r"\supseteq", Rel), '≺' => (r"\prec", Rel), '≻' => (r"\succ", Rel),
        '⪰' => (r"\succeq", Rel), '⪯' => (r"\preceq", Rel), '≼' => (r"\preceq", Rel), '≽' => (r"\succeq", Rel),
        '⊥' => (r"\perp", Rel), '≅' => (r"\cong", Rel), '≃' => (r"\simeq", Rel), '≜' => (r"\triangleq", Rel),
        '≔' => (":=", Rel), '∣' => (r"\mid", Rel), '⊨' => (r"\models", Rel), '⊢' => (r"\vdash", Rel),
        '≲' => (r"\lesssim", Rel), '≳' => (r"\gtrsim", Rel), '≍' => (r"\asymp", Rel), '∥' => (r"\parallel", Rel),
        '⊊' => (r"\subsetneq", Rel), '⊋' => (r"\supsetneq", Rel), '∋' => (r"\ni", Rel),
        '→' => (r"\to", Arrow), '←' => (r"\leftarrow", Arrow), '↔' => (r"\leftrightarrow", Arrow),
        '⇒' => (r"\Rightarrow", Arrow), '⇐' => (r"\Leftarrow", Arrow), '⇔' => (r"\Leftrightarrow", Arrow),
        '⟹' => (r"\Longrightarrow", Arrow), '⟸' => (r"\Longleftarrow", Arrow), '⟺' => (r"\Longleftrightarrow", Arrow),
        '↦' => (r"\mapsto", Arrow), '⟶' => (r"\longrightarrow", Arrow), '⟵' => (r"\longleftarrow", Arrow),
        '↑' => (r"\uparrow", Arrow), '↓' => (r"\downarrow", Arrow), '⇝' => (r"\leadsto", Arrow),
        '↛' => (r"\nrightarrow", Arrow), '⇏' => (r"\nRightarrow", Arrow),
        '+' => ("+", Bin), '−' => ("-", Bin), '·' => (r"\cdot", Bin), '⋅' => (r"\cdot", Bin),
        '×' => (r"\times", Bin), '÷' => (r"\div", Bin), '±' => (r"\pm", Bin), '∓' => (r"\mp", Bin),
        '∘' => (r"\circ", Bin), '⊗' => (r"\otimes", Bin), '⊕' => (r"\oplus", Bin), '⊙' => (r"\odot", Bin),
        '∪' => (r"\cup", Bin), '∩' => (r"\cap", Bin), '∧' => (r"\wedge", Bin), '∨' => (r"\vee", Bin),
        '∖' => (r"\setminus", Bin), '/' => ("/", Bin), '⊔' => (r"\sqcup", Bin), '⊓' => (r"\sqcap", Bin),
        '⋃' | '⋂' if crate::ver() < crate::Ver::V6 => return None,
        '⋃' => (r"\bigcup", Pre),
        '⋂' => (r"\bigcap", Pre),
        '∑' => (r"\sum", Pre), '∏' => (r"\prod", Pre), '∫' => (r"\int", Pre), '∮' => (r"\oint", Pre),
        '∂' => (r"\partial", Pre), '∇' => (r"\nabla", Pre), '√' => (r"\sqrt", Pre), '¬' => (r"\neg", Pre),
        '∀' => (r"\forall", Pre), '∃' => (r"\exists", Pre), '∄' => (r"\nexists", Pre), '□' => (r"\Box", Pre),
        '◇' => (r"\Diamond", Pre), '∛' => (r"\sqrt[3]", Pre),
        '∞' => (r"\infty", Ord), '∅' => (r"\emptyset", Ord), 'ℵ' => (r"\aleph", Ord), '†' => (r"^\dagger", Post),
        '⋯' => (r"\cdots", Ord), '∠' => (r"\angle", Ord), '△' => (r"\triangle", Ord),
        _ => return None,
    })
}

/// OPS lookup on a token text (a single char, or a 2-char ASCII compound).
pub fn op_str(t: &str) -> Option<(&'static str, OpCls)> {
    let mut it = t.chars();
    match (it.next(), it.next()) {
        (Some(c), None) => op(c),
        _ => None,
    }
}

/// The ASCII compound relations the lexer recognizes, and their LaTeX.
pub const COMPOUNDS: [(&str, &str); 6] =
    [("<=", r"\leq"), (">=", r"\geq"), ("!=", r"\neq"), (">>", r"\gg"), ("<<", r"\ll"), (":=", ":=")];

pub fn compound_latex(t: &str) -> Option<&'static str> {
    COMPOUNDS.iter().find(|(k, _)| *k == t).map(|(_, v)| *v)
}

pub const STRONG_OPS: &str = "≡∝∼≪≫∈∉⊂⊃⊆⊇≺≻⪰⪯≼≽⊥≅≃≜≔∣⊨⊢≲≳≍⊊⊋∋↦⟹⟸⟺∓∘⊗⊕⊙∪∩∧∨∖⊔⊓∑∏∫∮∂∇¬∀∃∄∞∅ℵ⋯";

pub fn is_strong_op_char(t: &str) -> bool {
    let mut it = t.chars();
    matches!((it.next(), it.next()), (Some(c), None) if STRONG_OPS.contains(c))
}

pub fn open_latex(c: &str) -> Option<&'static str> {
    Some(match c {
        "(" => "(", "[" => "[", "{" => r"\{", "⟨" => r"\langle", "⟦" => r"\llbracket", "⌊" => r"\lfloor", "⌈" => r"\lceil",
        _ => return None,
    })
}

pub fn close_latex(c: &str) -> Option<&'static str> {
    Some(match c {
        ")" => ")", "]" => "]", "}" => r"\}", "⟩" => r"\rangle", "⟧" => r"\rrbracket", "⌋" => r"\rfloor", "⌉" => r"\rceil",
        _ => return None,
    })
}

pub fn pair(o: &str) -> Option<&'static str> {
    Some(match o {
        "(" => ")", "[" => "]", "{" => "}", "⟨" => "⟩", "⟦" => "⟧", "⌊" => "⌋", "⌈" => "⌉",
        _ => return None,
    })
}

pub fn pair_rev(c: &str) -> Option<&'static str> {
    Some(match c {
        ")" => "(", "]" => "[", "}" => "{", "⟩" => "⟨", "⟧" => "⟦", "⌋" => "⌊", "⌉" => "⌈",
        _ => return None,
    })
}

pub fn combining(c: char) -> Option<&'static str> {
    Some(match c {
        '\u{302}' => r"\hat", '\u{303}' => r"\tilde", '\u{304}' => r"\bar", '\u{305}' => r"\bar",
        '\u{307}' => r"\dot", '\u{308}' => r"\ddot", '\u{20d7}' => r"\vec", '\u{306}' => r"\breve",
        '\u{30c}' => r"\check", '\u{301}' => r"\acute", '\u{300}' => r"\grave",
        _ => return None,
    })
}

pub fn func(w: &str) -> Option<&'static str> {
    Some(match w {
        "max" => r"\max", "min" => r"\min", "sup" => r"\sup", "inf" => r"\inf", "lim" => r"\lim",
        "log" => r"\log", "ln" => r"\ln", "exp" => r"\exp", "sin" => r"\sin", "cos" => r"\cos",
        "tan" => r"\tan", "det" => r"\det", "arg" => r"\arg", "Pr" => r"\Pr", "limsup" => r"\limsup",
        "liminf" => r"\liminf", "tanh" => r"\tanh", "gcd" => r"\gcd", "dim" => r"\dim", "ker" => r"\ker",
        "argmax" => r"\operatorname{argmax}", "argmin" => r"\operatorname{argmin}",
        "Var" => r"\operatorname{Var}", "Cov" => r"\operatorname{Cov}", "tr" => r"\operatorname{tr}",
        "softmax" => r"\operatorname{softmax}", "sgn" => r"\operatorname{sgn}", "sign" => r"\operatorname{sign}",
        "diag" => r"\operatorname{diag}", "rank" => r"\operatorname{rank}", "KL" => r"\mathrm{KL}",
        "Corr" => r"\operatorname{Corr}", "erf" => r"\operatorname{erf}", "poly" => r"\operatorname{poly}",
        _ => return None,
    })
}

pub const UNITS_AFTER_MU: &str = "smgLlVAFWJKHΩ";

/// `LARGE = set('∑∏∫∮∇∂⋃⋂')` (v6 spelling; v3-v5 wrote
/// `… | {'Σ', 'Π'} - {'Σ', 'Π'}`, which is the same set).
pub fn is_large(t: &str) -> bool {
    let mut it = t.chars();
    matches!((it.next(), it.next()), (Some(c), None) if "∑∏∫∮∇∂⋃⋂".contains(c))
}

pub const LABEL_NOUNS: &[&str] = &[
    "appendix", "section", "table", "figure", "fig", "eq", "equation", "chapter", "part", "step", "phase",
    "option", "plan", "model", "class", "type", "level", "case", "tier", "track", "theorem", "lemma",
    "hypothesis", "proposition", "corollary", "definition", "assumption", "condition", "example",
    "exhibit", "annex", "stage", "variant", "version", "scenario", "group", "team", "set", "item",
    "box", "panel", "row", "column", "gate", "wave", "round", "batch", "pass", "regime", "mode",
];

/// REL_LATEX: rel/arrow LaTeX from OPS plus the listed extras.
pub fn rel_latex() -> Vec<&'static str> {
    let mut v: Vec<&'static str> = Vec::new();
    for c in "=<>≤≥≠≈≡∝∼≪≫∈∉⊂⊃⊆⊇≺≻⪰⪯≼≽⊥≅≃≜≔∣⊨⊢≲≳≍∥⊊⊋∋→←↔⇒⇐⇔⟹⟸⟺↦⟶⟵↑↓⇝↛⇏".chars() {
        let (l, k) = op(c).unwrap();
        assert!(matches!(k, OpCls::Rel | OpCls::Arrow));
        v.push(l);
    }
    for x in [
        r"\leq", r"\geq", r"\neq", r"\gg", r"\ll", ":=", "=", "+", "-", r"\cdot", r"\times", r"\pm", r"\cup",
        r"\cap", r"\circ", r"\otimes", r"\oplus", r"\wedge", r"\vee", r"\setminus", r"\mid", r"\approx",
    ] {
        v.push(x);
    }
    v.sort();
    v.dedup();
    v
}
