#!/usr/bin/env python3
"""Pass-1 batch-1 gold labels, expressed as exact substring replacements on the input.

Each replacement's `old` must occur exactly once (or `n` times) in the text at the
moment it is applied, so a typo fails loudly instead of silently producing an
out-of-shape gold. validate() then re-checks the shape constraint independently.
"""
import json, re, sys
from pathlib import Path

D = Path(__file__).resolve().parent.parent
TASKS = D / "tasks" / "batch-1.jsonl"
OUT = D / "pass1" / "batch-1.jsonl"

N_PAST = r"n_{\text{past}}"
N_FUT = r"n_{\text{future}}"

L = {}  # gid -> (reps, conf, note)

def lab(gid, reps, conf, note=""):
    assert gid not in L, gid
    L[gid] = (reps, conf, note)

lab("DU027", [("M_t", "$M_t$"), ("W_t", "$W_t$")], "high")
lab("DL026", [("ν_{ij}", r"$\nu_{ij}$")], "high")
lab("DU004", [("ϵ-Capacity", r"$\epsilon$-Capacity")], "medium",
    "Bibliographic title in a relata import table (context checked). The paper title's ϵ is math, but a tool consuming the citation text might prefer the Unicode; author's call.")
lab("DL113", [], "high", "Needed context: a table cell recording a category-code change (E/D/P codes), not math.")
lab("DU062", [("N_h > 1", r"$N_h \gt 1$")], "high", "A, B, C are counterexample names, not variables.")
lab("DN004", [], "high", "Already spanned.")
lab("DU111", [("n>1100", r"$n \gt 1100$")], "medium",
    "A sample size inside a citation bracket in a harvest log. Wrapped by stats-typesetting convention; leaving it bare is defensible.")
lab("DN023", [], "high", "Already spanned.")
lab("DL034", [("v = v × e^(β × tooling_time) × e^(-γ × features × time)",
               r"$v = v \times e^{\beta \times \text{tooling-time}} \times e^{-\gamma \times \text{features} \times \text{time}}$")],
    "medium",
    "Whole formula as one span. e^(…) parens are grouping, so they become braces. Word variables go in \\text, with '-' for '_' per the SOP. (v = v·… is presumably v_t; kept as written.)")
lab("DL054", [("ties α to", r"ties $\alpha$ to")], "high", "The backticked S and C(D_test,S) are code spans, left alone.")
lab("DL156", [("λ_min", r"$\lambda_{\min}$")], "high")
lab("DN039", [], "high", "MDX docs with escaped template braces; no math.")
lab("DL017", [("top-β ≈ 20%", r"top-$\beta \approx 20\%$")], "medium",
    "β is the top fraction; its stated value is included in the span. Stopping at $\\beta$ is the other reading. The O(…) in backticks is code.")
lab("DN007", [], "high", "Transcript tool-call text; no math.")
lab("DL188", [("APIs have k* ≈ 0.", r"APIs have $k^\ast \approx 0$.")], "high", "The backticked `k*` stays code.")
lab("DN049", [], "high", "A filename.")
lab("DU133", [("for k branches", r"for $k$ branches")], "high", "Bare k is the variable inside the adjacent $O(2^k)$.")
lab("DN038", [], "high")
lab("DL145", [("κ × 𝒜", r"$\kappa \times \mathcal{A}$")], "high", "Inside an italic paper title, kept inside the *…*.")

_xy = [("Accept X extra", "Accept $X$ extra"), ("save Y minutes", "save $Y$ minutes"),
       ("when X < n_future × Y.", rf"when $X \lt {N_FUT} \times Y$."),
       ("And n_future = n_past from", rf"And ${N_FUT} = {N_PAST}$ from")]
lab("DL143", _xy, "medium",
    "Transcript. The bare X/Y are the rule's variables, so wrapped; the inequality and identity are high-confidence.")
lab("DL027", [("externalize M_t into Ω", r"externalize $M_t$ into $\Omega$")], "high")
lab("DL044", [("the H_b operationalization of κ**", r"the $H_b$ operationalization of $\kappa$**"),
              ("κ = 1 - H_b(G_t | M_τ⁺, e_τ) / H(G_t).", r"$\kappa = 1 - H_b(G_t \mid M_\tau^+, e_\tau) / H(G_t)$."),
              ("→ H_b ≈ H(G_t) → κ ≈ 0.", r"→ $H_b \approx H(G_t)$ → $\kappa \approx 0$."),
              ("→ H_b ≈ 0 → κ ≈ 1.", r"→ $H_b \approx 0$ → $\kappa \approx 1$.")],
    "medium",
    "The → chains read as prose 'leads to' (the first link starts from prose), so the arrows stay outside. Joining '$H_b \\approx H(G_t) \\to \\kappa \\approx 0$' into one span is equally defensible.")
lab("DN034", [], "high", "'ZIP < 100 MB' is a prose size limit.")
lab("DU045", [("n_future from n̂_future", rf"${N_FUT}$ from $\hat{{n}}_{{\text{{future}}}}$")], "high")
lab("DN024", [], "high")
lab("DN037", [], "high", "Already spanned.")
lab("DL058", [("for κ_processing", r"for $\kappa_{\text{processing}}$"), ("estimator A(e)", r"estimator $A(e)$")], "high")
lab("DL100", [], "high", "Needed context: a cell in a generated Unicode glyph table. The bracket shows the glyph itself, so it must not change.")
lab("DU069", [], "medium",
    "Everything mathematical is already spanned. The ⟹ between roman case labels '(ii ⟹ iii)' and between two spans is a prose-level symbol, left alone; wrapping just the ⟹ as $\\Longrightarrow$ is a defensible alternative.")
lab("DU126", [(r"\(P_G\)", "$P_G$"), (r"\(Q\)", "$Q$"),
              (r"\(D_{1/2}(P_G\|Q)\)", r"$D_{1/2}(P_G \Vert Q)$"),
              (r"\(G\)", "$G$"), (r"\(M\)", "$M$"),
              (r"\(I(G;M) = I\)", "$I(G;M) = I$"), (r"large \(I\)", "large $I$")],
    "high",
    "Explicit \\(…\\) math converted to $…$, since GitHub renders \\( as a literal paren. \\| becomes \\Vert per the SOP.")
lab("DL130", [("(λ=0.15:", r"($\lambda=0.15$:")], "high",
    "The existing $0.00$ and $0.73$ are kept. cont_surv is a metric identifier; I didn't absorb it.")
lab("DU003", [("10.1 M_t already", "10.1 $M_t$ already")], "high", "A section heading.")
lab("DL115", [("N ∈ [0,1] where N=1 is", r"$N \in [0,1]$ where $N=1$ is"), ("and N=0 is", "and $N=0$ is")], "high",
    "Inside a quotation, but this is a defined score variable.")
lab("DL045", [("Spearman ρ is", r"Spearman $\rho$ is"), ("With N=12", "With $N=12$"), ("the ρ estimate", r"the $\rho$ estimate")],
    "medium", "ρ is high-confidence. N=12 is a sample size, wrapped by stats convention. The `params_M` × `metric` code spans are left alone.")
lab("DN032", [], "high")
lab("DL041", [("(β-vs-ρ in", r"($\beta$-vs-$\rho$ in")], "high")
lab("DL057", [], "low",
    "Needed context: the header of a disposition table whose cells are change codes (=, CTX↑, LOAD↑). Here Δ is a 'change' shorthand, not a quantity; '$\\Delta$ category' is also defensible.")
lab("DL078", [("coefficient ι_ij)**", r"coefficient $\iota_{ij}$)**"),
              ("segment. ι_ij ∈ [0,1] is", r"segment. $\iota_{ij} \in [0,1]$ is")], "high")
lab("DL178", list(_xy), "medium", "Same text as the DL143 transcript; same treatment.")
lab("DN046", [], "high", "Already spanned.")
lab("DN003", [], "high")
lab("DL171", [("n ≥ 3", r"$n \ge 3$")], "high")
lab("DU030", [], "low",
    "Transcript meta-mention: the sentence says n_past lacks LaTeX formatting, so wrapping it would contradict the sentence. The honest form is a code span, which is out of shape. Wrapping it is the other reading.")
lab("DL024", [("convergence-ε", r"convergence-$\varepsilon$")], "medium",
    "ε is the named convergence tolerance. The ⟂ between two quoted words is a prose glyph, left alone.")
lab("DL065", [("κ-as-scalar", r"$\kappa$-as-scalar")], "high", "The backticked κ_processing is code.")
lab("DU007", [("S_id at draft", r"$S_{\text{id}}$ at draft"), ("of S_id properties", r"of $S_{\text{id}}$ properties")], "high")
lab("DU077", [("enable O(1) lookups", "enable $O(1)$ lookups")], "medium",
    "Big-O in general-prose docs: a careful writer typesets it, but it is often left bare. one_for_one/one_for_all are Elixir atoms, not math.")
lab("DU103", [("check n_past,", rf"check ${N_PAST}$,")], "medium", "Transcript; TST symbol.")
lab("DL151", [("β(t) = β₀ (constant)", r"$\beta(t) = \beta_0$ (constant)")], "high")
lab("DU009", [("at N=256 cells", "at $N=256$ cells")], "medium",
    "Needed context: a table header. N is the grid resolution parameter.")
lab("DL182", [("β_exp (exponential", r"$\beta_{\text{exp}}$ (exponential")], "high")
lab("DU113", [("P(T|T>t) ∝ 1/T² for T>t", r"$P(T \mid T \gt t) \propto 1/T^2$ for $T \gt t$")], "high")
lab("DL140", [("ρ = 0.991", r"$\rho = 0.991$")], "high")
lab("DN010", [], "high", "Already spanned.")
lab("DL147", [("a_k → 0 implies I_n → 0, and the upper level ā/(1−η̄).",
               r"$a_k \to 0$ implies $I_n \to 0$, and the upper level $\bar{a}/(1-\bar{\eta})$.")], "high")
lab("DL093", [("Sub-scope β is absent.", r"Sub-scope $\beta$ is absent."),
              ("sub-scope β entry", r"sub-scope $\beta$ entry"), ("buys β.", r"buys $\beta$.")], "high",
    "The ASF canon writes these labels as 'sub-scope $\\beta$' (checked in 01-aat-core/src).")
lab("DN017", [], "high", "Already spanned.")
lab("DU029", [("T_scan = visual", r"$T_{\text{scan}}$ = visual")], "high", "'= <words>' is a prose gloss, kept outside.")
lab("DL137", [("— γ as function", r"— $\gamma$ as function")], "high")
lab("DL124", [("ρ = +0.87", r"$\rho = +0.87$")], "high")
lab("DN008", [], "high", "A heading with a block-ref anchor.")
lab("DL141", [("spectral radius ρ(A) of", r"spectral radius $\rho(A)$ of"),
              ("bound E[size] ≤ 1/(1-ρ);", r"bound $E[\text{size}] \le 1/(1-\rho)$;"),
              ("resolvent (I-A)^(-1) analysis", r"resolvent $(I-A)^{-1}$ analysis"),
              ("transitions at ρ=1;", r"transitions at $\rho=1$;")],
    "medium",
    "The math reading itself is clear. But this reads as a lit-search query prompt (Undermind research goals); if it is pasted into a search tool, LaTeX may be unwanted. Author intent.")
lab("DL048", [("rate ρ(t)**", r"rate $\rho(t)$**"), ("changes in Ω.", r"changes in $\Omega$.")], "high")
lab("DL091", [("α = 0.118", r"$\alpha = 0.118$")], "high")
lab("DU016", [("(p=.215)", "($p=.215$)"), ("(Z=1.968, p<.026 —", r"($Z=1.968$, $p \lt .026$ —"),
              ('as "p>.026"', r'as "$p \gt .026$"')],
    "medium",
    "Statistical reporting with italic p/Z per LaTeX/APA norms. The quoted misprint is wrapped too for consistency; leaving it literal (it quotes printed text) is defensible.")
lab("DU123", [("ASTs of P and P' are equal", "ASTs of $P$ and $P'$ are equal"),
              ('(pronounced "P and P\' are similar")', '(pronounced "$P$ and $P\'$ are similar")'),
              ("the free variable x.", "the free variable $x$.")],
    "medium",
    "A PDF-extracted paper. P, P' and x are the paper's math italics. The odd double spaces around the existing spans are outside them, so untouched. Wrapping inside the pronunciation gloss is the least certain choice.")
lab("DL152", [("failed_systems(t) = {s ∈ S : dependency_timeout(s) < wait_time(B(t))}",
               r"$\text{failed-systems}(t) = \{s \in S : \text{dependency-timeout}(s) \lt \text{wait-time}(B(t))\}$")],
    "medium", "One set-builder span with word-functions in \\text and '-' for '_' per the SOP; \\operatorname is equally fine.")
lab("DU021", [("(O(n²) to O(n))", "($O(n^2)$ to $O(n)$)")], "high")
lab("DU147", [("different P_true (", r"different $P_{\text{true}}$ (")], "medium",
    "The bold do(survey_population = doctors) heading is left as prose, a pseudo-code intervention label; wrapping it as $\\mathrm{do}(\\ldots)$ is defensible.")
lab("DU042", [], "medium", "t=5 is a field in a session-log listing (turn count), not math.")
lab("DL193", [("proximity(changeset) = 1 / Σ(distance(change_i, change_j))",
               r"$\text{proximity}(\text{changeset}) = 1 / \sum(\text{distance}(\text{change}_i, \text{change}_j))$"),
              ("time_implementation ∝ 1/proximity(changeset)",
               r"$\text{time}_{\text{implementation}} \propto 1/\text{proximity}(\text{changeset})$")],
    "medium",
    "Needed context: two formula lines (no blank line between) that unwrapping joined. That gives two spans, with the separating space left outside. Σ is a summation, so \\sum.")
lab("DL096", [("∪", r"$\cup$")], "medium",
    "Needed context: the Symbol column of a relational-algebra operator table (sibling cells σ π ⋈ × − ρ). The operator symbol is math, though it is shown as a glyph.")
lab("DL173", [("θ sweep, κ findings incl. the χ-gain", r"$\theta$ sweep, $\kappa$ findings incl. the $\chi$-gain")], "high")
lab("DN002", [], "high", "A Unicode glyph-table cell.")
lab("DU012", [("**cl_str(X)**", r"**$\mathrm{cl}_{\text{str}}(X)$**"),
              ("certifiable from X (", "certifiable from $X$ ("),
              ("P ⇒ cl_str reaches M⁻ (", r"$P$ ⇒ $\mathrm{cl}_{\text{str}}$ reaches $M^-$ ("),
              ("+ H_D3 pass", r"+ $H_{\mathrm{D3}}$ pass")],
    "low",
    "cl_str is a closure operator and X, P, M⁻ are its objects. ⇒ has a prose consequent, so it stays outside. H_D3 is a diagnostic label (it sits beside C1/C2/GC2 throughout the file, context checked); wrapping the label is arguable.")
lab("DL105", [("*sub-scope metric-α₂*", r"*sub-scope metric-$\alpha_2$*"),
              ("Hurwitz) remains β;", r"Hurwitz) remains $\beta$;"),
              ("threshold) remain β.", r"threshold) remain $\beta$.")], "high",
    "The canon writes sub-scope labels in math.")
lab("DN026", [], "high", "Already spanned.")
lab("DL158", [("R3-ε-error", r"R3-$\varepsilon$-error")], "low",
    "ε inside an item-ID label. Wrapping gives the canonical math form; leaving an ID verbatim is also reasonable.")
lab("DL059", [("isotropic Λ epistemic", r"isotropic $\Lambda$ epistemic")], "high")
lab("DU002", [("that n_past predicts n_future,", rf"that ${N_PAST}$ predicts ${N_FUT}$,")], "medium", "Transcript; TST symbols.")
lab("DU082", [("(n_past)", rf"(${N_PAST}$)")], "high")
lab("DN016", [], "high", "A Unicode glyph-table cell.")
lab("DU125", [("o_t", "$o_t$")], "high", "Needed context: the Universal symbol column of a mapping table.")
lab("DL108", [(r"calculate X < $\hat{n}_{\text{future}}$ × Y.", r"calculate $X \lt \hat{n}_{\text{future}} \times Y$."),
              ("(5 < 20 × 0.25)", r"($5 \lt 20 \times 0.25$)")],
    "medium",
    "The existing span is absorbed into the whole inequality. The numeric instance is wrapped too; leaving it as prose arithmetic is defensible.")
lab("DL019", [("claims λ = 1 − α_c/ν_c as", r"claims $\lambda = 1 - \alpha_c/\nu_c$ as"),
              ("bounds dV/dt (", "bounds $dV/dt$ ("),
              ("map f_c(·, o).", r"map $f_c(\cdot, o)$.")], "high")
lab("DL031", [(r"η\*", r"$\eta^\ast$")], "high", "The escaped \\* is the star superscript, inside bold.")
lab("DN009", [], "high", "Already spanned.")
lab("DU148", [("reserve Δρ***:", r"reserve $\Delta\rho^\ast$**:"),
              ("before R* exceeds R.", r"before $R^\ast$ exceeds $R$.")],
    "medium", "Read 'Δρ***' as Δρ* plus the closing ** of the bold.")
lab("DU067", [("W₂ compliance", "$W_2$ compliance"), ("in W₂-ish mode", "in $W_2$-ish mode")], "medium",
    "Inside a verbatim italic quotation; wrapping changes rendering only.")
lab("DL010", [], "low",
    "Garbled PDF extraction of Van Antwerpen et al. 2018: 'sb0' is a subscripted scope variable, '7→' is ↦, and so on. A faithful math form needs reconstruction the text doesn't support; partial wrapping would be guesswork.")
lab("DU112", [("-- M_t Updates Before Sigma_t Before O_t,", r"-- $M_t$ Updates Before $\Sigma_t$ Before $O_t$,")], "medium",
    "Title-case heading. The ASCII-spelled 'Sigma_t' is read as Σ_t.")
lab("DL068", [("Threshold ε", r"Threshold $\varepsilon$")], "high")
lab("DL104", [("any F satisfying δᵀF ≥ α‖δ‖² is", r"any $F$ satisfying $\delta^\top F \ge \alpha \lVert\delta\rVert^2$ is")], "high")
lab("DL119", [("log-det, **and** λ_max", r"log-det, **and** $\lambda_{\max}$")], "high")
lab("DU105", [("T_obs + T_explore + T_probe", r"$T_{\text{obs}} + T_{\text{explore}} + T_{\text{probe}}$")], "high",
    "Kept as T; the canon's \\mathcal{T} for tempo isn't implied by the text.")
lab("DU131", [("Often G₃ at", "Often $G_3$ at"), ("top, G₁ at", "top, $G_1$ at")], "high")
lab("DL052", [("floor λ_min *is*", r"floor $\lambda_{\min}$ *is*")], "high", "The backticked ε₁ I and I_min are code.")
lab("DL035", [("(n_future ≈ 20-50)", rf"(${N_FUT} \approx 20\text{{–}}50$)")], "medium",
    "Range: the whole value is inside the span, with an en dash in \\text (a raw '-' would render as minus). '$…\\approx 20$-50' is the other in-shape option.")
lab("DL064", [("area μ(T)", r"area $\mu(T)$")], "high")
lab("DL009", [("the κ × 𝒜 paper", r"the $\kappa \times \mathcal{A}$ paper")], "high")
lab("DU037", [("at N=2-3 hierarchies", r"at $N=2\text{–}3$ hierarchies")], "low",
    "Informal 'N=2-3' (two or three hierarchies). Leaving it bare is reasonable; if wrapped, the range needs a dash, not a minus.")
lab("DL121", [("**: Σ_t edges", r"**: $\Sigma_t$ edges"),
              ("(p_ij = P(j | do(i), M_t))", r"($p_{ij} = P(j \mid \mathrm{do}(i), M_t)$)")], "high")
lab("DU053", [("The G_t Formalism", "The $G_t$ Formalism")], "high", "A heading.")
lab("DU132", [("fall in W₂ (", "fall in $W_2$ ("), ("instance of W₁.", "instance of $W_1$.")], "high",
    "A whole table row; the pipes are table structure. observation→memory is a prose arrow.")
lab("DU099", [("weights W^{12} frozen", "weights $W^{12}$ frozen")], "high", "'frozen at 0 → …' is a prose arrow.")
lab("DL139", [("Accept X extra minutes now to save Y per future change when X < n_future × Y.",
               rf"Accept $X$ extra minutes now to save $Y$ per future change when $X \lt {N_FUT} \times Y$.")],
    "medium", "Same rule as DL143. Bare X/Y are wrapped as variables.")
lab("DU096", [("p<0.0001)", r"$p \lt 0.0001$)"), ("(p=0.0002)", "($p=0.0002$)")], "medium",
    "p-values wrapped by stats convention. The IRR ranges and the × multipliers stay prose.")
lab("DU032", [(r"For n=64 stack frames: $\rho$ ≈ 5.3x faster", r"For $n=64$ stack frames: $\rho \approx 5.3$x faster")],
    "medium", "The existing $\\rho$ is absorbed with its value. The 'x' (times faster) stays prose; '5.3\\times' inside the span is an alternative.")
lab("DU098", [("W_eff", r"$W_{\text{eff}}$")], "high")
lab("DU041", [("at N=2-3,", r"at $N=2\text{–}3$,")], "low", "Same as DU037.")
lab("DU046", [("discontinuity_cost < duplication_cost × n_future",
               rf"$\text{{discontinuity-cost}} \lt \text{{duplication-cost}} \times {N_FUT}$")],
    "medium", "The whole inequality is one span, with word variables in \\text ('-' for '_' per the SOP).")
lab("DU058", [("angle ≤ 45° at", r"angle $\le 45^\circ$ at"),
              ("by √(1−r²)", r"by $\sqrt{1-r^2}$"),
              (r"$\alpha$₃", r"$\alpha_3$")],
    "medium",
    "The stray subscript after the existing $\\alpha$ is absorbed into $\\alpha_3$. 'log-odds angle' stays prose, with the relation and value as the span; leaving '≤ 45°' as prose is defensible.")
lab("DL125", [("selection: ρ = 0.96,", r"selection: $\rho = 0.96$,")], "high", "MAE = 5.9% is an acronym metric, left as prose.")
lab("DL085", [("being η_edge is", r"being $\eta_{\text{edge}}$ is")], "high")
lab("DL176", [("(Hκ)", r"(H$_\kappa$)")], "medium",
    "Needed context: the paper this TODO is about writes the hypothesis label as '(H$_\\kappa$)', so κ is a subscript. '(H$\\kappa$)' would be the context-free guess.")
lab("DU088", [("(n=15)", "($n=15$)")], "medium", "A quoted external abstract; sample size.")
lab("DU050", [("O(1) lookup", "$O(1)$ lookup")], "medium", "Big-O in general docs.")
lab("DL066", [("training ρ of", r"training $\rho$ of")], "high")
lab("DU047", [("O(1) access", "$O(1)$ access")], "medium", "Big-O in general docs.")
lab("DL132", [], "medium", "Math is already spanned. 'Model (Σ)' is a quoted model-name label, and the paper's own source (neurips 02 05-mechanism.md) writes it with a bare Σ in prose, so it is left alone; 'Model ($\\Sigma$)' is defensible. Context checked.")
lab("DL023", [("of δ:", r"of $\delta$:")], "high")
lab("DL097", [("Definition δ_regret = A_O(M_t; Π, N_h) − V_O(M_t, π_current; N_h) ≥ 0:",
               r"Definition $\delta_{\text{regret}} = A_O(M_t; \Pi, N_h) - V_O(M_t, \pi_{\text{current}}; N_h) \ge 0$:"),
              ("signal for Σ_t revision", r"signal for $\Sigma_t$ revision"),
              ("large δ_sat + small δ_regret =", r"large $\delta_{\text{sat}}$ + small $\delta_{\text{regret}}$ ="),
              ("check M_t/Π/N_h then consider revising O_t.", r"check $M_t$/$\Pi$/$N_h$ then consider revising $O_t$."),
              ("Large δ_sat + large δ_regret =", r"Large $\delta_{\text{sat}}$ + large $\delta_{\text{regret}}$ ="),
              ("revise Σ_t first", r"revise $\Sigma_t$ first")],
    "medium",
    "In the diagnostic, '+' and '=' join qualified symbols ('large X and small Y means …'), so they are prose and stay outside. M_t/Π/N_h slashes mean 'or', so these are separate spans (as the canon does with $\\alpha$/$\\beta$).")
lab("DL075", [], "high", "Needed context: ∅ is a status marker in a file-map table column, not math.")
lab("DU026", [], "medium", "t=7 is a field in a session-log listing.")
lab("DL001", [(r"$\alpha$ ≈ 2-5 seconds", r"$\alpha \approx 2\text{–}5$ seconds")], "medium",
    "Existing span absorbed with its value range. Leaving the line unchanged is defensible.")
lab("DL077", [("sub-scope β to sub-scope metric-α,", r"sub-scope $\beta$ to sub-scope metric-$\alpha$,")], "high",
    "Canon sub-scope label form.")
lab("DU028", [("(F²)", "($F^2$)")], "high")
lab("DL086", [], "high", "Already spanned. The → are prose diagram arrows.")
lab("DL196", [("α ≈ 0.2", r"$\alpha \approx 0.2$")], "high")
lab("DU074", [("(b) O(h²), (c) O(1)", "(b) $O(h^2)$, (c) $O(1)$")], "high")
lab("DL186", [("(α ≈ 0.3, β ≈ 0.1)", r"($\alpha \approx 0.3$, $\beta \approx 0.1$)")], "high")
lab("DL074", [("sub-scope α/β agents", r"sub-scope $\alpha$/$\beta$ agents")], "high",
    "The canon writes exactly 'sub-scope $\\alpha$/$\\beta$'. The ⊂ between prose noun phrases ('quadratic storage ⊂ general Willems storage') is left as a prose glyph.")
lab("DL102", [("{α, α₁, α₂, α₃, α', β}", r"$\{\alpha, \alpha_1, \alpha_2, \alpha_3, \alpha', \beta\}$")], "high",
    "A set of sub-scope labels; braces escaped.")


def apply(text, reps):
    out = text
    for r in reps:
        old, new = r[0], r[1]
        n = r[2] if len(r) > 2 else 1
        c = out.count(old)
        if c != n:
            raise ValueError(f"expected {n} occurrence(s) of {old!r}, found {c}")
        out = out.replace(old, new)
    return out


SPAN = re.compile(r"\$[^$]+\$")

def validate(text, gold):
    """Shape check: gold == text with contiguous regions replaced by $…$ spans."""
    parts, last = [], 0
    for m in SPAN.finditer(gold):
        parts.append(re.escape(gold[last:m.start()]))
        parts.append("(.+?)")
        last = m.end()
    parts.append(re.escape(gold[last:]))
    if not re.fullmatch("".join(parts), text, re.S):
        return "shape"
    # New spans only: house-convention lint.
    old_spans = set(SPAN.findall(text))
    for s in SPAN.findall(gold):
        if s in old_spans:
            continue
        body = s[1:-1]
        if body != body.strip():
            return f"padding in {s}"
        if re.search(r"[<>*|]", body.replace(r"\|", "")):
            return f"raw <>*| in {s}"
        if body.count("{") - body.count(r"\{") != body.count("}") - body.count(r"\}"):
            return f"brace imbalance in {s}"
        if re.search(r"[^\x00-\x7f]", body.replace("–", "")):
            return f"non-ASCII in {s}"
    return None


def main():
    items = [json.loads(l) for l in TASKS.read_text().splitlines() if l.strip()]
    gids = [d["gid"] for d in items]
    missing = [g for g in gids if g not in L]
    extra = [g for g in L if g not in gids]
    assert not missing and not extra, (missing, extra)
    rows, bad = [], 0
    for d in items:
        reps, conf, note = L[d["gid"]]
        gold = apply(d["text"], reps)
        err = validate(d["text"], gold)
        if err:
            bad += 1
            print(d["gid"], err, file=sys.stderr)
        rows.append({"gid": d["gid"], "gold": gold, "conf": conf, "note": note})
    if bad:
        sys.exit(f"{bad} invalid")
    OUT.write_text("".join(json.dumps(r, ensure_ascii=False) + "\n" for r in rows))
    from collections import Counter
    print(len(rows), "written;", Counter(r["conf"] for r in rows),
          "changed:", sum(r["gold"] != d["text"] for r, d in zip(rows, items)))


if __name__ == "__main__":
    main()
