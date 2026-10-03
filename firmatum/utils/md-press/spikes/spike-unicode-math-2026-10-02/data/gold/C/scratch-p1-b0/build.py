#!/usr/bin/env python3
"""Build pass-1 gold for batch-0 from ordered (old -> new) replacements.

Each item is: gid -> (reps, conf, note). reps is a list of (old, new) pairs
applied left-to-right: each `old` is searched for starting where the previous
replacement ended, so everything outside the replaced regions is byte-identical
to the input by construction. `new` must be a single $...$ span (or several
spans separated only by characters copied verbatim from `old` -- checked).
"""
import json, re, sys, os

HERE = os.path.dirname(os.path.abspath(__file__))
TASKS = os.path.join(HERE, '..', 'tasks', 'batch-0.jsonl')
OUT = os.path.join(HERE, '..', 'pass1', 'batch-0.jsonl')

H = '̂'  # combining circumflex
NP = r'$n_{\text{past}}$'
NF = r'$n_{\text{future}}$'

G = {}
def g(gid, reps, conf, note=''):
    assert gid not in G, gid
    G[gid] = (reps, conf, note)

# ---------------------------------------------------------------- items
g('CL083', [('G_t', '$G_t$'), ('Σ_t', r'$\Sigma_t$')], 'high',
  '"strategy ≠ intention" left as prose: ≠ between quoted English words is a prose shorthand, not math.')
g('CL042', [('R = P_K·H_K·U_K + P_P·H_P·U_P + P_C·H_C·U_C',
             r'$R = P_K \cdot H_K \cdot U_K + P_P \cdot H_P \cdot U_P + P_C \cdot H_C \cdot U_C$')], 'high', '')
g('CP125', [('B_O > B_Σ > B_M', r'$B_O \gt B_\Sigma \gt B_M$')], 'high', 'Span inside the parens; parens are prose.')
g('CU020', [('K=1', '$K=1$')], 'medium',
  'K is an iteration count parameter; wrapping is what a LaTeX writer would do, but leaving "K=1" plain is defensible. E6/E9/rank-1 are labels.')
g('CU015', [('n' + H + '_future', r'$\hat{n}_{\text{future}}$')], 'high',
  'Only the symbol: "= acknowledging ..." is a prose gloss, not an equation.')
g('CP074', [('P-MUT(T)', 'P-MUT($T$)'), (' e ', ' $e$ '), ('h-signal', '$h$-signal'), ('into b ', 'into $b$ '),
            ('with T ', 'with $T$ '), ('while T ', 'while $T$ '), ('Φ(K,b)', r'$\Phi(K,b)$'), ('Ω/T', r'$\Omega/T$')], 'low',
  'Context read: e, h, b, T, K are the spike\'s formal objects (agent e, signal h, kingdom K...), so single letters wrapped. P-MUT(T) is a notation label applied to T: wrapped only T. A writer could instead leave single Latin letters bare and wrap only Φ(K,b), Ω/T.')
g('CP089', [('α-leak', r'$\alpha$-leak')], 'high', 'Context: α is the alignment rate (α=1 etc. elsewhere in file).')
g('CL033', [('κ-processing', r'$\kappa$-processing'), ('where κ ', r'where $\kappa$ ')], 'high', '')
g('CN002', [], 'high', 'Already display math; left unchanged.')
g('CU028', [('3.0 × 10⁹ km³', r'$3.0 \times 10^{9}\,\text{km}^3$')], 'medium',
  'Included the unit in the span (superscript ³ is Unicode math too); "$3.0 \\times 10^9$ km³" is the main alternative boundary.')
g('CP072', [], 'high', 'Φ-Arena is a proper name of a project; not math.')
g('CU110', [], 'high', 'Already converted.')
g('CP049', [('a_t = π(M_t)', r'$a_t = \pi(M_t)$')], 'high', '')
g('CL029', [('Markov-of-Ω', r'Markov-of-$\Omega$'), ('extending Ω', r'extending $\Omega$'), ('saying Ω', r'saying $\Omega$')], 'high', '')
g('CU057', [('n_past', NP)], 'high', '')
g('CL076', [('ΔE′', r"$\Delta E'$")], 'medium',
  'Context: CAM02-UCS colour difference ΔE′ (file heading). Standard colour-science symbol; wrapping is right by house rule but it is conventionally typeset as text in some fields.')
g('CP013', [('δ_sat > 0', r'$\delta_{\text{sat}} \gt 0$')], 'high', 'Matches the ASF segment spelling $\\delta_{\\text{sat}} \\gt 0$.')
g('CU076', [('O(1)', '$O(1)$')], 'medium', 'Big-O in a software-analysis note; a LaTeX writer would wrap it, leaving it plain is also common.')
g('CP098', [('∞', r'$\infty$')], 'medium',
  'Context needed: fragment of "observation noise → ∞ on a direction" -- the fragmenter split at the arrow, so the ideal span ($\\text{noise} \\to \\infty$ or "→ ∞") is not reachable from this fragment.')
g('CL061', [('P ⇔ ¬Q', r'$P \Leftrightarrow \neg Q$'), ('P being', '$P$ being'), ('Q-non', '$Q$-non')], 'high', '')
g('CP034', [('η', r'$\eta$')], 'high', '')
g('CU064', [('W₀⁺', '$W_0^+$')], 'high', '')
g('CP069', [('M_t/G_t/Σ_t', r'$M_t$/$G_t$/$\Sigma_t$')], 'medium',
  'Slash is a list separator ("and"), not division, so three spans; one span $M_t/G_t/\\Sigma_t$ would read as a quotient.')
g('CP120', [('sub-scope α', r'sub-scope $\alpha$')], 'high',
  'Inside an eq-tag *[...]*; ASF segments write "sub-scope $\\alpha$" this way. Existing spans untouched.')
g('CU072', [('O(1)', '$O(1)$')], 'medium', 'Same as CU076.')
g('CL066', [('α / β', r'$\alpha$ / $\beta$')], 'high', 'Two symbols, not a quotient.')
g('CP130', [('= Λ', r'= $\Lambda$'), ('under Λ', r'under $\Lambda$')], 'high', '(A1)–(A4) are labels; left.')
g('CL078', [('α-op/β-op', r'$\alpha$-op/$\beta$-op'), ("'s α/β", r"'s $\alpha$/$\beta$"), ('; β fails', r'; $\beta$ fails')], 'high',
  "Backticked (O-A2') left alone.")
g('CU004', [('W₀ / W₁ / W₂', '$W_0$ / $W_1$ / $W_2$')], 'high',
  'Regime labels. Note: ASF segments themselves write W₁ in Unicode in prose (e.g. der-class-coercion-via-wrapping), but agents.sop says no bare Unicode math in any file.')
g('CU079', [('H_D3 p', '$H_{D3}$ $p$')], 'low',
  'Context: table header in a generated results file; H_D3 is a hypothesis label, p its p-value. Leaving "H_D3 p" unchanged is defensible (identifier-like, generated output).')
g('CU121', [('n_past < complexity_level', r'$n_{\text{past}} \lt \text{complexity-level}$')], 'low',
  'Relation is math, but complexity_level is an identifier; SOP forbids _ inside \\text so it becomes "-". Narrower alternative: only "$n_{\\text{past}}$ < complexity_level".')
g('CU134', [('[n_f]', '[$n_f$]')], 'medium', 'Placeholder brackets kept as prose.')
g('CP138', [('P(failure) × T(recovery)', r'$P(\text{failure}) \times T(\text{recovery})$')], 'medium', '')
g('CL022', [('t_consensus + n × t_compromise', r'$t_{\text{consensus}} + n \times t_{\text{compromise}}$')], 'high', '')
g('CN016', [], 'high', 'Transcript prose, no math.')
g('CU019', [('N=29', '$N=29$')], 'medium',
  'Sample size; house rule (math always LaTeX) says wrap, but many writers leave N=29 as text.')
g('CU101', [('p=0.001', '$p=0.001$')], 'medium', 'Table cell with a p-value; 10/10 is a count, left.')
g('CN026', [], 'high', 'Already converted.')
g('CP003', [("α'", r"$\alpha'$")], 'high', "Context: table cell; α' is sub-scope α' (ASF writes $\\alpha'$). C-iv is a label.")
g('CP010', [('E_{a~π}[I_o(a)]', r'$\mathbb{E}_{a \sim \pi}[I_o(a)]$')], 'high', '')
g('CP132', [('δ_strategic', r'$\delta_{\text{strategic}}$')], 'high', '')
g('CN001', [], 'high', 'Already converted.')
g('CU065', [('R∘L', r'$R \circ L$')], 'high', 'Context: R∘L = id (restrict ∘ lift) elsewhere in file.')
g('CN018', [], 'high', 'Cherokee letter in a Unicode table, not math.')
g('CP143', [('P̂(F | IS) = (k_is  + α) / (n_is  + α + β)',
             r'$\hat{P}(F \mid \text{IS}) = (k_{\text{is}} + \alpha) / (n_{\text{is}} + \alpha + \beta)$'),
            ('P̂(F | NOT) = (k_not + α) / (n_not + α + β)',
             r'$\hat{P}(F \mid \text{NOT}) = (k_{\text{not}} + \alpha) / (n_{\text{not}} + \alpha + \beta)$')], 'medium',
  'Context: on disk these are two lines in a 2-space-indented block (not a code block), so md-press unwrapped them into one line; gold keeps them as two separate spans. The real fix is upstream (line break or code block).')
g('CU010', [('$T_{change}(F_i)$ = 0.5', r'$T_{change}(F_i) = 0.5$')], 'medium',
  'Absorbed the existing span to take "= 0.5" into math (unit "hours" stays prose). Leaving the text unchanged is the conservative alternative.')
g('CU145', [('W₀ / W₂ / W₁', '$W_0$ / $W_2$ / $W_1$')], 'high', '')
g('CP134', [('Σ_t', r'$\Sigma_t$'), ('O_t', '$O_t$')], 'high', '')
g('CL023', [], 'medium',
  'Prose arrows (→) and word equations ("numerator = denominator", "ratio = 1") between existing spans; left as prose shorthand. Wrapping "ratio = 1" is a possible alternative.')
g('CP111', [('n_past ≈ 2-3', r'$n_{\text{past}} \approx 2\text{–}3$')], 'medium',
  'Range 2-3 kept inside the span as a text dash (raw "-" in math would render as minus).')
g('CU080', [('W₀', '$W_0$')], 'high', '')
g('CU060', [('1.2^discontinuities', r'$1.2^{\text{discontinuities}}$')], 'high', '')
g('CP027', [('Accept X ', 'Accept $X$ '), ('save Y ', 'save $Y$ '),
            ('X < $n_{\\text{future}}$ × Y', r'$X \lt n_{\text{future}} \times Y$')], 'medium',
  'Absorbed the existing $n_{\\text{future}}$ into the full inequality; X and Y wrapped as the variables they are.')
g('CP090', [('b′(s′) = [ O(s′, a, o) Σ_{s∈S} T(s, a, s′) b(s) ] / Pr(o | a, b)',
             r"$b'(s') = \left[ O(s', a, o) \sum_{s \in S} T(s, a, s') b(s) \right] / \Pr(o \mid a, b)$"),
            ('SE(b, a, o)', r'$\mathrm{SE}(b, a, o)$'), ('state b′', "state $b'$")], 'medium',
  'Quoted Kaelbling et al. POMDP update. SE is a named function (roman). Quote marks and … stay prose.')
g('CL024', [('δ_Σ', r'$\delta_\Sigma$'), ('(1-λ)', r'$(1-\lambda)$')], 'high', 'Parens are mathematical grouping, so inside the span.')
g('CP062', [('α₂', r'$\alpha_2$')], 'high', 'B1 is a label.')
g('CU029', [('n_past', NP)], 'high', '')
g('CU033', [], 'high',
  'Glyph-perception survey: √∛∜ and ↓/↑ are the glyphs under study and the answers, not math; converting would destroy the data. ">1" likewise left.')
g('CP056', [('|α_k|² Φ_k > 1', r'$\lvert\alpha_k\rvert^2 \Phi_k \gt 1$'), ('some k', 'some $k$'), ('G = ∞', r'$G = \infty$')], 'high', '')
g('CP050', [('ρ(A)', r'$\rho(A)$')], 'high', '')
g('CP107', [('$t_l$ ≈ 4', r'$t_l \approx 4$'), ('$n_{break}$ ≈ 3-5', r'$n_{break} \approx 3\text{–}5$')], 'medium',
  'Absorbed existing spans so ≈ is inside math; units stay prose. Range dash as text.')
g('CU047', [], 'high', 'Already converted.')
g('CP094', [('κ_processing', r'$\kappa_{\text{processing}}$'), ("agent's κ", r"agent's $\kappa$")], 'high', '')
g('CN040', [], 'high', 'Already converted; percentages are prose.')
g('CP009', [('ω < 1', r'$\omega \lt 1$')], 'high', '')
g('CU106', [('W₁', '$W_1$')], 'high', 'Heading-like line; 3.1 is a section number.')
g('CP086', [('ε', r'$\varepsilon$')], 'high', '')
g('CN031', [], 'high', 'Already converted.')
g('CL011', [('δ: S × A~ → S', r'$\delta: S \times \tilde{A} \to S$')], 'low',
  'Context: table cell in a Miller automata bridge; "A~" is presumably A-tilde (opponent/extended action set) -- guess. "~" cannot stay raw in math (it is a space).')
g('CP109', [('quality = Σ coherence(module_i) / Σ coupling(module_i, module_j)',
             r'$\text{quality} = \sum \text{coherence}(\text{module}_i) / \sum \text{coupling}(\text{module}_i, \text{module}_j)$')], 'medium',
  'Word-variable formula; wrapped whole with \\text words. Unmatched opening quote is in the source.')
g('CN011', [], 'high', 'Bibliography line, no math.')
g('CU140', [('f_M', '$f_M$')], 'high', '')
g('CU077', [('d*', r'$d^\ast$')], 'medium',
  'effective_complexity = min(machine_complexity, ...) left alone: an identifier-style pseudo-formula (would be code/backticks, not math). Wrapping it with \\text words is the alternative.')
g('CL017', [('λ=0.6', r'$\lambda=0.6$')], 'high', '')
g('CU088', [('update M_t', 'update $M_t$'), ('2×2', r'$2 \times 2$'), ('revise O_t', 'revise $O_t$')], 'medium',
  '"2×2 diagnostic" as $2 \\times 2$ is medium; M_t/O_t high.')
g('CU041', [], 'high', 'Already converted; "goal→belief" is a prose arrow between words.')
g('CP105', [('Ω_t', r'$\Omega_t$')], 'high', '')
g('CP082', [('M×N', r'$M \times N$')], 'medium', '"M×N combinatorial promise" -- symbolic product of counts.')
g('CP084', [('δ_objective', r'$\delta_{\text{objective}}$')], 'high', '')
g('CU144', [], 'high', 'Filename.')
g('CL098', [('ρ < 0.9', r'$\rho \lt 0.9$')], 'high', '')
g('CL063', [('δ_objective', r'$\delta_{\text{objective}}$')], 'high', '')
g('CL031', [('α ≈ 0.2', r'$\alpha \approx 0.2$')], 'high', '')
g('CP110', [('κ × A', r'$\kappa \times A$')], 'high', '')
g('CU053', [('W₁', '$W_1$')], 'high', 'Heading text; math in headings renders on GitHub/Obsidian.')
g('CL012', [('M_t ∈ M where M is', r'$M_t \in \mathcal{M}$ where $\mathcal{M}$ is')], 'low',
  'Set "M" upgraded to \\mathcal{M} per notation convention (calligraphic for spaces; AAT model space is $\\mathcal{M}$). Faithful alternative keeps plain M.')
g('CP145', [('r_t', '$r_t$'), ('‖δ_G‖', r'$\lVert\delta_G\rVert$'), ('on δ_G', r'on $\delta_G$')], 'high', '')
g('CU011', [('( \\\\mathrm{PC} \\= \\\\frac{1}{n}\\\\sum\\_{i=1}^n \\\\frac{|P(i)|}{n} )',
             r'$\mathrm{PC} = \frac{1}{n}\sum_{i=1}^n \frac{\lvert P(i)\rvert}{n}$')], 'medium',
  'Pasted export where \\( \\) lost its backslashes and the LaTeX got markdown-escaped (\\\\, \\=, \\_); replaced the whole "( ... )" with one clean span. Parens judged to be the lost delimiters.')
g('CP058', [('T > ρ/‖δ_critical‖', r'$T \gt \rho/\lVert\delta_{\text{critical}}\rVert$')], 'high', '')
g('CU125', [('P_adm = (P1, P2, P3)', r'$P_{\text{adm}} = (\text{P1}, \text{P2}, \text{P3})$')], 'medium',
  'P1-P3 are condition labels, so \\text inside; alternative is to wrap only P_adm.')
g('CL026', [('n_future ≈ 0', r'$n_{\text{future}} \approx 0$')], 'high', '')
g('CN006', [], 'high', 'Bibliography line, no math.')
g('CL044', [], 'medium', '"GitHub-MathJax ∩ Obsidian ∩ LaTeX" is set-notation shorthand over product names; left as prose.')
g('CL082', [('κ_processing', r'$\kappa_{\text{processing}}$')], 'high', '')
g('CP103', [('κ×A', r'$\kappa \times A$'), ('W₁/W₂', '$W_1$/$W_2$'), ('T/ν/η\\*', r'$T$/$\nu$/$\eta^\ast$')], 'medium',
  'Slashes are list separators, so separate spans. Source has markdown-escaped "η\\*" -- the escape is absorbed into the span.')
g('CP083', [], 'medium',
  'A, C are option labels in a track ("A → C → F-full → G" in source; fragment split mid-chain). Arrows between labels are prose.')
g('CU016', [], 'high', 'Filename.')
g('CL030', [('ν · η* > ρ', r'$\nu \cdot \eta^\ast \gt \rho$')], 'high', '')
g('CP139', [('Δ=-2.018', r'$\Delta=-2.018$')], 'medium',
  'Context: fragment of an italic line "ratio 1.118 → -0.899 (Δ=-2.018)*" split at the arrow; trailing * is the italic closer. -0.899 left as a plain number.')
g('CU087', [('W₀/W₂/W₁', '$W_0$/$W_2$/$W_1$')], 'high', '')
g('CP092', [('π', r'$\pi$')], 'medium',
  'Context needed: fragment of "𝒞_t→conversation history, π→forward pass"; fragmenter split at arrows, so the π→ mapping is severed.')
g('CU008', [('S(message | no context) >> S(message | shared context)',
             r'$S(\text{message} \mid \text{no context}) \gg S(\text{message} \mid \text{shared context})$')], 'medium', '')
g('CU063', [('v_B', '$v_B$'), ('v_A', '$v_A$')], 'high', '')
g('CP077', [('κ_processing = I(G_t ; M_τ⁺ | e_τ) / H(G_t | e_τ)',
             r'$\kappa_{\text{processing}} = I(G_t ; M_{\tau^+} \mid e_\tau) / H(G_t \mid e_\tau)$')], 'high', '')
g('CL084', [('t_min ≈ (t_spec)^{3/4}', r'$t_{\text{min}} \approx (t_{\text{spec}})^{3/4}$')], 'high', '')
g('CU042', [], 'high', 'Filename.')
g('CU113', [('W₀/W₂/W₁', '$W_0$/$W_2$/$W_1$')], 'high', '')
g('CP078', [('X < n_past × Y', r'$X \lt n_{\text{past}} \times Y$')], 'high', '')
g('CU066', [], 'high', 'Filename.')
g('CL092', [('ε-degraded', r'$\varepsilon$-degraded')], 'high', "A2', G-BP2 are labels.")
g('CP124', [('$\\beta$ ≈ 0.4', r'$\beta \approx 0.4$')], 'medium', 'Absorbed existing span to bring ≈ into math.')
g('CN035', [], 'high', 'Already display math (has raw \\| and | which violate house style, but rewriting inside existing math is out of shape).')
g('CP075', [('κ×A', r'$\kappa \times A$'), ('coupling κ', r'coupling $\kappa$')], 'high', '')
g('CU130', [('n_future = 500-1000', r'$n_{\text{future}} = 500\text{–}1000$')], 'medium', 'Range dash as text inside span.')
g('CU104', [('W₀', '$W_0$')], 'high', '')
g('CU035', [('O(n²)', '$O(n^2)$'), ('n < 1000', r'$n \lt 1000$')], 'high', '')
g('CN005', [], 'high', 'Already display math.')
g('CP117', [('μ(I)', r'$\mu(I)$')], 'high', 'μ(I) rheology.')
g('CP067', [], 'high', 'Context: ∂ is a UDON column sigil (∂(status) column), not a partial derivative.')
g('CU112', [], 'medium',
  'Context: verbatim terminal/diff transcript ("39 +-", "stro +ngest" are wrapped diff lines). Left as verbatim output; p<10⁻⁶ would otherwise become $p \\lt 10^{-6}$. md-press unwrapping this transcript is itself a problem.')
g('CL094', [], 'high', '"ref ⊂ path" is prose shorthand between words; no math.')
g('CL018', [], 'high', '"structure⊃prose⊃structure" is prose shorthand.')
g('CL016', [('η\\* = U_M/(U_M+U_o)', r'$\eta^\ast = U_M/(U_M+U_o)$'), ('on η\\*', r'on $\eta^\ast$')], 'high',
  'Markdown-escaped η\\* absorbed. Matches ASF segment spelling.')
g('CN037', [], 'high', 'Already converted.')
g('CP052', [('Λ-projection', r'$\Lambda$-projection')], 'medium',
  '⟺ joins prose phrases (MZ memory kernel = ...), left as prose.')
g('CL080', [('n_past ≥ 3', r'$n_{\text{past}} \geq 3$')], 'high', '')
g('CP046', [('N_r ∈ [1, ∞]', r'$N_r \in [1, \infty]$')], 'high', '')
g('CP030', [('N=41', '$N=41$'), ('F(1,39)=1.11', '$F(1,39)=1.11$'), ('p=.30', '$p=.30$'), ('η²p=.03', r'$\eta^2_p=.03$'),
            ('F(1,39)=2.41', '$F(1,39)=2.41$'), ('p=.13', '$p=.13$'), ('η²p=.06', r'$\eta^2_p=.06$')], 'medium',
  'APA statistics. η²p is partial eta squared (subscript p). One span per statistic; plain-text stats are also a defensible gold.')
g('CN014', [], 'high', 'Already converted.')
g('CU073', [('W₁', '$W_1$')], 'high', '')
g('CU061', [('64²', '$64^2$')], 'medium', '64² = grid size.')
g('CU039', [('W₀-versus-W₂', '$W_0$-versus-$W_2$')], 'high', '')
g('CL003', [('$t_{base}^{BFT}$ ≈ 200', r'$t_{base}^{BFT} \approx 200$')], 'medium', 'Absorbed existing span; unit stays prose.')
g('CN044', [], 'high', 'No math (rule labels R6/L6/L0).')
g('CP095', [('α/β', r'$\alpha$/$\beta$')], 'high', '')
g('CL062', [('λ(t)', r'$\lambda(t)$')], 'high', '')
g('CL043', [('\\|Δ_fan\\|', r'$\lvert\Delta_{\text{fan}}\rvert$')], 'medium',
  'Table header "mean|Δ_fan|°"; the escaped pipes become \\lvert/\\rvert. "mean" and ° left as prose; one span covering $\\operatorname{mean}\\lvert...\\rvert^\\circ$ is an alternative.')
g('CU040', [('G₃', '$G_3$')], 'high', 'Context: G₁/G₂/G₃ code-age generations.')
g('CP148', [('ξ‐q', r'$\xi$‐$q$'), ('on ξ', r'on $\xi$')], 'medium',
  'ξ‐q model named for its variables; U+2010 hyphen kept between spans. "$\\xi$‐q" (q unwrapped) is a possible alternative.')
g('CU084', [('n=3', '$n=3$')], 'medium', '')
g('CP004', [('λ(t) > μ(t)', r'$\lambda(t) \gt \mu(t)$')], 'high', '')
g('CN029', [], 'high', 'Filename / prose.')
g('CU071', [('n' + H + '_future', r'$\hat{n}_{\text{future}}$')], 'high', '')
g('CU100', [], 'high', 'Already converted.')
g('CP100', [('θ', r'$\theta$')], 'high', '')
g('CN038', [], 'high', 'Comparison is inside a code span.')
g('CP068', [('ΔE≈2', r'$\Delta E \approx 2$')], 'medium', 'Colour difference; see CL076.')
g('CL014', [('T_A = ν_A · η_A*', r'$T_A = \nu_A \cdot \eta_A^\ast$')], 'high', '')
g('CP032', [('Σ', r'$\Sigma$')], 'medium', 'Context: table header ("report Σ | my Σ | ..."), Σ is a measured statistic.')
g('CU120', [], 'high', 'Filename.')

# ---------------------------------------------------------------- build
def apply(text, reps):
    out, pos = [], 0
    for old, new in reps:
        i = text.find(old, pos)
        if i < 0:
            raise ValueError(f'not found after {pos}: {old!r}')
        out.append(text[pos:i]); out.append(new); pos = i + len(old)
    out.append(text[pos:])
    return ''.join(out)

def shape_ok(text, gold):
    # gold must equal text with contiguous regions replaced by $...$ spans:
    # build a regex from gold where each $...$ span matches any nonempty region.
    parts = re.split(r'(\$[^$]+\$)', gold)
    pat = ''.join('(.+?)' if (p.startswith('$') and p.endswith('$') and len(p) > 1) else re.escape(p) for p in parts)
    return re.fullmatch(pat, text, re.S) is not None

items = [json.loads(l) for l in open(TASKS)]
missing = [d['gid'] for d in items if d['gid'] not in G]
extra = set(G) - {d['gid'] for d in items}
if missing or extra:
    sys.exit(f'missing={missing} extra={extra}')
bad = 0
with open(OUT, 'w') as f:
    for d in items:
        reps, conf, note = G[d['gid']]
        gold = apply(d['text'], reps)
        if not shape_ok(d['text'], gold):
            print('SHAPE FAIL', d['gid']); bad += 1
        if re.search(r'\$ |[^$] \$(?![^$]*\$)', ''):
            pass
        for span in re.findall(r'\$([^$]+)\$', gold):
            if span != span.strip(): print('PADDED', d['gid'], span); bad += 1
            if re.search(r'[<>*|]', span) and d['gid'] not in ('CN002','CN035','CU110','CN026','CN001','CU047','CN040','CN031','CU041','CN037','CU100','CN014','CN005'):
                print('RAW CHAR', d['gid'], span)
        f.write(json.dumps({'gid': d['gid'], 'gold': gold, 'conf': conf, 'note': note}, ensure_ascii=False) + '\n')
print('wrote', len(items), 'bad', bad)
