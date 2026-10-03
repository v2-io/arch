"""Visual atoms: what a reader sees, for "effectively equivalent" scoring.

A markdown piece with $...$ spans becomes a sequence of atoms
(glyph, script-path, font):
  glyph        one visible character ('α', 'x', '≤', '√', ...)
  script-path  '' for the baseline, '_' / '^' / '_^' ... inside scripts,
               plus '√' / accents for radicands and accented bases
  font         'tx' prose text, 'mi' math italic, 'mu' math upright
               (digits, \\text, \\mathrm, operator names), or a named
               alphabet: 'cal' 'bb' 'bf' 'frak' 'sf' 'tt'

Whitespace and grouping braces are not atoms. Comparison rules live in
compare(); see score_v2 for the verdict ladder.
"""
from __future__ import annotations

import difflib
import re
import unicodedata

from umath import GREEK, OPS, mathalpha_latex

CMD_GLYPH = {}
for ch, cmd in GREEK.items():
    if cmd.startswith('\\') and cmd not in CMD_GLYPH:
        CMD_GLYPH[cmd] = ch
for ch, (cmd, _) in OPS.items():
    if cmd.startswith('\\') and cmd not in CMD_GLYPH:
        CMD_GLYPH[cmd] = ch
CMD_GLYPH.update({
    r'\epsilon': 'ε', r'\varepsilon': 'ε', r'\phi': 'φ', r'\varphi': 'φ', r'\vartheta': 'θ',
    r'\varrho': 'ρ', r'\varsigma': 'σ', r'\varpi': 'π', r'\varkappa': 'κ',
    r'\lVert': '‖', r'\rVert': '‖', r'\Vert': '‖', r'\|': '‖', r'\lvert': '|', r'\rvert': '|',
    r'\vert': '|', r'\mid': '|', r'\lt': '<', r'\gt': '>', r'\le': '≤', r'\leq': '≤', r'\ge': '≥',
    r'\geq': '≥', r'\ne': '≠', r'\neq': '≠', r'\ast': '*', r'\cdot': '·', r'\times': '×',
    r'\to': '→', r'\rightarrow': '→', r'\leftarrow': '←', r'\gets': '←', r'\infty': '∞',
    r'\ldots': '…', r'\dots': '…', r'\cdots': '⋯', r'\sum': 'Σ', r'\prod': 'Π', r'\Sigma': 'Σ',
    r'\Pi': 'Π', r'\lnot': '¬', r'\neg': '¬', r'\varnothing': '∅', r'\emptyset': '∅',
    r'\implies': '⇒', r'\Rightarrow': '⇒', r'\Longrightarrow': '⇒', r'\iff': '⇔',
    r'\Leftrightarrow': '⇔', r'\Longleftrightarrow': '⇔', r'\leftrightarrow': '↔',
    r'\langle': '⟨', r'\rangle': '⟩', r'\{': '{', r'\}': '}', r'\%': '%', r'\#': '#', r'\_': '_',
    r'\$': '$', r'\&': '&', r'\top': 'T', r'\intercal': 'T', r'\ell': 'ℓ', r'\hbar': 'ℏ',
    r'\prime': "'", r'\dagger': '†', r'\star': '⋆', r'\bullet': '•', r'\sim': '∼', r'\simeq': '≃',
    r'\approx': '≈', r'\propto': '∝', r'\partial': '∂', r'\nabla': '∇', r'\sqrt': '√',
    r'\lfloor': '⌊', r'\rfloor': '⌋', r'\lceil': '⌈', r'\rceil': '⌉', r'\llbracket': '⟦',
    r'\rrbracket': '⟧', r'\uparrow': '↑', r'\downarrow': '↓', r'\mapsto': '↦', r'\circ': '∘',
    r'\Delta': 'Δ', r'\colon': ':', r'\triangleq': '≜', r'\coloneqq': '≔', r'\models': '⊨',
})
FONT_CMDS = {r'\mathcal': 'cal', r'\mathscr': 'cal', r'\mathbb': 'bb', r'\mathbf': 'bf',
             r'\boldsymbol': 'bf', r'\bm': 'bf', r'\mathfrak': 'frak', r'\mathsf': 'sf',
             r'\mathtt': 'tt', r'\mathrm': 'mu', r'\operatorname': 'mu', r'\text': 'mu',
             r'\textrm': 'mu', r'\textit': 'mu', r'\mathit': 'mi', r'\textbf': 'bf',
             r'\textnormal': 'mu', r'\mbox': 'mu'}
ACCENTS = {r'\hat': '^', r'\widehat': '^', r'\bar': '¯', r'\overline': '¯', r'\tilde': '~',
           r'\widetilde': '~', r'\dot': '˙', r'\ddot': '¨', r'\vec': '→', r'\breve': '˘',
           r'\check': 'ˇ', r'\acute': '´', r'\grave': '`'}
NAMED_OPS = {'max', 'min', 'sup', 'inf', 'lim', 'log', 'ln', 'exp', 'sin', 'cos', 'tan', 'det',
             'arg', 'Pr', 'limsup', 'liminf', 'tanh', 'gcd', 'dim', 'ker', 'deg', 'hom'}
INVISIBLE = {r'\left', r'\right', r'\big', r'\Big', r'\bigg', r'\Bigg', r'\bigl', r'\bigr',
             r'\Bigl', r'\Bigr', r'\,', r'\;', r'\:', r'\!', r'\ ', r'\quad', r'\qquad',
             r'\displaystyle', r'\textstyle', r'\limits', r'\nolimits', r'\mathop', r'\>'}


def tex_tokens(s):
    out = []
    i = 0
    while i < len(s):
        c = s[i]
        if c == '\\':
            m = re.match(r'\\([A-Za-z]+|.)', s[i:])
            if m:
                out.append(m.group())
                i += len(m.group())
                continue
        out.append(c)
        i += 1
    return out


def math_atoms(src, path='', font=None):
    toks = tex_tokens(src)
    out = []
    pos = 0

    def group():
        """read one argument: {..} or a single token; returns source string"""
        nonlocal pos
        while pos < len(toks) and toks[pos] == ' ':
            pos += 1
        if pos >= len(toks):
            return ''
        if toks[pos] == '{':
            depth = 0
            start = pos
            while pos < len(toks):
                if toks[pos] == '{':
                    depth += 1
                elif toks[pos] == '}':
                    depth -= 1
                    if depth == 0:
                        pos += 1
                        return ''.join(toks[start + 1:pos - 1])
                pos += 1
            return ''.join(toks[start + 1:])
        t = toks[pos]
        pos += 1
        return t

    while pos < len(toks):
        t = toks[pos]
        pos += 1
        if t in (' ', '~', '{', '}', '&') or t in INVISIBLE:
            continue
        if t in ('_', '^'):
            arg = group()
            if arg in FONT_CMDS or arg in ACCENTS or arg in (r'\sqrt',):
                arg = arg + '{' + group() + '}'   # `_\text{obs}`
            out += math_atoms(arg, path + t, font)
            continue
        if t == "'":
            out.append(("'", path + '^', 'mu'))
            continue
        if t in FONT_CMDS:
            f = FONT_CMDS[t]
            arg = group()
            if f == 'mu' and t in (r'\text', r'\textrm', r'\mbox', r'\textnormal', r'\textit'):
                for ch in arg:
                    if not ch.isspace():
                        out.append((ch, path, 'mu'))
            else:
                out += math_atoms(arg, path, f)
            continue
        if t in ACCENTS:
            arg = group()
            out += math_atoms(arg, path + ACCENTS[t], font)
            continue
        if t == r'\sqrt':
            # optional [n]
            if pos < len(toks) and toks[pos] == '[':
                while pos < len(toks) and toks[pos] != ']':
                    pos += 1
                pos += 1
            arg = group()
            out.append(('√', path, 'mu'))
            out += math_atoms(arg, path + '√', font)
            continue
        if t in (r'\frac', r'\dfrac', r'\tfrac'):
            a = group()
            b = group()
            out += math_atoms(a, path, font)
            out.append(('/', path, 'mu'))
            out += math_atoms(b, path, font)
            continue
        if t.startswith('\\') and len(t) > 1 and t[1:] in NAMED_OPS:
            for ch in t[1:]:
                out.append((ch, path, 'mu'))
            continue
        if t in CMD_GLYPH:
            g = CMD_GLYPH[t]
            f = font or ('mi' if g.isalpha() and unicodedata.category(g).startswith('L') and g not in 'ℓℏ' else 'mu')
            if g in 'ΣΠΔΓΘΛΞΦΨΩ' and font is None:
                f = 'mu'   # capital Greek is upright in TeX
            out.append((g, path, f))
            continue
        if t.startswith('\\'):
            out.append((t, path, 'mu'))   # unknown command: keep as an opaque atom
            continue
        if t.isalpha():
            out.append((t, path, font or 'mi'))
        else:
            out.append((t, path, font if font in ('bb', 'bf') else 'mu'))
    return out


def piece_atoms(s):
    """Atoms for a whole markdown piece (prose chars + $..$ spans)."""
    out = []
    i = 0
    n = len(s)
    while i < n:
        c = s[i]
        if c == '\\' and i + 1 < n and s[i + 1] == '$':
            out.append(('$', '', 'tx'))
            i += 2
            continue
        if c == '$':
            d = '$$' if s.startswith('$$', i) else '$'
            j = s.find(d, i + len(d))
            while j > 0 and s[j - 1] == '\\':
                j = s.find(d, j + 1)
            if j < 0:
                out.append(('$', '', 'tx'))
                i += 1
                continue
            out += math_atoms(s[i + len(d):j])
            i = j + len(d)
            continue
        if not c.isspace():
            # a bare Unicode math-alphanumeric in prose: its glyph is its letter
            out.append((c, '', 'tx'))
        i += 1
    return out


def _canon_glyph(g):
    """Prose Unicode glyphs that typeset to a styled letter: 𝒜 -> ('A','cal')."""
    lat = mathalpha_latex(g) if len(g) == 1 else None
    if lat:
        m = re.match(r'\\(mathcal|mathbb|mathbf|mathfrak|mathsf|mathtt)\{(.+)\}$', lat)
        if m:
            return m.group(2), {'mathcal': 'cal', 'mathbb': 'bb', 'mathbf': 'bf', 'mathfrak': 'frak',
                                'mathsf': 'sf', 'mathtt': 'tt'}[m.group(1)]
    return g, None


def normalize_atoms(atoms):
    out = []
    for g, p, f in atoms:
        if f == 'tx':
            g2, f2 = _canon_glyph(g)
            if f2:
                g, f = g2, 'tx-' + f2
        g = {'−': '-', '∗': '*', '′': "'", 'ϵ': 'ε', 'ϕ': 'φ', '∑': 'Σ', '∏': 'Π', '∣': '|',
             '…': '…', '⋅': '·', '–': '-', '\u2016': '‖'}.get(g, g)
        if g == '|' and p.endswith('|'):
            pass
        out.append((g, p, f))
    # `||x||` in prose is a norm: two bars -> one double bar
    res = []
    for a in out:
        if res and a[0] == '|' and res[-1][0] == '|' and a[1] == res[-1][1] and a[2] == res[-1][2] == 'tx':
            res[-1] = ('‖', a[1], 'tx')
        else:
            res.append(a)
    return res


LETTERISH = lambda g: len(g) == 1 and g.isalpha()


def font_relation(fs, fg, glyph):
    """How the system's font for an atom relates to the gold's."""
    if fs == fg:
        return 'same'
    S = {fs, fg}
    if not LETTERISH(glyph):
        return 'same'          # digits, punctuation, operators: font not meaningful
    if S <= {'tx', 'mu'}:
        return 'same'          # upright either way
    if S <= {'mi', 'mu'} or glyph == 'E' and 'bb' in S and S <= {'bb', 'mi', 'mu', 'tx'}:
        return 'style'
    if fs.startswith('tx-') and fs[3:] == fg or fg.startswith('tx-') and fg[3:] == fs:
        return 'under' if fs.startswith('tx') else 'over'
    if fs.startswith('tx') and fg not in ('tx', 'mu') and not fg.startswith('tx'):
        return 'under'         # system left it as prose, gold typesets it
    if fg.startswith('tx') and fs not in ('tx', 'mu') and not fs.startswith('tx'):
        greek = '\u0370' <= glyph <= '\u03ff'
        return 'over' if greek else 'wrong'
    return 'wrong'


def compare(sys_s, gold_s, lenient=False):
    """Verdict ladder: exact < equivalent < degraded < over < wrong.
    Returns (verdict, details). `lenient` (synthetic round-trips only): the
    source was rewritten from the gold, so information the rewrite dropped
    (an alphabet like \\mathcal, an implicit product written as `·`) is not
    the converter's to recover."""
    if sys_s == gold_s:
        return 'exact', {}
    A = normalize_atoms(piece_atoms(sys_s))
    B = normalize_atoms(piece_atoms(gold_s))
    if lenient:
        A = [(g, p, ('mi' if f in ('cal', 'bb', 'bf', 'frak', 'sf') else f)) for g, p, f in A if not (g == '·' and not f.startswith('tx'))]
        B = [(g, p, ('mi' if f in ('cal', 'bb', 'bf', 'frak', 'sf') else f)) for g, p, f in B if not (g == '·' and not f.startswith('tx'))]
    ka = [(g, p) for g, p, f in A]
    kb = [(g, p) for g, p, f in B]
    det = {'style': 0, 'under': 0, 'over': 0, 'wrong': 0, 'struct': 0}
    if ka != kb:
        sm = difflib.SequenceMatcher(None, ka, kb, autojunk=False)
        for op, i1, i2, j1, j2 in sm.get_opcodes():
            if op == 'equal':
                for x, y in zip(A[i1:i2], B[j1:j2]):
                    r = font_relation(x[2], y[2], x[0])
                    if r != 'same':
                        det[r] += 1
            else:
                sa, gb = A[i1:i2], B[j1:j2]
                if all(f.startswith('tx') for _, _, f in sa) and not all(f.startswith('tx') for _, _, f in gb):
                    det['under'] += max(len(sa), len(gb))     # system left this region as written
                elif all(f.startswith('tx') for _, _, f in gb) and not all(f.startswith('tx') for _, _, f in sa):
                    if any(g.isascii() and g.isalpha() for g, _, f in sa if not f.startswith('tx')):
                        det['wrong'] += max(len(sa), len(gb))  # prose letters typeset as math
                    else:
                        det['over'] += max(len(sa), len(gb))
                else:
                    det['struct'] += max(len(sa), len(gb))
    else:
        for x, y in zip(A, B):
            r = font_relation(x[2], y[2], x[0])
            if r != 'same':
                det[r] += 1
    if det['struct'] or det['wrong']:
        v = 'wrong'
    elif det['over']:
        v = 'over'
    elif det['under']:
        v = 'degraded'
    else:
        v = 'equivalent'
    return v, det
