"""LaTeX span -> agent-style Unicode, with seeded dialect variation.

Used to turn the estate's existing `$...$` prose (the expected output) into
synthetic Unicode sources (Joseph's suggestion, 2026-10-02). The variation
knobs are *my* guesses at the agent dialect; agent-written unicodifications
(data/synth-agent/) exist to check them.
"""
from __future__ import annotations

import random
import re

from atoms import tex_tokens
from umath import GREEK, OPS

GREEK_OF = {}
for ch, cmd in GREEK.items():
    if cmd.startswith('\\') and cmd not in GREEK_OF:
        GREEK_OF[cmd] = ch
GREEK_OF.update({r'\epsilon': 'ε', r'\varepsilon': 'ε', r'\varphi': 'φ', r'\phi': 'φ', r'\vartheta': 'θ',
                 r'\varrho': 'ρ', r'\varsigma': 'σ', r'\varpi': 'π'})
SYM = {}
for ch, (cmd, _) in OPS.items():
    if cmd.startswith('\\') and cmd not in SYM:
        SYM[cmd] = ch
SYM.update({r'\le': '≤', r'\leq': '≤', r'\ge': '≥', r'\geq': '≥', r'\ne': '≠', r'\neq': '≠', r'\lt': '<',
            r'\gt': '>', r'\to': '→', r'\rightarrow': '→', r'\infty': '∞', r'\cdot': '·', r'\times': '×',
            r'\ldots': '…', r'\dots': '…', r'\cdots': '⋯', r'\mid': '|', r'\vert': '|', r'\lvert': '|',
            r'\rvert': '|', r'\Vert': '‖', r'\lVert': '‖', r'\rVert': '‖', r'\|': '‖', r'\ast': '*',
            r'\star': '*', r'\langle': '⟨', r'\rangle': '⟩', r'\{': '{', r'\}': '}', r'\%': '%',
            r'\ell': 'ℓ', r'\emptyset': '∅', r'\varnothing': '∅', r'\neg': '¬', r'\lnot': '¬',
            r'\implies': '⇒', r'\iff': '⇔', r'\top': 'ᵀ', r'\prime': "'", r'\dagger': '†',
            r'\sim': '∼', r'\approx': '≈', r'\propto': '∝', r'\sum': 'Σ', r'\prod': 'Π',
            r'\partial': '∂', r'\nabla': '∇', r'\in': '∈', r'\notin': '∉', r'\subset': '⊂',
            r'\subseteq': '⊆', r'\supset': '⊃', r'\supseteq': '⊇', r'\cup': '∪', r'\cap': '∩',
            r'\Rightarrow': '⇒', r'\Leftrightarrow': '⇔', r'\leftrightarrow': '↔', r'\mapsto': '↦',
            r'\equiv': '≡', r'\pm': '±', r'\circ': '∘', r'\forall': '∀', r'\exists': '∃',
            r'\perp': '⊥', r'\gg': '≫', r'\ll': '≪', r'\otimes': '⊗', r'\oplus': '⊕', r'\wedge': '∧',
            r'\vee': '∨', r'\setminus': '∖', r'\succ': '≻', r'\prec': '≺', r'\succeq': '⪰',
            r'\preceq': '⪯', r'\simeq': '≃', r'\cong': '≅', r'\leftarrow': '←', r'\gets': '←',
            r'\uparrow': '↑', r'\downarrow': '↓', r'\int': '∫', r'\lfloor': '⌊', r'\rfloor': '⌋',
            r'\lceil': '⌈', r'\rceil': '⌉', r'\triangleq': '≜', r'\models': '⊨', r'\vdash': '⊢'})
ASCII_REL = {'≤': '<=', '≥': '>=', '≠': '!=', '→': '->'}
SUBD = str.maketrans('0123456789+-=()', '₀₁₂₃₄₅₆₇₈₉₊₋₌₍₎')
SUPD = str.maketrans('0123456789+-=()n', '⁰¹²³⁴⁵⁶⁷⁸⁹⁺⁻⁼⁽⁾ⁿ')
SCRIPT = {c: chr(0x1D49C + i) for i, c in enumerate('ABCDEFGHIJKLMNOPQRSTUVWXYZ')}
SCRIPT.update({'B': 'ℬ', 'E': 'ℰ', 'F': 'ℱ', 'H': 'ℋ', 'I': 'ℐ', 'L': 'ℒ', 'M': 'ℳ', 'R': 'ℛ'})
BB = {'R': 'ℝ', 'N': 'ℕ', 'Z': 'ℤ', 'Q': 'ℚ', 'C': 'ℂ', 'P': 'ℙ', 'E': '𝔼', '1': '𝟙'}
NAMED = {'max', 'min', 'sup', 'inf', 'lim', 'log', 'ln', 'exp', 'sin', 'cos', 'tan', 'det', 'arg',
         'Pr', 'limsup', 'liminf', 'tanh', 'gcd', 'dim', 'ker'}
STRUCT_INVISIBLE = {r'\left', r'\right', r'\big', r'\Big', r'\bigg', r'\Bigg', r'\bigl', r'\bigr',
                    r'\Bigl', r'\Bigr', r'\displaystyle', r'\limits', r'\nolimits', r'\,', r'\;',
                    r'\:', r'\!', r'\ ', r'\quad', r'\qquad'}


class Unmappable(Exception):
    pass


class Rev:
    def __init__(self, rng: random.Random):
        self.r = rng
        # per-document dialect choices (an author is fairly consistent)
        self.uni_digits = rng.random() < 0.45     # x₁ / x² vs x_1 / x^2
        self.ascii_rel = rng.random() < 0.12      # <= / >= / -> instead of ≤ ≥ →
        self.tight = rng.random() < 0.35          # α>ρ/R vs α > ρ/R
        self.script_caps = rng.random() < 0.8     # 𝒜 vs A for \mathcal{A}
        self.dbar = rng.random() < 0.8            # ‖x‖ vs ||x||
        self.keep_braces = rng.random() < 0.5     # x_{t+1} vs x_(t+1)

    def span(self, src: str) -> str:
        toks = tex_tokens(src)
        self.toks = toks
        self.pos = 0
        out = self.seq()
        out = re.sub(r'\s+', ' ', out).strip()
        return out

    def group(self):
        while self.pos < len(self.toks) and self.toks[self.pos] == ' ':
            self.pos += 1
        if self.pos >= len(self.toks):
            return ''
        if self.toks[self.pos] == '{':
            depth = 0
            start = self.pos
            while self.pos < len(self.toks):
                if self.toks[self.pos] == '{':
                    depth += 1
                elif self.toks[self.pos] == '}':
                    depth -= 1
                    if depth == 0:
                        self.pos += 1
                        return ''.join(self.toks[start + 1:self.pos - 1])
                self.pos += 1
            raise Unmappable('brace')
        t = self.toks[self.pos]
        self.pos += 1
        return t

    def sub(self, src):
        saved = (self.toks, self.pos)
        r = self.span(src)
        self.toks, self.pos = saved
        return r

    def script(self, kind, arg):
        # `_\text{obs}`: an unbraced command argument takes its own argument
        if arg.startswith('\\') and arg in (r'\text', r'\mathrm', r'\operatorname', r'\mathcal', r'\mathbb',
                                            r'\mathbf', r'\hat', r'\bar', r'\tilde', r'\boldsymbol', r'\mathfrak'):
            arg = arg + '{' + self.group() + '}'
        inner = self.sub(arg)
        simple_digits = re.fullmatch(r'[0-9+\-=()]+|n', inner) is not None
        if self.uni_digits and simple_digits and (kind == '^' or re.fullmatch(r'[0-9]+', inner)):
            return inner.translate(SUPD if kind == '^' else SUBD)
        if kind == '^' and inner in ('*', '∗'):
            return '*'
        if kind == '^' and inner == "'":
            return "'"
        if kind == '^' and inner == 'ᵀ':
            return 'ᵀ'
        if len(inner) == 1 or re.fullmatch(r'[A-Za-z0-9]+', inner) or re.fullmatch(r'[α-ωΑ-Ω]', inner):
            return kind + inner
        if self.keep_braces:
            return kind + '{' + inner + '}'
        return kind + '(' + inner + ')'

    def seq(self):
        out = []
        while self.pos < len(self.toks):
            t = self.toks[self.pos]
            self.pos += 1
            if t in ('{', '}'):
                continue
            if t == ' ' or t in STRUCT_INVISIBLE or t == '~':
                out.append(' ')
                continue
            if t in ('_', '^'):
                out.append(self.script(t, self.group()))
                continue
            if t in (r'\mathcal', r'\mathscr'):
                a = self.group()
                if len(a) == 1 and a in SCRIPT:
                    out.append(SCRIPT[a])
                    continue
                raise Unmappable('mathcal-multi')
            if t == r'\mathbb':
                a = self.group()
                out.append(BB.get(a, a) if self.r.random() < 0.8 else a)
                continue
            if t in (r'\mathbf', r'\boldsymbol', r'\bm', r'\mathsf', r'\mathtt', r'\mathfrak', r'\mathit'):
                a = self.group()
                style = {r'\mathbf': 'BOLD', r'\boldsymbol': 'BOLD ITALIC', r'\bm': 'BOLD ITALIC',
                         r'\mathsf': 'SANS-SERIF', r'\mathtt': 'MONOSPACE', r'\mathfrak': 'FRAKTUR',
                         r'\mathit': 'ITALIC'}[t]
                if len(a) == 1 and a.isascii() and a.isalpha():
                    import unicodedata
                    nm = f"MATHEMATICAL {style} {'CAPITAL' if a.isupper() else 'SMALL'} {a.upper()}"
                    try:
                        out.append(unicodedata.lookup(nm))
                        continue
                    except KeyError:
                        # letterlike exceptions (ℭ ℌ ℑ ℜ ℨ for fraktur)
                        lk = {('FRAKTUR', 'C'): 'ℭ', ('FRAKTUR', 'H'): 'ℌ', ('FRAKTUR', 'I'): 'ℑ',
                              ('FRAKTUR', 'R'): 'ℜ', ('FRAKTUR', 'Z'): 'ℨ'}.get((style, a))
                        if lk:
                            out.append(lk)
                            continue
                raise Unmappable(t + '{' + a + '}')
            if t == r'\mathcal' and False:
                pass
            if t in (r'\text', r'\mathrm', r'\operatorname', r'\textrm', r'\textit', r'\textbf', r'\mbox', r'\textnormal'):
                out.append(self.group().replace(' ', ' '))
                continue
            if t in (r'\hat', r'\widehat'):
                a = self.sub(self.group())
                out.append(a + '̂' if len(a) == 1 else 'hat(' + a + ')')
                continue
            if t in (r'\bar', r'\overline'):
                a = self.sub(self.group())
                out.append(a + '̄' if len(a) == 1 else 'bar(' + a + ')')
                continue
            if t in (r'\tilde', r'\widetilde'):
                a = self.sub(self.group())
                out.append(a + '̃' if len(a) == 1 else a + '~')
                continue
            if t == r'\dot':
                a = self.sub(self.group())
                out.append(a + '̇')
                continue
            if t == r'\vec':
                a = self.sub(self.group())
                out.append(a + '⃗')
                continue
            if t == r'\sqrt':
                if self.pos < len(self.toks) and self.toks[self.pos] == '[':
                    raise Unmappable('nthroot')
                a = self.sub(self.group())
                out.append('√' + (a if re.fullmatch(r'[\wͰ-Ͽ]+', a) else '(' + a + ')'))
                continue
            if t in (r'\frac', r'\dfrac', r'\tfrac'):
                a = self.sub(self.group())
                b = self.sub(self.group())
                wa = a if re.fullmatch(r'[\wͰ-Ͽ′\'*]+', a) else '(' + a + ')'
                wb = b if re.fullmatch(r'[\wͰ-Ͽ′\'*]+', b) else '(' + b + ')'
                out.append(wa + '/' + wb)
                continue
            if t == r'\begin' or t == r'\end' or t == r'\tag' or t == r'\label':
                raise Unmappable(t)
            if t in GREEK_OF:
                out.append(GREEK_OF[t])
                continue
            if t in SYM:
                g = SYM[t]
                if self.ascii_rel and g in ASCII_REL:
                    g = ASCII_REL[g]
                if g in '≤≥≠<>=≈→∈⊂⊆⊃⊇∝≡⇒⇔↔≪≫∼' or g in ('<=', '>=', '!=', '->'):
                    out.append(g if self.tight else ' ' + g + ' ')
                elif g in '·×±∪∩':
                    out.append(g if self.tight else ' ' + g + ' ')
                else:
                    out.append(g)
                continue
            if t.startswith('\\') and t[1:] in NAMED:
                out.append(t[1:] + ' ')
                continue
            if t.startswith('\\') and len(t) > 1 and t[1].isalpha():
                raise Unmappable(t)
            if t in '=<>+':
                out.append(t if self.tight else ' ' + t + ' ')
                continue
            out.append(t)
        return ''.join(out)


RANGES = []


def unicodify_line(line: str, rng: random.Random, p_reverse: float = 1.0):
    """Side effect: RANGES holds the output char ranges that came from spans.
    Replace every $..$ span in a line with agent-style Unicode. Returns
    (unicode_line) or raises Unmappable."""
    rv = Rev(rng)
    RANGES.clear()
    out = []
    i = 0
    while i < len(line):
        if line[i] == '\\' and i + 1 < len(line):
            out.append(line[i:i + 2])
            i += 2
            continue
        if line[i] == '`':
            j = line.find('`', i + 1)
            if j < 0:
                out.append(line[i:])
                break
            out.append(line[i:j + 1])
            i = j + 1
            continue
        if line[i] == '$':
            if line.startswith('$$', i):
                raise Unmappable('display')
            j = i + 1
            while j < len(line) and line[j] != '$':
                j += 2 if line[j] == '\\' else 1
            if j >= len(line):
                raise Unmappable('unbalanced')
            if rng.random() < p_reverse:
                st = sum(len(x) for x in out)
                out.append(rv.span(line[i + 1:j]))
                RANGES.append((st, st + len(out[-1])))
            else:
                out.append(line[i:j + 1])
            i = j + 1
            continue
        out.append(line[i])
        i += 1
    return ''.join(out)
