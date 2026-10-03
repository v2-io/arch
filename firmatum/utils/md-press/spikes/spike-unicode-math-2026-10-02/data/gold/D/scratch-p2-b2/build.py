#!/usr/bin/env python3
"""Build pass2/batch-2 gold from explicit (old -> new) replacements, then verify shape.

Each entry: gid -> (replacements, conf, note). A replacement is (old, new) applied to every
occurrence, or (old, new, n) applied to the n-th occurrence only (1-based).
`new` must be a $...$ span (or a short prefix/suffix kept verbatim around one, checked below).
"""
import json, re, sys
from pathlib import Path

D = Path(__file__).resolve().parent.parent
SRC = D / "tasks" / "batch-2.jsonl"
OUT = D / "pass2" / "batch-2.jsonl"

items = [json.loads(l) for l in SRC.open()]
by = {d["gid"]: d for d in items}

def t(s):  # \text{} helper for readability
    return r"\text{" + s + "}"

NP = r"n_{\text{past}}"
NF = r"n_{\text{future}}"

G = {
"DL169": ([("Σ_t", r"$\Sigma_t$")], "high", "Heading with symbol."),
"DU149": ([("r > 0.5", r"$r \gt 0.5$")], "high", "Variable comparison; 200% left as prose."),
"DU013": ([("M_t", r"$M_t$"), ("G_t", r"$G_t$")], "high", "M-then-G compound in heading left as prose (letters as words)."),
"DL004": ([("β", r"$\beta$")], "high", "Sub-scope label; canon writes sub-scope $\\beta$."),
"DN013": ([], "high", "Already converted; '= time to...' is prose."),
"DL111": ([("the κ derivation", r"the $\kappa$ derivation"), ("κ ≈ 18–25×", r"$\kappa \approx 18\text{–}25\times$"), ("bounds on κ", r"bounds on $\kappa$")], "low",
          "h_eq / actual_h left alone: the same file writes `h_eq` as a code identifier (heading 'scope anchor `h_eq`'; actual_human_hours in a code block). The '18–25×' span boundary is arbitrary ($\\kappa \\approx 18$–25× is equally defensible). ±20%, 1.4× left as prose. Needed context."),
"DU048": ([], "high", "t=9 is session-listing metadata (turn count) in a log line, not math. Checked context."),
"DL107": ([("(O, Σ)", r"$(O, \Sigma)$")], "high", ""),
"DN020": ([], "high", "Already math."),
"DL128": ([("κ ≠ 0", r"$\kappa \neq 0$")], "high", "P2 is a prediction label."),
"DL050": ([("W₀/W₂/W₁", r"$W_0/W_2/W_1$"), ("(O,Σ)", r"$(O,\Sigma)$")], "medium",
          "W-regime labels promoted (asf canon mostly keeps W₀/W₂ as Unicode labels in prose but writes $\\kappa_{W_2}$ in math; I judge them notation). One span vs $W_0$/$W_2$/$W_1$ is a coin flip. (A1)–(A4), file:line refs left."),
"DL106": ([], "high", "Rating-code table cell (P → E grades). Checked context."),
"DL183": ([("κ>0", r"$\kappa\gt 0$"), ("effective η", r"effective $\eta$")], "medium",
          "'Confirmation bias ≈ **...**' is prose-role ≈ between phrases; left. Existing spans kept."),
"DU075": ([("p=0", r"$p=0$")], "medium", "p-value in a results cell; promoting bare-ASCII stats is a judgment call."),
"DL090": ([("α_ij → α_ij + 1", r"$\alpha_{ij} \to \alpha_{ij} + 1$")], "high", "Update rule; arrow is math-role."),
"DN027": ([], "high", "No math; underscores are emphasis."),
"DU137": ([("N workers", r"$N$ workers"), ("M requests", r"$M$ requests"), ("M >> N", r"$M \gg N$")], "medium",
          "ASCII single-letter variables; a LaTeX writer would italicize all three. Code-doc context, so some would leave it."),
"DL038": ([("κ_selection", r"$\kappa_{\text{selection}}$")], "high", ""),
"DU006": ([("p_v", r"$p_v$")], "high", ""),
"DU092": ([("tracking n_past in", r"tracking $" + NP + "$ in")], "low",
          "Quoted code comment \"CHANGE 1 (n_past=1)\" left verbatim. Second n_past names the TST quantity being tracked, but it could be read as the literal code token; transcript."),
"DL144": ([("liminf I_n > 0", r"$\liminf I_n \gt 0$"), ("liminf a_k > 0", r"$\liminf a_k \gt 0$"), ("I_{k+1} ≤ η_k I_k + a_k", r"$I_{k+1} \le \eta_k I_k + a_k$")], "high",
          "Two spans around the bold **iff** to keep the emphasis outside math."),
"DN031": ([], "high", "S1″ is a specimen label, not math."),
"DL002": ([("Σ_t", r"$\Sigma_t$")], "high", ""),
"DU144": ([("V_O", r"$V_O$")], "high", ""),
"DU119": ([("G₂", r"$G_2$")], "medium", "G₂ is an age-group symbol (G₁/G₂/G₃ defined with 30 days < A < 1 year in same file). Checked context."),
"DU070": ([("F²", r"$F^2$")], "high", "F = change frequency; file has TD = C × F². Checked context."),
"DL195": ([("sub-scope α derivation", r"sub-scope $\alpha$ derivation"), ("sub-scope α partition", r"sub-scope $\alpha$ partition")], "high", "(PI), B1 are labels."),
"DL013": ([("$\\beta$ ≈ 0.7", r"$\beta \approx 0.7$")], "high", "Existing span absorbed into the wider equation."),
"DU017": ([], "medium", "H_D3 is a hypothesis label; the causal-language paper source writes **H_D3** as a label throughout (checked). Existing $\\times$ kept."),
"DL164": ([("Δ=+0.030", r"$\Delta=+0.030$")], "medium", "large→m3 and 0.608 → 0.639 are prose-role arrows (model names / before→after numbers); left."),
"DN000": ([], "high", "No math."),
"DU145": ([("1.2^d", r"$1.2^d$")], "high", "20% left as prose."),
"DL000": ([("M_t/Σ_t", r"$M_t/\Sigma_t$")], "medium", "≡ and → here join prose/ledger names; left as prose-role."),
"DL127": ([("stochastic-μ", r"stochastic-$\mu$"), ("ε-canonical", r"$\varepsilon$-canonical")], "high", "Existing $\\Omega$ kept."),
"DL029": ([("κ=0", r"$\kappa=0$")], "high", "Inside bold; span sits within the ** **."),
"DN035": ([], "high", "16-64KB is a unit range, not math."),
"DL197": ([("If e performs", r"If $e$ performs"), ("endo b holding", r"endo $b$ holding"), ("prior that T holds", r"prior that $T$ holds"),
           ("b faces", r"$b$ faces"), ("observed-¬T", r"observed-$\neg T$"), ("believed-T", r"believed-$T$"), ("A coherent b's", r"A coherent $b$'s"),
           ("\"rational b\"", r'"rational $b$"')], "low",
          "¬T is the only Unicode trigger, but e/b/T are single-letter variables throughout; promoted all standalone ones for consistency. P-MUT(T), P-MUT(LAW) left as operator labels. Minimal ($\\neg T$ only) is also defensible."),
"DU000": ([("n_d < 5", r"$n_d \lt 5$")], "high", ""),
"DL047": ([("α factor", r"$\alpha$ factor")], "high", ""),
"DL088": ([("π_cont", r"$\pi_{\text{cont}}$")], "high", "§92/§93, b02-3.5 are refs."),
"DL174": ([("define Ω as", r"define $\Omega$ as"), ("M_t = φ(C_t)", r"$M_t = \varphi(C_t)$"), ("M_t", r"$M_t$")], "high",
          "φ (U+03C6) → \\varphi per glyph; C_t kept plain C as written (not \\mathcal)."),
"DU106": ([("W₀⁺", r"$W_0^+$")], "medium", "Regime label W₀⁺; promoted as notation (see W-label note)."),
"DL146": ([], "high", "Φ(fight) Research is an organization's name."),
"DL011": ([("η → 0", r"$\eta \to 0$")], "high", "Both occurrences, including inside the quoted tracker row."),
"DL154": ([("α'", r"$\alpha'$")], "high", ""),
"DU089": ([("M_t", r"$M_t$"), ("G_t", r"$G_t$")], "high", ""),
"DL079": ([("ι_ij", r"$\iota_{ij}$")], "high", ""),
"DL049": ([("α = 0.2", r"$\alpha = 0.2$")], "high", "8.67x left as prose."),
"DU093": ([], "high", "Run footprint metadata; o(640,5376) is an origin coordinate tag, not math."),
"DU116": ([], "low",
          "Source is a raw Claude Code terminal diff capture (lines with +/- gutters) that md-press unwrapping mangled ('Strong/w +eak'). p<10⁻⁶ would be $p \\lt 10^{-6}$ in real prose, but promoting math inside a verbatim terminal dump seems wrong; this file likely wants a .md-pressignore. Checked context."),
"DL006": ([("δ_t = o_t − ô_t", r"$\delta_t = o_t - \hat{o}_t$")], "high", "140 · is an index number."),
"DL046": ([("κ×A", r"$\kappa \times A$")], "high", "C3 label; 60/30/6/4 is a ratio label."),
"DU109": ([("n_past=2", "$" + NP + "=2$")], "medium", "TST variable; TST canon writes $n_{\\text{past}}$ in math. Transcript."),
"DN043": ([], "high", "Already display math."),
"DU055": ([("n_past=0", "$" + NP + "=0$")], "low",
          "Joseph's typed message in a transcript; n_past=0 names the TST quantity. Path with {…} and * is not math. Whether transcripts should be touched at all is a policy question above the converter."),
"DU085": ([("n_past", "$" + NP + "$")], "medium", "TST variable inside bold; transcript."),
"DL112": ([("α/T", r"$\alpha/T$")], "high", "Kept T as written (not \\mathcal{T})."),
"DU090": ([("n_past < 3", "$" + NP + r" \lt 3$")], "medium", "TST variable."),
"DL092": ([("M_t", r"$M_t$"), ("Σ_t", r"$\Sigma_t$"), ("O_t", r"$O_t$")], "high", ""),
"DL089": ([("λ=0.1", r"$\lambda=0.1$")], "high", ""),
"DU068": ([("P(T) ∝ 1/T", r"$P(T) \propto 1/T$"), ("time t,", r"time $t$,"), ("1/T² for T>t", r"$1/T^2$ for $T\gt t$"), ("equal to t.", r"equal to $t$.")], "medium",
          "ASCII t/T promoted alongside the Unicode-triggered parts. Transcript text with ›* « markers left alone."),
"DL159": ([("Γ_min", r"$\Gamma_{\min}$")], "high", ""),
"DL191": ([("metric-α₁", r"metric-$\alpha_1$"), ("metric-α₂", r"metric-$\alpha_2$"), ("metric-β", r"metric-$\beta$")], "high", "Canon writes metric-$\\alpha_2$ (prefix outside)."),
"DU127": ([("O(n²)", r"$O(n^2)$"), ("O(n log n)", r"$O(n \log n)$")], "high", "Big-O left as prose."),
"DL138": ([("than α/β", r"than $\alpha/\beta$"), ("α (fixed-gain", r"$\alpha$ (fixed-gain"), ("α₁/α₂ (fixed", r"$\alpha_1/\alpha_2$ (fixed"),
           ("α′ (controlled", r"$\alpha'$ (controlled"), ("α₃ (Fisher", r"$\alpha_3$ (Fisher"), ("→ β.", r"→ $\beta$."),
           ("Kalman α₂", r"Kalman $\alpha_2$"), ("α₁ inner / β outer", r"$\alpha_1$ inner / $\beta$ outer"), ("full α (", r"full $\alpha$ (")], "medium",
          "Ladder arrows → between labels+glosses kept as prose. α′ (U+2032) → \\alpha'. MG-1/MG-2 labels. Existing spans kept."),
"DU143": ([("induced-O_c", r"induced-$O_c$")], "high", "F-V3/F8, SP-21, TODO:95 are refs."),
"DU141": ([("time ≈ k₂n²", r"$\text{time} \approx k_2 n^2$"), ("k₂ << k₁", r"$k_2 \ll k_1$")], "medium",
          "Span start is the judgment: 'time' could stay outside (time $\\approx k_2 n^2$)."),
"DU139": ([("W₂", r"$W_2$")], "medium", "W-regime label (see DL050 note)."),
"DL095": ([("$\\lambda$ ≈ 2.5", r"$\lambda \approx 2.5$")], "high", "Absorb existing span into the equation."),
"DL194": ([("τ", r"$\tau$")], "high", ""),
"DL016": ([], "high", "Φ(fight) is the prompt/org name; code path left."),
"DL015": ([("δ_strategic", r"$\delta_{\text{strategic}}$"), (None, r"$\delta_k = \hat{p}_k - \theta_k$")], "high", "p̂ uses a combining circumflex in source."),
"DL056": ([("β-cases", r"$\beta$-cases")], "low",
          "R0 ⟸ R1 ⟸ R2: arrows between premise labels. I left them as prose-role glyphs (labels aren't math objects); a LaTeX writer could equally write R0 $\\Longleftarrow$ R1 $\\Longleftarrow$ R2. C1, (L2), §7.3 are labels."),
"DU086": ([("p_v", r"$p_v$")], "high", ""),
"DL073": ([("O_t ⊆ S", r"$O_t \subseteq S$")], "high", ""),
"DU117": ([], "medium", "H_D1 hypothesis label (paper convention, checked); cf_fwd/cf_rev are condition identifiers; 0.91 is prose."),
"DL160": ([("p ≤ p_crit", r"$p \le p_{\text{crit}}$")], "high", "Span inside the *…* italics."),
"DU035": ([("p<0.0001", r"$p\lt 0.0001$")], "medium", "p-value; bare-ASCII stat."),
"DU118": ([], "high", "X⁸ is a rating code X with footnote marker ⁸ in a table (cell notes follow). Checked context."),
"DL149": ([("λ ≈ 0.02", r"$\lambda \approx 0.02$")], "medium", "'/year' kept outside as prose unit; $\\lambda \\approx 0.02/\\text{year}$ also fine."),
"DL043": ([("A∪B∪C", r"$A \cup B \cup C$")], "medium", "A/B/C are source-set labels, but the union makes them set operands."),
"DN021": ([], "high", "Already math."),
"DU057": ([("H_t = (o_1, a_1, o_2, a_2, ..., a_{t-1}, o_t)", r"$H_t = (o_1, a_1, o_2, a_2, \ldots, a_{t-1}, o_t)$")], "high", "... → \\ldots inside span."),
"DU101": ([("n_future = 500-1000", "$" + NF + r" = 500\text{-}1000$")], "medium", "Hyphen is a range, not minus, so kept as \\text{-}; transcript."),
"DL087": ([("(σ scatters", r"($\sigma$ scatters"), ("every Δx, λ in", r"every $\Delta x$, $\lambda$ in"), ("tracking Δx down", r"tracking $\Delta x$ down")], "high", "16× left as prose."),
"DU129": ([("p_crit", r"$p_{\text{crit}}$")], "high", ""),
"DL133": ([("n_past ≈ 2-3", "$" + NP + r" \approx 2\text{-}3$")], "medium", "Range hyphen kept as \\text{-}; transcript."),
"DL172": ([("ρ = -0.060", r"$\rho = -0.060$"), ("p = 0.8531", r"$p = 0.8531$"), ("N = 12", r"$N = 12$")], "medium", "params_M is a column identifier; shuf/orig prose."),
"DU001": ([], "medium", "H_D3/GC2 are labels (paper convention); ⇔ joins quoted prose claims, left as prose-role. $\\Leftrightarrow$ on the arrow alone is the other reading."),
"DL175": ([], "medium", "⊃ expresses nesting between code spans in a fence-behavior table (file has no math at all); prose-role glyph. Checked context."),
"DU005": ([("n_future", "$" + NF + "$")], "medium", "TST variable."),
"DU023": ([("n=1", r"$n=1$")], "low", "n = number of dirs in aspectus census design notes ('Name beats count at n=1'); bare-ASCII, many would leave it. Checked context."),
"DL053": ([("α'''", r"$\alpha'''$"), ("ε-passive", r"$\varepsilon$-passive")], "high", "Inside bold."),
"DU054": ([("n_past = 2", "$" + NP + " = 2$")], "medium", "TST variable; transcript."),
"DN036": ([], "high", "Already math."),
"DN044": ([], "medium", "'>7 chunks' is prose 'more than seven'; left."),
"DU083": ([("for W₂.", r"for $W_2$."), ], "medium", "Same line writes $\\kappa_{W_2}$ in math, supporting $W_2$ for the label."),
"DU040": ([("M_t", r"$M_t$")], "high", ""),
"DL018": ([("T > ρ/δ_critical", r"$T \gt \rho/\delta_{\text{critical}}$")], "high", ""),
"DL122": ([("ρ-factorization", r"$\rho$-factorization")], "high", "#3, (AV) labels."),
"DN045": ([], "high", "Already math."),
"DL076": ([("2× Δt", r"2× $\Delta t$"), ("Equal-Δt", r"Equal-$\Delta t$"), ("unequal-Δt", r"unequal-$\Delta t$")], "medium",
          "'max diff ≈ 0.95 m, ≈ 0.16 %' left as prose approx with units (prose LHS); `n = 1` is code. 2× left as prose."),
"DL126": ([("η* > optimal", r"$\eta^\ast \gt \text{optimal}$"), ("U_o", r"$U_o$")], "low",
          "'> optimal' compares to a word; $\\eta^\\ast$ > optimal (operator outside) is equally defensible."),
"DU072": ([("W₀⁺", r"$W_0^+$")], "medium", "W-regime label."),
"DU071": ([("n_future = 500-1000", "$" + NF + r" = 500\text{-}1000$")], "medium", "As DU101."),
"DU097": ([], "high", "Existing $+$ between roman-numeral labels kept; nothing else is math."),
"DU122": ([("S₂", r"$S_2$")], "medium", "S₂ is a state-dependent quantity in the NeurIPS outline; context thin."),
"DL114": ([("μ measured", r"$\mu$ measured")], "high", "2×4, popcount/8, ranges left as prose."),
"DL157": ([("ε* ν_c", r"$\varepsilon^\ast \nu_c$")], "high", "Table cell: a product rate×defect (row comment says so). Checked context."),
"DL061": ([("$\\alpha_i$ ≈ 0.3", r"$\alpha_i \approx 0.3$")], "high", "Absorb existing span; '= repository...' is prose definition."),
"DL180": ([("ρ = 0.648", r"$\rho = 0.648$")], "high", "M10/M15 labels."),
"DU061": ([("o_t", r"$o_t$"), ("(i, j)", r"$(i, j)$")], "high", ""),
"DL072": ([("b = 2 / b = 3/2 / b → 1", r"$b = 2$ / $b = 3/2$ / $b \to 1$")], "high", "Path in code span left."),
"DU011": ([("W₀ / W₂ / W₁", r"$W_0$ / $W_2$ / $W_1$"), ("W₁ wrapping", r"$W_1$ wrapping"), ("(W₂ wrapping)", r"($W_2$ wrapping)")], "medium", "W-regime labels."),
"DN014": ([], "high", "Already math."),
"DL153": ([("(ν, U_o, coverage)", r"($\nu$, $U_o$, coverage)")], "medium", "Tuple mixes symbols and a word; separate spans."),
"DL028": ([("dλ/dt = -∂H/∂x", r"$d\lambda/dt = -\partial H/\partial x$")], "high", ""),
"DN042": ([], "high", "Already math."),
"DL161": ([("α'/β'", r"$\alpha'/\beta'$"), ("β' macro", r"$\beta'$ macro"), ("γ'-sub-scope", r"$\gamma'$-sub-scope")], "high", "Canon writes sub-scope $\\alpha'/\\beta'$."),
"DL199": ([("metric-α₁", r"metric-$\alpha_1$")], "high", "DA2' label."),
"DL170": ([("rate ρ", r"rate $\rho$")], "high", ""),
"DN028": ([], "high", "Rule line."),
"DL118": ([("ν_O", r"$\nu_O$"), ("ν_Σ", r"$\nu_\Sigma$"), ("ν_M", r"$\nu_M$")], "high", ""),
"DU079": ([("n=10", r"$n=10$")], "medium", "Sample size in a ledger; bare-ASCII stat."),
"DL179": ([("pure Σ-source", r"pure $\Sigma$-source"), ("M→Σ→f_M→M", r"$M \to \Sigma \to f_M \to M$"), ("contraction in M)", r"contraction in $M$)"),
           ("pure O-source", r"pure $O$-source"), ("(O is exogenous", r"($O$ is exogenous")], "medium",
          "stage × source × form is prose-role ×. Promoted ASCII M/O where they are the model/objective symbols."),
"DL167": ([("Σ_t", r"$\Sigma_t$")], "high", ""),
"DN018": ([], "high", "Already math."),
"DL005": ([("T ≤ Σ_k ν^(k)·η^(k)*", r"$T \le \sum_k \nu^{(k)} \cdot \eta^{(k)\ast}$")], "high", "Σ_k is a summation here, so \\sum_k."),
"DU146": ([("H_b-decomposition", r"$H_b$-decomposition")], "medium", "Inside an eq-tag *[Derived (name, from #slug)]*; spike writes $H_b$. Unsure whether eq-tag tooling tolerates math in the name slot."),
"DU015": ([], "medium", "m/s, kg/m³, kg/(m²·s) are units in prose; Unicode renders fine and math would need \\mathrm; left."),
"DL190": ([("V_T extension", r"$V_T$ extension"), ("π*", r"$\pi^\ast$")], "medium",
          "π* is the optimal policy (file uses π*_t, V^{π*}). Taking the * into the span also repairs the emphasis, which as written closes after 'π' and leaves a stray * at the end. Formula in code span left."),
"DL071": ([("ε*", r"$\varepsilon^\ast$")], "high", "'=' is prose definition."),
"DU065": ([], "medium", "H_D1, E10 H4 are labels (paper convention)."),
}

def apply(text, reps, gid):
    for r in reps:
        if r[0] is None:  # DL015 combining-char case: find δ_k = p̂_k − θ_k by regex
            m = re.search(r"δ_k = p̂?\S*_k − θ_k", text) or re.search(r"δ_k = .{1,4}_k − θ_k", text)
            assert m, (gid, "regex")
            text = text[:m.start()] + r[1] + text[m.end():]
            continue
        old, new = r[0], r[1]
        parts = re.split(r"(\$[^$]+\$)", text)
        hit = False
        for i in range(0, len(parts), 2):  # only outside existing/new spans
            if old in parts[i]:
                hit = True
                parts[i] = parts[i].replace(old, new)
        if not hit and old in text and old.startswith("$"):  # absorbing an existing span
            text = text.replace(old, new); continue
        assert hit, (gid, old)
        text = "".join(parts)
    return text

def shape_ok(src, gold):
    # Non-span literals of gold must appear in order in src with gaps = replaced regions.
    parts = re.split(r"(\$[^$]+\$)", gold)
    lits = parts[0::2]
    rx = "^" + "(.+?)".join(re.escape(l) for l in lits) + "$"
    return re.match(rx, src, re.S) is not None

out = []
missing = [d["gid"] for d in items if d["gid"] not in G]
extra = [g for g in G if g not in by]
assert not missing and not extra, (missing, extra)
for d in items:
    reps, conf, note = G[d["gid"]]
    gold = apply(d["text"], reps, d["gid"])
    assert shape_ok(d["text"], gold), d["gid"]
    for span in re.findall(r"\$[^$]+\$", gold):
        if span in d["text"]:
            continue
        body = span[1:-1]
        assert body == body.strip(), (d["gid"], span)
        assert not re.search(r"[<>*|]", body), (d["gid"], span)
        assert not re.search(r"[^\x00-\x7f]", body.replace("–", "")), (d["gid"], span)  # en-dash only allowed in \text{}
    out.append({"gid": d["gid"], "gold": gold, "conf": conf, "note": note})

OUT.parent.mkdir(exist_ok=True)
with OUT.open("w") as f:
    for o in out:
        f.write(json.dumps(o, ensure_ascii=False) + "\n")
print(len(out), "written;", sum(o["gold"] != by[o["gid"]]["text"] for o in out), "changed")
if "-v" in sys.argv:
    for o in out:
        if o["gold"] != by[o["gid"]]["text"]:
            print(o["gid"], o["conf"], "|", o["gold"][:300])
