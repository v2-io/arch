#!/usr/bin/env python3
"""Pass-1 gold labels for gold/D batch-0.

Each label is a list of (old, new) edits applied left-to-right with a cursor;
every `new` must be a single $...$ span, so the output is in-shape by
construction (input with contiguous regions replaced by $...$ spans).
"""
import json, sys, pathlib

HERE = pathlib.Path(__file__).resolve().parent
TASKS = HERE.parent / "tasks" / "batch-0.jsonl"
OUT = HERE.parent / "pass1" / "batch-0.jsonl"

NP = r"n_{\text{past}}"
NF = r"n_{\text{future}}"
WLAB = ("W-regime names (W₀/W₁/W₂) left raw: canon prose writes them as labels "
        "(der-class-coercion-via-wrapping, impl-orient-cascade); math only as "
        "subscripts like \\kappa_{W_1}. Alternative: $W_2$.")
TRANSCRIPT = " Source is a dialog transcript; whether to touch it at all is a file-level question."

L = {}
def lab(gid, edits, conf, note):
    L[gid] = (edits, conf, note)

lab("DU018", [], "medium", WLAB)
lab("DL007", [("N_h²", "$N_h^2$"),
              ("V_max·N_h^{3/2}√(SA)", r"$V_{\max} \cdot N_h^{3/2}\sqrt{SA}$"),
              ("N_h²", "$N_h^2$")], "high", "Parens around the middle expression are prose.")
lab("DL080", [("Σ_t = (V, E, p, γ)", r"$\Sigma_t = (V, E, p, \gamma)$")], "high", "")
lab("DU044", [("M_t", "$M_t$")], "medium",
    "M_t inside a hyphenated compound; span only the symbol.")
lab("DL039", [("ε", r"$\varepsilon$")], "high", "")
lab("DU056", [], "medium",
    "Context: a captured diff in a terminal transcript; T_active is a template name, → is prose. Leave.")
lab("DN005", [], "high", "Existing span left as is (it has a raw * — md-press masks existing math).")
lab("DL099", [("α", r"$\alpha$")], "high", "")
lab("DU130", [("0.9 × 0.5^(100 × 0.01 / 365) ≈ 0.88",
               r"$0.9 \times 0.5^{100 \times 0.01 / 365} \approx 0.88$")], "high",
    "ASCII ^(...) grouping parens become braces.")
lab("DU080", [], "medium", WLAB)
lab("DU135", [], "medium", WLAB)
lab("DU078", [("n_past", f"${NP}$")], "medium", "Inside a quoted question." + TRANSCRIPT)
lab("DU043", [], "medium",
    "Context: W₁ᶜ is a verdict label in a table ('W₁ by commitment'); same label convention as W₁.")
lab("DL116", [], "high", "μTOSCA is a name.")
lab("DL189", [("φ", r"$\phi$"), ("T_comm", r"$T_{\text{comm}}$")], "high",
    "\\phi per house NOTATION (φ written \\phi in canon).")
lab("DL163", [("$O_t$ → $V_{O_t}$", r"$O_t \to V_{O_t}$")], "low",
    "Absorbs two existing spans across a math-to-math arrow. The parallel '($\\Sigma_t$ → action)' "
    "suggests the author used → informally in both; leaving unchanged is defensible.")
lab("DU063", [("n_past < 3", f"${NP} \\lt 3$")], "high", "")
lab("DL117", [("π*", r"$\pi^\ast$"), ("π*", r"$\pi^\ast$")], "high",
    "Span only the symbol inside 'deterministic-π*'.")
lab("DU051", [("n_future", f"${NF}$")], "high", "")
lab("DL181", [("‖δ_G‖", r"$\lVert\delta_G\rVert$")], "high", "")
lab("DU095", [("n_future ≫ n_past", f"${NF} \\gg {NP}$")], "high", TRANSCRIPT.strip())
lab("DL103", [("(x =", None), ("Δτ", r"$\Delta\tau$"),
              ("Δη*·‖δ‖", r"$\Delta\eta^\ast \cdot \lVert\delta\rVert$"),
              ("ρ·Δτ", r"$\rho \cdot \Delta\tau$"),
              ("Δτ*", r"$\Delta\tau^\ast$"),
              ("Δτ", r"$\Delta\tau$")], "medium",
    "Expressions are clear. '(x = deliberation time Δτ)' is split as '$x$ = deliberation time $\\Delta\\tau$' (= ties a symbol to a phrase).")
lab("DL187", [("T = Σ ν·η*", r"$T = \sum \nu \cdot \eta^\ast$")], "medium",
    "Span clear. House would write tempo as \\mathcal{T}, but the source letter is T, so it is kept. "
    "'more telemetry ≠ more adaptation' is prose.")
lab("DN012", [], "high", "")
lab("DL081", [("⟨S, X, T, R⟩", r"$\langle S, X, T, R\rangle$"),
              ("T : S × X × S → [0,1]", r"$T : S \times X \times S \to [0,1]$"),
              ("R : S × X → ℝ", r"$R : S \times X \to \mathbb{R}$"),
              ("γ ∈ [0,1)", r"$\gamma \in [0,1)$"),
              ("every t is", None)], "high", "Bare time index 't' also promoted.")
lab("DL109", [("α^depth", r"$\alpha^{\text{depth}}$")], "high", "")
lab("DL032", [("ε", r"$\varepsilon$")], "high", "")
lab("DN048", [], "high", "Filename.")
lab("DL003", [("η", r"$\eta$")], "high", "Arrows are prose.")
lab("DL069", [("δλ = v^T δA v", r"$\delta\lambda = v^T \delta A\, v$")], "high", "")
lab("DL060", [("δt", r"$\delta t$")], "high", "")
lab("DL014", [("H_κ", r"$H_\kappa$")], "medium",
    "Hypothesis label with a Greek subscript; the ASCII underscore does not render. Inner parens stay prose. Raw (H_κ) is the only spelling in the estate.")
lab("DL123", [("τ_high", r"$\tau_{\text{high}}$")], "high", "'Tier A = 0.98' is a quoted prose label.")
lab("DU124", [], "high", "Unit m² and code span; nothing to promote.")
lab("DL168", [("σ", r"$\sigma$")], "high", "$\\sigma$-algebra.")
lab("DN022", [], "high", "Existing display math.")
lab("DU107", [("f_M", "$f_M$"), ("G_t", "$G_t$"), ("f_G", "$f_G$"), ("M_t", "$M_t$")], "high", "")
lab("DL101", [("σ", r"$\sigma$")], "high",
    "Only the bare σ-algebra; existing spans and \\cite are left alone.")
lab("DU100", [], "high", "Context: Unicode code-chart cell (U+039x row); the glyph is the content.")
lab("DL030", [("C_t", "$C_t$"), ("ν", r"$\nu$"), ("φ", r"$\phi$"), ("M_t", "$M_t$")], "high",
    "Source letter C kept (house chronica is \\mathcal C_t). ν is inside a quoted row name, but it is still the tempo symbol.")
lab("DU049", [("H", "$H$"), ("Δρ_CLM = 25", r"$\Delta\rho_{\text{CLM}} = 25$")], "medium",
    "Context: H is elevation (the table below defines it). Unit 'kg/m³' left in prose; including it as \\,\\mathrm{kg/m^3} is also fine.")
lab("DU019", [("n̂_future", r"$\hat{n}_{\text{future}}$")], "high", "Inside bold.")
lab("DU094", [("n > 10-20", r"$n \gt 10\text{-}20$")], "medium",
    "A range as the bound; hyphen kept as text. '$n \\gt$ 10-20' or '$n \\gt 10$-20' are other in-shape choices.")
lab("DL198", [("C₁, C₂", "$C_1, C_2$"), ("F:", None),
              ("time(C₁) > time(C₂)", r"$\mathrm{time}(C_1) \gt \mathrm{time}(C_2)$"),
              ("C₁ takes", None),
              ("E[Σ(time_future(Fᵢ | C₁))] < E[Σ(time_future(Fᵢ | C₂))]",
               r"$E[\sum(\mathrm{time_{future}}(F_i \mid C_1))] \lt E[\sum(\mathrm{time_{future}}(F_i \mid C_2))]$"),
              ("prefer(C₁) ∝ E[num_future_changes]",
               r"$\mathrm{prefer}(C_1) \propto E[\text{num-future-changes}]$")], "low",
    "Context: in the source these are four pseudocode lines (if/but/then with // comments) that unwrapping joined into one paragraph. "
    "The honest fix is a code or display block, which is out of shape. These are in-shape spans; num_future_changes uses '-' per the house \\text{} underscore rule; \\mathrm{time_{future}} (not \\mathrm{time}_{\\text{future}}) avoids two '}_' sequences that GFM emphasis can pair inside one line.")
lab("DL135", [("α", r"$\alpha$"), ("β", r"$\beta$"), ("γ", r"$\gamma$")], "medium",
    "Greek enumerators. Canon writes '($\\alpha$) deterministic outcomes' (def-pearl-causal-hierarchy). Leaving them as labels is defensible.")
lab("DL037", [("M_t = φ(C_t)", r"$M_t = \phi(C_t)$"), ("space M", None), ("M_t", "$M_t$")], "high",
    "Model space letter M kept as written (house: \\mathcal{M}).")
lab("DU060", [("n_past", f"${NP}$")], "high", TRANSCRIPT.strip())
lab("DU008", [("O(n)", "$O(n)$")], "medium",
    "Context: a cost-column table cell (complexity). ASCII big-O; leaving it is defensible.")
lab("DU081", [("n=1", "$n=1$")], "medium", "")
lab("DL025", [("max|Δh|≈150", r"$\max\lvert\Delta h\rvert \approx 150$")], "high",
    "Unit 'm' left in prose. ≡ between file:line refs is prose.")
lab("DL008", [("|δ|_ss = ρ / (ν · η*)", r"$\lvert\delta\rvert_{\text{ss}} = \rho / (\nu \cdot \eta^\ast)$")], "high", "")
lab("DL148", [("∀", r"$\forall$"), ("I₁, I₂", "$I_1, I_2$"), ("F:", None),
              ("∀", r"$\forall$"),
              ("m ∈ M\\{time}", r"$m \in M \setminus \{\text{time}\}$"),
              ("m(I₁) = m(I₂)", "$m(I_1) = m(I_2)$"),
              ("optimal(I₁, I₂) = argmin(time(I₁), time(I₂))",
               r"$\operatorname{optimal}(I_1, I_2) = \operatorname{argmin}(\operatorname{time}(I_1), \operatorname{time}(I_2))$")], "medium",
    "Semi-formal logic in prose. Source 'M\\{time}' renders as 'M{time}' in markdown (escaped brace), so set-minus is reconstructed. Boundaries are judgment calls.")
lab("DL094", [("S_system < S_1 × S_2", r"$S_{\text{system}} \lt S_1 \times S_2$")], "high", "")
lab("DL055", [("α > ρ/R", r"$\alpha \gt \rho/R$")], "high", "Inside emphasis and quotes.")
lab("DU084", [("n=9", "$n=9$")], "low",
    "Sample-size n in ops prose; leaving 'n=9' is equally defensible.")
lab("DL185", [("κ_processing = I(G_t ; M_τ⁺ | e_τ) / H(G_t | e_τ)",
               r"$\kappa_{\text{processing}} = I(G_t ; M_{\tau^+} \mid e_\tau) / H(G_t \mid e_\tau)$")], "high",
    "M_τ⁺ → M_{\\tau^+}, matching canon (hyp-truth-serving-loophole).")
lab("DL040", [], "high",
    "Context: a table cell 'Current → Proposed' over category labels (D, E, P). Not math.")
lab("DL150", [("T_agent > ρ_environment", r"$T_{\text{agent}} \gt \rho_{\text{environment}}$")], "high", "")
lab("DU076", [("O(n²)", "$O(n^2)$")], "high", "")
lab("DU039", [("n < 3", r"$n \lt 3$")], "high", "")
lab("DL022", [("$\\lambda$ ≈ 2.0", r"$\lambda \approx 2.0$")], "medium",
    "Absorbs the existing $\\lambda$ into the relation; '$c$ = completeness…' ties a symbol to a phrase, so it stays.")
lab("DU134", [("O(1)", "$O(1)$")], "high", "Bold −0.5 and the ⇒ implication are prose.")
lab("DL136", [("α", r"$\alpha$"), ("β", r"$\beta$"), ("γ", r"$\gamma$")], "medium",
    "Same Greek enumerators as theory-of-agentic-tooling (α)/(β)/(γ); canon writes ($\\alpha$). '$\\alpha/\\beta/\\gamma$' as one span is equivalent.")
lab("DU025", [("n_past", f"${NP}$"), ("n_past", f"${NP}$")], "high",
    "Second one is inside a quote." + TRANSCRIPT)
lab("DU033", [("O(n²)", "$O(n^2)$"), ("O(n)", "$O(n)$"), ("n is", None)], "high", "")
lab("DL063", [], "medium",
    "Existing span left as is (it has a raw | and multi-letter italic, but md-press masks it). "
    "'≈ 5-10 lines' reads as prose; absorbing it would leave an awkward '$… \\approx 5$-10'.")
lab("DL042", [("γ_t : V_t → {AND, OR}", r"$\gamma_t : V_t \to \{\text{AND}, \text{OR}\}$")], "high", "")
lab("DL142", [("β", r"$\beta$")], "high",
    "Canon writes 'sub-scope $\\beta$' (163 times vs 85 raw).")
lab("DL162", [("κ × 𝒜", r"$\kappa \times \mathcal{A}$")], "high", "")
lab("DU024", [], "high",
    "Context: table cell holding label E plus footnote marker ³ (siblings E¹, E², ⁴…). Converting to E^3 would be wrong.")
lab("DL131", [("α ≈ 1.5-2.0", r"$\alpha \approx 1.5\text{-}2.0$"),
              ("λ ≈ 0.01-0.05", r"$\lambda \approx 0.01\text{-}0.05$")], "medium",
    "Spans are clear; the in-math spelling of a range is a choice.")
lab("DN030", [], "high", "Function name.")
lab("DN040", [], "high", "Existing span.")
lab("DL155", [("impact(t) = Σ(users(s) × criticality(s))",
               r"$\operatorname{impact}(t) = \sum(\operatorname{users}(s) \times \operatorname{criticality}(s))$"),
              ("s ∈ failed_systems(t)", r"$s \in \text{failed-systems}(t)$")], "medium",
    "Pseudo-math with word functions. 'for' kept as prose between two spans; failed_systems uses '-' per the house \\text{} underscore rule.")
lab("DU142", [], "medium", WLAB)
lab("DU038", [("n_past", f"${NP}$")], "medium",
    "TST variable in an activation protocol; it could be read as a field name, but it is the TST quantity.")
lab("DU020", [("C supports", None), ("(s, l, s')", "$(s, l, s')$"), ("some s'", None)], "medium",
    "PDF-extracted paper text; existing double-padded spans left alone. The bare variables are promoted.")
lab("DL110", [("k=80", "$k=80$"), ("k≈60", r"$k\approx 60$"), ("k≈120", r"$k\approx 120$")], "medium",
    "k-means k. 'L2-normalize' and '50–100' left.")
lab("DL098", [("mean + 1.5σ", r"$\text{mean} + 1.5\sigma$"), ("k values", None), ("τ", r"$\tau$")], "medium",
    "'mean + 1.5σ' spanned whole with a word operand; 'mean + $1.5\\sigma$' is the conservative alternative.")
lab("DU066", [("n_past < 10", f"${NP} \\lt 10$")], "high", "")
lab("DN041", [], "high", "Code span and prose ⇒.")
lab("DU052", [("P(o_{t+1} | do(a_t), M_t)", r"$P(o_{t+1} \mid \operatorname{do}(a_t), M_t)$")], "high", "")
lab("DL165", [("ν", r"$\nu$")], "high", "")
lab("DL083", [], "high",
    "The line is about the raw glyph 𝒜 itself (a sweep replacing it). Converting would erase the subject.")
lab("DU121", [("T_p = 1430", "$T_p = 1430$"), ("T*_p = 1280", r"$T^\ast_p = 1280$")], "high",
    "°C left in prose; '150–200 °C' is prose.")
lab("DN033", [], "high", "Existing spans; Table~\\ref is LaTeX-in-prose, outside this task.")
lab("DN047", [], "high", "")
lab("DN006", [], "high", "Code span; ≡ is prose.")
lab("DL192", [("G_t", "$G_t$"), ("O_t", "$O_t$"), ("Σ_t", r"$\Sigma_t$")], "high",
    "'/' between O_t and Σ_t kept as prose; '$O_t/\\Sigma_t$' as one span is equivalent.")
lab("DU120", [], "low",
    "'~10⁵-entry' is an engineering magnitude; Unicode superscript renders fine. '~$10^5$-entry' is defensible.")
lab("DN011", [], "high", "Code-chart cell.")
lab("DL082", [("χ", r"$\chi$")], "high", "")
lab("DU114", [("K_tacit", r"$K_{\text{tacit}}$")], "high", "")
lab("DU010", [], "high", "Context: 't=45' is a per-session record field in a list of session ids, not math.")
lab("DN019", [], "high", "Code-chart cell.")
lab("DL012", [("δ_strategic", r"$\delta_{\text{strategic}}$"), ("L²", "$L^2$")], "medium",
    "L² as $L^2$ is a typographic choice (Unicode renders); δ_strategic is clear.")
lab("DN001", [], "high", "Existing spans.")
lab("DL084", [("a_t = π(M_t)", r"$a_t = \pi(M_t)$"), ("a_t ~ π(·|M_t)", r"$a_t \sim \pi(\cdot \mid M_t)$")], "high", "")
lab("DL134", [("‖δ‖", r"$\lVert\delta\rVert$")], "high", "")
lab("DU091", [(r"\(\mathrm{Hel}^2(P,Q) := 1 - \mathrm{BC}(P,Q)\)", r"$\mathrm{Hel}^2(P,Q) := 1 - \mathrm{BC}(P,Q)$"),
              (r"\(\mathrm{Hel}\in[0,1]\)", r"$\mathrm{Hel}\in[0,1]$")], "high",
    "\\(…\\) delimiters become $…$ (in markdown, \\( renders as a literal paren).")
lab("DL129", [("Δp", r"$\Delta p$")], "low",
    "A table row label in a vendored llama.cpp README (external clone). Δp is a quantity, but the file is not Joseph's.")
lab("DU104", [("p^n", "$p^n$")], "high", "")
lab("DU022", [("investment < n_future × savings", f"$\\text{{investment}} \\lt {NF} \\times \\text{{savings}}$")], "medium",
    "A word equation, spanned whole. 'investment < $n_{\\text{future}}$ × savings' would leave raw < and × in prose." + TRANSCRIPT)
lab("DN025", [], "high", "Existing span (multi-letter italics, but masked).")
lab("DU102", [], "medium",
    "H_D3 is a hypothesis ID in an experiment series (cf. '**H_D3**: direction symmetry'), so it stays a label.")
lab("DL070", [("φ", r"$\phi$")], "high", "")
lab("DU087", [], "medium", "H_D3 is a hypothesis ID label in a results file; left raw.")
lab("DU073", [("#Deps(Mj)", r"$\#\mathrm{Deps}(M_j)$"),
              ("⋃ℓ>i Lℓ", r"$\bigcup_{\ell \gt i} L_\ell$"),
              ("Mj", "$M_j$")], "low",
    "Context: pasted research summary whose subscripts were lost throughout (L1,…,Ln, Mj). Reconstructing M_j/L_ℓ reads intent. "
    "Conservative alternative: only the ⋃ expression.")
lab("DL067", [("M_t", "$M_t$"),
              ("T > ρ / ‖δ_critical‖", r"$T \gt \rho / \lVert\delta_{\text{critical}}\rVert$"),
              ("G_t", "$G_t$")], "high", "")
lab("DU036", [("M_x", "$M_x$"), ("f_X", "$f_X$"), ("X = x", "$X = x$")], "high", "")
lab("DL184", [("θ", r"$\theta$"), ("Fr > 1.5", r"$\mathrm{Fr} \gt 1.5$")], "medium",
    "Fr is the Froude number (upright two-letter symbol). θ names a parameter row.")
lab("DU138", [], "medium",
    "Context: S₀/S₁/S₂ are surrogate-class labels (the author code-spans `S₀` in prose). $S_1$ is defensible.")
lab("DU014", [(r"\(a\)", "$a$"), (r"\(Q_a(r)\)", "$Q_a(r)$"), (r"\(r\)", "$r$")], "high", "\\(…\\) to $…$.")
lab("DL036", [("Ω_t", r"$\Omega_t$")], "high", "'L1 vs L2 norm' left as names.")
lab("DU059", [("n_past", f"${NP}$")], "high", TRANSCRIPT.strip())
lab("DU034", [("G_t", "$G_t$"), ("M_t", "$M_t$")], "high", "")
lab("DU140", [], "medium", WLAB + " Existing $G$/$O$/$\\Sigma$ spans left as is.")
lab("DL120", [("α₁/α₂/β", r"$\alpha_1$/$\alpha_2$/$\beta$")], "medium",
    "Canon writes '$\\alpha_1$/$\\alpha_2$/$\\beta$' for these sub-scope labels. C1/C2/C3, Tier, Regime A/B/C stay prose.")
lab("DL051", [("A→B→G", r"$A \to B \to G$")], "medium", "A node chain in the strategy DAG.")
lab("DL177", [("π*", r"$\pi^\ast$")], "high", "Existing span left as is.")
lab("DL020", [("Ω", r"$\Omega$"), ("O", "$O$"), ("A", "$A$"), ("h", "$h$"), ("T", "$T$"), ("ε", r"$\varepsilon$")], "medium",
    "Context: definitions table, a list of symbols. Separate spans; one span '$\\Omega, O, A, h, T, \\varepsilon$' also works.")
lab("DL033", [("δ(P)", r"$\delta(P)$"), ("δ", r"$\delta$")], "high", "")
lab("DU031", [("n < 100", r"$n \lt 100$")], "high", "")
lab("DU108", [(r"$\Delta^{\downarrow}$ ,  $\Delta^{\uparrow} \vdash C$",
               r"$\Delta^{\downarrow}, \Delta^{\uparrow} \vdash C$"),
              ("constraint C", None)], "medium",
    "The first two existing spans are one judgment (Δ↓, Δ↑ ⊢ C) split by PDF extraction, so they are merged. Later spans are left as they are.")
lab("DU128", [("n_future", f"${NF}$")], "high", TRANSCRIPT.strip())
lab("DL062", [("ρ = 0.991", r"$\rho = 0.991$")], "high", "")
lab("DN029", [], "high",
    "Existing span left as is (it has raw < and \\| — house violations, but masked).")
lab("DL166", [("|A| = 2", r"$\lvert A\rvert = 2$"), ("Δ = 0.1", r"$\Delta = 0.1$"),
              ("t ∈ [1, T/2]", r"$t \in [1, T/2]$"),
              ("t ∈ [T/2 + 1, T]", r"$t \in [T/2 + 1, T]$"),
              ("a* is", None)], "high", "Code spans untouched.")
lab("DU110", [("O(n²)", "$O(n^2)$")], "high", "")
lab("DU064", [(r"\(\mathrm{Hel}\)", r"$\mathrm{Hel}$")], "high", "\\(…\\) to $…$.")
lab("DU115", [("n_c < 5", r"$n_c \lt 5$")], "high", "")
lab("DL021", [("κ", r"$\kappa$"), ("κ × A", r"$\kappa \times A$")], "high",
    "Table cell. Source A kept (the neighbour uses \\mathcal{A}; the author may have meant 𝒜).")
lab("DU136", [], "high", "All math already in spans; 'the W axis' is prose.")
lab("DN015", [], "high", "Code span.")

# Edits with new=None: promote a single-letter variable that sits as the first
# token of `old` (old is a disambiguating context window).
SINGLE = {"(x =": ("x", "$x$"), "every t is": ("t", "$t$"), "F:": ("F", "$F$"), "C₁ takes": ("C₁", "$C_1$"),
          "space M": ("M", "$M$"), "n is": ("n", "$n$"), "C supports": ("C", "$C$"),
          "some s'": ("s'", "$s'$"), "k values": ("k", "$k$"), "a* is": ("a*", r"$a^\ast$"),
          "constraint C": ("C", "$C$")}

def apply(text, edits):
    out, cur = [], 0
    for old, new in edits:
        if new is None:
            sym, rep = SINGLE[old]
            i = text.find(old, cur)
            if i < 0: raise ValueError(f"context {old!r} not found")
            # sym's first occurrence inside the context window
            old, new, i = sym, rep, i + old.index(sym)
        else:
            i = text.find(old, cur)
            if i < 0: raise ValueError(f"{old!r} not found after {cur}")
        assert new.startswith("$") and new.endswith("$") and len(new) > 2, new
        out.append(text[cur:i]); out.append(new); cur = i + len(old)
    out.append(text[cur:])
    return "".join(out)

items = [json.loads(l) for l in TASKS.read_text().splitlines() if l.strip()]
missing = [d["gid"] for d in items if d["gid"] not in L]
extra = set(L) - {d["gid"] for d in items}
assert not missing and not extra, (missing, extra)
OUT.parent.mkdir(parents=True, exist_ok=True)
with OUT.open("w") as f:
    for d in items:
        edits, conf, note = L[d["gid"]]
        gold = apply(d["text"], edits)
        f.write(json.dumps({"gid": d["gid"], "gold": gold, "conf": conf, "note": note}, ensure_ascii=False) + "\n")
        if "-v" in sys.argv and gold != d["text"]:
            print(d["gid"], "|", gold)
print("wrote", len(items), "->", OUT)
