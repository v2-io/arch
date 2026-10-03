#!/usr/bin/env python3
"""Build pass2/batch-0 golds as exact substring replacements on the input text,
then check the shape constraint (outside-span chars unchanged, in order) and
house rules inside new spans."""
import json, re, sys
from pathlib import Path

BASE = Path(__file__).resolve().parent.parent
TASKS = BASE / "tasks" / "batch-0.jsonl"
OUT = BASE / "pass2" / "batch-0.jsonl"

NP = r"$n_{\text{past}}$"
NF = r"$n_{\text{future}}$"

# gid -> (list of (old, new[, count]), conf, note).  Empty list = unchanged.
L = {}
def e(gid, reps, conf, note):
    L[gid] = (reps, conf, note)

e("DU018", [], "medium", "W₂ is a regime label; asf canon (der-class-coercion-via-wrapping) writes W₀/W₁/W₂ as Unicode in prose, $…$ only inside math. Existing spans fine.")
e("DL007", [("N_h² displayed", r"$N_h^2$ displayed"),
            ("(V_max·N_h^{3/2}√(SA);", r"($V_{\max}\cdot N_h^{3/2}\sqrt{SA}$;"),
            ("gives N_h²)", r"gives $N_h^2$)")], "high", "√(SA) parens were plain-text grouping → \\sqrt{SA}.")
e("DL080", [("Σ_t = (V, E, p, γ)", r"$\Sigma_t = (V, E, p, \gamma)$")], "high", "")
e("DU044", [("comprehension-as-M_t-construction", r"comprehension-as-$M_t$-construction")], "medium", "M_t inside a hyphenated compound; math span inside the compound is what I'd write.")
e("DL039", [("tolerance ε)", r"tolerance $\varepsilon$)")], "high", "[P + D] is a tag, not math.")
e("DU056", [], "high", "Context: a diff listing; T_active is a template name, → is prose.")
e("DN005", [], "high", "Nothing new to promote. Existing span has raw * (R^*) against the \\ast rule, but existing math is masked/out of scope.")
e("DL099", [("hidden in α;", r"hidden in $\alpha$;")], "high", "")
e("DU130", [("0.9 × 0.5^(100 × 0.01 / 365) ≈ 0.88", r"$0.9 \times 0.5^{100 \times 0.01 / 365} \approx 0.88$")], "high", "Plain-text exponent parens become braces.")
e("DU080", [], "medium", "W₀/W₁/W₂ regime labels left as Unicode per canon practice (see DU018).")
e("DU135", [], "medium", "W₀/W₂ regime labels left as Unicode per canon practice.")
e("DU078", [("with n_past\"", "with " + NP + "\"")], "medium", "n_past is a TST variable; canon writes n_{\\text{past}} (45 uses). Transcript text, inside a quoted claim.")
e("DU043", [], "medium", "Table-cell regime label W₁ᶜ (variants W₀⁺, W₀ᴱ, W₁ⁱ nearby); labels stay Unicode like canon W₁. Checked context.")
e("DL116", [], "high", "μTOSCA is a name.")
e("DL189", [("coefficient φ provides", r"coefficient $\phi$ provides"),
            ("minimize T_comm.", r"minimize $T_{\text{comm}}$.")], "high", "\\phi to match canon (170 \\phi vs 3 \\varphi).")
e("DL163", [], "medium", "Could absorb into $O_t \\to V_{O_t}$, but the arrow parallels '($\\Sigma_t$ → action)', a prose mapping; leaving the author's spans.")
e("DU063", [("(n_past < 3 from", r"($n_{\text{past}} \lt 3$ from")], "high", "")
e("DL117", [("deterministic-π* as", r"deterministic-$\pi^\ast$ as"),
            ("at deterministic-π* itself", r"at deterministic-$\pi^\ast$ itself")], "high", "")
e("DU051", [("with n_future approaching", "with " + NF + " approaching")], "medium", "n_future as TST variable (see DU078).")
e("DL181", [("reduce ‖δ_G‖.", r"reduce $\lVert\delta_G\rVert$.")], "high", "")
e("DU095", [("n_future ≫ n_past", r"$n_{\text{future}} \gg n_{\text{past}}$")], "high", "")
e("DL103", [("(x = deliberation time Δτ)", r"($x$ = deliberation time $\Delta\tau$)"),
            ("\"Benefit: Δη*·‖δ‖.\"", r'"Benefit: $\Delta\eta^\ast\cdot\lVert\delta\rVert$."'),
            ("\"Cost: ρ·Δτ.\"", r'"Cost: $\rho\cdot\Delta\tau$."'),
            ("cross at Δτ* (marked", r"cross at $\Delta\tau^\ast$ (marked"),
            ("massive Δτ.\"", r'massive $\Delta\tau$."')], "medium", "The x axis label: $x$ is my call; leaving it bare is defensible. The rest is high.")
e("DL187", [("it T = Σ ν·η*)", r"it $T = \sum \nu\cdot\eta^\ast$)")], "medium", "Kept T as written, though house would write \\mathcal{T} for tempo. Σ = summation over channels. '≠' between words is prose.")
e("DN012", [], "high", "")
e("DL081", [("MDP tuple ⟨S, X, T, R⟩ with", r"MDP tuple $\langle S, X, T, R\rangle$ with"),
            ("fixed transition T : S × X × S → [0,1] and", r"fixed transition $T : S \times X \times S \to [0,1]$ and"),
            ("fixed reward R : S × X → ℝ.", r"fixed reward $R : S \times X \to \mathbb{R}$."),
            ("fixed γ ∈ [0,1).", r"fixed $\gamma \in [0,1)$."),
            ("at every t is", r"at every $t$ is")], "high", "Bare 't' (time index) included.")
e("DL109", [("(α^depth)", r"($\alpha^{\text{depth}}$)")], "high", "")
e("DL032", [("ε-exploration", r"$\varepsilon$-exploration")], "high", "")
e("DN048", [], "high", "Filename.")
e("DL003", [("(frozen η →", r"(frozen $\eta$ →")], "high", "Arrows are prose.")
e("DL069", [("δλ = v^T δA v", r"$\delta\lambda = v^T \delta A v$")], "high", "")
e("DL060", [("implicit-scheme δt error", r"implicit-scheme $\delta t$ error")], "high", "Citation numbers left.")
e("DL014", [("(challenges (H_κ);", r"(challenges ($H_\kappa$);")], "high", "Hypothesis label; parens stay prose like (A5).")
e("DL123", [("crossing τ_high with", r"crossing $\tau_{\text{high}}$ with")], "high", "'Tier A = 0.98' is a quoted code-ish literal; left.")
e("DU124", [], "medium", "Only m² (a unit), left as Unicode; code span masked.")
e("DL168", [("C3 σ-algebra", r"C3 $\sigma$-algebra")], "high", "")
e("DN022", [], "high", "Display math; asterisk rule doesn't apply to $$.")
e("DU107", [("Derived. f_M is G_t-independent. f_G depends on M_t.", r"Derived. $f_M$ is $G_t$-independent. $f_G$ depends on $M_t$.")], "high", "")
e("DL101", [("Borel σ-algebra", r"Borel $\sigma$-algebra")], "high", "\\cite/[[...]] outside math left as is.")
e("DU100", [], "high", "Character table entry.")
e("DL030", [("chronica C_t is", r"chronica $C_t$ is"),
            ("tempo ν\"", r'tempo $\nu$"'),
            ("(lossy φ means", r"(lossy $\phi$ means"),
            ("from M_t alone", r"from $M_t$ alone")], "high", "C_t kept as written (canon would be \\mathcal C_t). ν is inside a quoted row label but still a symbol.")
e("DU049", [("Results — H through time (for Δρ_CLM = 25 kg/m³)", r"Results — $H$ through time (for $\Delta\rho_{\text{CLM}} = 25$ kg/m³)")], "medium", "Context needed: H = elevation (table header below). Units left outside the span.")
e("DU019", [("**n\u0302_future** =", r"**$\hat{n}_{\text{future}}$** =")], "high", "n̂ is n + combining circumflex.")
e("DU094", [("suggests n > 10-20, but", r"suggests $n \gt 10\text{-}20$, but")], "low", "Range bound. I put the whole 'n > 10-20' in one span with a text dash (bare '-' would be minus). $n \\gt 10$-20 is the main alternative.")
e("DL198", [("options C₁, C₂ for feature F:", r"options $C_1, C_2$ for feature $F$:"),
            ("if time(C₁) > time(C₂)  //", r"if $\mathrm{time}(C_1) \gt \mathrm{time}(C_2)$  //"),
            ("// C₁ takes longer", r"// $C_1$ takes longer"),
            ("but E[Σ(time_future(Fᵢ | C₁))] < E[Σ(time_future(Fᵢ | C₂))]  //",
             r"but $E[\sum(\mathrm{time}_{\text{future}}(F_i \mid C_1))] \lt E[\sum(\mathrm{time}_{\text{future}}(F_i \mid C_2))]$  //"),
            ("then prefer(C₁) ∝ E[num_future_changes]", r"then $\mathrm{prefer}(C_1) \propto E[\text{num-future-changes}]$")],
  "low", "Context: 3-line indented pseudo-code under 'Formal Expression' (not fenced), joined into one line by the unwrap, so '//' comments now swallow the following code. Spans per formula; if/but/then left as prose. The SOP's _→- rule inside \\text changes num_future_changes.")
e("DL135", [("(α) deterministic", r"($\alpha$) deterministic"),
            ("(β) commensurate", r"($\beta$) commensurate"),
            ("(γ) content-addressed", r"($\gamma$) content-addressed")], "medium", "Greek enumerators; canon writes ($\\alpha$)/($\\beta$)/($\\gamma$). Leaving them Unicode is defensible.")
e("DL037", [("Formulation M_t = φ(C_t):", r"Formulation $M_t = \phi(C_t)$:"),
            ("to model space M.", r"to model space $M$."),
            ("complete state M_t that", r"complete state $M_t$ that")], "high", "")
e("DU060", [("priors (n_past)", "priors (" + NP + ")")], "medium", "n_past convention (DU078).")
e("DU008", [("O(n)", r"$O(n)$")], "medium", "Table cell in a cost column (siblings O(n log n), O(n·oct)); checked context.")
e("DU081", [("(n=1)", r"($n=1$)")], "high", "")
e("DL025", [("stale-kernel max|Δh|≈150 m,", r"stale-kernel $\max\lvert\Delta h\rvert\approx 150$ m,")], "medium", "≡ between file:line refs is prose. Unit outside.")
e("DL008", [("|δ|_ss = ρ / (ν · η*)", r"$\lvert\delta\rvert_{\text{ss}} = \rho / (\nu \cdot \eta^\ast)$")], "high", "")
e("DL148", [("∀ implementations I₁, I₂ of feature F: if ∀ metric m ∈ M\\{time}: m(I₁) = m(I₂) then optimal(I₁, I₂) = argmin(time(I₁), time(I₂))",
             r"$\forall$ implementations $I_1, I_2$ of feature $F$: if $\forall$ metric $m \in M \setminus \{\text{time}\}$: $m(I_1) = m(I_2)$ then $\mathrm{optimal}(I_1, I_2) = \operatorname{argmin}(\mathrm{time}(I_1), \mathrm{time}(I_2))$")],
  "low", "Context: indented pseudo-logic under 'Formal Expression'. 'M\\{time}' is set difference, but as markdown it renders 'M{time}'. Where to cut between logic words and math is a judgment call.")
e("DL094", [("whether S_system < S_1 × S_2 to", r"whether $S_{\text{system}} \lt S_1 \times S_2$ to")], "high", "")
e("DL055", [("*\"α > ρ/R iff persistence\"*", r'*"$\alpha \gt \rho/R$ iff persistence"*')], "high", "'iff persistence' is prose.")
e("DU084", [("— n=9 documented", r"— $n=9$ documented")], "medium", "Count shorthand in an ops doc; a math span is what I'd write, plain text is also fine.")
e("DL185", [("definition κ_processing = I(G_t ; M_τ⁺ | e_τ) / H(G_t | e_τ) is", r"definition $\kappa_{\text{processing}} = I(G_t ; M_\tau^+ \mid e_\tau) / H(G_t \mid e_\tau)$ is")], "high", "")
e("DL040", [], "high", "Context: a grade change D → E in a table; not math.")
e("DL150", [("whether T_agent > ρ_environment.", r"whether $T_{\text{agent}} \gt \rho_{\text{environment}}$.")], "high", "")
e("DU076", [("as O(n²) in", r"as $O(n^2)$ in")], "high", "")
e("DU039", [("for n < 3 uses", r"for $n \lt 3$ uses")], "high", "")
e("DL022", [("$\\lambda$ ≈ 2.0", r"$\lambda \approx 2.0$")], "medium", "Absorbs the existing $\\lambda$ span with its value. '$c$ = completeness…' stays a prose definition.")
e("DU134", [("— O(1) relative", r"— $O(1)$ relative")], "medium", "−0.5 is a plain (bold) number; ⇒ used as a prose connective. Both left.")
e("DL136", [("(α/β/γ) conjunction", r"($\alpha$/$\beta$/$\gamma$) conjunction")], "medium", "Labels; canon writes $\\alpha$/$\\beta$ as separate spans around a prose slash.")
e("DU025", [("treats n_past like", "treats " + NP + " like"),
            ("\"check n_past\"", "\"check " + NP + "\"")], "medium", "n_past convention (DU078).")
e("DU033", [("from O(n²) to O(n) where n is", r"from $O(n^2)$ to $O(n)$ where $n$ is")], "high", "")
e("DL063", [], "medium", "The honest fix is $\\lvert\\text{configs}\\rvert \\approx$ 5–10 (the existing span has raw | and an italic word), but that rewrites existing math. I left it rather than absorb ≈ 5-10 into a span with a range.")
e("DL042", [("γ_t : V_t → {AND, OR}:", r"$\gamma_t : V_t \to \{\text{AND}, \text{OR}\}$:")], "high", "")
e("DL142", [("sub-scope β.", r"sub-scope $\beta$.")], "high", "Canon writes 'sub-scope $\\beta$' (26 uses).")
e("DL162", [("by κ × 𝒜.", r"by $\kappa \times \mathcal{A}$.")], "high", "")
e("DU024", [], "high", "Context: grade E with footnote marker ³ in a table; not E cubed.")
e("DL131", [("(α ≈ 1.5-2.0, λ ≈ 0.01-0.05)", r"($\alpha \approx 1.5\text{-}2.0$, $\lambda \approx 0.01\text{-}0.05$)")], "low", "Ranges: whole range in the span with a text dash (see DU094). '$\\alpha \\approx 1.5$-2.0' is the alternative.")
e("DN030", [], "high", "send_message is a code identifier.")
e("DN040", [], "high", "")
e("DL155", [("impact(t) = Σ(users(s) × criticality(s)) for s ∈ failed_systems(t)",
             r"$\mathrm{impact}(t) = \sum(\mathrm{users}(s) \times \mathrm{criticality}(s))$ for $s \in \text{failed-systems}(t)$")],
  "medium", "'for' left as prose between spans. failed_systems → \\text{failed-systems} per SOP (_ in \\text breaks GitHub), which changes how the identifier is spelled.")
e("DU142", [], "medium", "W₂/W₁ regime labels left as Unicode per canon practice.")
e("DU038", [("Count actual n_past in", "Count actual " + NP + " in")], "medium", "n_past convention (DU078).")
e("DU020", [("denoting that C supports an edge (s, l, s') for some s'.", r"denoting that $C$ supports an edge $(s, l, s')$ for some $s'$.")], "high", "PDF-extract text; the padding around existing spans is outside them and left.")
e("DL110", [("**k=80**", r"**$k=80$**"), ("at k≈60", r"at $k\approx 60$"), ("at k≈120.", r"at $k\approx 120$.")], "high", "L2-normalize, 50–100 left.")
e("DL098", [("(mean + 1.5σ)", r"($\text{mean} + 1.5\sigma$)"),
            ("different k values", r"different $k$ values"),
            ("threshold for τ (tipping", r"threshold for $\tau$ (tipping")], "medium", "Whole 'mean + 1.5σ' as one expression; 'mean + $1.5\\sigma$' is the alternative.")
e("DU066", [("For n_past < 10:", r"For $n_{\text{past}} \lt 10$:")], "high", "")
e("DN041", [], "high", "→ and ⇒ are prose; code span masked.")
e("DU052", [("mapping: P(o_{t+1} | do(a_t), M_t).", r"mapping: $P(o_{t+1} \mid \mathrm{do}(a_t), M_t)$.")], "high", "")
e("DL165", [("mutant ν appears", r"mutant $\nu$ appears")], "high", "")
e("DL083", [], "high", "Use-mention: names the glyph 𝒜 as the thing being swept to $\\mathcal{A}$. Promoting it would erase the meaning.")
e("DU121", [("uses T_p = 1430 °C (present T*_p = 1280 °C)", r"uses $T_p = 1430$ °C (present $T_p^\ast = 1280$ °C)")], "medium", "Units outside the spans. Temperature ranges in °C are prose.")
e("DN033", [], "high", "")
e("DN047", [], "high", "")
e("DN006", [], "high", "Math is inside a code span (masked); ≡ is between code and words.")
e("DL192", [("Develop the G_t formalism", r"Develop the $G_t$ formalism"), ("Now O_t/Σ_t.", r"Now $O_t$/$\Sigma_t$.")], "medium", "Slash means 'and', so separate spans (canon style); one $O_t/\\Sigma_t$ span would also render.")
e("DU120", [("~10⁵-entry", r"~$10^5$-entry")], "low", "10⁵ already renders as a glyph, so promoting it gains little. Tilde left as prose 'about'.")
e("DN011", [], "high", "Character table.")
e("DL082", [("The χ criterion", r"The $\chi$ criterion")], "high", "")
e("DU114", [("Limited K_tacit means", r"Limited $K_{\text{tacit}}$ means")], "high", "")
e("DU010", [], "high", "t=45 is a metadata field (session turns) in a list of session ids; checked context.")
e("DN019", [], "high", "Character table.")
e("DL012", [("**δ_strategic (calibration residual)**: L² aggregation", r"**$\delta_{\text{strategic}}$ (calibration residual)**: $L^2$ aggregation")], "high", "")
e("DN001", [], "high", "")
e("DL084", [("a_t = π(M_t)    [deterministic] a_t ~ π(·|M_t)  [stochastic]", r"$a_t = \pi(M_t)$    [deterministic] $a_t \sim \pi(\cdot \mid M_t)$  [stochastic]")], "high", "")
e("DL134", [("dynamics of ‖δ‖ over", r"dynamics of $\lVert\delta\rVert$ over")], "high", "")
e("DU091", [("\\(\\mathrm{Hel}^2(P,Q) := 1 - \\mathrm{BC}(P,Q)\\)", r"$\mathrm{Hel}^2(P,Q) := 1 - \mathrm{BC}(P,Q)$"),
            ("\\(\\mathrm{Hel}\\in[0,1]\\)", r"$\mathrm{Hel}\in[0,1]$")], "medium", "\\(…\\) doesn't render on GitHub or Obsidian (\\( is an escaped paren), so I converted the delimiters to $…$ and kept the contents. Policy question: is existing \\(…\\) 'existing math' to leave alone?")
e("DL129", [("99.0% Δp", r"99.0% $\Delta p$")], "medium", "Table row label in a vendored llama.cpp README, which probably shouldn't be pressed at all.")
e("DU104", [("robust result. p^n is", r"robust result. $p^n$ is")], "high", "")
e("DU022", [("investment < n_future × savings", r"$\text{investment} \lt n_{\text{future}} \times \text{savings}$")], "medium", "Whole inequality, words as \\text; it needs a span anyway for the raw <.")
e("DN025", [], "high", "Already math (italic word-names inside are an existing-span issue).")
e("DU102", [], "medium", "H_D3 is a hypothesis id (H_D1, H_F_ctrl…); causal-language never writes it in math (1300+ bare uses). Left.")
e("DL070", [("form of φ subject", r"form of $\phi$ subject")], "high", "")
e("DU087", [], "medium", "Hypothesis id label in a table, as in DU102.")
e("DU073", [("#Deps(Mj): number", r"#Deps($M_j$): number"),
            ("(⋃ℓ>i Lℓ)", r"($\bigcup_{\ell \gt i} L_\ell$)"),
            ("any file in Mj (reachability", r"any file in $M_j$ (reachability")],
  "low", "Context: the doc writes Mj, Li, L1,…,Ln with subscripts flattened, so I restored them. #Deps is kept as a prose metric name.")
e("DL067", [("If M_t has a persistence condition (T > ρ / ‖δ_critical‖), does G_t have",
             r"If $M_t$ has a persistence condition ($T \gt \rho / \lVert\delta_{\text{critical}}\rVert$), does $G_t$ have")], "high", "T kept (not \\mathcal{T}) as written.")
e("DU036", [("submodel M_x (replace f_X with X = x)", r"submodel $M_x$ (replace $f_X$ with $X = x$)")], "high", "")
e("DL184", [("(θ row and", r"($\theta$ row and"), ("run Fr > 1.5.", r"run $\mathrm{Fr} \gt 1.5$.")], "high", "Fr = Froude number, upright.")
e("DU138", [], "medium", "Context: S₀/S₁/S₂ class labels in a table (also used in code spans); treated like W labels and left.")
e("DU014", [("element \\(a\\), a label/score \\(Q_a(r)\\) indicating architectural quality at version \\(r\\).",
             r"element $a$, a label/score $Q_a(r)$ indicating architectural quality at version $r$.")], "medium", "\\(…\\) → $…$ (see DU091).")
e("DL036", [("software Ω_t scope", r"software $\Omega_t$ scope")], "high", "L1/L2 left as prose spellings (L1 may be a level label here).")
e("DU059", [("given n_past?", "given " + NP + "?")], "medium", "n_past convention (DU078).")
e("DU034", [("split, G_t complexity bounded by M_t, compound", r"split, $G_t$ complexity bounded by $M_t$, compound")], "high", "")
e("DU140", [], "medium", "W₀/W₁/W₂ regime labels left as Unicode (canon practice). Arrows in the typology map are prose. Existing spans fine.")
e("DL120", [("A2'-scope α₁/α₂/β)", r"A2'-scope $\alpha_1$/$\alpha_2$/$\beta$)")], "medium", "Scope labels: canon puts Greek sub-scope labels in math and writes slash lists as separate spans.")
e("DL051", [("chain A→B→G.", r"chain $A \to B \to G$.")], "high", "")
e("DL177", [("exact under deterministic π*)", r"exact under deterministic $\pi^\ast$)")], "high", "")
e("DL020", [("Ω, O, A, h, T, ε", r"$\Omega$, $O$, $A$, $h$, $T$, $\varepsilon$")], "medium", "Table cell listing symbols; separate spans per symbol. One span '$\\Omega, O, A, h, T, \\varepsilon$' is equally defensible.")
e("DL033", [("read as \"in δ(P)\"", r'read as "in $\delta(P)$"'),
            ("with equal δ can", r"with equal $\delta$ can"),
            ("gauge diameter → 0, hence", r"gauge diameter $\to 0$, hence")], "medium", "'→ 0' means 'tends to zero', so I promoted it; leaving the arrow as prose is defensible.")
e("DU031", [("(n < 100)", r"($n \lt 100$)")], "high", "")
e("DU108", [("$\\Delta^{\\downarrow}$ ,  $\\Delta^{\\uparrow} \\vdash C$", r"$\Delta^{\downarrow}, \Delta^{\uparrow} \vdash C$"),
            ("the constraint C requires", r"the constraint $C$ requires")],
  "medium", "PDF extraction split the judgment Δ↓, Δ↑ ⊢ C into two spans; I absorbed both whole into one. Leaving them split is the conservative alternative.")
e("DU128", [("high n_future)", "high " + NF + ")")], "medium", "n_future convention.")
e("DL062", [("Cross-validation ρ = 0.991 against", r"Cross-validation $\rho = 0.991$ against")], "high", "")
e("DN029", [], "high", "Nothing new. Existing span has raw < and \\| (house violations), but existing math is masked.")
e("DL166", [("Bandit, |A| = 2, action gap Δ = 0.1.", r"Bandit, $\lvert A\rvert = 2$, action gap $\Delta = 0.1$."),
            ("for t ∈ [1, T/2], then", r"for $t \in [1, T/2]$, then"),
            ("for t ∈ [T/2 + 1, T].", r"for $t \in [T/2 + 1, T]$."),
            ("Argmax a* is", r"Argmax $a^\ast$ is")], "high", "Code spans masked; 'Argmax' left as a prose word.")
e("DU110", [("O(n²) relationships", r"$O(n^2)$ relationships")], "high", "")
e("DU064", [("\\(\\mathrm{Hel}\\)", r"$\mathrm{Hel}$")], "medium", "\\(…\\) → $…$ (see DU091).")
e("DU115", [("(n_c < 5)", r"($n_c \lt 5$)")], "high", "")
e("DL021", [("high-κ, high-$\\mathcal{A}$", r"high-$\kappa$, high-$\mathcal{A}$"),
            ("The κ × A law", r"The $\kappa \times A$ law")], "medium", "'κ × A' almost certainly means κ × 𝒜 (the same cell writes $\\mathcal{A}$), but I kept plain A as written.")
e("DU136", [], "high", "Already $W_1$/$W_2$; nothing new.")
e("DN015", [], "high", "Code span.")

SPAN = re.compile(r"(?<!\\)\$(?!\$)(.+?)(?<!\\)\$")

def check_shape(src, gold):
    """gold must equal src with some contiguous regions replaced by $…$ spans."""
    parts, last = [], 0
    for m in SPAN.finditer(gold):
        parts.append(gold[last:m.start()]); last = m.end()
    parts.append(gold[last:])
    pat = "^" + "(.*?)".join(re.escape(p) for p in parts) + "$"
    return re.match(pat, src, re.S) is not None

def check_spans(src, gold):
    probs = []
    old = set(m.group(0) for m in SPAN.finditer(src))
    for m in SPAN.finditer(gold):
        s, body = m.group(0), m.group(1)
        if s in old: continue
        if body != body.strip(): probs.append("padding " + s)
        if re.search(r"[<>*]", body): probs.append("raw <>* " + s)
        if re.search(r"(?<!\\)\|", body) or "\\|" in body: probs.append("raw bar " + s)
        if body.count("{") != body.count("}"): probs.append("braces " + s)
        if re.search(r"[^\x00-\x7f]", body): probs.append("non-ascii " + s)
    return probs

items = [json.loads(l) for l in TASKS.open()]
missing = [d["gid"] for d in items if d["gid"] not in L]
extra = set(L) - {d["gid"] for d in items}
assert not missing and not extra, (missing, extra)

out, bad = [], 0
for d in items:
    reps, conf, note = L[d["gid"]]
    g = d["text"]
    for r in reps:
        old, new = r[0], r[1]
        n = g.count(old)
        if n != 1:
            print(f"!! {d['gid']}: {old!r} occurs {n}x"); bad += 1; continue
        g = g.replace(old, new)
    if not check_shape(d["text"], g):
        print(f"!! {d['gid']}: shape violation"); bad += 1
    for p in check_spans(d["text"], g):
        print(f"!! {d['gid']}: {p}"); bad += 1
    out.append({"gid": d["gid"], "gold": g, "conf": conf, "note": note})

if bad and "--force" not in sys.argv:
    sys.exit(f"{bad} problems; not writing")
OUT.parent.mkdir(parents=True, exist_ok=True)
with OUT.open("w") as f:
    for o in out:
        f.write(json.dumps(o, ensure_ascii=False) + "\n")
from collections import Counter
print("wrote", len(out), "items;", Counter(o["conf"] for o in out),
      "; changed:", sum(o["gold"] != d["text"] for o, d in zip(out, items)))
