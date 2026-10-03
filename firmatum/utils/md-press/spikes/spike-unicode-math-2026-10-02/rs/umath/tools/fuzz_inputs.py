"""Seeded fuzz inputs for the differential, beyond what the corpus exercises.

  python3 rs/umath/tools/fuzz_inputs.py [seed] [n] > rs/umath/scratch/fuzz.jsonl

Three families:
  r*  random strings over the converter's special alphabet (operators, Greek,
      brackets, scripts, math alphanumerics, combining marks, markdown/code/$
      delimiters, Unicode spaces, letters, digits, prose words);
  m*  mutations (insert/delete/replace/splice) of real estate sites that the
      converter changes;
  c*  every BMP codepoint (and every math-alphanumeric / sub-sup / special
      plane char) dropped into a handful of math contexts — exercises the
      generated Unicode property tables character by character.
"""
import json
import random
import sys

seed = int(sys.argv[1]) if len(sys.argv) > 1 else 1
N = int(sys.argv[2]) if len(sys.argv) > 2 else 200000
R = random.Random(seed)

ops = list('=<>≤≥≠≈≡∝∼≪≫∈∉⊂⊃⊆⊇≺≻⪰⪯≼≽⊥≅≃≜≔∣⊨⊢≲≳≍∥⊊⊋∋→←↔⇒⇐⇔⟹⟸⟺↦⟶⟵↑↓⇝↛⇏+−·⋅×÷±∓∘⊗⊕⊙∪∩∧∨∖/⊔⊓∑∏∫∮∂∇√¬∀∃∄□◇∛∞∅ℵ†⋯∠△⋃⋂')
greek = list('αβγδεϵζηθϑικλμνξπϖρϱσςτυφϕχψωΓΔΘΛΞΠΣΥΦΨΩοΑΒΕΖΗΙΚΜΝΟΡΤΧϰάέήίόύώἀᾶ͵ʹ΄')
brk = list('()[]{}⟨⟩⟦⟧⌊⌋⌈⌉|‖')
scripts = list('⁰¹²³⁴⁵⁶⁷⁸⁹⁺⁻⁼⁽⁾ⁿⁱᵀᵢⱼₖₗₘₙₜₓ₀₁₂₃₄₅₆₇₈₉₊₋ᵝᵞᵅ')
malpha = [chr(c) for c in range(0x1D400, 0x1D800, 7)] + list('ℝℕℤℚℂℙℓℏℋℒℱℰℳℬℛℐ℘ℑℜ𝟙ℊℯℴ𝟊𝟋')
comb = ['̂', '̃', '̄', '̅', '̇', '̈', '⃗', '̆', '̌', '́', '̀', '̧']
precomp = list('ṘẋŷẍāēīōūñõãäöüâêîôûṙẏżėȧḃċḋḟġḣṁṅṗṡṫẇẏÅÉéèàç')
delims = list('$$$``\\\\_^*~**__') + ['\\(', '\\)', '](', '[[', ']]', '<a href="x">', '</b>', 'http://x.y/z', 'www.q.r', '$$']
spaces = [' ', ' ', ' ', '\t', ' ', ' ', '　', '​', '\x1c', ' ']
punct = list(',;:.?!…—–"“”‘’\'′″‴%&#@-') + ["'s", "n't"]
words = ['a', 'I', 'A', 'x', 'y', 'n', 't', 'k', 'M_t', 'f(x)', 'max', 'min', 'log', 'exp', 'Var', 'O', 'o', 'E1', 'T4',
         'the', 'is', 'and', 'Appendix', 'Table', 'Class', 'Section', 'e.g.', 'i.e.', 'p.', 'kg', 'm', 's', 'ms', '200k',
         '3rd', 'κ_processing', 'n_past', 'a_b_c', 'snake_case_ident', '83k_cont_pre', 'W₀', 'W₁', 'W₂', 'B∞', 'dt', 'dx',
         'Casella,', 'G.', 'var x = 1;', 'const y=2;', 'function (', '=> {', '<script', 'is-a-', 'μs', 'kΩ', 'δ¹⁸O',
         'τ1', 'η2p', 'café', 'rôle', 'Ελληνικά', 'Привет', '中文', '१२३', '٣', '０１', '½', 'Ⅻ']
nums = ['0', '1', '2', '3.14', '1,000', '10', '0.5', '42', '7']
FAM = [(ops, 6), (greek, 5), (brk, 4), (scripts, 3), (malpha, 2), (comb, 1), (precomp, 1), (delims, 3), (spaces, 8),
       (punct, 3), (words, 6), (nums, 3), (list('abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789'), 6)]
pool = [f for f, w in FAM for _ in range(w)]


def rnd():
    return ''.join(R.choice(R.choice(pool)) for _ in range(R.randint(1, 40)))


def w(i, t):
    sys.stdout.write(json.dumps({'id': i, 'text': t}, ensure_ascii=False) + '\n')


nr = N // 2
for i in range(nr):
    w(f'r{i}', rnd())

base = [json.loads(l)['text'] for l in open('rs/umath/scratch/estate.jsonl') if any(c in l for c in '_^αβγδηθκλμσρτφωΣΔ≤≥∈→‖√∑')]
R.shuffle(base)
for i in range(N - nr):
    t = list(R.choice(base))
    for _ in range(R.randint(1, 4)):
        op = R.random()
        p = R.randint(0, len(t))
        if op < 0.35:
            t[p:p] = list(R.choice(R.choice(pool)))
        elif op < 0.6 and t:
            del t[min(p, len(t) - 1)]
        elif op < 0.85 and t:
            t[min(p, len(t) - 1)] = R.choice(R.choice(pool))
        else:
            o = R.choice(base)
            q = R.randint(0, len(o))
            t[p:p] = list(o[q:q + R.randint(1, 20)])
    w(f'm{i}', ''.join(t))

ctx = ['{c}', 'x{c}', '{c}_t', '{c}² = 1', 'η {c} κ', 'a{c}b', '({c}, β)', '‖{c}‖', '{c}ᵀ', 'f({c})', '{c}′ ≤ 2', '$x$ {c}', 'M_{c}']
cps = [c for c in range(0x10000) if not 0xD800 <= c <= 0xDFFF] + \
      list(range(0x1D400, 0x1D800)) + list(range(0x1F00, 0x2000)) + [0x1D7CA, 0x1D7CB, 0x10FFFD, 0x1F600, 0xE0001]
for c in cps:
    w(f'c{c:x}', R.choice(ctx).format(c=chr(c)))
