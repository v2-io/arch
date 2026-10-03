#!/usr/bin/env python3
"""Gold labels for gold/D batch-2, pass 1.

Each rule maps a contiguous substring of the ORIGINAL text to exactly one
$...$ span, so the output is in-shape by construction. A rule is
(old, new) -> replace every occurrence (count asserted via n=), or
(old, new, [idx...]) -> replace only those occurrence indices (0-based).
"""
import json, re, sys, os

HERE = os.path.dirname(os.path.abspath(__file__))
D = os.path.dirname(HERE)
TASKS = os.path.join(D, "tasks", "batch-2.jsonl")
OUT = os.path.join(D, "pass1", "batch-2.jsonl")

NP = r"$n_{\text{past}}"
NF = r"$n_{\text{future}}"

# gid -> (rules, conf, note); rules: list of tuples (old, new[, n_or_idx])
L = {
"DL169": ([("Σ_t", r"$\Sigma_t$")], "high", "Heading text; strategy symbol."),
"DU149": ([("r > 0.5", r"$r\gt 0.5$")], "high", "Threshold on variable r; 15 minutes / 200% stay prose."),
"DU013": ([("M-then-G", r"$M$-then-$G$", None), ("M_t", r"$M_t$"), ("G_t", r"$G_t$")], "medium",
          "M_t/G_t certain. 'M-then-G' made $M$-then-$G$ following ASF canon's $O$-source / $N$-agent compound style; another labeler may leave the bold heading compound raw."),
"DL004": ([("β", r"$\beta$")], "high", "Sub-scope symbol."),
"DN013": ([], "high", "Already math; '= time to...' is a prose gloss."),
"DL111": ([("κ derivation", r"$\kappa$", None), ("κ ≈ 18–25×", r"$\kappa\approx 18\text{–}25\times$"), ("on κ", r"$\kappa$", None)], "low",
          "kappa certain. Range '18–25×' absorbed into the relation span (could also be $\\kappa\\approx$ 18–25× with range in prose). 'h_eq / actual_h' LEFT raw: context shows this doc uses code-style identifiers (`h_eq` in backticks, a code-block formula with actual_human_hours); a math rendering would be $h_{\\text{eq}}/h_{\\text{actual}}$, which reorders the author's actual_h. Needed context."),
"DU048": ([], "medium", "t=9 is a per-session count in a listing of session IDs (hundreds of identical lines); not math. Checked context."),
"DL107": ([("(O, Σ)", r"$(O,\Sigma)$")], "high", "Objective/strategy pair."),
"DN020": ([], "high", "Already correct."),
"DL128": ([("κ ≠ 0", r"$\kappa\neq 0$")], "high", "P2 is a prediction label, stays prose."),
"DL050": ([("(O,Σ)", r"$(O,\Sigma)$")], "medium",
          "(O,Σ)-recursion converted. W₀/W₂/W₁ LEFT as Unicode: ASF canon writes the wrapping regimes W₀/W₁/W₂ as prose labels (~190 Unicode uses vs math only inside subscripts like $\\kappa_{W_1}$). Existing $\\varepsilon^\\ast_{\\text{track}}$ kept."),
"DL106": ([], "high", "Table cell 'P → E' = rating codes (Partial→Established) in a verification table; not math. Checked context."),
"DL183": ([("κ>0", r"$\kappa\gt 0$"), ("effective η", r"$\eta$", None)], "high",
          "Raw kappa>0 and trailing eta converted; existing spans kept. Prose '≈' between words ('Confirmation bias ≈ systematically...') left as prose."),
"DU075": ([("p=0", r"$p=0$")], "medium", "p-value in results table; converted as an equation. Some would leave statistical reporting raw."),
"DL090": ([("α_ij → α_ij + 1", r"$\alpha_{ij}\to\alpha_{ij}+1$")], "high", "Update rule."),
"DN027": ([], "high", "Underscores are emphasis; no math."),
"DU137": ([("N workers", r"$N$", None), ("M requests", r"$M$", None), ("M >> N", r"$M\gg N$")], "medium",
          "Single-letter count variables; >> is \\gg. Some would leave N/M raw in a code doc."),
"DL038": ([("κ_selection", r"$\kappa_{\text{selection}}$")], "high", ""),
"DU006": ([("p_v", r"$p_v$")], "high", ""),
"DU092": ([], "low",
          "Both n_past here refer to the literal token in code comments ('CHANGE 1 (n_past=1)' is a quoted comment string; 'tracking n_past in the code comments'), so left raw. Elsewhere I convert n_past as the TST quantity; the second occurrence is arguable."),
"DL144": ([("liminf I_n > 0", r"$\liminf I_n\gt 0$"), ("liminf a_k > 0", r"$\liminf a_k\gt 0$"), ("I_{k+1} ≤ η_k I_k + a_k", r"$I_{k+1}\le\eta_k I_k+a_k$")], "high",
          "Bold **iff** stays between the two spans (prose connective, emphasis must survive). R2 is a label."),
"DN031": ([], "high", "S1″ is a specimen label."),
"DL002": ([("Σ_t", r"$\Sigma_t$")], "high", "AND/OR stays prose."),
"DU144": ([("V_O", r"$V_O$")], "high", ""),
"DU119": ([("G₂", r"$G_2$")], "medium", "In this doc G₁/G₂/G₃ are age-band sets (used in |G₃|/(|G₁|+...)), so math. Checked context."),
"DU070": ([("F²", r"$F^2$")], "high", ""),
"DL195": ([("α derivation", r"$\alpha$", None), ("α partition", r"$\alpha$", None)], "high", "Sub-scope alpha; (PI), B1 are labels; existing $M_t$ kept."),
"DL013": ([("$\\beta$ ≈ 0.7", r"$\beta\approx 0.7$")], "high", "Existing span absorbed into the relation."),
"DU017": ([], "medium",
          "H_D3 LEFT raw: it is a test/control name, and the causal-language paper source itself writes **H_D3**/H_D1 raw (8 uses, never in math). Existing $\\times$ kept."),
"DL164": ([("Δ=+0.030", r"$\Delta=+0.030$")], "medium", "Delta convert; '0.608 → 0.639' is a prose from-to arrow between numbers, left."),
"DN000": ([], "high", "Code/prose; no math."),
"DU145": ([("1.2^d", r"$1.2^d$")], "high", "20% stays prose."),
"DL000": ([("M_t", r"$M_t$"), ("Σ_t", r"$\Sigma_t$")], "high", "≡ and = here join prose labels, left."),
"DL127": ([("μ", r"$\mu$"), ("ε-canonical", r"$\varepsilon$", None)], "high", "Existing $\\Omega$ kept."),
"DL029": ([("κ=0", r"$\kappa=0$")], "high", "Inside bold; span sits within the ** markers."),
"DN035": ([], "high", "Units/ranges (16-64KB) are prose."),
"DL197": ([("If e performs", r"$e$", None), ("P-MUT(T)", r"$T$", None), ("endo b", r"$b$", None), ("that T holds", r"$T$", None),
           ("b faces", r"$b$", None), ("¬T", r"$\neg T$"), ("believed-T", r"$T$", None), ("coherent b", r"$b$", None), ("rational b", r"$b$", None)], "medium",
          "e, b, T are formal variables (exo agent, endo agent, law) in a formalization spike that is written entirely without $ (0 math spans in file). Converted as a LaTeX writer would; LAW / META / P-MUT stay prose labels. Checked context."),
"DU000": ([("n_d < 5", r"$n_d\lt 5$")], "high", ""),
"DL047": ([("α", r"$\alpha$")], "high", ""),
"DL088": ([("π_cont", r"$\pi_{\text{cont}}$")], "high", "§92/§93 stay prose."),
"DL174": ([("Ω", r"$\Omega$"), ("M_t", r"$M_t$", [0, 1, 3]), ("M_t = φ(C_t)", r"$M_t=\varphi(C_t)$")], "medium",
          "φ (U+03C6) -> \\varphi. C_t kept as written rather than house chronica $\\mathcal{C}_t$ (source is plain C)."),
"DU106": ([], "medium", "W₀⁺ is a wrapping-regime label; ASF writes these as Unicode labels, not math."),
"DL146": ([], "medium", "'Φ(fight) Research' is an organization's proper name; left as written."),
"DL011": ([("η → 0", r"$\eta\to 0$")], "high", "Both occurrences, including inside the quoted tracker text."),
"DL154": ([("α'", r"$\alpha'$")], "high", ""),
"DU089": ([("M_t", r"$M_t$"), ("G_t", r"$G_t$")], "high", ""),
"DL079": ([("ι_ij", r"$\iota_{ij}$")], "high", ""),
"DL049": ([("α = 0.2", r"$\alpha=0.2$")], "high", "8.67x stays prose."),
"DU093": ([], "medium", "o(640,5376) is a tile-origin coordinate in a run footprint, L13/f1 labels; reads as config, not math."),
"DU116": ([("p<10⁻⁶", r"$p\lt 10^{-6}$")], "medium", "Garbled diff-like text ('16 +4.', 'w +eak') left exactly; only the p-value converted. H1/E7 labels."),
"DL006": ([("δ_t = o_t − ô_t", r"$\delta_t=o_t-\hat{o}_t$")], "high", ""),
"DL046": ([("κ×A", r"$\kappa\times A$")], "high", "kappa times A (canon also writes $\\kappa \\times A$). C3 and 60/30/6/4 stay prose."),
"DU109": ([("n_past=2", NP + r"=2$")], "medium", "n_past as the TST quantity ($n_{past}$ appears ~80x in math in the TST vault)."),
"DN043": ([], "high", "Display math already correct."),
"DU055": ([("n_past=0", NP + r"=0$")], "medium", "Joseph's typed message; n_past=0 as the TST quantity. Path glob with * untouched."),
"DU085": ([("n_past", NP + "$")], "medium", "TST quantity in bold heading."),
"DL112": ([("α/T", r"$\alpha$/$T$", None)], "medium",
          "Kept as two symbols with prose slash ('relationship between alpha and T', not a ratio); $\\alpha/T$ as one span is also defensible. T kept as written (canon tempo is $\\mathcal{T}$)."),
"DU090": ([("n_past < 3", NP + r"\lt 3$")], "high", "FP-003 is a label."),
"DL092": ([("M_t", r"$M_t$"), ("Σ_t", r"$\Sigma_t$"), ("O_t", r"$O_t$")], "high", ""),
"DL089": ([("λ=0.1", r"$\lambda=0.1$")], "high", ""),
"DU068": ([("P(T) ∝ 1/T", r"$P(T)\propto 1/T$"), ("time t", r"$t$", None), ("1/T²", r"$1/T^2$"), ("T>t", r"$T\gt t$"), ("equal to t", r"$t$", None)], "high",
          "Transcript; stray »›« characters left. Single-letter t/T are the variables of the formula."),
"DL159": ([("Γ_min", r"$\Gamma_{\text{min}}$")], "high", ""),
"DL191": ([("α₁", r"$\alpha_1$"), ("α₂", r"$\alpha_2$"), ("metric-β", r"$\beta$", None)], "high", "A2'-scope is a label."),
"DU127": ([("O(n²)", r"$O(n^2)$"), ("O(n log n)", r"$O(n\log n)$")], "high", "Big-O stays prose."),
"DL138": ([("α/β", r"$\alpha$/$\beta$", None), ("α (fixed", r"$\alpha$", None), ("α₁", r"$\alpha_1$"), ("α₂", r"$\alpha_2$"), ("α′", r"$\alpha'$"),
           ("α₃", r"$\alpha_3$"), ("→ β", r"$\beta$", None), ("β outer", r"$\beta$", None), ("full α", r"$\alpha$", None)], "high",
          "All sub-scope symbols; ladder arrows → between labels kept as prose; MG-1/MG-2 labels; existing spans kept. α/β kept as two symbols (an alternation, not a ratio)."),
"DU143": ([("O_c", r"$O_c$")], "high", "induced-$O_c$ compound; SP-21, F-V3/F8 labels."),
"DU141": ([("≈ k₂n²", r"$\approx k_2n^2$", None), ("k₂ << k₁", r"$k_2\ll k_1$")], "medium",
          "Span starts at the relation, leaving the word 'time' as prose; $\\text{time}\\approx k_2n^2$ as one span is the main alternative."),
"DU139": ([], "medium", "W₂ is a wrapping-regime label (ASF writes these as Unicode labels)."),
"DL095": ([("$\\lambda$ ≈ 2.5", r"$\lambda\approx 2.5$")], "high", "Existing span absorbed into relation."),
"DL194": ([("τ", r"$\tau$")], "high", ""),
"DL016": ([], "medium", "Φ(fight) is a proper name (organization/prompt), left as written."),
"DL015": ([("δ_strategic", r"$\delta_{\text{strategic}}$"), ("δ_k = p̂_k − θ_k", r"$\delta_k=\hat{p}_k-\theta_k$")], "high", "Codex #1 is prose."),
"DL056": ([("R0 ⟸ R1 ⟸ R2", r"$\text{R0}\Longleftarrow\text{R1}\Longleftarrow\text{R2}$"), ("β-cases", r"$\beta$", None)], "low",
          "Implication chain between rung labels; converted as one statement with upright labels. Leaving it raw (⟸ renders as Unicode) is equally defensible; '(R0)' etc. later stay prose; β-cases -> $\\beta$-cases is certain."),
"DU086": ([("p_v", r"$p_v$")], "high", ""),
"DL073": ([("O_t ⊆ S", r"$O_t\subseteq S$")], "high", ""),
"DU117": ([], "medium", "H_D1 is a test name (paper writes it raw); cf_fwd/cf_rev are condition identifiers; C1/C2 route labels."),
"DL160": ([("p ≤ p_crit", r"$p\le p_{\text{crit}}$")], "high", "Span inside the *...* emphasis, which is preserved."),
"DU035": ([("p<0.0001", r"$p\lt 0.0001$")], "medium", "p-value; same policy as other stats items."),
"DU118": ([], "high", "Table cell code 'X' with footnote marker ⁸ (column of X⁸ cells, notes below); not a power. Checked context."),
"DL149": ([("λ ≈ 0.02", r"$\lambda\approx 0.02$")], "medium", "'/year' unit left in prose; $\\lambda\\approx 0.02/\\text{year}$ also fine."),
"DL043": ([("A∪B∪C", r"$A\cup B\cup C$")], "medium", "Union of named source sets A, B, C."),
"DN021": ([], "high", "Already math."),
"DU057": ([("H_t = (o_1, a_1, o_2, a_2, ..., a_{t-1}, o_t)", r"$H_t=(o_1,a_1,o_2,a_2,\ldots,a_{t-1},o_t)$")], "high", ""),
"DU101": ([("n_future = 500-1000", NF + r"=500\text{–}1000$")], "medium",
          "TST quantity; hyphen range rendered as text dash inside math so it does not read as subtraction. Alternative: convert only n_future."),
"DL087": ([("σ", r"$\sigma$"), ("Δx", r"$\Delta x$"), ("λ", r"$\lambda$")], "high", "16× and cell counts stay prose."),
"DU129": ([("p_crit", r"$p_{\text{crit}}$")], "high", ""),
"DL133": ([("n_past ≈ 2-3", NP + r"\approx 2\text{–}3$")], "medium", "TST quantity with a range; dash as text."),
"DL172": ([("ρ = -0.060", r"$\rho=-0.060$"), ("p = 0.8531", r"$p=0.8531$"), ("N = 12", r"$N=12$")], "medium",
          "Stats reporting converted; params_M left as a column identifier (could be $\\text{params}_M$)."),
"DU001": ([], "medium", "H_D3 is a test name (raw in the paper); GC2, D2 labels; ⇔ inside a quoted informal claim and → in '(hypothesis → partially proven)' are prose connectives, left."),
"DL175": ([], "medium", "⊃ between code spans in a fence-nesting table; reads as notation for nesting, renders fine as Unicode; left."),
"DU005": ([("n_future", NF + "$")], "medium", "TST quantity; FP-009 label."),
"DU023": ([("n=1", r"$n=1$")], "medium", "Count n=1 (the doc's rule 'Name beats count at n=1'). Code-design doc; leaving raw also reasonable. Checked context."),
"DL053": ([("α'''", r"$\alpha'''$"), ("ε-passive", r"$\varepsilon$", None)], "high", ""),
"DU054": ([("n_past = 2", NP + r"=2$")], "medium", "TST quantity."),
"DN036": ([], "high", "Already math."),
"DN044": ([(">7", r"$\gt 7$")], "low", "Comparison '>7 chunks'; converted per no-raw-> convention, but '>7' as prose shorthand is also reasonable to leave."),
"DU083": ([], "medium", "W₂ in the heading is a regime label (ASF convention: Unicode in prose, math only in subscripts); existing $\\kappa_{W_2}$ kept."),
"DU040": ([("M_t", r"$M_t$")], "high", ""),
"DL018": ([("T > ρ/δ_critical", r"$T\gt\rho/\delta_{\text{critical}}$")], "high", ""),
"DL122": ([("ρ", r"$\rho$")], "high", "#3, Tier 2, (AV) are prose."),
"DN045": ([], "high", "Already math."),
"DL076": ([("2× Δt", r"$2\times\Delta t$"), ("Equal-Δt", r"$\Delta t$", None), ("unequal-Δt", r"$\Delta t$", None)], "medium",
          "'2× Δt' read as twice the timestep (one quantity). Code `n = 1`, f32/f64, ≈ 0.95 m and % figures left as prose."),
"DL126": ([("η*", r"$\eta^\ast$"), ("U_o", r"$U_o$")], "medium", "'> optimal' left in prose (comparison against a word); $\\eta^\\ast\\gt$ optimal is an alternative."),
"DU072": ([], "medium", "W₀⁺ is a regime label; §-refs, S4/S5a labels. No change."),
"DU071": ([("n_future = 500-1000", NF + r"=500\text{–}1000$")], "medium", "Same as the other n_future range item."),
"DU097": ([], "high", "(ii)/(iii) are item labels; existing $+$ kept."),
"DU122": ([("S₂", r"$S_2$")], "medium", "S₀/S₂ are strategy-rule classes in a math outline table (siblings: k≥2, Tr(I_min · I_o)); treated as symbols. Checked context."),
"DL114": ([("μ", r"$\mu$")], "high", "Ratios/MAE figures stay prose."),
"DL157": ([("ε* ν_c", r"$\varepsilon^\ast\nu_c$")], "high", "A product rate×defect per the table. Checked context."),
"DL061": ([("$\\alpha_i$ ≈ 0.3", r"$\alpha_i\approx 0.3$")], "high", "Existing span absorbed into relation; '= repository...' is gloss."),
"DL180": ([("ρ = 0.648", r"$\rho=0.648$")], "high", "M10/M15/v1.5 labels; percentages prose."),
"DU061": ([("o_t", r"$o_t$"), ("(i, j)", r"$(i,j)$")], "high", ""),
"DL072": ([("b = 2", r"$b=2$"), ("b = 3/2", r"$b=3/2$"), ("b → 1", r"$b\to 1$")], "high", "Slashes between regimes stay prose."),
"DU011": ([], "medium", "W₀/W₂/W₁ are regime labels (ASF canon writes them in Unicode)."),
"DN014": ([], "high", "Already math."),
"DL153": ([("ν", r"$\nu$"), ("U_o", r"$U_o$")], "high", "Converted separately within the tuple; 'coverage' is a word."),
"DL028": ([("dλ/dt = -∂H/∂x", r"$d\lambda/dt=-\partial H/\partial x$")], "high", ""),
"DN042": ([], "high", "Already math."),
"DL161": ([("α'/β'", r"$\alpha'$/$\beta'$", None), ("β' macro", r"$\beta'$", None), ("γ'", r"$\gamma'$")], "high",
          "α'/β' occurs twice (both converted); line-89 is prose."),
"DL199": ([("α₁", r"$\alpha_1$")], "high", "DA2'-inc is a label."),
"DL170": ([("ρ", r"$\rho$")], "high", ""),
"DN028": ([], "high", "Separator line."),
"DL118": ([("ν_O", r"$\nu_O$"), ("ν_Σ", r"$\nu_\Sigma$"), ("ν_M", r"$\nu_M$")], "high", ""),
"DU079": ([("n=10", r"$n=10$")], "medium", "Sample size; could be left raw in a file ledger."),
"DL179": ([("Σ-source", r"$\Sigma$", None), ("M→Σ→f_M→M", r"$M\to\Sigma\to f_M\to M$"), ("contraction in M", r"$M$", None),
           ("O-source", r"$O$", None), ("(O is", r"$O$", None)], "high",
          "Canon writes $O$-source. 'stage × source × form' is a prose product of words, left."),
"DL167": ([("Σ_t", r"$\Sigma_t$")], "high", ""),
"DN018": ([], "high", "Already math."),
"DL005": ([("T ≤ Σ_k ν^(k)·η^(k)*", r"$T\le\sum_k\nu^{(k)}\cdot\eta^{(k)\ast}$")], "high",
          "Σ_k is a summation; η^(k)* = optimal gain of channel k."),
"DU146": ([], "medium",
          "Eq-tag name slot 'H_b-decomposition' left as-is: canon keeps eq-tag names math-free (cf. *[Derived (Hb-variance-decomposition; ...)]*) and puts $ only in the condition text. $H_b$-decomposition is the alternative."),
"DU015": ([], "medium", "Units (m/s, kg/m³, kg/(m²·s)) in a code-design note; not converted to math."),
"DL190": ([("V_T", r"$V_T$"), ("π*", r"$\pi^\ast$")], "high",
          "The middle * is pi-star, not emphasis: the same file writes 'deterministic-$\\pi^*$'. Converting it also repairs the *provided ... preserved* emphasis. Code span untouched. Checked context."),
"DL071": ([("ε*", r"$\varepsilon^\ast$")], "high", ""),
"DU065": ([], "medium", "H_D1, E10, H4 are test labels."),
}


def find_all(s, sub):
    out, i = [], s.find(sub)
    while i >= 0:
        out.append(i)
        i = s.find(sub, i + 1)
    return out


def apply(text, rules):
    edits = []
    for r in rules:
        old, new = r[0], r[1]
        sel = r[2] if len(r) > 2 else "all"
        occ = find_all(text, old)
        if not occ:
            raise ValueError(f"not found: {old!r}")
        if sel is None:  # anchor-in-context: old contains context; the symbol is the
            # part that `new` replaces; derive the symbol substring heuristically
            raise ValueError("None selector must be handled by caller")
        idxs = range(len(occ)) if sel == "all" else sel
        for k in idxs:
            edits.append((occ[k], occ[k] + len(old), new))
    edits.sort()
    for a, b in zip(edits, edits[1:]):
        if a[1] > b[0]:
            raise ValueError(f"overlap {a} {b}")
    out, pos = [], 0
    for s, e, n in edits:
        out.append(text[pos:s]); out.append(n); pos = e
    out.append(text[pos:])
    return "".join(out)


# Rules with selector None carry context in `old`: the symbol to replace is the
# substring of `old` that is NOT a plain word/space context. Expand them here
# into exact-position edits by naming the symbol explicitly.
SYMBOL_IN_CONTEXT = {
    "M-then-G": [("M", 0), ("G", 7)],
    "κ derivation": [("κ", 0)], "on κ": [("κ", 3)],
    "α derivation": [("α", 0)], "α partition": [("α", 0)],
    "If e performs": [("e", 3)], "P-MUT(T)": [("T", 6)], "endo b": [("b", 5)],
    "that T holds": [("T", 5)], "b faces": [("b", 0)], "believed-T": [("T", 9)],
    "coherent b": [("b", 9)], "rational b": [("b", 9)],
    "ε-canonical": [("ε", 0)], "effective η": [("η", 10)],
    "N workers": [("N", 0)], "M requests": [("M", 0)],
    "α/T": [("α", 0), ("T", 2)], "α/β": [("α", 0), ("β", 2)],
    "α (fixed": [("α", 0)], "→ β": [("β", 2)], "β outer": [("β", 0)], "full α": [("α", 5)],
    "≈ k₂n²": [("≈ k₂n²", 0)],
    "metric-β": [("β", 7)], "time t": [("t", 5)], "equal to t": [("t", 9)],
    "Equal-Δt": [("Δt", 6)], "unequal-Δt": [("Δt", 8)],
    "α'/β'": [("α'", 0), ("β'", 3)], "β' macro": [("β'", 0)],
    "ε-passive": [("ε", 0)], "Σ-source": [("Σ", 0)], "contraction in M": [("M", 15)],
    "O-source": [("O", 0)], "(O is": [("O", 1)], "β-cases": [("β", 0)],
}


def expand(rules):
    out = []
    for r in rules:
        if len(r) > 2 and r[2] is None:
            old, new = r[0], r[1]
            syms = SYMBOL_IN_CONTEXT[old]
            news = re.findall(r"\$[^$]+\$", new)
            assert len(news) == len(syms), (old, new)
            out.append(("CTX", old, list(zip(syms, news))))
        else:
            out.append(r)
    return out


def apply_all(text, rules):
    edits = []
    for r in expand(rules):
        if r[0] == "CTX":
            _, ctx, pairs = r
            occ = find_all(text, ctx)
            if not occ:
                raise ValueError(f"ctx not found: {ctx!r}")
            for o in occ:
                for (sym, off), new in pairs:
                    assert ctx[off:off + len(sym)] == sym, (ctx, sym, off)
                    edits.append((o + off, o + off + len(sym), new))
        else:
            old, new = r[0], r[1]
            sel = r[2] if len(r) > 2 else "all"
            occ = find_all(text, old)
            if not occ:
                raise ValueError(f"not found: {old!r}")
            idxs = range(len(occ)) if sel == "all" else sel
            for k in idxs:
                edits.append((occ[k], occ[k] + len(old), new))
    edits = sorted(set(edits))
    for a, b in zip(edits, edits[1:]):
        if a[1] > b[0]:
            raise ValueError(f"overlap {a} {b} in {text[:60]!r}")
    for s, e, n in edits:
        assert re.fullmatch(r"\$[^$\s][^$]*[^$\s]\$|\$[^$\s]\$", n), f"bad span {n!r}"
        inner = n[1:-1]
        assert not re.search(r"[<>*|]", inner), f"raw char in {n!r}"
        seg = text[s:e]
        # old region must contain only whole existing spans (or none)
        assert seg.count("$") % 2 == 0, f"partial span in {seg!r}"
    out, pos = [], 0
    for s, e, n in edits:
        out.append(text[pos:s]); out.append(n); pos = e
    out.append(text[pos:])
    return "".join(out), len(edits)


def main():
    items = [json.loads(l) for l in open(TASKS)]
    missing = [d["gid"] for d in items if d["gid"] not in L]
    extra = set(L) - {d["gid"] for d in items}
    assert not missing and not extra, (missing, extra)
    rows = []
    for d in items:
        rules, conf, note = L[d["gid"]]
        gold, n = apply_all(d["text"], rules)
        rows.append({"gid": d["gid"], "gold": gold, "conf": conf, "note": note})
        if "-v" in sys.argv and gold != d["text"]:
            print(d["gid"], n, "|", gold[:400])
    with open(OUT, "w") as f:
        for r in rows:
            f.write(json.dumps(r, ensure_ascii=False) + "\n")
    from collections import Counter
    print(len(rows), "rows;", Counter(r["conf"] for r in rows),
          "changed:", sum(r["gold"] != d["text"] for r, d in zip(rows, items)))


if __name__ == "__main__":
    main()
