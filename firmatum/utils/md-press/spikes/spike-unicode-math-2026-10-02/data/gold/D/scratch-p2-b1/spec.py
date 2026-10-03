# Pass-2 batch-1 gold spec. Each entry: gid -> (replacements, conf, note).
# Replacements are (old, new) applied left-to-right with an advancing cursor,
# so repeated tokens are matched in reading order.
NFUT = r'n_{\text{future}}'
NPAST = r'n_{\text{past}}'

S = {}
def g(gid, reps, conf, note):
    S[gid] = (reps, conf, note)

g('DU027', [('M_t', '$M_t$'), ('W_t', '$W_t$')], 'high', 'symbol subscripts')
g('DL026', [('ν_{ij}', r'$\nu_{ij}$')], 'high', '')
g('DU004', [('ϵ-Capacity', r'$\epsilon$-Capacity')], 'medium',
  'bibliographic title; the paper title itself is "ε-capacity" so math epsilon is the faithful rendering, but some would leave a citation string untouched')
g('DL113', [], 'high', 'context checked: table cell "Current -> Proposed" category letters; prose arrow, not math')
g('DU062', [('N_h > 1', r'$N_h \gt 1$')], 'high', '')
g('DN004', [], 'high', 'already formatted')
g('DU111', [('n>1100', r'$n \gt 1100$')], 'medium',
  'sample size inside a citation bracket; journal vol/pages left alone. Some would leave stats shorthand in a source-harvest note')
g('DN023', [], 'high', 'already formatted')
g('DL034', [('v = v × e^(β × tooling_time) × e^(-γ × features × time)',
             r'$v = v \times e^{\beta \times \text{tooling\_time}} \times e^{-\gamma \times \text{features} \times \text{time}}$')],
  'medium', r'whole formula one span; parens around exponents are grouping -> braces. Kept identifier underscore as \_ inside \text (SOP suggests "-" instead; meaning-preserving choice here)')
g('DL054', [('ties α', r'ties $\alpha$')], 'high', 'code spans left as code')
g('DL156', [('λ_min', r'$\lambda_{\min}$')], 'high', '')
g('DN039', [], 'high', 'escaped template braces + JSX tag, not math')
g('DL017', [('β ≈ 20%', r'$\beta \approx 20\%$')], 'medium',
  'code-span big-O left as code; "top-" stays prose. Alternative: only $\\beta$ with "≈ 20%" as prose')
g('DN007', [], 'high', 'transcript with grep regex, not math')
g('DL188', [('k* ≈ 0', r'$k^\ast \approx 0$')], 'high', 'first `k*` is a code span, left alone')
g('DN049', [], 'high', 'filename')
g('DU133', [('for k branches', 'for $k$ branches')], 'high', 'bare k matches its own $O(k)$ neighbors')
g('DN038', [], 'high', 'no math')
g('DL145', [('κ × 𝒜', r'$\kappa \times \mathcal{A}$')], 'high', 'inside italic title; dates arrow is prose')
g('DL143', [('Accept X', 'Accept $X$'), ('save Y', 'save $Y$'),
            ('X < n_future × Y', r'$X \lt ' + NFUT + r' \times Y$'),
            ('n_future = n_past', '$' + NFUT + ' = ' + NPAST + '$')], 'medium',
  'bare X/Y variables converted for consistency with the inequality; T-06/T-04 are theorem labels')
g('DL027', [('M_t', '$M_t$'), ('Ω', r'$\Omega$')], 'high', '')
g('DL044', [('H_b operationalization', '$H_b$ operationalization'), ('of κ**', r'of $\kappa$**'),
            ('κ = 1 - H_b(G_t | M_τ⁺, e_τ) / H(G_t)', r'$\kappa = 1 - H_b(G_t \mid M_\tau^+, e_\tau) / H(G_t)$'),
            ('H_b ≈ H(G_t)', r'$H_b \approx H(G_t)$'), ('κ ≈ 0', r'$\kappa \approx 0$'),
            ('H_b ≈ 0', r'$H_b \approx 0$'), ('κ ≈ 1', r'$\kappa \approx 1$')], 'medium',
  'definition is high-confidence; the "prose -> math -> math" chains kept as separate spans with arrows as prose. One could instead span "H_b ≈ H(G_t) → κ ≈ 0" whole with \\to')
g('DN034', [], 'medium', '"ZIP < 100 MB" is a size shorthand, not math')
g('DU045', [('n_future', '$' + NFUT + '$'), ('n̂_future', r'$\hat n_{\text{future}}$')], 'high', '')
g('DN024', [], 'high', 'transcript, no math')
g('DN037', [], 'high', 'already formatted')
g('DL058', [('κ_processing', r'$\kappa_{\text{processing}}$'), ('A(e)', '$A(e)$')], 'high',
  'kept A as written (elsewhere observation ambiguity is calligraphic, but the author wrote A here)')
g('DL100', [], 'high', 'context needed and checked: Unicode glyph table cell; the character itself is the subject')
g('DU069', [('(ii ⟹ iii)', r'(ii $\Longrightarrow$ iii)'), ('(iii ⟹ ii)', r'(iii $\Longrightarrow$ ii)'),
            ('$e$ ⟹ $e', r'$e$ $\Longrightarrow$ $e'),
            ('(ii ⟹ i)', r'(ii $\Longrightarrow$ i)'), ('(i ⟹ ii)', r'(i $\Longrightarrow$ ii)')], 'medium',
  'only raw ⟹ remain; case labels stay text. Mid-sentence one could alternatively merge the flanking spans; leaving the glyph unconverted is also defensible')
g('DU126', [(r'\(P_G\)', '$P_G$'), (r'\(Q\)', '$Q$'), (r'\(D_{1/2}(P_G\|Q)\)', r'$D_{1/2}(P_G \Vert Q)$'),
            (r'\(G\)', '$G$'), (r'\(M\)', '$M$'), (r'\(I(G;M) = I\)', '$I(G;M) = I$'), (r'\(I\)', '$I$')],
  'medium', r'\(…\) delimiters do not render on GitHub (escaped parens); converted to $…$ with \| -> \Vert per SOP. Medium only because it is unclear whether delimiter migration is in scope for this yardstick')
g('DL130', [('λ=0.15', r'$\lambda=0.15$')], 'medium', 'cont_surv is a code identifier; left as-is with its existing $0.00$')
g('DU003', [('M_t', '$M_t$')], 'high', 'heading')
g('DL115', [('N ∈ [0,1]', r'$N \in [0,1]$'), ('N=1', '$N=1$'), ('N=0', '$N=0$')], 'high', 'inside a quotation, but plainly math')
g('DL045', [('Spearman ρ', r'Spearman $\rho$'), ('N=12', '$N=12$'), ('the ρ estimate', r'the $\rho$ estimate')], 'medium',
  'rho high; N=12 sample-size shorthand is the medium part. 330×, 23M etc left as prose')
g('DN032', [], 'high', 'no math')
g('DL041', [('β-vs-ρ', r'$\beta$-vs-$\rho$')], 'high', '')
g('DL057', [], 'low',
  'context checked: table header "Δ category" (Δ = "change"), siblings use CTX↑/LOAD↑ glyph shorthand. Read as typographic abbreviation; converting to $\\Delta$ is also defensible')
g('DL078', [('ι_ij)', r'$\iota_{ij}$)'), ('ι_ij ∈ [0,1]', r'$\iota_{ij} \in [0,1]$')], 'high', '')
g('DL178', [('Accept X', 'Accept $X$'), ('save Y', 'save $Y$'),
            ('X < n_future × Y', r'$X \lt ' + NFUT + r' \times Y$'),
            ('n_future = n_past', '$' + NFUT + ' = ' + NPAST + '$')], 'medium', 'same as DL143')
g('DN046', [], 'high', 'already formatted')
g('DN003', [], 'high', 'no math')
g('DL171', [('n ≥ 3', r'$n \ge 3$')], 'high', '')
g('DU030', [], 'medium',
  'mention, not use: the sentence says n_past lacks LaTeX formatting; converting would falsify it. Transcript register')
g('DL024', [('convergence-ε', r'convergence-$\varepsilon$'), ('Realized ⟂ Lawful', r'Realized $\perp$ Lawful')], 'medium',
  'only the relation symbol converted between words; whole-phrase \\text{} span is the alternative')
g('DL065', [('κ-as-scalar', r'$\kappa$-as-scalar')], 'high', '`κ_processing` is a code span, left alone')
g('DU007', [('S_id at', r'$S_{\text{id}}$ at'), ('S_id properties', r'$S_{\text{id}}$ properties')], 'high', '')
g('DU077', [('O(1)', '$O(1)$')], 'high', 'one_for_one/one_for_all are identifiers')
g('DU103', [('n_past', '$' + NPAST + '$')], 'high', '')
g('DL151', [('β(t) = β₀', r'$\beta(t) = \beta_0$')], 'high', '')
g('DU009', [('N=256', '$N=256$')], 'medium', 'fragment; parameter value')
g('DL182', [('β_exp', r'$\beta_{\text{exp}}$')], 'high', 'percent range left as prose')
g('DU113', [('P(T|T>t) ∝ 1/T²', r'$P(T \mid T \gt t) \propto 1/T^2$'), ('T>t', r'$T \gt t$')], 'high',
  '"for" kept as prose between spans')
g('DL140', [('ρ = 0.991', r'$\rho = 0.991$')], 'high', '')
g('DN010', [], 'high', 'already formatted; note existing $n_d^*$ uses raw * (house style wants \\ast) - left as-is since existing spans are out of scope')
g('DL147', [('a_k → 0', r'$a_k \to 0$'), ('I_n → 0', r'$I_n \to 0$'), ('ā/(1−η̄)', r'$\bar a/(1-\bar\eta)$')], 'high', '')
g('DL093', [('Sub-scope β', r'Sub-scope $\beta$'), ('sub-scope β entry', r'sub-scope $\beta$ entry'), ('buys β', r'buys $\beta$')],
  'high', 'context: ASF segments write sub-scope labels as $\\beta$ (counted in 01-aat-core/src)')
g('DN017', [], 'high', 'already formatted')
g('DU029', [('T_scan', r'$T_{\text{scan}}$')], 'high', '')
g('DL137', [('γ', r'$\gamma$')], 'high', '')
g('DL124', [('ρ = +0.87', r'$\rho = +0.87$')], 'high', '')
g('DN008', [], 'high', 'block-id anchor')
g('DL141', [('ρ(A)', r'$\rho(A)$'), ('E[size] ≤ 1/(1-ρ)', r'$E[\text{size}] \le 1/(1-\rho)$'),
            ('(I-A)^(-1)', '$(I-A)^{-1}$'), ('ρ=1', r'$\rho=1$')], 'high', 'E kept roman as written (\\mathbb{E} would be a reinterpretation)')
g('DL048', [('ρ(t)', r'$\rho(t)$'), ('Ω', r'$\Omega$')], 'high', '')
g('DL091', [('α = 0.118', r'$\alpha = 0.118$')], 'high', '')
g('DU016', [('p=.215', '$p=.215$'), ('Z=1.968', '$Z=1.968$'), ('p<.026', r'$p \lt .026$'), ('p>.026', r'$p \gt .026$')],
  'medium', 'statistics in prose; the quoted misprint "p>.026" converted too since it is still a math statement')
g('DU123', [('"P and P\'', '"$P$ and $P\'$'), ('ASTs of P and P\'', "ASTs of $P$ and $P'$"), ('variable x.', 'variable $x$.')],
  'medium', 'PDF-extracted paper; existing spans and odd outer spacing left untouched; P/P\' inside the pronunciation gloss also converted (original paper sets them italic)')
g('DL152', [('failed_systems(t) = {s ∈ S : dependency_timeout(s) < wait_time(B(t))}',
             r'$\text{failed\_systems}(t) = \{s \in S : \text{dependency\_timeout}(s) \lt \text{wait\_time}(B(t))\}$')],
  'medium', r'identifier names as \text with \_ (SOP alternative is "-"); set braces escaped')
g('DU021', [('O(n²)', '$O(n^2)$'), ('O(n))', '$O(n)$)')], 'high', '')
g('DU147', [('P_true', r'$P_{\text{true}}$')], 'low',
  'P_true clear. Context: the bold do(survey_population = doctors) is one of three parallel identifier-style labels incl. do(temporal_shift); left as prose. Converting it to \\mathrm{do}(...) is the main alternative')
g('DU042', [], 'medium', 'session listing; t=5 is a data field, not math')
g('DL193', [('proximity(changeset) = 1 / Σ(distance(change_i, change_j))',
             r'$\text{proximity}(\text{changeset}) = 1 / \sum(\text{distance}(\text{change}_i, \text{change}_j))$'),
            ('time_implementation ∝ 1/proximity(changeset)',
             r'$\text{time}_{\text{implementation}} \propto 1/\text{proximity}(\text{changeset})$')],
  'low', 'two formulas joined by unwrap -> two spans. Σ read as summation (\\sum). time_implementation read as subscript (parallel to change_i); \\text{time\\_implementation} is the alternative')
g('DL096', [('∪', r'$\cup$')], 'medium',
  'context needed and checked: "Symbol" column of a relational-algebra table (σ, π, ⋈, ×, ∪, −, ρ); math operator')
g('DL173', [('θ sweep', r'$\theta$ sweep'), ('κ findings', r'$\kappa$ findings'), ('χ-gain', r'$\chi$-gain')], 'high', 'code span untouched')
g('DN002', [], 'high', 'Unicode glyph table cell')
g('DU012', [('**cl_str(X)**', r'**$\mathrm{cl}_{\text{str}}(X)$**'), ('from X', 'from $X$'),
            ('P ⇒ cl_str reaches M⁻', r'$P \Rightarrow \mathrm{cl}_{\text{str}} \text{ reaches } M^-$'),
            ('H_D3', '$H_{D3}$')],
  'low', 'context checked: P is a proposition, cl_str a closure operator. Implication spanned whole with \\text{ reaches } so the arrow scopes correctly; alternative "$P \\Rightarrow \\mathrm{cl}_{\\text{str}}$ reaches $M^-$". H_D3 is a hypothesis label')
g('DL105', [('metric-α₂', r'metric-$\alpha_2$'), ('remains β;', r'remains $\beta$;'), ('remain β.', r'remain $\beta$.')], 'high',
  'sub-scope labels per ASF convention')
g('DN026', [], 'high', 'already formatted')
g('DL158', [('R3-ε-error', r'R3-$\varepsilon$-error')], 'medium', 'Greek inside a hyphenated item label')
g('DL059', [('Λ', r'$\Lambda$')], 'high', '')
g('DU002', [('n_past', '$' + NPAST + '$'), ('n_future', '$' + NFUT + '$')], 'high', '')
g('DU082', [('n_past', '$' + NPAST + '$')], 'high', '')
g('DN016', [], 'high', 'Unicode glyph table cell')
g('DU125', [('o_t', '$o_t$')], 'high', 'context: symbol column of a mapping table')
g('DL108', [(r'X < $\hat{n}_{\text{future}}$ × Y', r'$X \lt \hat{n}_{\text{future}} \times Y$'),
            ('5 < 20 × 0.25', r'$5 \lt 20 \times 0.25$')], 'medium',
  'existing span absorbed into the whole inequality (high); the numeric check is the medium part')
g('DL019', [('λ = 1 − α_c/ν_c', r'$\lambda = 1 - \alpha_c/\nu_c$'), ('dV/dt', '$dV/dt$'), ('f_c(·, o)', r'$f_c(\cdot, o)$')], 'high', '')
g('DL031', [(r'η\*', r'$\eta^\ast$')], 'high', 'markdown-escaped asterisk is the superscript star')
g('DN009', [], 'high', 'already formatted')
g('DU148', [('Δρ*', r'$\Delta\rho^\ast$'), ('R* exceeds R.', r'$R^\ast$ exceeds $R$.')], 'medium',
  'source "Δρ***" = Δρ* + closing bold; first asterisk taken as the star, "**" left to close bold')
g('DU067', [('W₂ compliance', '$W_2$ compliance'), ('W₂-ish', '$W_2$-ish')], 'high', '')
g('DL010', [], 'low',
  'PDF-extracted text with lost glyphs ("7→" is a broken ↦, an edge arrow missing in "sb0 s", "#1" scope names). An honest fix needs reconstruction outside the allowed shape; partial spans would enshrine the garble')
g('DU112', [('M_t', '$M_t$'), ('Sigma_t', r'$\Sigma_t$'), ('O_t', '$O_t$')], 'medium', 'spelled-out Sigma read as the Greek symbol')
g('DL068', [('ε', r'$\varepsilon$')], 'high', 'heading')
g('DL104', [('any F', 'any $F$'), ('δᵀF ≥ α‖δ‖²', r'$\delta^\top F \ge \alpha \lVert\delta\rVert^2$')], 'high', '')
g('DL119', [('λ_max', r'$\lambda_{\max}$')], 'high', '')
g('DU105', [('T_obs + T_explore + T_probe', r'$T_{\text{obs}} + T_{\text{explore}} + T_{\text{probe}}$')], 'medium',
  'kept T as written; SOP uses \\mathcal{T} for tempo, so the author might want $\\mathcal{T}$')
g('DU131', [('G₃', '$G_3$'), ('G₁', '$G_1$')], 'high', '')
g('DL052', [('λ_min', r'$\lambda_{\min}$')], 'high', 'code spans `ε₁ I`, `I_min` left alone')
g('DL035', [('n_future ≈ 20-50', '$' + NFUT + r' \approx 20\text{-}50$')], 'medium',
  'range hyphen kept as text inside the span so it does not render as minus')
g('DL064', [('μ(T)', r'$\mu(T)$')], 'high', '')
g('DL009', [('κ × 𝒜', r'$\kappa \times \mathcal{A}$')], 'high', '')
g('DU037', [('N=2-3', r'$N=2\text{-}3$')], 'low', 'casual range; leaving it prose is equally defensible')
g('DL121', [('Σ_t', r'$\Sigma_t$'), ('p_ij = P(j | do(i), M_t)', r'$p_{ij} = P(j \mid \mathrm{do}(i), M_t)$')], 'high', '')
g('DU053', [('G_t', '$G_t$')], 'high', 'heading')
g('DU132', [('W₂', '$W_2$'), ('W₁', '$W_1$')], 'high',
  'item is a whole table row incl. pipes, not a cell; prose arrow observation→memory left')
g('DU099', [('W^{12}', '$W^{12}$')], 'high', 'arrow is prose "leads to"')
g('DL139', [('Accept X', 'Accept $X$'), ('save Y', 'save $Y$'), ('X < n_future × Y', r'$X \lt ' + NFUT + r' \times Y$')],
  'medium', 'same as DL143; slug refs and "580 ·" separator left')
g('DU096', [('p<0.0001', r'$p \lt 0.0001$'), ('p=0.0002', '$p=0.0002$')], 'medium',
  'p-values only; percentages, IRR ranges and N× multipliers left as prose')
g('DU032', [('n=64', '$n=64$'), (r'$\rho$ ≈ 5.3', r'$\rho \approx 5.3$')], 'medium',
  'existing span widened to the relation; the "x" multiplier suffix left as prose')
g('DU098', [('W_eff', r'$W_{\text{eff}}$')], 'high', '')
g('DU041', [('N=2-3', r'$N=2\text{-}3$')], 'low', 'same as DU037')
g('DU046', [('discontinuity_cost < duplication_cost × n_future',
             r'$\text{discontinuity\_cost} \lt \text{duplication\_cost} \times ' + NFUT + '$')], 'medium',
  'identifier names as \\text with \\_')
g('DU058', [('angle ≤ 45°', r'angle $\le 45^\circ$'), ('√(1−r²)', r'$\sqrt{1-r^2}$'), (r'$\alpha$₃', r'$\alpha_3$')], 'high',
  'existing $\\alpha$ absorbed with its stray Unicode subscript')
g('DL125', [('ρ = 0.96', r'$\rho = 0.96$')], 'high', 'MAE = 5.9% left as prose metric report')
g('DL085', [('η_edge', r'$\eta_{\text{edge}}$')], 'high', '')
g('DL176', [('(Hκ)', r'($H_\kappa$)')], 'medium',
  'context needed and checked: the paper writes this assumption label as (H$_\\kappa$), i.e. κ is a subscript; without context $H\\kappa$ would be the naive reading')
g('DU088', [('n=15', '$n=15$')], 'medium', 'abstract text; double spaces preserved')
g('DU050', [('O(1)', '$O(1)$')], 'high', '')
g('DL066', [('training ρ', r'training $\rho$')], 'high', '')
g('DU047', [('O(1)', '$O(1)$')], 'high', '')
g('DL132', [('(Σ)', r'($\Sigma$)')], 'medium', 'model label Σ matches the $\\rho_\\Sigma$ subscripts; rest already formatted')
g('DL023', [('δ:', r'$\delta$:')], 'high', '')
g('DL097', [('δ_regret = A_O(M_t; Π, N_h) − V_O(M_t, π_current; N_h) ≥ 0',
             r'$\delta_{\text{regret}} = A_O(M_t; \Pi, N_h) - V_O(M_t, \pi_{\text{current}}; N_h) \ge 0$'),
            ('Σ_t revision', r'$\Sigma_t$ revision'),
            ('δ_sat', r'$\delta_{\text{sat}}$'), ('δ_regret', r'$\delta_{\text{regret}}$'),
            ('M_t/Π/N_h', r'$M_t/\Pi/N_h$'), ('O_t', '$O_t$'),
            ('δ_sat', r'$\delta_{\text{sat}}$'), ('δ_regret', r'$\delta_{\text{regret}}$'),
            ('Σ_t first', r'$\Sigma_t$ first')], 'medium',
  'definition high; diagnostic lines mix words and symbols ("large δ_sat + small δ_regret = ...") so only symbols spanned')
g('DL075', [], 'high', 'context needed and checked: status-marker column in a file table (~ / ∅), not math')
g('DU026', [], 'medium', 'session listing; t=7 is a data field')
g('DL001', [(r'$\alpha$ ≈ 2-5', r'$\alpha \approx 2\text{-}5$')], 'medium', 'existing span widened to the relation; range hyphen as text')
g('DL077', [('sub-scope β', r'sub-scope $\beta$'), ('metric-α,', r'metric-$\alpha$,')], 'high', 'ASF sub-scope label convention')
g('DU028', [('F²', '$F^2$')], 'high', '')
g('DL086', [], 'medium', 'arrows describe a diagram flow between existing spans; prose')
g('DL196', [('α ≈ 0.2', r'$\alpha \approx 0.2$')], 'high', '')
g('DU074', [('O(h²)', '$O(h^2)$'), ('O(1)', '$O(1)$')], 'high', '')
g('DL186', [('α ≈ 0.3', r'$\alpha \approx 0.3$'), ('β ≈ 0.1', r'$\beta \approx 0.1$')], 'high', '"10x" left')
g('DL074', [('⊂', r'$\subset$'), ('α/β', r'$\alpha$/$\beta$')], 'medium',
  'sub-scope labels per ASF habit ($\\alpha$/$\\beta$); ⊂ between words converted alone')
g('DL102', [('{α, α₁, α₂, α₃, α\', β}', r"$\{\alpha, \alpha_1, \alpha_2, \alpha_3, \alpha', \beta\}$")], 'high', 'set of sub-scope labels')
