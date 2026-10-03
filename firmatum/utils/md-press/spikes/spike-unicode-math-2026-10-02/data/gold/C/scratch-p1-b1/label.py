#!/usr/bin/env python3
"""Pass-1 batch-1 gold labels. Each entry: gid -> (reps, conf, note).
reps: list of (old, new) substring replacements applied in order (each old must
occur exactly `count` times, default: all occurrences, at least once).
A checker then verifies gold == text with contiguous regions replaced by $..$ spans."""
import json, re, sys, os

HERE = os.path.dirname(os.path.abspath(__file__))
TASKS = os.path.join(HERE, '..', 'tasks', 'batch-1.jsonl')
OUT = os.path.join(HERE, '..', 'pass1', 'batch-1.jsonl')

NF = r'$n_{\text{future}}$'
L = {}

def lab(gid, reps, conf, note):
    assert gid not in L, gid
    L[gid] = (reps, conf, note)

U = []  # unchanged

lab('CL085', [('C < n_future × S', r'$C \lt n_{\text{future}} \times S$')], 'high', 'TST threshold inequality; n_future is the TST variable.')
lab('CP118', [('η*', r'$\eta^\ast$')], 'high', 'heading-like title; eta-star per house \\ast.')
lab('CP023', [('Ω-routed', r'$\Omega$-routed')], 'high', 'condition-4 is a label, left.')
lab('CP071', [('ṙ ≥ nα/2', r'\dot{r} \geq n\alpha/2')], 'high', '')
lab('CN020', U, 'high', 'transcript; line numbers and diff counts are not math.')
lab('CU082', U, 'high', 'already converted; (0.82) is a plain number.')
lab('CU094', U, 'medium', 'needed context: a table cell where E¹ is rating "E" plus footnote marker ¹ (E², D³, —¹⁰ elsewhere in the table), not a superscript.')
lab('CU133', [('n_past', r'$n_{\text{past}}$')], 'medium', 'TST variable inside a quoted thought in a session transcript; math, but a verbatim-transcript file may deserve .md-pressignore.')
lab('CN033', U, 'high', 'existing span; SA3 is a label.')
lab('CU132', U, 'medium', 'needed context: W₀/W₁/W₂ are wrapping-regime labels; asf src writes them in Unicode 247x vs $W_1$ once, even beside $O$/$\\Sigma$ in this same table. A context-free LaTeX writer might make them $W_2$/$W_1$.')
lab('CN000', U, 'high', 'existing span (t_{switch} unbraced \\text is the author\'s, not ours to edit).')
lab('CL025', [('"cycle rate" ν', r'"cycle rate" $\nu$'), ('T = ν × η*', r'$T = \nu \times \eta^\ast$')], 'medium', 'kept T as written (house tempo symbol is \\mathcal{T}, but changing it is a meaning edit); "rate × gain" is verbal and left as prose.')
lab('CU021', U, 'medium', 'needed context: W₀⁺ is the same wrapping-regime label family (W₀/W₁/W₂), written in Unicode throughout its source file.')
lab('CP031', [('(θ < 1)', r'($\theta \lt 1$)')], 'high', 'existing spans left (they contain raw <, not ours to edit).')
lab('CL020', [('X < n_future × Y', r'$X \lt n_{\text{future}} \times Y$')], 'high', 'transcript, but the inequality is unambiguous math. T-06 is a label.')
lab('CU099', [('|changeset|', r'$\lvert\text{changeset}\rvert$')], 'medium', 'cardinality bars around a word; ~30% left as prose.')
lab('CU114', U, 'high', 'all math already in spans; (H$_\\kappa$) keeps roman H outside the span deliberately (hypothesis label).')
lab('CP066', [('α/β', r'$\alpha$/$\beta$')], 'high', 'two labels, slash as prose separator (house writes $\\alpha$/$\\beta$).')
lab('CP101', [('G_t', '$G_t$'), ('M_τ⁺', r'$M_\tau^+$'), ('e_τ', r'$e_\tau$')], 'high', 'three separate symbols in prose.')
lab('CU142', [(r'\(\hat{x}_k\)', r'$\hat{x}_k$'), (r'\(K_k\)', '$K_k$')], 'medium', 'already-math \\(..\\) delimiters converted to house $..$; if \\(..\\) handling is out of scope for the converter, unchanged is the alternative. ≈ is prose "roughly".')
lab('CU024', [('n_past', r'$n_{\text{past}}$')], 'medium', 'TST variable in transcript dialogue; D-01 is a label.')
lab('CU014', [('<10⁻⁶', r'$\lt 10^{-6}$')], 'high', 'table cell p-value bound.')
lab('CP091', [('pool** θ', r'pool** $\theta$'), ('U(θ, x)', r'$U(\theta, x)$'), ('text x arrives', 'text $x$ arrives')], 'high', '')
lab('CP126', [('φ', r'$\phi$')], 'high', 'spelling: Unicode φ is the loopy glyph (\\varphi) but house uses \\phi 175x vs \\varphi 4x; boundary is certain.')
lab('CP045', [('between O and Σ', r'between $O$ and $\Sigma$')], 'high', 'AAT objective/strategy symbols (house writes $O$, $\\Sigma$).')
lab('CP047', [('α₁/α₂/β', r'$\alpha_1$/$\alpha_2$/$\beta$')], 'high', 'sub-scope labels, separate spans per house practice. A2′ is a label.')
lab('CL038', [('α-model', r'$\alpha$-model')], 'medium', 'named model, alpha as a symbol prefix like sub-scope-$\\alpha$.')
lab('CP016', [('δ_strategic', r'$\delta_{\text{strategic}}$')], 'high', '')
lab('CL071', [('ρ_Σ=R_Σ/2', r'$\rho_\Sigma=R_\Sigma/2$')], 'high', '')
lab('CP007', [('X_i → X_j →', r'$X_i \to X_j \to$')], 'medium', 'fragment cuts the expression mid-chain (source: "X_i → X_j → ··· → X_i would require $..."); trailing arrow kept in the span.')
lab('CP019', [('sub-scope-α', r'sub-scope-$\alpha$')], 'high', 'code-span slug left.')
lab('CP057', [('γ_t', r'$\gamma_t$'), ('period τ', r'period $\tau$'), ('τ=1', r'$\tau=1$')], 'high', 'tau inside bold stays inside bold.')
lab('CU055', [('n=238', '$n=238$')], 'medium', 'sample-size notation; many writers leave n=238 plain.')
lab('CP096', [('θ-smoothing', r'$\theta$-smoothing')], 'high', '')
lab('CU006', U, 'medium', 'needed context: W₁ is a wrapping-regime label, Unicode by house practice (see CU132).')
lab('CL005', U, 'low', 'needed context: T1-α is a diagram-catalog ID (tier 1, item α; cf. "demonstrated on T1-α"), an enumerator not a variable. Converting to T1-$\\alpha$ is defensible.')
lab('CL032', [('β_exp = 0', r'$\beta_{\text{exp}} = 0$')], 'high', '')
lab('CU051', U, 'low', 'needed context: S₂ (with S₀) reads as a named structure label in this report, which writes S₀/S₂ in Unicode beside real math ($k\\ge2$). If it is a variable, $S_2$-style.')
lab('CN019', U, 'high', 'existing span; ~ and footnote refs untouched.')
lab('CN048', U, 'high', 'C<sup>#</sup> is the language C#; citations not math.')
lab('CL048', [('finite-ν', r'finite-$\nu$')], 'medium', 'math inside backtick code spans left (code is out of reach); 1.481 vs 3/2 and AR(1) left as prose numbers/model name.')
lab('CN036', U, 'high', '')
lab('CP026', [('ρ_LOO', r'$\rho_{\text{LOO}}$'), ('ρ = 0.99', r'$\rho = 0.99$'), ('ρ = 0.97', r'$\rho = 0.97$'), ('|ρ| = 0.93', r'$\lvert\rho\rvert = 0.93$')], 'medium', 'range after rho_LOO (no =) left as prose; could also be included.')
lab('CU050', U, 'high', 'H$_\\kappa$ already in house form; section/lemma labels not math.')
lab('CL008', [('High-ρ', r'High-$\rho$')], 'high', '')
lab('CL069', [('U_M (shared', r'$U_M$ (shared'), ('U_O (shared', r'$U_O$ (shared'), ('U_Σ (coord', r'$U_\Sigma$ (coord'), ('U_obs (shared', r'$U_{\text{obs}}$ (shared'),
              ('↔ 1 − U_M', r'↔ $1 - U_M$'), ('↔ 1 − U_O', r'↔ $1 - U_O$'), ('(1 − U_Σ)(1 − U_obs)', r'$(1 - U_\Sigma)(1 - U_{\text{obs}})$')], 'medium', '↔ maps prose names to expressions, so it stays outside the spans.')
lab('CU123', U, 'high', 'filename.')
lab('CL041', [('α ≈ 0.2', r'$\alpha \approx 0.2$'), ('different α values', r'different $\alpha$ values')], 'high', '')
lab('CN008', U, 'high', 'transcript; file names not math.')
lab('CU022', [('O(1)', '$O(1)$')], 'medium', 'Landau notation inside a quoted example string in an agent prompt; leaving it plain is also common.')
lab('CN012', U, 'high', '')
lab('CP113', [('α=1', r'$\alpha=1$')], 'medium', 'distribution name kept as prose; $\\mathrm{Pareto}(\\alpha=1)$ as one span also defensible.')
lab('CL055', [('sub-scope-α', r'sub-scope-$\alpha$'), ('β-list', r'$\beta$-list'), ('(Λ doesn', r'($\Lambda$ doesn')], 'high', '')
lab('CU089', [('n_future > 0', r'$n_{\text{future}} \gt 0$')], 'high', '')
lab('CU103', U, 'medium', 'needed context: H_D3 is a hypothesis/control label; the project\'s own LaTeX paper sets it as text (\\textbf{H\\_D3}), not math. $H_{D3}$ is the context-free guess.')
lab('CL028', [('additive ρ', r'additive $\rho$')], 'high', '')
lab('CP005', [('σ,', r'$\sigma$,'), ('π,', r'$\pi$,'), ('⋈,', r'$\bowtie$,'), ('∪', r'$\cup$')], 'high', 'relational-algebra operator symbols.')
lab('CU007', U, 'high', 'existing spans.')
lab('CL072', [('X < $n_{\\text{past}}$ × Y', r'$X \lt n_{\text{past}} \times Y$'), ('(5 < 20 × 0.25)', r'($5 \lt 20 \times 0.25$)')], 'high', 'existing span absorbed whole into the full inequality.')
lab('CU102', [('𝔄', r'$\mathfrak{A}$')], 'high', 'fraktur A; step/lemma numbers not math.')
lab('CL036', [('ε *is*', r'$\varepsilon$ *is*')], 'high', '')
lab('CU135', [('scope s.', 'scope $s$.'), ('module of s (', 'module of $s$ (')], 'medium', 'PDF-extracted paper: bare single-letter variable s in prose; padded existing spans left as-is.')
lab('CU005', [('n_future', NF)], 'high', '')
lab('CU124', [('n=1', '$n=1$')], 'low', 'idiom ("an n of 1") in a system prompt; leaving plain is equally defensible. Kept consistent with sample-size items.')
lab('CL047', [('U_o', '$U_o$'), ('η*', r'$\eta^\ast$'), ('𝒯', r'$\mathcal{T}$')], 'medium', 'chain starts at a prose word (code-quality), so arrows stay prose and each symbol gets its own span; one span "U_o → η* → 𝒯" also defensible.')
lab('CN032', U, 'high', '')
lab('CU026', [('n̂_future', r'$\hat{n}_{\text{future}}$'), ('vs n_future', r'vs $n_{\text{future}}$')], 'high', '')
lab('CN041', U, 'high', '')
lab('CP000', [('metric-α₁', r'metric-$\alpha_1$')], 'high', 'existing span left; DA2\'-inc is a label.')
lab('CP129', [('H(G_t) (or', '$H(G_t)$ (or'), ('H(G_t | e_τ)', r'$H(G_t \mid e_\tau)$')], 'high', '')
lab('CU085', [('lambda_e', r'$\lambda_e$'), ('U_M', '$U_M$')], 'medium', 'ASCII-spelled lambda in a sketch file; meaning is clearly the Greek variable.')
lab('CU018', U, 'medium', 'debugging trace over code variables (elem_col, ACTUAL_COL, "2 <= 5?"); code-ish comparison, not math notation.')
lab('CU074', [(r'\(x_t\)', '$x_t$'), (r'\(y_t\)', '$y_t$')], 'medium', '\\(..\\) delimiters converted to house $..$ (see CU142).')
lab('CN004', U, 'high', '')
lab('CP044', [('κ_proc', r'$\kappa_{\text{proc}}$')], 'high', '')
lab('CU111', U, 'medium', 'needed context: step in a worked column-stack trace in udon spec ("3 <= 9? Pop child-of-c"), code-style comparison.')
lab('CU118', [('F(3,1625)=1.93', '$F(3,1625)=1.93$'), ('MSE=2460.4', r'$\mathrm{MSE}=2460.4$'), ('p=0.12', '$p=0.12$')], 'low', 'APA statistics in a quoted paper excerpt; italic F/p is standard, but leaving a quote untouched is also reasonable. 4 s / 60 s are units.')
lab('CP017', [('ρ', r'$\rho$')], 'high', 'table header.')
lab('CP037', [('2Δ', r'$2\Delta$')], 'high', 'twice the grid spacing.')
lab('CP149', [('γ_t', r'$\gamma_t$')], 'high', '')
lab('CL064', [('α = 0.2', r'$\alpha = 0.2$')], 'high', '')
lab('CU139', [('some C.', 'some $C$.')], 'medium', 'PDF-extracted proof; private<sub>P</sub> HTML left (an in-shape absorb into $\\mathit{private}_P(x_d)$ would be overreach). D-DEF is a rule name.')
lab('CL093', [('τ_b', r'$\tau_b$'), ('τ_s', r'$\tau_s$'), ('slope S.', 'slope $S$.')], 'high', '')
lab('CU098', U, 'medium', 'needed context: W₁ is a wrapping-regime label (Unicode by house practice, 247x in asf src); $\\varepsilon(\\kappa)$ already a span.')
lab('CU105', [('R = 1.0', '$R = 1.0$'), ('b = 1.234', '$b = 1.234$')], 'medium', 'results list item; the double space between them is preserved.')
lab('CL051', [("β' VI", r"$\beta'$ VI")], 'high', '')
lab('CU090', U, 'medium', 'H_D3 is a hypothesis/control label (paper sets it as \\textbf{H\\_D3}); ≈ and = here are prose glue between named conditions.')
lab('CU049', [('scope S with parent P', 'scope $S$ with parent $P$')], 'medium', 'PDF-extracted paper: bare single-letter variables; += is the operator name, left.')
lab('CL079', [('Markov-of-Ω', r'Markov-of-$\Omega$'), ('(define Ω', r'(define $\Omega$'), ('Markov-of-Mt', 'Markov-of-$M_t$')], 'medium', '"Mt" read as ASCII for M_t (house ASCII form "Mt / Ot / Σt"); the rest is high.')
lab('CP144', U, 'medium', 'needed context: ∂(..) and ⟦..⟧ are udon spec-lite\'s own computed-column and field notation (∂(status), ⟦force⟧), not math.')
lab('CL060', U, 'low', 'ΔMAE is a metric name and z_match_MAE a field name from the experiment code; a paper would write $z_{\\text{match}}$, but in this brief they act as identifiers. Converting to $\\Delta\\mathrm{MAE}$ / $z_{\\text{match-MAE}} = +26.88$ is the other honest reading.')
lab('CL090', [("Conditional α'", r"Conditional $\alpha'$"), ("ε-degraded α'", r"$\varepsilon$-degraded $\alpha'$")], 'high', '')
lab('CU146', [('P_breaking(L)', r'$P_{\text{breaking}}(L)$')], 'high', '')
lab('CP055', U, 'low', 'Δaxis is a metric name (change in axis-aligned fraction); Unicode Δ renders fine. $\\Delta\\text{axis}$ also defensible. Signed numbers left.')
lab('CU129', [('n_future', NF)], 'medium', 'memory transcript, but n_future is the TST variable.')
lab('CL067', [('(O, Σ)', r'$(O, \Sigma)$')], 'high', '')
lab('CL037', [('κ × A', r'$\kappa \times A$')], 'high', 'kept A as written (elsewhere it is \\mathcal{A}).')
lab('CL077', [('ε* → 0', r'$\varepsilon^\ast \to 0$')], 'high', 'existing spans left.')
lab('CP012', [('P(change_together) ≈ 0', r'$P(\text{change-together}) \approx 0$')], 'medium', 'underscore in \\text{} becomes - per SOP; \\text{change\\_together} is the alternative spelling.')
lab('CN030', U, 'high', '')
lab('CU037', [('continuous Q̂', r'continuous $\hat{Q}$'), ('Q̂ was', r'$\hat{Q}$ was')], 'high', 'degree values left as prose numbers.')
lab('CP059', [('κ × 𝒜', r'\kappa \times \mathcal{A}')], 'high', '')
lab('CN010', U, 'high', '')
lab('CU046', U, 'medium', 'needed context: W₁ is a wrapping-regime label (see CU132).')
lab('CP061', [('η_edge = U_edge / (U_edge + U_obs)', r'$\eta_{\text{edge}} = U_{\text{edge}} / (U_{\text{edge}} + U_{\text{obs}})$')], 'high', '')
lab('CP035', [('[0,1]', '$[0,1]$'), ('‖J‖² → 0', r'$\lVert J\rVert^2 \to 0$')], 'medium', 'interval is math but many leave [0,1] plain; the norm is high.')
lab('CU149', U, 'high', 'filename.')
lab('CU034', U, 'high', 'session id/timestamp; t=9 is a record field.')
lab('CP136', [('α₁/α₂', r'$\alpha_1$/$\alpha_2$')], 'high', '')
lab('CU023', U, 'high', 'existing spans (including the odd $\\{$...$\\}$).')
lab('CN043', U, 'high', 'existing spans; (T2) is a label.')
lab('CL081', [('α₁/α₂', r'$\alpha_1$/$\alpha_2$'), ('of α.', r'of $\alpha$.')], 'high', '')
lab('CL034', [('ρ = 0.75–0.90', r'$\rho = 0.75\text{–}0.90$')], 'medium', 'range included in the span; $\\rho = 0.75$–0.90 is a defensible boundary.')
lab('CU093', [('P_t', '$P_t$'), ('K_t', '$K_t$')], 'medium', 'arrows link prose states ("steady state"), so they stay prose.')
lab('CL013', [('metric-α₂', r'metric-$\alpha_2$')], 'high', '')
lab('CU075', U, 'high', 'filename.')
lab('CL040', [('caps 𝒯', r'caps $\mathcal{T}$'), ('rate ν', r'rate $\nu$'), ('sufficient ρ', r'sufficient $\rho$')], 'high', '')
lab('CP076', [('\u2126( (cid:112) \\| D X ∪ S \\| T )', r'$\Omega(\sqrt{\lvert D_{X \cup S}\rvert T})$'), ('where T is', 'where $T$ is'),
              ('and D X ∪ S is', r'and $D_{X \cup S}$ is'), ('treatments X and covariates S .', 'treatments $X$ and covariates $S$ .')], 'low',
    'garbled PDF extraction in a quoted abstract: (cid:112) is the sqrt glyph, "D X ∪ S" is D_{X∪S}. Reconstructed in-shape; leaving the garble untouched is the conservative alternative. The space before the final period is source text.')
lab('CP088', [('κ≈1', r'$\kappa \approx 1$')], 'high', '')
lab('CP014', [('o_t = h(Ω_t, a_{t-1}, ε_t)', r'$o_t = h(\Omega_t, a_{t-1}, \varepsilon_t)$'), ('; h lossy', '; $h$ lossy'), ('NEITHER h NOR ε', r'NEITHER $h$ NOR $\varepsilon$')], 'high', '')
lab('CU107', [('D_security', r'$D_{\text{security}}$')], 'high', '')
lab('CP015', [('$\\gamma$ ≈ 0.05', r'$\gamma \approx 0.05$')], 'high', 'existing span absorbed into the approximation.')
lab('CL049', [('max \\|σ₁σ₂ − 1\\|', r'$\max\lvert\sigma_1\sigma_2 - 1\rvert$')], 'medium', 'table cell (\\| are escaped pipes); "= **1.1e-7**" left outside because bold can\'t live in math.')
lab('CP001', [('α = 0.2', r'$\alpha = 0.2$')], 'low', 'escaped (\\n) transcript dump; math is right if anything is touched, but this is effectively verbatim data. "alpha being around 0.2" is the word, left.')
lab('CP018', [('where 𝒜 is', r'where $\mathcal{A}$ is'), ('𝒜≈0', r'$\mathcal{A} \approx 0$')], 'medium', 'math inside the backtick code span left (code out of reach); author likely meant it as math.')
lab('CP102', [('of α (', r'of $\alpha$ ('), ('α ≈ 0.001', r'$\alpha \approx 0.001$'), ('α ≈ 0.1', r'$\alpha \approx 0.1$')], 'high', '')
lab('CL019', [('δ_strategic', r'$\delta_{\text{strategic}}$')], 'high', '')
lab('CP002', [('P̂_Σ', r'$\hat P_\Sigma$')], 'high', 'L1 is a label.')
lab('CP133', [('ω = 25.17°', r'$\omega = 25.17^\circ$')], 'medium', 'degree sign moved into the span; the lone 3.75° stays prose. $\\omega$ = 25.17° is the narrower alternative.')
lab('CU095', [('P certifies M⁻', 'P certifies $M^-$'), ('(proven) refuted: P', '(proven) refuted: $P$'), ('Q is non-cert of M (', '$Q$ is non-cert of $M$ (')], 'medium', 'needed context: table of bridges where P, Q, M, M⁻ are formal properties/mechanisms; the source file uses no LaTeX at all.')
lab('CU148', [('T_A', '$T_A$')], 'high', '')
lab('CP036', [('a_t = π(M_t, G_t)', r'$a_t = \pi(M_t, G_t)$')], 'high', '')
lab('CL089', [('k=1:', '$k=1$:'), ('k≥2:', r'$k \geq 2$:')], 'high', '')
lab('CP137', [('β/ρ', r'$\beta$/$\rho$')], 'high', 'context confirms two knobs (source says "β-vs-ρ"), not a ratio.')
lab('CP033', [('μ_prox', r'$\mu_{\text{prox}}$')], 'high', '')
lab('CP073', U, 'medium', 'CSV row of a garbled PDF abstract; the layout is unrecoverable (ordering of symbols lost), so nothing honest can be promoted.')
lab('CL050', U, 'high', 'needed context: cell of a generated UTF-8 glyph table; [ξ] shows the character itself.')
lab('CP115', [('δ_s', r'$\delta_s$'), ('P̂_Σ', r'$\hat P_\Sigma$'), ('and Φ', r'and $\Phi$')], 'high', '')
lab('CU096', U, 'high', 'transcript tool calls; >/dev/null is shell.')
lab('CL096', [('z = +26.88', '$z = +26.88$')], 'medium', 'plain z-statistic converted; ΔMAE left as a metric name (see CL060).')
lab('CP140', [('$\\beta$ ≈ 0.3', r'$\beta \approx 0.3$')], 'high', 'existing span absorbed; "= empirical..." is prose glue.')
lab('CL021', [('κ×$\\mathcal{A}$', r'$\kappa \times \mathcal{A}$')], 'high', 'existing span absorbed.')
lab('CU122', [('N²/8', '$N^2/8$')], 'high', '~10 and L6 left.')
lab('CU043', [("p' < p", r"$p' \lt p$")], 'high', '')
lab('CP006', [('π*', r'$\pi^\ast$')], 'high', '')
lab('CU091', [('O(n²)', '$O(n^2)$')], 'medium', 'Landau notation in prose; leaving O(n²) plain is common in software writing.')
lab('CP116', [('n_future ≈ 5+', r'$n_{\text{future}} \approx 5+$')], 'medium', '"5+" = five or more; kept the + in the span.')
lab('CP085', U, 'high', 'needed context: node IDs of a paper-dependency graph (W → D → C → R → RR → E → M → CH), not math.')
lab('CP021', [('σ²', r'$\sigma^2$')], 'high', '')
lab('CP099', [('α>ρ/R', r'$\alpha \gt \rho/R$')], 'high', '')

# fix CP071 / CP059 entries that need $ wrapping (written without $ above)
def fix(gid):
    reps, c, n = L[gid]
    L[gid] = ([(o, '$' + nw + '$') for o, nw in reps], c, n)
fix('CP071'); fix('CP059')

def apply(text, reps):
    for old, new in reps:
        k = text.count(old)
        if k == 0:
            raise SystemExit(f'not found: {old!r} in {text[:80]!r}')
        text = text.replace(old, new)
    return text

SPAN = re.compile(r'\$[^$]+\$')

def in_shape(text, gold):
    # gold literal segments must appear in text in order; each span covers a nonempty region
    parts = SPAN.split(gold)
    pat = '(.+?)'.join(re.escape(p) for p in parts)
    return re.fullmatch(pat, text, re.S) is not None

def lint(gold, text):
    probs = []
    for s in SPAN.findall(gold):
        if s in text:
            continue
        inner = s[1:-1]
        if inner != inner.strip():
            probs.append('padding:' + s)
        if re.search(r'[<>*|]', inner):
            probs.append('raw char:' + s)
        if re.search(r'[α-ωΑ-Ω𝒜𝒯ℓ‖⁰-⁹₀-₉]', inner):
            probs.append('unicode left:' + s)
    return probs

items = [json.loads(l) for l in open(TASKS)]
missing = [d['gid'] for d in items if d['gid'] not in L]
extra = set(L) - {d['gid'] for d in items}
assert not missing and not extra, (missing, extra)
out = []
bad = 0
for d in items:
    reps, conf, note = L[d['gid']]
    gold = apply(d['text'], reps)
    if not in_shape(d['text'], gold):
        print('SHAPE FAIL', d['gid']); bad += 1
    for p in lint(gold, d['text']):
        print('LINT', d['gid'], p); bad += 1
    out.append({'gid': d['gid'], 'gold': gold, 'conf': conf, 'note': note})
if '--write' in sys.argv and not bad:
    with open(OUT, 'w') as f:
        for o in out:
            f.write(json.dumps(o, ensure_ascii=False) + '\n')
    print('wrote', len(out), OUT)
print('bad', bad)
from collections import Counter
print(Counter(o['conf'] for o in out), 'unchanged', sum(o['gold'] == d['text'] for o, d in zip(out, items)))
