"""Deterministic Unicode-math -> $LaTeX$ converter for md-press prose pieces.

Prototype (spike 2026-10-02). Three stages:

  lex     text -> fine tokens (protected ranges opaque; existing $..$ kept)
  spans   tokens -> math spans (seed on unambiguous anchors, grow across
          operators / application / glued adjacency, TN28-style; trim)
  latex   span tokens -> house LaTeX (char maps + a small sub/sup grammar)

The output is always "the input with some regions replaced by $...$ spans";
nothing outside a span is ever touched, by construction.
"""
from __future__ import annotations

import re
import unicodedata
from dataclasses import dataclass, field

# ---------------------------------------------------------------- tables

GREEK = {
    'α': r'\alpha', 'β': r'\beta', 'γ': r'\gamma', 'δ': r'\delta', 'ε': r'\varepsilon',
    'ϵ': r'\epsilon', 'ζ': r'\zeta', 'η': r'\eta', 'θ': r'\theta', 'ϑ': r'\vartheta',
    'ι': r'\iota', 'κ': r'\kappa', 'λ': r'\lambda', 'μ': r'\mu', 'ν': r'\nu', 'ξ': r'\xi',
    'π': r'\pi', 'ϖ': r'\varpi', 'ρ': r'\rho', 'ϱ': r'\varrho', 'σ': r'\sigma', 'ς': r'\varsigma',
    'τ': r'\tau', 'υ': r'\upsilon', 'φ': r'\phi', 'ϕ': r'\phi', 'χ': r'\chi', 'ψ': r'\psi',
    'ω': r'\omega', 'Γ': r'\Gamma', 'Δ': r'\Delta', 'Θ': r'\Theta', 'Λ': r'\Lambda',
    'Ξ': r'\Xi', 'Π': r'\Pi', 'Σ': r'\Sigma', 'Υ': r'\Upsilon', 'Φ': r'\Phi', 'Ψ': r'\Psi',
    'Ω': r'\Omega', 'ο': 'o',
    # Greek capitals that are Latin-shaped
    'Α': 'A', 'Β': 'B', 'Ε': 'E', 'Ζ': 'Z', 'Η': 'H', 'Ι': 'I', 'Κ': 'K', 'Μ': 'M',
    'Ν': 'N', 'Ο': 'O', 'Ρ': 'P', 'Τ': 'T', 'Χ': 'X',
    # math-italic/Greek symbol variants sometimes typed
    'ϰ': r'\varkappa',
}

# operator glyph -> (latex, class); classes: rel, bin, arrow, pre (prefix/large), ord
OPS = {
    '=': ('=', 'rel'), '<': (r'\lt', 'rel'), '>': (r'\gt', 'rel'), '≤': (r'\leq', 'rel'),
    '≥': (r'\geq', 'rel'), '≠': (r'\neq', 'rel'), '≈': (r'\approx', 'rel'), '≡': (r'\equiv', 'rel'),
    '∝': (r'\propto', 'rel'), '∼': (r'\sim', 'rel'), '≪': (r'\ll', 'rel'), '≫': (r'\gg', 'rel'),
    '∈': (r'\in', 'rel'), '∉': (r'\notin', 'rel'), '⊂': (r'\subset', 'rel'), '⊃': (r'\supset', 'rel'),
    '⊆': (r'\subseteq', 'rel'), '⊇': (r'\supseteq', 'rel'), '≺': (r'\prec', 'rel'), '≻': (r'\succ', 'rel'),
    '⪰': (r'\succeq', 'rel'), '⪯': (r'\preceq', 'rel'), '≼': (r'\preceq', 'rel'), '≽': (r'\succeq', 'rel'),
    '⊥': (r'\perp', 'rel'), '≅': (r'\cong', 'rel'), '≃': (r'\simeq', 'rel'), '≜': (r'\triangleq', 'rel'),
    '≔': (':=', 'rel'), '∣': (r'\mid', 'rel'), '⊨': (r'\models', 'rel'), '⊢': (r'\vdash', 'rel'),
    '≲': (r'\lesssim', 'rel'), '≳': (r'\gtrsim', 'rel'), '≍': (r'\asymp', 'rel'), '∥': (r'\parallel', 'rel'),
    '⊊': (r'\subsetneq', 'rel'), '⊋': (r'\supsetneq', 'rel'), '∋': (r'\ni', 'rel'),
    '→': (r'\to', 'arrow'), '←': (r'\leftarrow', 'arrow'), '↔': (r'\leftrightarrow', 'arrow'),
    '⇒': (r'\Rightarrow', 'arrow'), '⇐': (r'\Leftarrow', 'arrow'), '⇔': (r'\Leftrightarrow', 'arrow'),
    '⟹': (r'\Longrightarrow', 'arrow'), '⟸': (r'\Longleftarrow', 'arrow'), '⟺': (r'\Longleftrightarrow', 'arrow'),
    '↦': (r'\mapsto', 'arrow'), '⟶': (r'\longrightarrow', 'arrow'), '⟵': (r'\longleftarrow', 'arrow'),
    '↑': (r'\uparrow', 'arrow'), '↓': (r'\downarrow', 'arrow'), '⇝': (r'\leadsto', 'arrow'),
    '↛': (r'\nrightarrow', 'arrow'), '⇏': (r'\nRightarrow', 'arrow'),
    '+': ('+', 'bin'), '−': ('-', 'bin'), '·': (r'\cdot', 'bin'), '⋅': (r'\cdot', 'bin'),
    '×': (r'\times', 'bin'), '÷': (r'\div', 'bin'), '±': (r'\pm', 'bin'), '∓': (r'\mp', 'bin'),
    '∘': (r'\circ', 'bin'), '⊗': (r'\otimes', 'bin'), '⊕': (r'\oplus', 'bin'), '⊙': (r'\odot', 'bin'),
    '∪': (r'\cup', 'bin'), '∩': (r'\cap', 'bin'), '∧': (r'\wedge', 'bin'), '∨': (r'\vee', 'bin'),
    '∖': (r'\setminus', 'bin'), '/': ('/', 'bin'), '⊔': (r'\sqcup', 'bin'), '⊓': (r'\sqcap', 'bin'),
    '∑': (r'\sum', 'pre'), '∏': (r'\prod', 'pre'), '∫': (r'\int', 'pre'), '∮': (r'\oint', 'pre'),
    '∂': (r'\partial', 'pre'), '∇': (r'\nabla', 'pre'), '√': (r'\sqrt', 'pre'), '¬': (r'\neg', 'pre'),
    '∀': (r'\forall', 'pre'), '∃': (r'\exists', 'pre'), '∄': (r'\nexists', 'pre'), '□': (r'\Box', 'pre'),
    '◇': (r'\Diamond', 'pre'), '∛': (r'\sqrt[3]', 'pre'),
    '∞': (r'\infty', 'ord'), '∅': (r'\emptyset', 'ord'), 'ℵ': (r'\aleph', 'ord'), '†': (r'^\dagger', 'post'),
    '⋯': (r'\cdots', 'ord'), '∠': (r'\angle', 'ord'), '△': (r'\triangle', 'ord'),
}
# operators that by themselves prove a span is math (the rest are also prose
# punctuation in this estate, or plain ASCII)
STRONG_OPS = set('≡∝∼≪≫∈∉⊂⊃⊆⊇≺≻⪰⪯≼≽⊥≅≃≜≔∣⊨⊢≲≳≍⊊⊋∋↦⟹⟸⟺∓∘⊗⊕⊙∪∩∧∨∖⊔⊓∑∏∫∮∂∇¬∀∃∄∞∅ℵ⋯')
# ≈ → · × ± √ ⇒ ⇔ ↔ ← are also house punctuation: math only by role
WEAK_OPS = set('→·×≈±√⇒⇔↔←⟶⟵↑↓⇝↛⇏⋅÷≤≥≠') | set('=<>+−/')

OPEN = {'(': '(', '[': '[', '{': r'\{', '⟨': r'\langle', '⟦': r'\llbracket', '⌊': r'\lfloor', '⌈': r'\lceil'}
CLOSE = {')': ')', ']': ']', '}': r'\}', '⟩': r'\rangle', '⟧': r'\rrbracket', '⌋': r'\rfloor', '⌉': r'\rceil'}
PAIR = {'(': ')', '[': ']', '{': '}', '⟨': '⟩', '⟦': '⟧', '⌊': '⌋', '⌈': '⌉'}

COMBINING = {'̂': r'\hat', '̃': r'\tilde', '̄': r'\bar', '̅': r'\bar',
             '̇': r'\dot', '̈': r'\ddot', '⃗': r'\vec', '̆': r'\breve',
             '̌': r'\check', '́': r'\acute', '̀': r'\grave'}

FUNCS = {'max': r'\max', 'min': r'\min', 'sup': r'\sup', 'inf': r'\inf', 'lim': r'\lim',
         'log': r'\log', 'ln': r'\ln', 'exp': r'\exp', 'sin': r'\sin', 'cos': r'\cos',
         'tan': r'\tan', 'det': r'\det', 'arg': r'\arg', 'Pr': r'\Pr', 'limsup': r'\limsup',
         'liminf': r'\liminf', 'tanh': r'\tanh', 'gcd': r'\gcd', 'dim': r'\dim', 'ker': r'\ker',
         'argmax': r'\operatorname{argmax}', 'argmin': r'\operatorname{argmin}',
         'Var': r'\operatorname{Var}', 'Cov': r'\operatorname{Cov}', 'tr': r'\operatorname{tr}',
         'softmax': r'\operatorname{softmax}', 'sgn': r'\operatorname{sgn}', 'sign': r'\operatorname{sign}',
         'diag': r'\operatorname{diag}', 'rank': r'\operatorname{rank}', 'KL': r'\mathrm{KL}',
         'Corr': r'\operatorname{Corr}', 'erf': r'\operatorname{erf}', 'poly': r'\operatorname{poly}'}

UNITS_AFTER_MU = set('smgLlVAFWJKHΩ')
LARGE = set('∑∏∫∮∇∂⋃⋂') | {'Σ', 'Π'} - {'Σ', 'Π'}


def mathalpha_latex(c: str) -> str | None:
    """Mathematical alphanumerics / letterlike symbols -> LaTeX, or None."""
    special = {'ℝ': r'\mathbb{R}', 'ℕ': r'\mathbb{N}', 'ℤ': r'\mathbb{Z}', 'ℚ': r'\mathbb{Q}',
               'ℂ': r'\mathbb{C}', 'ℙ': r'\mathbb{P}', 'ℓ': r'\ell', 'ℏ': r'\hbar', 'ℋ': r'\mathcal{H}',
               'ℒ': r'\mathcal{L}', 'ℱ': r'\mathcal{F}', 'ℰ': r'\mathcal{E}', 'ℳ': r'\mathcal{M}',
               'ℬ': r'\mathcal{B}', 'ℛ': r'\mathcal{R}', 'ℐ': r'\mathcal{I}', '℘': r'\wp',
               'ℑ': r'\Im', 'ℜ': r'\Re', '𝟙': r'\mathbb{1}', 'ℊ': 'g', 'ℯ': 'e', 'ℴ': 'o'}
    if c in special:
        return special[c]
    if not (0x1D400 <= ord(c) <= 0x1D7FF):
        return None
    name = unicodedata.name(c, '')
    m = re.match(r'MATHEMATICAL (.*?) ?(CAPITAL|SMALL|DIGIT) (\w+)$', name)
    if not m:
        return None
    style, case, letter = m.groups()
    if case == 'DIGIT':
        ch = str(['ZERO', 'ONE', 'TWO', 'THREE', 'FOUR', 'FIVE', 'SIX', 'SEVEN', 'EIGHT', 'NINE'].index(letter))
    elif len(letter) == 1:
        ch = letter if case == 'CAPITAL' else letter.lower()
    else:  # Greek in math alphanumerics
        g = unicodedata.lookup(f"GREEK {'CAPITAL' if case == 'CAPITAL' else 'SMALL'} LETTER {letter}")
        ch = GREEK.get(g, g)
    wrap = {'SCRIPT': r'\mathcal', 'BOLD SCRIPT': r'\mathcal', 'DOUBLE-STRUCK': r'\mathbb',
            'FRAKTUR': r'\mathfrak', 'BOLD FRAKTUR': r'\mathfrak', 'BOLD': r'\mathbf',
            'SANS-SERIF': r'\mathsf', 'MONOSPACE': r'\mathtt', 'ITALIC': '', 'BOLD ITALIC': r'\boldsymbol'}
    w = wrap.get(style, '')
    return f'{w}{{{ch}}}' if w else ch


def subsup_of(c: str):
    """('sub'|'sup', ascii-ish char) for Unicode sub/superscript glyphs."""
    d = unicodedata.decomposition(c)
    m = re.match(r'<(sub|super)> ([0-9A-F]+)$', d)
    if not m:
        if c == 'ᵀ':
            return ('sup', 'T')
        return None
    base = chr(int(m.group(2), 16))
    if base == '−':
        base = '-'
    return ('sub' if m.group(1) == 'sub' else 'sup', base)


def is_greek(c: str) -> bool:
    return c in GREEK and c not in 'ο'


def is_greek_script(c: str) -> bool:
    """Any Greek-block letter (incl. accented, polytonic) — for Greek-word detection."""
    o = ord(c)
    return (0x0370 <= o <= 0x03FF or 0x1F00 <= o <= 0x1FFF) and c.isalpha()


# ---------------------------------------------------------------- protected ranges (port of md-press)

def protected_ranges(s: str):
    out = []
    i = 0
    n = len(s)
    while i < n:
        c = s[i]
        if c == '\\':
            i += 2
            continue
        if c == '`':
            k = i
            while k < n and s[k] == '`':
                k += 1
            fence = s[i:k]
            j = s.find(fence, k)
            if j >= 0:
                out.append((i, j + len(fence), 'code'))
                i = j + len(fence)
            else:
                i = k
            continue
        if c == '$':
            d = '$$' if s.startswith('$$', i) else '$'
            j = i + len(d)
            end = None
            while j < n:
                if s[j] == '\\':
                    j += 2
                    continue
                if s.startswith(d, j):
                    end = j + len(d)
                    break
                j += 1
            end = end if end is not None else n
            out.append((i, end, 'math'))
            i = end
            continue
        if c == ']' and i + 1 < n and s[i + 1] == '(':
            depth = 0
            j = i + 1
            while j < n:
                if s[j] == '\\':
                    j += 2
                    continue
                if s[j] == '(':
                    depth += 1
                elif s[j] == ')':
                    depth -= 1
                    if depth == 0:
                        break
                j += 1
            end = min(j + 1, n)
            out.append((i + 1, end, 'link'))
            i = end
            continue
        if s.startswith('[[', i):
            j = s.find(']]', i)
            if j >= 0:
                out.append((i, j + 2, 'wiki'))
                i = j + 2
                continue
        if c == '<':
            j = s.find('>', i)
            if j >= 0:
                inner = s[i + 1:j]
                tagish = inner[:1].isascii() and (inner[:1].isalpha() or inner[:1] in '/!')
                simple = not re.search(r'\s', inner)
                with_attrs = '=' in inner and re.match(r'^[A-Za-z0-9/]+\s', inner + ' ') is not None
                if tagish and (simple or with_attrs) and inner:
                    out.append((i, j + 1, 'html'))
                    i = j + 1
                    continue
        if s.startswith(('http://', 'https://', 'www.'), i):
            m = re.compile(r'\s').search(s, i)
            end = m.start() if m else n
            out.append((i, end, 'url'))
            i = end
            continue
        i += 1
    return out


# ---------------------------------------------------------------- lexer

@dataclass
class Tok:
    kind: str          # see lex()
    text: str
    a: int             # start offset in piece
    b: int             # end offset
    cls: str = ''      # operator class / extra info
    sp_before: bool = False   # whitespace immediately before


PUNCT = set(',;:.?!')
DASHES = set('—–')
QUOTES = set('"“”‘’«»')


def lex(s: str):
    prot = protected_ranges(s)
    pmap = {}
    for a, b, k in prot:
        pmap[a] = (b, k)
    toks: list[Tok] = []
    i, n = 0, len(s)
    while i < n:
        if i in pmap:
            b, k = pmap[i]
            toks.append(Tok('mspan' if k == 'math' else 'prot', s[i:b], i, b, k))
            i = b
            continue
        c = s[i]
        if c.isspace():
            j = i
            while j < n and s[j].isspace() and j not in pmap:
                j += 1
            toks.append(Tok('ws', s[i:j], i, j))
            i = j
            continue
        if c == '\\' and i + 1 < n:
            toks.append(Tok('esc', s[i:i + 2], i, i + 2, s[i + 1]))
            i += 2
            continue
        if c == '*' or c == '_' and (i == 0 or s[i - 1].isspace() or s[i - 1] in '([{"“‘\'*~>'):
            # emphasis delimiter runs (`_` only when not glued after an operand)
            j = i
            while j < n and s[j] == c:
                j += 1
            toks.append(Tok('star' if c == '*' and j - i == 1 else 'emph', s[i:j], i, j))
            i = j
            continue
        if c == '_':
            toks.append(Tok('us', c, i, i + 1))
            i += 1
            continue
        if c == '^':
            toks.append(Tok('caret', c, i, i + 1))
            i += 1
            continue
        if c.isdigit() and c.isascii() and (i == 0 or not (s[i - 1].isalnum() or s[i - 1] == '_')) and SNAKE.match(s, i):
            m = SNAKE.match(s, i)
            toks.append(Tok('word', m.group(), i, m.end(), 'ident'))   # 12k_example_identifier
            i = m.end()
            continue
        if c.isdigit() and c.isascii():
            m = re.compile(r'\d+(?:[.,]\d+)*').match(s, i)
            toks.append(Tok('num', m.group(), i, m.end()))
            i = m.end()
            continue
        if (c.isascii() and c.isalnum()) and (i == 0 or not (s[i - 1].isalnum() or s[i - 1] == '_')):
            m = SNAKE.match(s, i)
            if m:
                toks.append(Tok('word', m.group(), i, m.end(), 'ident'))   # some_long_snake_identifier
                i = m.end()
                continue
        if c.isascii() and c.isalpha() and toks and toks[-1].kind == 'num' and toks[-1].b == i:
            m = re.compile(r'[A-Za-z]+').match(s, i)
            toks.append(Tok('word', m.group(), i, m.end(), 'unit'))   # 200k, 5ms, 2x, 3rd
            i = m.end()
            continue
        if c.isascii() and c.isalpha():
            # letters, then glued digits make a label (E1, T4, RG0a); keep a
            # single letter + digits as a label too — `x2` is rare in this estate
            m = re.compile(r'[A-Za-z]+(?:[0-9]+[A-Za-z]*)*').match(s, i)
            w = m.group()
            if len(w) == 1 and w.isupper() and s[m.end():m.end() + 1] == '∞':
                toks.append(Tok('word', w + '∞', i, m.end() + 1, 'label'))   # D∞, B∞: names
                i = m.end() + 1
                continue
            toks.append(Tok('word', w, i, m.end(), 'label' if re.search(r'\d', w) else ''))
            i = m.end()
            continue
        if is_greek_script(c):
            # a run of Greek-script letters is a Greek word unless it is one letter
            j = i
            while j < n and (is_greek_script(s[j]) or unicodedata.category(s[j]) == 'Mn' and s[j] not in COMBINING):
                j += 1
            run = s[i:j]
            if j - i == 1 and _greek_numeral_after(s, j):
                toks.append(Tok('gword', s[i:j + 1], i, j + 1))
                i = j + 1
                continue
            if j - i == 1 and c in GREEK and j < n and s[j].isdigit() and s[j].isascii() and \
                    (i == 0 or not s[i - 1].isalnum()):
                # `τ1`, `η2p`: a subscript or superscript lost to flattening — unknowable which
                m = re.compile(r'\d+[A-Za-z]*').match(s, j)
                toks.append(Tok('gword', s[i:m.end()], i, m.end()))
                i = m.end()
                continue
            if j - i == 1 and c == 'μ' and j < n and s[j] in UNITS_AFTER_MU and not (j + 1 < n and s[j + 1].isalpha()):
                toks.append(Tok('word', s[i:j + 1], i, j + 1, 'unit'))
                j += 1
            elif j - i == 1 and c == 'Ω' and i > 0 and s[i - 1] in 'kMGm' and (i < 2 or not s[i - 2].isalpha()):
                toks.append(Tok('word', c, i, j, 'unit'))
            elif j - i == 1 and c in GREEK:
                toks.append(Tok('greek', c, i, j))
            elif j - i == 1 and _greek_numeral_after(s, j):
                toks.append(Tok('gword', s[i:j + 1], i, j + 1))
                j += 1
            elif all(ch in GREEK for ch in run) and len(run) <= 3 and not _greek_numeral_after(s, j):
                # glued Greek letters like `αβ` (product) — emit singly
                for k, ch in enumerate(run):
                    toks.append(Tok('greek', ch, i + k, i + k + 1))
            else:
                toks.append(Tok('gword', run, i, j))
            i = j
            continue
        if c.isalpha() and not c.isascii() and _precomposed_math(s, i):
            base = unicodedata.normalize('NFD', c)
            toks.append(Tok('word', base[0], i, i + 1))
            toks.append(Tok('comb', base[1], i, i + 1))
            i += 1
            continue
        if c.isalpha() and not c.isascii() and mathalpha_latex(c) is None and subsup_of(c) is None:
            m = re.compile(r'[^\W\d_]+', re.UNICODE).match(s, i)
            toks.append(Tok('word', m.group(), i, m.end(), 'foreign'))
            i = m.end()
            continue
        if mathalpha_latex(c) is not None:
            toks.append(Tok('malpha', c, i, i + 1))
            i += 1
            continue
        ss = subsup_of(c)
        if ss:
            j = i
            while j < n and subsup_of(s[j]) and subsup_of(s[j])[0] == ss[0]:
                j += 1
            toks.append(Tok(ss[0], s[i:j], i, j))
            i = j
            continue
        if c in COMBINING:
            toks.append(Tok('comb', c, i, i + 1))
            i += 1
            continue
        if c in "'′″‴" :
            toks.append(Tok('prime', c, i, i + 1))
            i += 1
            continue
        if c == '‖':
            toks.append(Tok('dbar', c, i, i + 1))
            i += 1
            continue
        if c == '|':
            if s.startswith('||', i):
                toks.append(Tok('dbar', '||', i, i + 2))
                i += 2
            else:
                toks.append(Tok('bar', c, i, i + 1))
                i += 1
            continue
        if c in OPEN:
            toks.append(Tok('open', c, i, i + 1))
            i += 1
            continue
        if c in CLOSE:
            toks.append(Tok('close', c, i, i + 1))
            i += 1
            continue
        if c == '-':
            toks.append(Tok('hyph', c, i, i + 1))
            i += 1
            continue
        # ASCII compound relations
        for two, lat in (('<=', r'\leq'), ('>=', r'\geq'), ('!=', r'\neq'), ('>>', r'\gg'), ('<<', r'\ll'), (':=', ':=')):
            if s.startswith(two, i):
                toks.append(Tok('op', two, i, i + 2, 'rel'))
                i += 2
                break
        else:
            if c in OPS:
                toks.append(Tok('op', c, i, i + 1, OPS[c][1]))
                i += 1
                continue
            if c in PUNCT:
                toks.append(Tok('punct', c, i, i + 1))
                i += 1
                continue
            if c == '…':
                toks.append(Tok('ellip', c, i, i + 1))
                i += 1
                continue
            if c in DASHES:
                toks.append(Tok('dash', c, i, i + 1))
                i += 1
                continue
            if c in '~':
                toks.append(Tok('tilde', c, i, i + 1))
                i += 1
                continue
            toks.append(Tok('other', c, i, i + 1))
            i += 1
            continue
    for k, t in enumerate(toks):
        t.sp_before = k > 0 and toks[k - 1].kind == 'ws'
    return toks


def _precomposed_math(s, i):
    """`Ṙ_min`, `ẋ = f(x)`, `ŷ_t`: a precomposed Latin letter with a math
    accent, standing alone (not inside a word like `rôle` or `café`)."""
    d = unicodedata.normalize('NFD', s[i])
    if len(d) != 2 or not d[0].isascii() or d[1] not in ('\u0307', '\u0308', '\u0302', '\u0304', '\u0303'):
        return False
    before = s[i - 1] if i > 0 else ' '
    after = s[i + 1] if i + 1 < len(s) else ' '
    if before.isalpha() or after.isalpha():
        return False
    # and in a math position: a script follows, or a relation is next
    # (Vietnamese `ô`, mojibake `â€` stand alone too)
    if after in '_^′\'':
        return True
    m = re.match(r'\s*(=|:=|≤|≥|<|>|≈|→|∈)', s[i + 1:])
    return m is not None


def _greek_numeral_after(s, j):
    return j < len(s) and s[j] in '\u0374\u0375\u0384\u02b9'


# ---------------------------------------------------------------- emphasis pairing

def mark_emphasis(toks):
    """Single `*` tokens that pair as markdown emphasis become 'emph'; the rest
    stay 'star' (a candidate superscript asterisk, `η*`)."""
    stars = [k for k, t in enumerate(toks) if t.kind == 'star']
    opener = None
    for k in stars:
        prev_ws = k == 0 or toks[k - 1].kind == 'ws' or toks[k - 1].kind in ('open', 'punct', 'dash', 'emph')
        next_ok = k + 1 < len(toks) and toks[k + 1].kind not in ('ws',)
        if opener is None:
            if prev_ws and next_ok:
                opener = k
        else:
            if not prev_ws:
                toks[opener].kind = 'emph'
                toks[k].kind = 'emph'
                opener = None
    return toks


# ---------------------------------------------------------------- terms

@dataclass
class Term:
    a: int                # token index range [a, b)
    b: int
    base: str             # 'word' 'greek' 'malpha' 'num' 'mspan' 'group' 'norm' 'abs'
    mathness: str         # strong | letter | num | func | label | word | ident
    latex: str = ''
    conf: float = 1.0
    why: str = ''


def _glued(toks, k):
    return k < len(toks) and not toks[k].sp_before and toks[k].kind != 'ws'


def _match_close(toks, k):
    """Index of the token closing the bracket at k (same line), or None."""
    o = toks[k].text
    c = PAIR[o]
    depth = 0
    for j in range(k, len(toks)):
        if toks[j].kind == 'open' and toks[j].text == o:
            depth += 1
        elif toks[j].kind == 'close' and toks[j].text == c:
            depth -= 1
            if depth == 0:
                return j
    return None


def _bar_close(toks, k, kind):
    for j in range(k + 1, min(len(toks), k + 12)):
        if toks[j].kind == kind:
            return j
        if toks[j].kind in ('ws',) and j + 1 < len(toks) and toks[j + 1].kind == 'word' and len(toks[j + 1].text) > 3:
            return None
    return None


SIMPLE_SUBWORD = re.compile(r'^(?:[A-Za-z]|[ijklmnpqrstuvxyzab]{2})$')


def tex_word_sub(w: str) -> str:
    """Subscript/superscript word -> LaTeX argument (without braces)."""
    if w in FUNCS and w in ('max', 'min', 'sup', 'inf', 'lim'):
        return FUNCS[w]
    if SIMPLE_SUBWORD.match(w) or re.match(r'^[A-Za-z]\d*$', w):
        return w
    return r'\text{' + w + '}'


def _add_sup(lat, extra):
    """Append to an existing trailing superscript instead of opening a second
    one (`η^(k)*` must be `\eta^{(k)\ast}`, not the invalid `^{(k)}^\ast`)."""
    m = re.search(r'\^(\{(?:[^{}]|\{[^{}]*\})*\}|\\[A-Za-z]+|[^{}\\])$', lat)
    if m and not lat.endswith("'"):
        inner = m.group(1)
        inner = inner[1:-1] if inner.startswith('{') else inner
        return lat[:m.start()] + '^{' + inner + ('' if extra.startswith('\\') and inner[-1:].isalpha() is False else '') + extra + '}'
    return lat + '^' + extra


def brace(x: str) -> str:
    x = x.strip()
    if re.match(r'^\\(min|max|sup|inf|lim|log|ln|exp|det|arg|Pr|limsup|liminf)$', x):
        return '{' + x + '}'   # KaTeX rejects `_\min` (a function with no argument)
    if len(x) == 1 or re.match(r'^\\[A-Za-z]+$', x):
        return x
    if re.match(r'^\\(text|mathrm|mathcal|mathbb)\{[^{}]*\}$', x):
        return '{' + x + '}'
    return '{' + x + '}'


def parse_script_arg(toks, k):
    """Parse the argument after `_` or `^` starting at token k (glued).
    Returns (end_index, latex, conf) or None."""
    if k >= len(toks) or toks[k].sp_before or toks[k].kind == 'ws':
        return None
    t = toks[k]
    if t.kind == 'open' and t.text in '({[':
        j = _match_close(toks, k)
        if j is None:
            return None
        inner = toks[k + 1:j]
        lat = render_tokens(inner)
        if lat is None:
            return None
        if t.text == '{':
            return j + 1, lat, 1.0
        if t.text == '[':
            return j + 1, '[' + lat + ']', 0.9
        # `x_(t+1)`, `δ_(crit,k)`: subscript parens group; `^(k)` keeps its
        # parens (the house channel index), `^(1/2)` is grouping
        if toks[k - 1].kind in ('us', 'emph') or any(x.kind in ('op', 'hyph') for x in inner):
            return j + 1, lat, 0.8
        return j + 1, '(' + lat + ')', 0.9
    if t.kind == 'word':
        lat = tex_word_sub(t.text)
        conf = 1.0
        if _glued(toks, k + 1) and toks[k + 1].kind == 'punct' and toks[k + 1].text == ',' and _glued(toks, k + 2) \
                and toks[k + 2].kind in ('word', 'num') and len(toks[k + 2].text) == 1 and \
                (k + 3 >= len(toks) or toks[k + 3].kind not in ('word', 'num', 'us', 'caret', 'sub', 'sup', 'open', 'emph')):
            return k + 3, lat + ',' + toks[k + 2].text, 0.9   # δ_critical,k ; Ṙ_min,i
        # `x_t+1` is rendered as written (x_t + 1). A heuristic that read it as
        # x_{t+1} was tried and removed: on held-out data it produced
        # `U_M/(U_{M+U}_o)` and `\Sigma_{t-as}` (spike LOG §9).
        return k + 1, lat, conf
    if t.kind == 'num':
        return k + 1, t.text, 1.0
    if t.kind == 'greek':
        return k + 1, GREEK[t.text], 1.0
    if t.kind == 'malpha':
        return k + 1, mathalpha_latex(t.text), 1.0
    if (t.kind == 'op' and t.text in ('+', '−') or t.kind == 'hyph') and _glued(toks, k + 1) and \
            toks[k + 1].kind == 'open' and toks[k + 1].text == '(':
        return k + 1, '+' if t.text == '+' else '-', 0.9   # λ_min^+(K·H)
    if t.kind == 'op' and t.text in ('<', '≤', '>', '≥', '−', '+') or t.kind == 'hyph':
        r = parse_script_arg(toks, k + 1)
        if r:
            sym = {'<': r'\lt ', '≤': r'\leq ', '>': r'\gt ', '≥': r'\geq ', '−': '-', '+': '+', '-': '-'}[t.text]
            return r[0], sym + r[1], r[2] * 0.9
    if t.kind == 'star':
        return k + 1, r'\ast', 1.0
    if t.kind == 'op' and OPS.get(t.text, ('', ''))[1] == 'ord':
        return k + 1, OPS[t.text][0], 1.0     # Σ_∞
    if t.kind in ('op', 'hyph') and t.text in ('+', '−', '-'):
        return k + 1, '+' if t.text == '+' else '-', 0.9   # τ^+, x^-
    return None


def parse_term(toks, k, allow_group=False):
    """Greedy operand term at token k: base + decorations, all glued."""
    if k >= len(toks):
        return None
    t = toks[k]
    conf = 1.0
    why = ''
    if t.kind == 'word' and t.cls != 'foreign':
        w = t.text
        if t.cls in ('label', 'unit'):
            base, lat, mness = 'word', r'\mathrm{' + w + '}', 'label'
        elif t.cls == 'ident':
            base, lat, mness = 'word', r'\text{' + w + '}', 'ident'
        elif w in FUNCS:
            base, lat, mness = 'word', FUNCS[w], 'func'
        elif len(w) == 1:
            base, lat, mness = 'word', w, 'letter'
        else:
            base, lat, mness = 'word', r'\text{' + w + '}', 'word'
    elif t.kind == 'greek':
        base, lat, mness = 'greek', GREEK[t.text], 'strong'
    elif t.kind == 'malpha':
        base, lat, mness = 'malpha', mathalpha_latex(t.text), 'strong'
    elif t.kind == 'num':
        base, lat, mness = 'num', t.text, 'num'
    elif t.kind == 'mspan':
        inner = t.text.strip('$')
        return Term(k, k + 1, 'mspan', 'strong', inner)
    elif t.kind in ('dbar', 'bar') or t.kind == 'esc' and t.cls == '|':
        kind = t.kind if t.kind != 'esc' else 'escbar'
        j = None
        for jj in range(k + 1, min(len(toks), k + 24)):
            if toks[jj].kind == t.kind and toks[jj].text == t.text:
                j = jj
                break
            if toks[jj].kind == 'ws' and (t.kind == 'bar' or jj + 1 < len(toks) and toks[jj + 1].kind == 'word' and len(toks[jj + 1].text) > 3):
                break   # `‖ x - y‖` may carry spaces; `| a | b |` table bars may not
        if j is None or j == k + 1:
            return None
        if not any(x.kind in ('greek', 'malpha', 'num', 'mspan') or x.kind == 'word' and len(x.text) == 1
                   for x in toks[k + 1:j]):
            return None   # `|---|` is a table rule, `|delta|` spelled-out prose
        inner = render_tokens(toks[k + 1:j])
        if inner is None:
            return None
        if t.kind == 'dbar':
            base, lat = 'norm', r'\lVert ' + inner + r'\rVert'
        else:
            base, lat = 'abs', r'\lvert ' + inner + r'\rvert'
        mness = 'strong'
        k = j
    elif t.kind == 'op' and t.text in LARGE and k + 1 < len(toks) and _glued(toks, k + 1) and \
            toks[k + 1].kind in ('us', 'caret', 'sub', 'sup') or t.kind == 'emph' and False:
        base, lat, mness = 'large', OPS[t.text][0], 'strong'
    elif t.kind == 'open' and allow_group and t.text in '([':
        j = _match_close(toks, k)
        if j is None:
            return None
        inner = render_tokens(toks[k + 1:j], allow_ws=True)
        if inner is None:
            return None
        base, lat, mness = 'group', t.text + inner + toks[j].text, 'group'
        k = j
    else:
        return None
    end = k + 1
    has_script = False
    single = base in ('greek', 'malpha', 'norm', 'abs', 'group', 'large') or base == 'word' and len(t.text) == 1
    # decorations
    while end < len(toks) and _glued(toks, end):
        d = toks[end]
        if d.kind == 'comb':
            lat = COMBINING[d.text] + '{' + lat + '}'
            has_script = True
            end += 1
        elif d.kind == 'prime':
            # possessive: α's
            if d.text == "'" and _glued(toks, end + 1) and toks[end + 1].kind == 'word':
                break  # possessive / contraction: α's, I'd
            lat += {"'": "'", '′': "'", '″': "''", '‴': "'''"}[d.text]
            has_script = True
            end += 1
        elif d.kind in ('sub', 'sup'):
            chars = ''.join(subsup_of(c)[1] for c in d.text)
            chars = re.sub(r'(?<=\d)x(?=\d)', r'\\times ', chars)   # 0₃ₓ₃
            if d.kind == 'sup' and chars == 'T':
                lat = _add_sup(lat, r'\top') if re.search(r'\^(\{[^{}]*\}|\\[A-Za-z]+|.)$', lat) else lat + r'^\top'
            elif d.kind == 'sup' and re.search(r'\^(\{[^{}]*\}|\\[A-Za-z]+|.)$', lat):
                lat = _add_sup(lat, chars)
            elif d.kind == 'sup' and chars in ('+', '-') and re.search(r'_(\\[A-Za-z]+|[a-z])$', lat):
                # M_τ⁺ = M_{τ^+} (asf canon: 390× `M_{\tau^+}`, 0× `M_\tau^+`); a digit index
                # keeps the sign outside: W₀⁺ = W_0^+
                m = re.search(r'_(\\[A-Za-z]+|[a-z])$', lat)
                inner = m.group(1)[1:-1] if m.group(1).startswith('{') else m.group(1)
                lat = lat[:m.start()] + '_{' + inner + '^' + chars + '}'   # M_τ⁺ = M_{τ^+}
            elif d.kind == 'sub' and re.search(r'_(\{[^{}]*\}|\\[A-Za-z]+|[^{}\\])$', lat):
                m = re.search(r'_(\{[^{}]*\}|\\[A-Za-z]+|[^{}\\])$', lat)
                inner = m.group(1)[1:-1] if m.group(1).startswith('{') else m.group(1)
                lat = lat[:m.start()] + '_{' + inner + '_' + brace(chars) + '}'   # κ_W₁
            else:
                lat += ('_' if d.kind == 'sub' else '^') + brace(chars)
            has_script = True
            end += 1
        elif d.kind in ('us', 'caret') or d.kind == 'emph' and d.text == '_' :
            r = parse_script_arg(toks, end + 1)
            if r is None:
                break
            e2, arg, c2 = r
            if d.text != '_':
                lat = _add_sup(lat, arg) if re.search(r'\^(\{[^{}]*\}|\\[A-Za-z]+|.)$', lat) else lat + '^' + brace(arg)
            elif re.search(r'_(\{(?:[^{}]|\{[^{}]*\})*\}|\\[A-Za-z]+|[^{}\\])$', lat):
                # κ_W_1, σ_source_phrase: a subscript on the subscript
                m = re.search(r'_(\{(?:[^{}]|\{[^{}]*\})*\}|\\[A-Za-z]+|[^{}\\])$', lat)
                inner = m.group(1)[1:-1] if m.group(1).startswith('{') else m.group(1)
                lat = lat[:m.start()] + '_{' + inner + '_' + brace(arg) + '}'
            else:
                lat += '_' + brace(arg)
            conf = min(conf, c2)
            has_script = True
            end = e2
        elif (d.kind == 'star' or d.kind == 'esc' and d.cls == '*') and (base in ('greek', 'malpha') or single):
            nxt = toks[end + 1] if end + 1 < len(toks) else None
            if nxt is None or nxt.kind in ('ws', 'punct', 'close', 'op', 'hyph', 'dash', 'emph', 'us', 'sub', 'caret') or nxt.kind == 'star' \
                    or nxt.kind == 'open' and nxt.text == '(':
                lat = _add_sup(lat, r'\ast')
                has_script = True
                end += 1
            else:
                break
        elif d.kind == 'op' and d.text == '†':
            lat += r'^\dagger'
            end += 1
        else:
            break
    if base == 'word':
        if has_script:
            only_primes = all(toks[x].kind == 'prime' for x in range(k + 1, end))
            if t.cls == 'label':
                mness = 'label'
            elif single and only_primes:
                mness = 'letter'   # `x'` / `i'`: a quote mark is as likely as a prime; needs evidence
            elif single and t.text.isupper() and all(toks[x].kind in ('star', 'esc') for x in range(k + 1, end)):
                mness = 'letter'   # `A*` is as often the search algorithm as a starred variable
            elif single or t.text in FUNCS:
                mness = 'strong' if t.text not in FUNCS else 'func'
            else:
                # snake_case identifier or multi-letter base: `if_else`, `CI_low`
                mness = 'ident'
        # star on a multi-letter word (`B1*`, `Codex*`) is not a superscript
    if base == 'num' and has_script:
        mness = 'strong' if '^' in lat else 'num'
    if end + 1 < len(toks) and _glued(toks, end) and toks[end].kind == 'punct' and toks[end].text == '.' and \
            _glued(toks, end + 1) and toks[end + 1].kind == 'word' and toks[end + 1].text.islower():
        mness = 'broken'   # `f_0001.html`: a file name
    if end < len(toks) and _glued(toks, end) and toks[end].kind in ('us', 'caret'):
        mness = 'broken'   # `t_{macro_cost}` we could not read: leave the whole token alone
    return Term(k if base not in ('norm', 'abs', 'group') else t.a and k, end, base, mness, lat, conf, why)


def render_tokens(ts, allow_ws=True):
    """Render a token run that is entirely math (inside braces / bars / groups)."""
    out = []
    k = 0
    while k < len(ts):
        t = ts[k]
        if t.kind == 'ws':
            k += 1
            continue
        term = parse_term(ts, k, allow_group=True)
        if term is not None and term.mathness not in ('word', 'ident', 'broken'):
            out.append(term.latex)
            k = term.b
            continue
        if term is not None and term.mathness == 'word':
            # δ_(critical,k), t_{total}; r_{one-for-one} stays one hyphenated word
            w = ts[k].text
            k = term.b
            while k + 1 < len(ts) and ts[k].kind == 'hyph' and not ts[k].sp_before and ts[k + 1].kind == 'word' \
                    and not ts[k + 1].sp_before:
                w += '-' + ts[k + 1].text
                k += 2
            out.append(r'\text{' + w + '}')
            continue
        if t.kind == 'word' and t.cls == 'ident':
            out.append(r'\text{' + t.text.replace('_', '-') + '}')
            k += 1
            continue
        if t.kind == 'op':
            out.append(OPS[t.text][0] if t.text in OPS else {'<=': r'\leq', '>=': r'\geq', '!=': r'\neq', '>>': r'\gg', '<<': r'\ll', ':=': ':='}[t.text])
        elif t.kind == 'hyph':
            out.append('-')
        elif t.kind == 'punct' and t.text in ',;:':
            out.append(t.text)
        elif t.kind == 'open':
            out.append(OPEN[t.text])
        elif t.kind == 'close':
            out.append(CLOSE[t.text])
        elif t.kind == 'bar':
            out.append(r'\mid')
        elif t.kind == 'dbar':
            out.append(r'\Vert')
        elif t.kind == 'ellip':
            out.append(r'\ldots')
        elif t.kind == 'star':
            out.append(r'\ast')
        elif t.kind == 'tilde':
            out.append(r'\sim')
        elif t.kind == 'other' and t.text == '%':
            out.append(r'\%')
        elif t.kind == 'esc' and t.cls == '|':
            out.append(r'\vert')
        else:
            return None
        k += 1
    return join_latex(out)


def join_latex(parts):
    s = ''
    for p in parts:
        if not p:
            continue
        if s and (re.search(r'\\[A-Za-z]+$', s) and re.match(r'[A-Za-z0-9]', p)):
            s += ' '
        elif s and _spaced(p) or s and _spaced_tail(s):
            s += ' '
        s += p
    return s


REL_LATEX = {v[0] for v in OPS.values() if v[1] in ('rel', 'arrow')} | {r'\leq', r'\geq', r'\neq', r'\gg', r'\ll', ':=', '=', '+', '-', r'\cdot', r'\times', r'\pm', r'\cup', r'\cap', r'\circ', r'\otimes', r'\oplus', r'\wedge', r'\vee', r'\setminus', r'\mid', r'\approx'}


def _spaced(p):
    return p in REL_LATEX


def _spaced_tail(s):
    return any(s.endswith(' ' + r) or s == r for r in REL_LATEX) or any(s.endswith(r) and (len(s) == len(r) or not s[-len(r) - 1].isalpha()) for r in REL_LATEX if r.startswith('\\'))


# ---------------------------------------------------------------- units and spans

@dataclass
class Unit:
    kind: str        # term | op | ws | punct | open | close | hyph | emph | prot | other | dash | ellip
    a: int           # token range [a, b)
    b: int
    term: Term | None = None
    text: str = ''
    cls: str = ''

    def mathy(self):
        """Operand class: 'strong' 'letter' 'num' 'func' 'label' 'word' 'ident' '' """
        return self.term.mathness if self.term else ''


def units_of(toks):
    us = []
    k = 0
    while k < len(toks):
        t = toks[k]
        term = parse_term(toks, k)
        if term is not None and term.b > k:
            us.append(Unit('term', k, term.b, term, ''.join(x.text for x in toks[k:term.b])))
            k = term.b
            continue
        kind = t.kind
        if kind in ('greek', 'malpha', 'word', 'num', 'mspan'):
            kind = 'other'
        if kind == 'op' and OPS.get(t.text, ('', ''))[1] in ('post', 'ord'):
            kind = 'other'   # ∞ ∅ ⋯ are operands, not operators
        us.append(Unit(kind, k, k + 1, None, t.text, t.cls))
        k += 1
    return us


def op_latex(text):
    if text in OPS:
        return OPS[text][0]
    return {'<=': r'\leq', '>=': r'\geq', '!=': r'\neq', '>>': r'\gg', '<<': r'\ll', ':=': ':='}.get(text, text)


def _is_strong_op(u):
    return u.kind == 'op' and (u.text in STRONG_OPS or u.text in ('<=', '>=', '!=', '>>', '<<'))


OPERAND_OK = {'strong', 'letter', 'num', 'func', 'group'}


class Span:
    def __init__(self, a, b):
        self.a, self.b = a, b   # unit indices [a, b)
        self.conf = 1.0
        self.why = []


def find_spans(us, toks, symbols=frozenset()):
    n = len(us)
    is_anchor = [False] * n
    def _label_number(j):
        # `Class 1`, `Fr 1.129`, `Tier 3`: a number named by the word before it
        k = j - 1
        while k >= 0 and us[k].kind == 'ws':
            k -= 1
        return k >= 0 and us[k].kind == 'term' and us[k].term.mathness in ('word', 'label')

    for i, u in enumerate(us):
        if u.kind == 'term' and u.term.mathness == 'strong':
            is_anchor[i] = True
            if i >= 2 and us[i - 1].kind == 'hyph' and not toks[us[i - 1].a].sp_before and not toks[u.a].sp_before \
                    and us[i - 2].kind == 'term' and us[i - 2].term.mathness == 'label':
                is_anchor[i] = False   # T1-α, GA-α: an identifier
        elif _is_strong_op(u):
            is_anchor[i] = True
        elif u.kind == 'op' and u.cls == 'pre':
            is_anchor[i] = True
    inspan = [False] * n

    def nxt(i, d):
        """index of next non-ws unit in direction d, and whether ws was skipped"""
        j = i + d
        sk = False
        while 0 <= j < n and us[j].kind == 'ws':
            j += d
            sk = True
        return (j if 0 <= j < n else None), sk

    def operand_right(j):
        """Does an operand start at unit j (rightward)? returns end index (exclusive) or None"""
        if j is None:
            return None
        u = us[j]
        if u.kind == 'term' and u.term.mathness == 'letter' and toks[u.a].text in ('a', 'I') and u.b - u.a == 1:
            k3, _ = nxt(j, +1)
            if k3 is not None and us[k3].kind == 'term' and us[k3].term.mathness in ('word', 'label', 'ident'):
                return None   # `N - a date`: the article (TN28 §5.1)
        if u.kind == 'term' and u.term.mathness == 'num' and j + 1 < n and us[j + 1].kind == 'term' and \
                toks[us[j + 1].a].cls == 'unit' and not toks[us[j + 1].a].sp_before:
            return None   # `t/3days`: 3days is a quantity
        if u.kind == 'term' and u.term.mathness in OPERAND_OK:
            e = j + 1
            # function application / tuple glued: f(x)
            e = absorb_application(e)
            return e
        if u.kind == 'term' and u.term.mathness == 'word' and re.match(r'^d[a-z]$', u.text) and j > 0 and \
                us[j - 1].kind == 'op' and us[j - 1].text == '/' and not toks[u.a].sp_before:
            u.term.latex = u.text   # a differential: d‖δ‖/dt
            return j + 1
        if u.kind == 'term' and u.term.base == 'large':
            k2, _ = nxt(j, +1)
            r = operand_right(k2)
            return r if r else j + 1   # ∑_k ν_k η_k
        if u.kind == 'op' and u.cls in ('pre',) or u.kind == 'op' and u.text in ('−', '-', '+', '±', '¬') or u.kind == 'hyph':
            # prefix operator glued to an operand
            k2 = j + 1
            if k2 < n and us[k2].kind == 'ws' and u.text in LARGE:
                k2, _ = nxt(j, +1)   # `∑ ν·η`: a large operator before a space
            if k2 is not None and k2 < n and us[k2].kind != 'ws':
                r = operand_right(k2)
                if r:
                    return r
            if u.cls == 'pre':
                return j + 1
            return None
        if u.kind == 'open' and u.text in '([⟨{':
            c = group_close(j)
            if c is not None and group_ok(j, c):
                e = c + 1
                # (…)^(…) or (…)² handled by trailing script
                e = absorb_script(e)
                return e
        if u.kind == 'other' and u.text == '∞':
            return j + 1
        return None

    def operand_left(j):
        """Does an operand end at unit j (leftward)? returns start index or None"""
        if j is None:
            return None
        u = us[j]
        if u.kind == 'term' and u.term.mathness in OPERAND_OK:
            return j
        if u.kind == 'close':
            o = group_open(j)
            if o is not None and group_ok(o, j):
                # application: f(x) — include the function term
                if o > 0 and us[o - 1].kind == 'term' and not toks[us[o].a].sp_before and us[o - 1].term.mathness in OPERAND_OK | {'func'}:
                    return o - 1
                return o
        if u.kind == 'other' and u.text == '∞':
            return j
        return None

    def group_close(j):
        o = us[j].text
        c = PAIR.get(o)
        depth = 0
        for k in range(j, n):
            if us[k].kind == 'open' and us[k].text == o:
                depth += 1
            elif us[k].kind == 'close' and us[k].text == c:
                depth -= 1
                if depth == 0:
                    return k
        return None

    def group_open(j):
        c = us[j].text
        o = {v: k for k, v in PAIR.items()}.get(c)
        depth = 0
        for k in range(j, -1, -1):
            if us[k].kind == 'close' and us[k].text == c:
                depth += 1
            elif us[k].kind == 'open' and us[k].text == o:
                depth -= 1
                if depth == 0:
                    return k
        return None

    def group_ok(o, c, words=0):
        """Is the bracket group content entirely math-compatible? (`words`:
        how many short prose words an application argument may carry)"""
        if c - o < 2:
            return False
        if us[o].text == '⟨' and (c - o > 12 or toks[us[c].a].a - toks[us[o].a].a > 40):
            return False   # ⟨Read(…) | TodoWrite⟩ in transcripts is not a bra-ket
        strong = False
        for k in range(o + 1, c):
            u = us[k]
            if u.kind == 'ws':
                continue
            if u.kind == 'term':
                m = u.term.mathness
                if m == 'strong':
                    strong = True
                elif m in ('letter', 'num', 'func', 'group'):
                    pass
                elif m == 'word' and len(u.text) <= 3 and u.text.isupper():
                    pass
                elif m == 'word' and words > 0 and len(u.text) <= 16:
                    words -= 1
                else:
                    return False
            elif u.kind == 'op' or u.kind == 'hyph':
                strong = strong or _is_strong_op(u)
            elif u.kind == 'punct' and u.text in ',;:':
                pass
            elif u.kind in ('open', 'close'):
                pass
            elif u.kind in ('ellip',) or u.kind == 'other' and u.text in '∞%':
                pass
            elif u.kind == 'other' and u.text == '|':
                pass
            elif u.kind == 'bar':
                pass
            elif u.kind == 'star':
                pass
            else:
                return False
        return True

    def unit_like(k):
        """A single Latin letter carrying only Unicode digit scripts, in a
        quantity: `100 m²`, `kg/m³`, `m/s²`."""
        u = us[k]
        tk = toks[u.a]
        if not (u.term.base == 'word' and len(tk.text) == 1 and tk.text.isascii()):
            return False
        if not all(toks[x].kind in ('sup', 'sub') for x in range(u.a + 1, u.b)) or u.b == u.a + 1:
            return False
        if not all(ch.isdigit() or ch == '-' for x in range(u.a + 1, u.b) for ch in (subsup_of(c)[1] for c in toks[x].text)):
            return False
        jl, sk = nxt(k, -1)
        if jl is None:
            return False
        if sk and us[jl].kind == 'term' and us[jl].term.mathness == 'num':
            return True
        if not sk and us[jl].kind == 'op' and us[jl].text == '/':
            j2 = jl - 1
            if j2 >= 0 and us[j2].kind == 'term' and (us[j2].term.mathness in ('word', 'label') or
                                                      us[j2].term.mathness == 'letter' and nxt(j2, -1)[0] is not None and
                                                      us[nxt(j2, -1)[0]].kind == 'term' and us[nxt(j2, -1)[0]].term.mathness == 'num'):
                return True
        return False

    def plausible_variable(k, known=False):
        """A single Latin letter that can be a variable here (not an article,
        an initial, an enumerator, or a label after a capitalized word)."""
        L = toks[us[k].a].text
        if L in ('a', 'A', 'I'):
            return False
        # a standalone token only: `\\n`, `x86`, `e-mail` are not variables
        t0 = toks[us[k].a]
        prev_t = toks[us[k].a - 1] if us[k].a > 0 else None
        next_t = toks[us[k].b] if us[k].b < len(toks) else None
        if prev_t is not None and not t0.sp_before and (prev_t.kind not in ('open', 'punct', 'dash', 'emph', 'quote', 'op', 'bar', 'dbar', 'star')
                                                         or prev_t.kind == 'punct' and prev_t.text not in ',;:'):
            return False
        if next_t is not None and next_t.kind == 'punct' and next_t.text == '.' and us[k].b + 1 < len(toks) and \
                (toks[us[k].b + 1].kind in ('punct', 'ellip', 'word') or
                 toks[us[k].b + 1].kind == 'ws' and us[k].b + 2 < len(toks) and toks[us[k].b + 2].kind == 'num'):
            return False  # `p...`, `e.g`, `p. 20`: truncation or abbreviation
        if next_t is not None and next_t.kind not in ('ws', 'close', 'punct', 'prime', 'emph', 'dash', 'op', 'bar', 'dbar', 'open', 'star') \
                and not (next_t.kind == 'other' and next_t.text in '’'):
            return False
        if k + 1 < n and us[k + 1].kind == 'punct' and us[k + 1].text == '.' and k + 2 < n and us[k + 2].kind == 'term':
            return False  # e.g. / J. Smith
        if k > 0 and us[k - 1].kind == 'open' and k + 1 < n and us[k + 1].kind == 'close':
            return False  # (a) (b) enumerators
        jl, _ = nxt(k, -1)
        if not known and jl is not None and us[jl].kind == 'term' and us[jl].term.mathness in ('word', 'label') and us[jl].text[:1].isupper() \
                and L.isupper():
            return False  # Plan B, Model S, Class C
        if jl is not None and us[jl].kind == 'term' and us[jl].text.lower() in LABEL_NOUNS and L.isupper():
            return False  # Appendix E, Section B, Table C — even when E is a symbol elsewhere
        if L.isupper() and next_t is not None and next_t.kind == 'punct' and next_t.text == '.':
            jp, _ = nxt(k, -1)
            if jp is not None and (us[jp].kind == 'punct' and us[jp].text in ',&' or
                                   us[jp].kind == 'term' and us[jp].term.mathness == 'letter' and
                                   us[jp].b < len(toks) and toks[us[jp].b].kind == 'punct' and toks[us[jp].b].text == '.'):
                return False  # `Casella, G.`, `E. L.`: initials
        if jl is not None and us[jl].kind == 'open' and jl > 0:
            j2, _ = nxt(jl, -1)
            if j2 is not None and us[j2].kind == 'term' and us[j2].term.mathness in ('word', 'label') and us[j2].text[:1].isupper() and L.isupper():
                return False  # Model (S)
        return True

    def is_binop(u):
        return u.kind == 'op' and (OPS.get(u.text, ('', 'rel'))[1] in ('rel', 'arrow', 'bin') or
                                   u.text in ('<=', '>=', '!=', '>>', '<<', ':=')) or u.kind == 'hyph'

    def attachable_rel(u):
        return u.kind == 'op' and u.text in ('<', '>', '≤', '≥', '≠', '∈', '∉', '⊂', '⊃', '⊆', '⊇', '≪', '≫', '<=', '>=', '≡', '∝')

    def span_has_relation(a, b):
        return any(us[k].kind == 'op' and (OPS.get(us[k].text, ('', ''))[1] == 'rel' or us[k].text in ('<=', '>=')) for k in range(a, b))

    def juxtaposable(u):
        if u.kind != 'term':
            return False
        if u.term.mathness in ('strong', 'group'):
            return True
        if u.term.mathness != 'letter' or toks[u.a].text in ('a', 'I', 'A'):
            return False
        pt = toks[u.a - 1] if u.a > 0 else None
        return pt is None or pt.kind in ('ws', 'open')   # not the `s` of `B6's`

    def span_has_op(a, b):
        return any(us[k].kind == 'op' and is_binop(us[k]) for k in range(a, b))

    def complex_term(u):
        t = u.term
        return t.mathness == 'strong' and (t.base in ('norm', 'abs', 'group') or any(ch in t.latex for ch in '_^'))

    def simple_tuple(o, c):
        items = [[]]
        for k in range(o + 1, c):
            u = us[k]
            if u.kind == 'ws':
                continue
            if u.kind == 'punct' and u.text == ',':
                items.append([])
            else:
                items[-1].append(u)
        if len(items) < 2 or any(len(it) != 1 for it in items):
            return False
        if any(it[0].kind != 'term' or it[0].term.mathness not in ('strong', 'letter', 'num') for it in items):
            return False
        return any(it[0].term.mathness == 'strong' for it in items)

    def absorb_application(e):
        """After a term ending at unit e-1: glued bracket group = application."""
        if e < n and us[e].kind == 'open' and us[e].text in '([' and not toks[us[e].a].sp_before:
            c = group_close(e)
            fn = us[e - 1] if e > 0 else None
            w = 3 if fn is not None and fn.kind == 'term' and fn.term.mathness == 'strong' else 0
            if c is not None and group_ok(e, c, words=w):
                return absorb_script(c + 1)
        return e

    def absorb_script(e):
        # (…)^(1/2), (…)², (…)_t — a script glued after a group
        if e < n and us[e].kind in ('sub', 'sup') and not toks[us[e].a].sp_before:
            return e + 1
        if e + 1 < n and us[e].kind in ('caret', 'us') and not toks[us[e].a].sp_before:
            r = parse_script_arg(toks, us[e + 1].a)
            if r:
                # advance units to cover token r[0]
                k = e + 1
                while k < n and us[k].b <= r[0]:
                    k += 1
                return k
        return e

    for k in range(n):
        u = us[k]
        if is_anchor[k] and u.kind == 'term' and unit_like(k):
            is_anchor[k] = False   # kg/m³, 100 m², m/s²
            continue
        if u.kind == 'open' and u.text == '(':
            c = group_close(k)
            if c is not None and simple_tuple(k, c):
                is_anchor[k] = True
        elif u.kind == 'other' and u.text == '∞':
            is_anchor[k] = True
        elif u.kind == 'term' and u.term.mathness == 'letter':
            L = toks[u.a].text
            if L in ('O', 'o') and k + 1 < n and us[k + 1].kind == 'open' and us[k + 1].text == '(' and \
                    not toks[us[k + 1].a].sp_before:
                c = group_close(k + 1)
                if c is not None and group_ok(k + 1, c):
                    is_anchor[k] = True       # O(h), O(n·k)
            elif L in symbols and plausible_variable(k, known=True):
                is_anchor[k] = True
        elif u.kind == 'op' and u.text in ('=', '<', '>', '<=', '>=', '≈', '≤', '≥', '≠'):
            jl, _ = nxt(k, -1)
            jr, _ = nxt(k, +1)
            if jl is not None and jr is not None:
                L, R = us[jl], us[jr]
                def var(x):
                    return x.kind == 'term' and x.term.mathness == 'letter' and x.text not in ('a', 'A', 'I') and plausible_variable(us.index(x))
                def val(x):
                    return x.kind == 'term' and x.term.mathness in ('num', 'letter', 'strong') or x.kind == 'other' and x.text == '∞'
                if var(L) and val(R) or var(R) and val(L) and u.text != '=':
                    is_anchor[k] = True       # n > 3, C = 40, t < 1
    spans = []
    i = 0
    while i < n:
        if not is_anchor[i] or inspan[i]:
            i += 1
            continue
        a, b = i, i + 1
        if us[i].kind == 'term':
            b = absorb_application(b)
        elif us[i].kind == 'open':
            b = absorb_script(group_close(i) + 1)
        elif us[i].kind == 'op':
            if us[i].cls == 'pre':
                if i + 1 < n and us[i + 1].kind != 'ws':
                    e = operand_right(i + 1)
                    if e:
                        b = e
            else:
                jr, _ = nxt(i, +1)
                e = operand_right(jr)
                jl, _ = nxt(i, -1)
                st = operand_left(jl)
                if e is None:
                    i += 1
                    continue
                if e:
                    b = e
                if st is not None and not (jl is not None and us[jl].kind == 'term' and us[jl].term.mathness == 'num' and _label_number(jl)):
                    a = st
        outer = True
        while outer:
          outer = False
          a0, b0 = a, b
          # ---- grow right
          changed = True
          while changed:
              changed = False
              j, sk = nxt(b - 1, +1)
              if j is None:
                  break
              u = us[j]
              glued = not sk
              last = us[b - 1]
              if is_binop(u):
                  if u.kind == 'hyph' and glued:
                      j2 = j + 1
                      if last.kind == 'term' and last.term.mathness == 'num' and j2 < n and \
                              us[j2].kind == 'term' and us[j2].term.mathness == 'num':
                          if span_has_relation(a, b):
                              u.cls = 'range'   # `ε ≈ 0.1-0.4`: a range inside the relation
                              b = j2 + 1
                              changed = True
                              continue
                          break
                      if j2 < n and us[j2].kind == 'term' and us[j2].term.mathness in ('word', 'ident', 'label'):
                          break
                      if j2 < n and us[j2].kind == 'ws':
                          break
                      if j2 + 1 < n and us[j2].kind == 'term' and us[j2].term.mathness == 'letter' and \
                              us[j2 + 1].kind == 'punct' and us[j2 + 1].text == '.':
                          break   # `$P_\theta$-a.e.`: an abbreviation, not a subtraction
                  k2, sk2 = nxt(j, +1)
                  if u.kind == 'hyph' and (glued != (not sk2)):
                      break
                  if last.kind == 'term' and last.term.base == 'mspan' and k2 is not None and \
                          us[k2].kind == 'term' and us[k2].term.base == 'mspan':
                      break  # two spans the author kept apart stay apart
                  e = operand_right(k2)
                  if e is None:
                      # a math relation keeps its place beside the math when
                      # the other side is prose: `$\eta^\ast \lt$ optimal`
                      if attachable_rel(u) and k2 is not None and us[k2].kind == 'term' and us[k2].term.mathness == 'word':
                          b = j + 1
                      break
                  b = e
                  changed = True
                  continue
              if u.kind == 'dash' and glued and last.kind == 'term' and last.term.mathness == 'num' and span_has_relation(a, b) \
                      and j + 1 < n and us[j + 1].kind == 'term' and us[j + 1].term.mathness == 'num':
                  u.cls = 'range'
                  b = j + 2
                  changed = True
                  continue
              if glued and u.kind == 'term' and u.term.mathness in ('strong', 'letter', 'num') and last.kind == 'term':
                  if last.kind == 'term' and toks[last.b - 1].kind == 'sup' and u.term.base == 'word' and u.text[:1].isupper():
                      break  # isotope δ¹⁸O
                  if u.term.base == 'mspan' or last.term.base == 'mspan':
                      break  # `H$_\kappa$`: the author kept the label upright
                  b = absorb_application(j + 1)
                  changed = True
                  continue
              if glued and u.kind == 'term' and u.term.mathness == 'word' and last.kind == 'term' and \
                      last.term.base == 'greek' and len(u.text) <= 3 and u.text.isupper():
                  b = j + 1
                  changed = True
                  continue
              if glued and u.kind == 'punct' and u.text == ',':
                  k2 = j + 1
                  if k2 < n and us[k2].kind == 'term' and us[k2].term.mathness in ('strong',) and not toks[us[k2].a].sp_before:
                      b = absorb_application(k2 + 1)
                      changed = True
                      continue
              if u.kind == 'term' and u.term.mathness == 'func' and last.kind == 'term':
                  k2, _ = nxt(j, +1)
                  if operand_right(k2) is not None:
                      b = operand_right(k2)
                      changed = True
                      continue
              if sk and u.kind == 'term' and complex_term(u) and last.kind == 'term' and complex_term(last):
                  b = absorb_application(j + 1)   # |α_k|² Φ_k
                  changed = True
                  continue
              if sk and juxtaposable(u) and juxtaposable(last) and span_has_op(a, b):
                  k3, _ = nxt(j, +1)
                  if k3 is not None and is_binop(us[k3]) and us[k3].kind != 'hyph':
                      b = absorb_application(j + 1)   # `= λ a_{t-1} + θ`
                      changed = True
                      continue
          # ---- grow left
          changed = True
          while changed:
              changed = False
              j, sk = nxt(a, -1)
              if j is None:
                  break
              u = us[j]
              glued = not sk
              first = us[a]
              if is_binop(u):
                  if u.kind == 'hyph' and glued:
                      j0 = j - 1
                      if j0 >= 0 and us[j0].kind == 'term' and us[j0].term.mathness in ('word', 'ident', 'label'):
                          break
                      if j0 >= 1 and us[j0].kind == 'term' and us[j0].term.mathness == 'letter' and \
                              us[j0 - 1].kind == 'hyph' and not toks[us[j0 - 1].a].sp_before:
                          break   # `is-a-$f(x)$`: a hyphenated phrase
                      if j0 < 0 or us[j0].kind in ('ws', 'open'):
                          a = j   # unary minus
                          break
                  k2, sk2 = nxt(j, -1)
                  if u.kind == 'hyph' and (glued != (not sk2)):
                      break
                  if first.kind == 'term' and first.term.base == 'mspan' and k2 is not None and \
                          us[k2].kind == 'term' and us[k2].term.base == 'mspan':
                      break
                  s = operand_left(k2)
                  if s is not None and us[k2].kind == 'term' and us[k2].term.mathness == 'num' and _label_number(k2) and u.text not in ('=', '<', '>', '≤', '≥'):
                      s = None   # `Class 1 → κ`
                  if s is None:
                      if glued and u.text in ('−', '¬', '±', '-') and (k2 is None or sk2):
                          a = j
                      elif attachable_rel(u) and k2 is not None and us[k2].kind == 'term' and us[k2].term.mathness == 'word':
                          a = j   # `depth $\gt 1/(1-\theta)$`
                      break
                  a = s
                  changed = True
                  continue
              if u.kind == 'op' and (u.cls == 'pre' or u.text in '↑↓') and glued:
                  a = j
                  changed = True
                  continue
              if glued and u.kind == 'term' and u.term.mathness in ('strong', 'letter', 'num') and \
                      (first.kind == 'term' or first.kind == 'op' and first.cls == 'pre') and \
                      not (u.term.base == 'mspan' or first.kind == 'term' and first.term.base == 'mspan'):
                  a = j
                  changed = True
                  continue
              if glued and u.kind == 'term' and u.term.mathness in OPERAND_OK | {'func'} and first.kind == 'open' and \
                      not toks[first.a].sp_before:
                  a = j   # application P(α, β)
                  changed = True
                  continue
              if glued and u.kind == 'open':
                  c = group_close(j)
                  if c is not None and group_ok(j, c) and j > 0 and us[j - 1].kind == 'term' and \
                          not toks[us[j].a].sp_before and us[j - 1].term.mathness in OPERAND_OK | {'func'}:
                      a = j - 1
                      b = max(b, absorb_script(c + 1))
                      changed = True
                      continue
                  if c is not None and c >= b and group_ok(j, c):
                      lft = us[j - 1] if j > 0 else None
                      rgt = us[c + 1] if c + 1 < n else None
                      att_l = lft is not None and not toks[us[j].a].sp_before and (lft.kind in ('op', 'hyph'))
                      att_r = rgt is not None and not toks[rgt.a].sp_before and rgt.kind in ('caret', 'sup', 'sub', 'us', 'op')
                      if att_l or att_r:
                          a = j
                          b = max(b, absorb_script(c + 1))
                          changed = True
                          continue
                  if c is not None and c >= b - 1 and simple_tuple(j, c):
                      a = j
                      b = max(b, c + 1)
                      changed = True
                      continue
              if glued and u.kind == 'punct' and u.text == ',':
                  k2 = j - 1
                  if k2 >= 0 and us[k2].kind == 'term' and us[k2].term.mathness == 'strong':
                      a = k2
                      changed = True
                      continue
              if u.kind == 'term' and u.term.mathness == 'func':
                  a = j
                  changed = True
                  continue
              if sk and u.kind == 'term' and complex_term(u) and first.kind == 'term' and complex_term(first):
                  a = j
                  changed = True
                  continue
              if sk and juxtaposable(u) and juxtaposable(first) and span_has_op(a, b) and \
                      (u.term.mathness == 'strong' or first.term.mathness == 'strong'):
                  a = j   # `ℳ J + Jᵀℳ`
                  changed = True
                  continue
          if (a, b) != (a0, b0):
              outer = True
        # an anchor op alone with no operand on either side is not a span
        if b - a == 1 and us[a].kind == 'op':
            # prefix op glued to an operand already handled; bare relation:
            j, _ = nxt(a, +1)
            if operand_right(j) is None:
                i += 1
                continue
            b = operand_right(j)
        for k in range(a, b):
            inspan[k] = True
        spans.append([a, b])
        i = b
    # merge spans that touch through glued/op connections
    spans.sort()
    merged = []
    for s in spans:
        if merged and s[0] <= merged[-1][1]:
            merged[-1][1] = max(merged[-1][1], s[1])
        else:
            merged.append(s)
    return [trim_span(us, toks, a, b) for a, b in merged]


def trim_span(us, toks, a, b):
    # strip edges that can't start/end math
    while a < b and us[a].kind in ('ws', 'punct', 'emph', 'dash', 'close') or a < b and us[a].kind == 'op' and us[a].cls in ('rel', 'arrow') :
        a += 1
    while b > a and (us[b - 1].kind in ('ws', 'punct', 'emph', 'dash', 'open', 'hyph') or us[b - 1].kind == 'op' and us[b - 1].cls in ('rel', 'arrow', 'bin')):
        b -= 1
    # balance brackets: drop an unmatched edge bracket
    def bal(a, b):
        d = 0
        for k in range(a, b):
            if us[k].kind == 'open':
                d += 1
            elif us[k].kind == 'close':
                d -= 1
                if d < 0:
                    return False
        return d == 0
    while a < b and not bal(a, b):
        if us[a].kind == 'open':
            a += 1
        elif us[b - 1].kind == 'close':
            b -= 1
        else:
            return None
    if a >= b:
        return None
    return (a, b)


def render_span(us, toks, a, b):
    parts = []
    wrap_next = False
    skip_to_tok = -1
    for k in range(a, b):
        u = us[k]
        if u.a < skip_to_tok:
            continue
        if u.kind == 'ws':
            continue
        if u.kind == 'term':
            lat = u.term.latex
            if u.term.mathness == 'word' and parts and parts[-1].startswith(r'\text{') and parts[-1].endswith('}') and \
                    k > a and us[k - 1].kind == 'ws':
                parts[-1] = parts[-1][:-1] + ' ' + u.text + '}'
                continue
            if wrap_next:
                lat = r'\sqrt{' + (lat[1:-1] if u.term.base == 'group' and lat.startswith('(') else lat) + '}'
                wrap_next = False
            parts.append(lat)
        elif u.kind == 'op' and u.text == '√' and k + 1 < b and us[k + 1].kind == 'open' and us[k + 1].text == '(' \
                and not toks[us[k + 1].a].sp_before:
            d = 0
            e = None
            for e2 in range(k + 1, b):
                d += us[e2].kind == 'open'
                d -= us[e2].kind == 'close'
                if d == 0:
                    e = e2
                    break
            if e is None:
                return None
            inner = render_span(us, toks, k + 2, e)
            if inner is None:
                return None
            parts.append(r'\sqrt{' + inner + '}')   # √(2η − η²)
            skip_to_tok = us[e].b
            continue
        elif u.kind == 'op' and u.text == '√' and not (k + 1 < b and us[k + 1].kind == 'term'):
            return None   # a bare √ with nothing to take: a mention, or a fragment
        elif u.kind == 'op' and u.text == '√' and k + 1 < b and us[k + 1].kind == 'term':
            # radicand = the next term plus any glued application group
            e = k + 2
            if e < b and us[e].kind == 'open' and not toks[us[e].a].sp_before:
                d = 0
                for e2 in range(e, b):
                    d += us[e2].kind == 'open'
                    d -= us[e2].kind == 'close'
                    if d == 0:
                        e = e2 + 1
                        break
            inner = render_span(us, toks, k + 1, e)
            if inner is None:
                return None
            parts.append(r'\sqrt{' + (inner[1:-1] if inner.startswith('(') and inner.endswith(')') and e == k + 2 else inner) + '}')
            skip_to_tok = us[e - 1].b
            continue
        elif u.kind == 'op':
            parts.append(op_latex(u.text))
        elif u.kind == 'hyph':
            parts.append(r'\text{–}' if u.cls == 'range' else '-')
        elif u.kind == 'dash' and u.cls == 'range':
            parts.append(r'\text{–}')
        elif u.kind == 'punct':
            parts.append(u.text)
        elif u.kind == 'open':
            parts.append(OPEN[u.text])
        elif u.kind == 'close':
            parts.append(CLOSE[u.text])
        elif u.kind == 'bar':
            parts.append(r'\mid')
        elif u.kind == 'dbar':
            parts.append(r'\Vert')
        elif u.kind == 'ellip':
            parts.append(r'\ldots')
        elif u.kind == 'star':
            parts.append(r'\ast')
        elif u.kind == 'tilde':
            parts.append(r'\sim')
        elif u.kind == 'other' and u.text == '%':
            parts.append(r'\%')
        elif u.kind == 'other' and u.text in OPS:
            parts.append(OPS[u.text][0])
        elif u.kind in ('caret', 'us', 'sub', 'sup'):
            # script glued after a group: rendered from the raw tokens
            tk = toks[u.a]
            if u.kind in ('sub', 'sup'):
                chars = ''.join(subsup_of(c)[1] for c in tk.text)
                parts[-1] += ('_' if u.kind == 'sub' else '^') + brace(chars)
            else:
                r = parse_script_arg(toks, u.b)
                if r is None or not parts:
                    return None
                parts[-1] += ('_' if u.kind == 'us' else '^') + brace(r[1])
                skip_to_tok = r[0]
        elif u.kind == 'esc' and u.cls == '|':
            parts.append(r'\vert')
        else:
            return None
    return join_latex(parts)


FLATTENED = re.compile(r'[A-Za-z][α-ωΑ-Ω][A-Za-z]')


def _unmatched_open(s, i):
    o = s[i]
    c = {'(': ')', '[': ']', '{': '}'}[o]
    d = 0
    for ch in s[i:]:
        d += ch == o
        d -= ch == c
        if d == 0:
            return False
    return True


LABEL_NOUNS = {'appendix', 'section', 'table', 'figure', 'fig', 'eq', 'equation', 'chapter', 'part', 'step', 'phase',
               'option', 'plan', 'model', 'class', 'type', 'level', 'case', 'tier', 'track', 'theorem', 'lemma',
               'hypothesis', 'proposition', 'corollary', 'definition', 'assumption', 'condition', 'example',
               'exhibit', 'annex', 'stage', 'variant', 'version', 'scenario', 'group', 'team', 'set', 'item',
               'box', 'panel', 'row', 'column', 'gate', 'wave', 'round', 'batch', 'pass', 'regime', 'mode'}
SNAKE = re.compile(r'[A-Za-z0-9]+(?:_[A-Za-z0-9]+){2,}\b')
CURRENCY = re.compile(r'(?<![\\$])\$\d')
CODEISH = re.compile(r'<script|</script>|function\s*\(|=>\s*\{|\b(?:var|const) \w+\s*=[^;]*;')
def dollar_hazard(s):
    """Is there a `$` on this text that is not a well-formed math delimiter?
    (GitHub/Obsidian rule: no whitespace just inside the delimiters.)"""
    for a, b, k in protected_ranges(s):
        if k != 'math':
            continue
        d = 2 if s.startswith('$$', a) else 1
        inner = s[a + d:b - d] if b - a >= 2 * d and s[b - d:b] == '$' * d else None
        if inner is None or not inner or inner[0].isspace() or inner[-1].isspace():
            return True
        if d == 1 and (len(inner) > 200 or ' | ' in inner or '&&' in inner or '](' in inner or '⟩ ⟨' in inner):
            return True   # `$(c…) | TodoWrite … $`: shell text, not a math span
    return False


SINGLE_LETTER_IN_TEX = re.compile(r'(?<![\\A-Za-z_^{])([B-HJ-Zb-z])(?![A-Za-z])')


def symbols_in(us, toks, spans):
    """Single Latin letters used as operands in math on this text: in the
    spans just found, and in existing $..$."""
    out = set()
    for sp in spans:
        if sp is None:
            continue
        a, b = sp
        for k in range(a, b):
            u = us[k]
            if u.kind == 'term' and u.term.base == 'word' and len(toks[u.a].text) == 1 and u.term.mathness in ('letter', 'strong'):
                out.add(toks[u.a].text)
    for u in us:
        if u.kind == 'term' and u.term.base == 'mspan':
            out |= set(SINGLE_LETTER_IN_TEX.findall(u.term.latex))
    return out


def normalize_paren_math(text):
    """Port of md-press's normalize_paren_math: `\\(x_t\\)` -> `$x_t$` when the
    interior reads as math. md-press runs it before any converter."""
    def interior_is_math(s):
        if re.search(r'\\[A-Za-z]', s) or any(c in OPS or c in GREEK for c in s):
            return True
        if '_' in s or '^' in s:
            return True
        t = s.strip()
        return 0 < len(t) <= 3 and t.isascii() and t.isalnum() and any(c.isalpha() for c in t)
    prot = protected_ranges(text)
    def inr(p):
        return any(a <= p < b for a, b, k in prot)
    out = []
    i = 0
    while i < len(text):
        if text.startswith('\\(', i) and not inr(i):
            rel = text.find('\\)', i + 2)
            if rel >= 0:
                interior = text[i + 2:rel]
                if not any(inr(p) for p in range(i, rel + 2)) and '`' not in interior and '$' not in interior \
                        and interior_is_math(interior):
                    out.append('$' + interior.strip() + '$')
                    i = rel + 2
                    continue
        out.append(text[i])
        i += 1
    return ''.join(out)


ARG_CMDS = {r'\sqrt': 1, r'\frac': 2, r'\text': 1, r'\mathrm': 1, r'\mathcal': 1, r'\mathbb': 1, r'\mathbf': 1,
            r'\mathfrak': 1, r'\mathsf': 1, r'\mathtt': 1, r'\boldsymbol': 1, r'\operatorname': 1, r'\hat': 1,
            r'\bar': 1, r'\tilde': 1, r'\dot': 1, r'\ddot': 1, r'\vec': 1, r'\breve': 1, r'\check': 1,
            r'\acute': 1, r'\grave': 1}


def tex_ok(lat: str) -> bool:
    """Structural well-formedness of an emitted span: balanced braces, every
    argument-taking command has its arguments, no double sub/superscript.
    (KaTeX and MathJax both reject what this rejects; measured over every
    span the converter emits on the estate.)"""
    if re.search(r'(?<!\\)[&#%]', lat):
        return False   # alignment tabs, macro parameters (house: no #slug in math), comments
    toks = re.findall(r'\\[A-Za-z]+|\\.|[{}_^]|\s+|.', lat)
    depth = 0
    for t in toks:
        depth += t == '{'
        depth -= t == '}'
        if depth < 0:
            return False
    if depth:
        return False

    def arg_end(i):
        while i < len(toks) and toks[i].isspace():
            i += 1
        if i >= len(toks) or toks[i] in ('}', '_', '^'):
            return None
        if toks[i] == '{':
            d = 0
            for j in range(i, len(toks)):
                d += toks[j] == '{'
                d -= toks[j] == '}'
                if d == 0:
                    return j + 1
            return None
        if toks[i] in ARG_CMDS:
            e = i + 1
            for _ in range(ARG_CMDS[toks[i]]):
                e = arg_end(e)
                if e is None:
                    return None
            return e
        return i + 1

    # per brace level: scripts seen on the current atom
    stack = [{'_': False, '^': False}]
    i = 0
    while i < len(toks):
        t = toks[i]
        if t in ARG_CMDS:
            e = i + 1
            for _ in range(ARG_CMDS[t]):
                e = arg_end(e)
                if e is None:
                    return False
            stack[-1] = {'_': False, '^': False}
            i = e
            continue
        if t in ('_', '^'):
            if stack[-1][t]:
                return False
            stack[-1][t] = True
            e = arg_end(i + 1)
            if e is None:
                return False
            i = e
            continue
        if t == "'":
            i += 1
            continue
        if t == '{':
            stack.append({'_': False, '^': False})
            i += 1
            continue
        if t == '}':
            stack.pop()
            stack[-1] = {'_': False, '^': False}
            i += 1
            continue
        if not t.isspace():
            stack[-1] = {'_': False, '^': False}
        i += 1
    return True


HOUSE_LABELS = set()
try:
    import json as _json, os as _os
    for _up in ('..', '../..'):
        _p = _os.path.join(_os.path.dirname(_os.path.abspath(__file__)), _up, 'data', 'house-labels-asf-canon.json')
        if _os.path.exists(_p):
            HOUSE_LABELS = set(_json.load(open(_p)))
            break
except Exception:
    pass


def only_house_labels(region):
    """Is this region nothing but house labels (and separators)? `W₁`,
    `W₀/W₁/W₂` — mined from asf canon, which writes these regime names as
    Unicode prose (W₁ 139× raw vs 1× in math) — stay as written."""
    if not HOUSE_LABELS:
        return False
    parts = [p for p in re.split(r'[\s/,]+', region) if p]
    return bool(parts) and all(p in HOUSE_LABELS or p in HOUSE_LABELS_SECONDARY for p in parts) and any(p in HOUSE_LABELS for p in parts)


HOUSE_LABELS_SECONDARY = {'W₂'}   # in a list with W₀/W₁ it is the regime, not Wasserstein


def convert(s: str, min_conf: float = 0.0, debug=False, symbols=None, use_symbols=True):
    """Return (converted_text, spans) where spans = [(start, end, latex, conf)]."""
    if any(0x2500 <= ord(c) <= 0x257F for c in s):
        return s, []   # box-drawing table: glyphs are layout, not math
    s0 = s
    s = normalize_paren_math(s)
    if s != s0:
        out, info = convert(s, min_conf, debug, symbols, use_symbols)
        return out, info
    if CURRENCY.search(s) or dollar_hazard(s):
        return s, []   # `$5 / unit … $25`, `$Y/day`: a literal `$` pairs with any new `$`
    if CODEISH.search(s) or s.count(';') >= 3 and sum(s.count(c) for c in '{}=') >= 6:
        return s, []   # inline script / code in prose position
    toks = mark_emphasis(lex(s))
    us = units_of(toks)
    spans = find_spans(us, toks)
    if use_symbols:
        syms = set(symbols or ())
        syms |= symbols_in(us, toks, spans)
        if syms:
            toks = mark_emphasis(lex(s))
            us = units_of(toks)
            spans = find_spans(us, toks, frozenset(syms))
    out = []
    pos = 0
    info = []
    for sp in spans:
        if sp is None:
            continue
        a, b = sp
        # a span must contain new math, not just existing $..$ and punctuation
        if all(us[k].kind in ('ws', 'punct', 'open', 'close') or us[k].kind == 'term' and us[k].term.base == 'mspan'
               for k in range(a, b)):
            continue
        lat = render_span(us, toks, a, b)
        if lat is not None and not re.search(r'[A-Za-z∞^_]', re.sub(r'\\[A-Za-z]+', '', lat.replace(r'\text{–}', ''))) \
                and not re.search(r'\\(infty|alpha|beta|gamma|delta|epsilon|varepsilon|zeta|eta|theta|iota|kappa|lambda|mu|nu|xi|pi|rho|sigma|tau|upsilon|phi|chi|psi|omega|Gamma|Delta|Theta|Lambda|Xi|Pi|Sigma|Phi|Psi|Omega|math|partial|nabla|sum|prod|int|ell|emptyset|sqrt|times|cdot)', lat):
            continue   # `$1.6$`, `$9 \leq 6$`: numbers alone are not worth a span
        start = toks[us[a].a].a
        end = toks[us[b - 1].b - 1].b
        if only_house_labels(s[start:end]):
            lat = None   # a house label, not a variable
        if FLATTENED.search(s[start:end]):
            lat = None   # `aθk′`: scripts lost to PDF extraction; unknowable which
        if start > 0 and s[start - 1] in '([{' and _unmatched_open(s, start - 1) and not s[end:].strip(' .,;'):
            lat = None   # `e^(-λ`: a fragment cut out of a larger expression
        if start > 0 and s[start - 1] in '"“`' and s[end:end + 1] in '"”`':
            lat = None   # "O(√h)": a quoted string is a mention, not math
        if s[end:end + 1] in ('^', '_') or start > 0 and s[start - 1] in ('^', '_'):
            lat = None   # `(sI-A')^(-1)`: a script we could not attach
        if s[end:end + 1] in ('}', '{') or s[start - 1:start] in ('{', '}') and start > 0:
            lat = None   # a fragment cut out of a larger braced expression
        conf = min([us[k].term.conf for k in range(a, b) if us[k].kind == 'term'] + [1.0])
        if lat is not None and not tex_ok(lat):
            lat = None   # never write a span that won't render
        if lat is None or conf < min_conf:
            info.append((start, end, None, conf))
            continue
        out.append(s[pos:start])
        out.append('$' + lat + '$')
        pos = end
        info.append((start, end, lat, conf))
    out.append(s[pos:])
    return ''.join(out), info
