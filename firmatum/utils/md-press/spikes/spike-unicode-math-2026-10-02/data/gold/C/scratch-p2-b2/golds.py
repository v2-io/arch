# Gold labels for batch-2 (pass 2). Each entry: gid -> (edits, conf, note).
# edits: ordered list of (old_region, new_span); each region is located at or
# after the end of the previous edit, so order matters and repeats are fine.
# An empty edit list means the gold is the input unchanged.
import json, re, sys

H, M, L = "high", "medium", "low"
U = []  # unchanged

G = {
"CL070": ([("O, Σ", r"$O$, $\Sigma$")], M,
  "Topic list; O (objective) and Σ (strategy) are AAT symbols, converted as separate spans. DS, L2 are abbreviations/labels, left."),
"CP079": ([("R_test = Σ(w_i", r"$R_{\text{test}} = \sum(w_i$")], L,
  "Item is cut mid-formula: source line is 'R_test = Σ(w_i × coverage_i) / Σ(w_i)'. Closest in-shape answer spans the visible part; the paren is unbalanced inside the span (renders, but ugly). The fragmenter should not split inside a formula. Needed context."),
"CN007": (U, H, "Only math is `k = 1` in a code span; left."),
"CU128": ([("r_t", "$r_t$")], H, ""),
"CL010": ([("β", r"$\beta$")], H, "Lone symbol (table cell)."),
"CL027": ([("β ≈ 0.3", r"$\beta \approx 0.3$")], H, ""),
"CU003": (U, H, "Math already in spans; arg-max/argmax are grep terms in prose."),
"CN039": (U, H, ""),
"CL035": ([("(h,T)", "$(h,T)$"), ("M=φ(C)", r"$M=\varphi(C)$"), ("α>ρ/R", r"$\alpha \gt \rho/R$"),
           ("X=(M,G)", "$X=(M,G)$"), ("orient O", "orient $O$"), ("O-tower", "$O$-tower"),
           ("T²/T^{3/2}", "$T^2/T^{3/2}$")], M,
  "Shorthand list. Kept the author's letterforms (C not \\mathcal{C}, T = transition in 'double opacity'); canon spells φ as \\phi. Left '⊥' between prose terms (sat-gap ⊥ control-regret) and 'identity=trajectory' as prose shorthand — low-confidence call; a stricter reader would make ⊥ $\\perp$. Context checked (h = observation, T = transition opacity)."),
"CU069": (U, M,
  "(P$^{\\Diamond}$) is a condition label used consistently beside '(P)' in the source, P deliberately upright; absorbing P into the span would italicize the label. Needed context."),
"CU012": ([("n_past", r"$n_{\text{past}}$")], M, "TST variable n_past; word subscript."),
"CU092": ([("H_B1", "$H_{B1}$")], L,
  "Hypothesis label (source table has H_A1, H_A1c, H_B1). Converted as a hypothesis symbol; leaving it as an identifier is equally defensible. Needed context."),
"CU078": ([("(p, theta)", r"$(p, \theta)$"), ("p_ij", "$p_{ij}$"), ("Theta", r"$\theta$"), ("p_ij", "$p_{ij}$")], M,
  "ASCII-spelled Greek. Converted sentence-initial 'Theta' too since it names the same parameter; that is the least certain edit here."),
"CL091": ([("κ→1", r"$\kappa \to 1$")], H, "`κ_processing ≈ 1` is a code span inside a quote; left."),
"CP119": ([("ρ=0.991", r"$\rho=0.991$"), ("|ρ|=0.928", r"$\lvert\rho\rvert=0.928$"), ("p≤0.005", r"$p\le 0.005$")], H,
  "'drop only 0.027' left as prose number."),
"CU068": ([("M_t", "$M_t$")], H, "Inside a quoted caption, still prose math."),
"CU013": (U, H, "Filename."),
"CP146": ([("ρ = 0.991", r"$\rho = 0.991$")], H, ""),
"CU109": (U, H, "Filename."),
"CL056": ([("P ⇔ ¬Q", r"$P \Leftrightarrow \neg Q$")], H, ""),
"CP020": ([("ε", r"$\varepsilon$")], H, "ε-absorption; '32 768 m' is a measurement, left."),
"CU027": ([("n_future = 500-1000", r"$n_{\text{future}} = 500\text{–}1000$")], M,
  "Range hyphen must not become a minus inside math; used an en dash in \\text. '$n_{\\text{future}}$ = 500-1000' (span on the variable only) is the other reasonable answer."),
"CU126": ([("n_past", r"$n_{\text{past}}$")], H, ""),
"CL088": ([("5 minutes < ∞ × 10 seconds", r"$5\text{ minutes} \lt \infty \times 10\text{ seconds}$")], L,
  "Whole line is a word-inequality. Spanning the whole thing with \\text units seemed right; minimal alternative is spans on '<', '∞ ×' only."),
"CP121": ([("file F", "file $F$"), ("ρ-factorization", r"$\rho$-factorization")], M,
  "F is a placeholder variable in a quoted pattern ('what file F asserts'); converting it is the less certain edit."),
"CP063": ([("κ̂", r"$\hat{\kappa}$")], H, "κ + combining circumflex."),
"CP080": ([("η*", r"$\eta^\ast$")], H, ""),
"CL004": ([("ν↑", r"$\nu\uparrow$"), ("⇒", r"$\Rightarrow$")], M,
  "⇒ joins two math clauses; made it its own span. Absorbing it into the following $\\mathcal{T}=…$ span is equally fine."),
"CU030": (U, H, "From a Unicode BMP character table: the glyph is the content, not math. Needed the filename."),
"CN028": (U, H, "Already spanned; '3D' is prose."),
"CU044": ([("Realignment time < n_future × comprehension savings",
            r"$\text{Realignment time} \lt n_{\text{future}} \times \text{comprehension savings}$")], L,
  "Word-equation. Whole-formula span with \\text words; alternative is to span only '< n_future ×'. Ambiguous boundaries."),
"CN017": (U, M, "context_factor is a snake_case identifier name, not a subscripted symbol; [2, 10] and 20-50x read as prose. Leaving unchanged."),
"CL009": ([("π_cont = π_current", r"$\pi_{\text{cont}} = \pi_{\text{current}}$")], H, ""),
"CL057": ([("β-vs-ρ", r"$\beta$-vs-$\rho$")], H, "Mirrors the existing '$\\mathcal{F}$-vs-$S$' in the same list."),
"CN022": (U, H, "Transcript markup."),
"CP042": ([("δ_s", r"$\delta_s$")], H, ""),
"CU137": ([("B_T→V_T", r"$B_T \to V_T$"), ("p=1/2", "$p=1/2$")], M,
  "MASTER (algorithm name) and '/' stay prose. '(→ their W2/K-04)' is a prose pointer arrow, left."),
"CL099": ([("α > β", r"$\alpha \gt \beta$")], H, ""),
"CU143": ([("A_delegate", r"$A_{\text{delegate}}$")], M, "Named evidence-channel symbol; P-A…P-H, D1–D9 are labels."),
"CP093": ([("K", "$K$"), ("θl", r"$\theta_l$"), ("θl", r"$\theta_l$"), ("θl⁢(d)", r"$\theta_l(d)$"), ("eθl′", r"$e_{\theta_l}'$"), ("θl′", r"$\theta_l'$")], L,
  "PDF/HTML-extracted paper text with flattened subscripts (MemoryLLM, arXiv 2402.04624). Read 'eθl′' as e_{θ_l}' (the new memory tokens, primed after the subscript), and θl⁢(d) has an invisible-times U+2062 absorbed into the span. Needed context to reconstruct."),
"CU036": ([("miss ≈ 1−1/n", r"$\text{miss} \approx 1-1/n$"), ("n=3 → 0.667", r"$n=3 \to 0.667$"), ("n=4 → 0.75", r"$n=4 \to 0.75$")], L,
  "Inside a quoted claim; 'miss' as a \\text variable. Arrows map a parameter to a value, read as math; '$n=3$ → 0.667' is the other plausible split."),
"CP038": ([("e_τ", r"$e_\tau$"), ("G_t", "$G_t$")], H, ""),
"CP053": ([("A^(1) ≤ A^RH ≤ A^B", r"$A^{(1)} \le A^{\mathrm{RH}} \le A^{B}$")], H, "RH (receding horizon) as upright superscript."),
"CU127": ([("U_target >= 0", r"$U_{\text{target}} \ge 0$")], H, ""),
"CP135": ([("ρ_B ≈ ν_A · η_A* · |Δa_A|", r"$\rho_B \approx \nu_A \cdot \eta_A^\ast \cdot \lvert\Delta a_A\rvert$"),
           ("ρ_A ≈ ν_B · η_B* · |Δa_B|", r"$\rho_A \approx \nu_B \cdot \eta_B^\ast \cdot \lvert\Delta a_B\rvert$")], H,
  "Agent letters A, B in the parenthetical are prose names, left."),
"CL001": ([("V_O(M_t, π; N_h)", r"$V_O(M_t, \pi; N_h)$")], H, ""),
"CN025": (U, H, "Display math, already LaTeX."),
"CU038": ([("O(1)", "$O(1)$")], M, "Big-O in a code doc; plain O(1) is also common practice."),
"CL095": ([("δ_regret", r"$\delta_{\text{regret}}$")], H, ""),
"CP011": ([("β.", r"$\beta$.")], H, "Final 'in β' is the sub-scope symbol."),
"CU048": ([("t_c", "$t_c$")], H, "~10x left as prose."),
"CU117": ([("n_past < 2", r"$n_{\text{past}} \lt 2$")], H, ""),
"CN047": (U, H, "Emoji shortcode."),
"CL007": (U, H, "No math; '→ a no-go' is a prose arrow."),
"CN045": (U, H, ""),
"CU056": ([("n̂_future", r"$\hat n_{\text{future}}$"), ("n_future", r"$n_{\text{future}}$")], H, "n + combining circumflex; brace-free \\hat n per format.sop's two-spans-with-underscore rule."),
"CL086": ([("α", r"$\alpha$"), ("t_internal_dsl", r"$t_{\text{internal-dsl}}$")], M,
  "Underscore inside \\text changed to '-' per format.sop (GitHub breaks on '_' in \\text); \\text{internal\\_dsl} is the faithful alternative."),
"CL052": ([("Av = λv", r"$Av = \lambda v$")], H, ""),
"CU108": (U, H, "Already spanned."),
"CU086": ([("n_past < 3", r"$n_{\text{past}} \lt 3$")], H, ""),
"CN021": (U, H, ""),
"CP040": ([("κ-factor", r"$\kappa$-factor")], H, ""),
"CU062": (U, H, "t=5 is a turn count in a key=value session listing, not math. Needed context."),
"CP064": ([("ε-reading", r"$\varepsilon$-reading")], H, ""),
"CN015": (U, H, "'$main' is a sigil, single dollar; nothing to promote. (A converter must not pair it with anything.)"),
"CU032": ([("n_past", r"$n_{\text{past}}$")], M, "Heading text; math in headings is fine."),
"CP123": ([("α/β", r"$\alpha/\beta$")], H, ""),
"CP051": ([("k", "$k$", "k memory"), ("θl", r"$\theta_l$"), ("hl", "$h_l$"), ("ϕl", r"$\phi_l$")], M,
  "PDF-flattened subscripts (MemoryLLM figure caption). ϕ (U+03D5) is \\phi."),
"CP112": ([("Φ", r"$\Phi$")], H, ""),
"CP097": ([("α₁", r"$\alpha_1$"), ("α₂", r"$\alpha_2$"), ("β", r"$\beta$")], H, "A2' is an assumption label."),
"CN003": (U, H, "Unicode BMP table entry."),
"CL039": ([("α>ρ/R", r"$\alpha \gt \rho/R$")], H, "Arrows between Parts are prose."),
"CU119": (U, M,
  "W₀ is a wrapping-regime label; ASF canon writes W₀/W₁/W₂ as Unicode text (hundreds of uses) and reserves $W_2$ math for Wasserstein. Needed canon context; without it $W_0$ would look right."),
"CP008": ([("α > ρ/R", r"$\alpha \gt \rho/R$")], H, ""),
"CU000": ([("M_t", "$M_t$")], H, ""),
"CP060": ([("χUKQ $\\chi _\\textUKQ$", r"$\chi_{UKQ}$")], L,
  "Bibliography abstract with an extraction artifact: plain 'χUKQ' followed by a broken span ($\\textUKQ$ is an undefined control sequence). Absorbed both into one span (U, K, Q = uplift, erodibility, precipitation, so italic subscript). Leaving it unchanged is the conservative alternative. Needed context."),
"CU059": ([("n_expected", r"$n_{\text{expected}}$")], M, ""),
"CP054": ([("κ", r"$\kappa$")], H, ""),
"CN023": (U, H, "Library README."),
"CU070": ([("P(t) ∝ 1/t", r"$P(t) \propto 1/t$"), ("t₀", "$t_0$")], H, ""),
"CP104": ([("κ×A", r"$\kappa\times A$"), ("(A)", "($A$)"), ("κ≈1", r"$\kappa\approx 1$")], M,
  "Kept author's A (canon writes \\mathcal{A}). Edits inside the italic metaphor quote are the less certain ones."),
"CU116": ([("Õ(log T/√T)", r"$\tilde{O}(\log T/\sqrt{T})$")], H, ""),
"CP081": ([("Σ_t", r"$\Sigma_t$")], H, ""),
"CL058": (U, M,
  "This is YAML frontmatter flattened into one prose line (the file opens with an HTML comment, so the --- block isn't at line 1). κ×A sits in a YAML comment; leave it. Worth checking whether md-press should be treating this block as prose at all. Needed context."),
"CP043": ([("π-weighted", r"$\pi$-weighted")], H, ""),
"CU141": ([("M < 0.7", r"$M \lt 0.7$")], H, ""),
"CN027": (U, H, "Already spanned."),
"CP114": ([("φ", r"$\varphi$")], M, "AAT compression map; canon spells it \\phi. Glyph-faithful \\varphi used here."),
"CL059": ([("(P, ΔH)", r"$(P, \Delta H)$")], H, ""),
"CP039": ([("π*", r"$\pi^\ast$")], H, ""),
"CN042": (U, H, "ASCII arrow in a workflow note."),
"CP108": ([("(O, Σ)", r"$(O, \Sigma)$"), ("Σ-nodes", r"$\Sigma$-nodes")], H, ""),
"CL006": (U, M,
  "∂(status) is udon's column-notation sigil for a derived field (∂(Field) / ※(Field)), literal header text, not a partial derivative. Needed context."),
"CU131": (U, H, "Filename."),
"CP122": ([("a_{t-1}", "$a_{t-1}$"), ("h", "$h$", "in h "), ("Ω", r"$\Omega$"), ("o_t", "$o_t$"), ("(𝒜)", r"($\mathcal{A}$)"), ("(h)", "($h$)")], H,
  "P-VIS / P-MUT are labels."),
"CL087": (U, M, "'Δ' names a table column whose header is the glyph; kept as text to match the header. Prose arrow left. Needed context."),
"CU138": ([("p=0.009", "$p=0.009$"), ("p<0.001", r"$p\lt 0.001$"), ("p=0.025", "$p=0.025$")], M,
  "p-values in a TODO; plain ASCII would also be acceptable practice."),
"CN024": (U, H, "Display math."),
"CL074": ([("φ ∈ [0,1]", r"$\varphi \in [0,1]$"), ("σ ∈ [0,1]", r"$\sigma \in [0,1]$"), ("ω ≥ 0", r"$\omega \ge 0$"),
           ("λ ≥ 0", r"$\lambda \ge 0$"), ("φ", r"$\varphi$"), ("σ", r"$\sigma$")], H, ""),
"CU097": ([("S₀", "$S_0$")], M, "Controller class S₀ (source: 'φ ∈ S₀'); it is a set, so math. Source itself writes it in code spans."),
"CN046": (U, H, "Already spanned."),
"CP141": ([("Hb", "$H_b$"), ("Uo", "$U_o$"), ("γ", r"$\gamma$")], M,
  "Hb/Uo are flattened H_b (agent opacity) and U_o (observation uncertainty); canon: 'H_b is the formal dual of U_o'. Needed context."),
"CP025": ([("θ", r"$\theta$"), ("Ω", r"$\Omega$"), ("ε", r"$\varepsilon$")], M,
  "↔ maps symbol to prose gloss; left as prose."),
"CL068": (U, H, "μTOSCA is a tool name."),
"CU147": ([("O(1)", "$O(1)$")], M, "Big-O in prose; leaving plain is also defensible."),
"CU017": ([("Riemannian M", "Riemannian $M$"), ("M-choices", "$M$-choices")], H, "(M0), (CT1), CM2-M are labels."),
"CP128": ([("N=40", "$N=40$"), ("N=36", "$N=36$"), ("η²p < .01", r"$\eta^2_p \lt .01$")], M,
  "η²p is partial eta-squared η_p². Sample sizes converted for consistency; leaving N=40 as text is acceptable."),
"CL000": ([("θ", r"$\theta$"), ("θ", r"$\theta$")], H, "Code span left."),
"CL002": ([("τ_i > t", r"$\tau_i \gt t$"), ("τ_i < τ_j", r"$\tau_i \lt \tau_j$"), ("τ_i < τ_j < ... < τ_i", r"$\tau_i \lt \tau_j \lt \dots \lt \tau_i$")], H, ""),
"CU025": (U, M, "W₁ is a regime label; canon keeps it as Unicode text (see CU119)."),
"CU045": (U, H, "Already spanned."),
"CU031": (U, H, "Turn count (see CU062)."),
"CL045": ([("c'θ", r"$c'\theta$"), ("c ≠ 0", r"$c \ne 0$"), ("K", "$K$", "of K")], M,
  "Inside a verbatim quotation; edits are formatting only. '⟺' between prose phrases left as a prose connective."),
"CL073": ([("P·H·U", r"$P \cdot H \cdot U$")], M, "Named decomposition R = Σ P·H·U (elsewhere in the same project). 'probability × severity' left as prose."),
"CL054": (U, M, "'≈C' means 'roughly category C' in a list of index categories (A)/(B). Needed context."),
"CU009": (U, H, "Already spanned."),
"CL097": ([("δ × ∂Σ/∂M", r"$\delta \times \partial\Sigma/\partial M$"), ("δ", r"$\delta$"), ("δ", r"$\delta$"), ("δ", r"$\delta$")], H, ""),
"CL075": ([("V", "$V$", "dominates V"), ("{0,1}", r"$\{0,1\}$"), ("α-invariant", r"$\alpha$-invariant")], M,
  "Model D/S, A.1S(iii), a.s. left as labels/abbreviations; bare 0 and 1 as prose."),
"CL065": ([("ψ*", r"$\psi^\ast$")], M, "Measurement row label; double space kept."),
"CP029": ([("π*", r"$\pi^\ast$")], H, ""),
"CP131": ([("α", r"$\alpha$"), ("β", r"$\beta$"), ("α₂", r"$\alpha_2$")], H, ""),
"CU081": (U, H, "Turn count (see CU062)."),
"CU052": ([("M_t", "$M_t$")], H, ""),
"CL046": ([("α > ρ/R", r"$\alpha \gt \rho/R$")], H, "'iff' stays prose."),
"CP048": ([("ρ", r"$\rho$")], H, ""),
"CU115": ([("g_M(M)", "$g_M(M)$")], H, "'= AUXILIA …' equates math to a prose noun; '=' left outside."),
"CP070": ([("β", r"$\beta$")], H, ""),
"CP127": ([("α", r"$\alpha$"), ("T", "$T$", "to T ")], M,
  "T is the older notation for tempo (now \\mathcal{T}); kept author's T. Needed context."),
"CU001": ([("p=0.947", "$p=0.947$")], M, "4/10 is a count, left."),
"CP028": ([("f-divergences", "$f$-divergences"), ("f∩Bregman", r"$f\cap\text{Bregman}$")], M,
  "'f∩Bregman' = intersection of the f-divergence and Bregman families; '$f$∩Bregman' is the alternative."),
"CU054": ([("√2", r"$\sqrt{2}$")], M,
  "'≈ 5.14', '≈ 1.41', '~3.6×' read as prose approximation typography and are left; converting '≈ 1.41' to '$\\approx 1.41$' would be defensible. Code spans left."),
"CN034": (U, H, "Already spanned ($n_{past}$ is not \\text but it is the author's span)."),
"CL053": ([("Σ/√L", r"$\Sigma/\sqrt{L}$")], H, "Σ is a measured quantity (table header), not a summation operator."),
"CU058": ([("t = -0.17", "$t = -0.17$"), ("df = 8", r"$\mathit{df} = 8$"), ("p = 0.87", "$p = 0.87$"), ("d = 0.056", "$d = 0.056$"),
           ("t = 1.02", "$t = 1.02$"), ("df = 8", r"$\mathit{df} = 8$"), ("p = 0.34", "$p = 0.34$"), ("d = 0.34", "$d = 0.34$"),
           ("t = -1.97", "$t = -1.97$"), ("df = 8", r"$\mathit{df} = 8$"), ("p = 0.08", "$p = 0.08$"), ("d = 0.66", "$d = 0.66$")], L,
  "Verbatim APA-style statistics from a quoted paper. ASCII renders fine; converting is a style choice. If converting, df needs to be one token. Leaving unchanged is a fully defensible gold."),
"CP147": ([("κ × 𝒜", r"$\kappa \times \mathcal{A}$")], H, ""),
"CP106": ([("Δt", r"$\Delta t$")], H, ""),
"CN049": (U, H, "Unicode BMP table entry."),
"CU002": (U, H, "Already spanned (existing span keeps a raw '|'; not ours to rewrite)."),
"CN009": (U, H, "Already spanned."),
"CP142": ([("α", r"$\alpha$")], H, ""),
"CP041": (U, M, "∂(Field) and ※ are udon column-notation sigils (literal header syntax), not math. Needed context."),
"CL015": ([("Σ G^k", r"$\sum G^k$")], M, "Σ as a summation over powers of G (implicit index)."),
"CP024": ([("λ", r"$\lambda$")], H, ""),
"CN013": (U, H, "Unicode BMP table entry."),
"CU136": (U, M, "W₁ is a regime label (see CU119)."),
"CU083": (U, M, "W₁/W₂ regime labels and (C2′) condition label stay text; Θ(ε²) already spanned."),
"CP022": ([("γ > 1", r"$\gamma \gt 1$")], H, ""),
"CP065": ([("ε-B1", r"$\varepsilon$-B1")], H, "B1 is a condition label."),
"CP087": (U, L,
  "'anchors ⊆ ASSUMPTIONS' and 'consumed⇒in-deps' use math glyphs as shorthand between prose/registry names. Left as prose; converting just the glyphs ($\\subseteq$, $\\Rightarrow$) is the alternative."),
"CU067": (U, H, "Already spanned."),
}

def apply(text, edits):
    out, cur = [], 0
    for e in edits:
        old, new = e[0], e[1]
        if len(e) > 2:  # anchor: locate old within the first occurrence of this context
            ctx = e[2]
            j = text.find(ctx, cur)
            assert j >= 0, (ctx, text[cur:cur+80])
            i = text.find(old, j)
        else:
            i = text.find(old, cur)
        assert i >= 0, (old, text[cur:cur+120])
        out.append(text[cur:i]); out.append(new); cur = i + len(old)
    out.append(text[cur:])
    return "".join(out)

def spans(s):
    # split into (is_math, str) using single-$ pairing (no $$ in our new spans)
    return re.split(r"(\$\$.*?\$\$|\$[^$]*\$)", s)

def validate(text, gold):
    # Gold must equal text with some contiguous regions replaced by $..$ spans.
    parts = spans(gold)
    pat = ""
    for k, p in enumerate(parts):
        if k % 2 == 0:
            pat += re.escape(p)
        else:
            # either the same existing span verbatim, or a region
            pat += "(?:" + re.escape(p) + "|(.+?))"
    m = re.fullmatch(pat, text, flags=re.S)
    if not m:
        return False
    for g in m.groups():
        if g and g.count("$") % 2:
            return False
    return True

if __name__ == "__main__":
    src, dst = sys.argv[1], sys.argv[2]
    items = [json.loads(l) for l in open(src)]
    missing = [d["gid"] for d in items if d["gid"] not in G]
    assert not missing, missing
    assert len(G) == len(items), (len(G), len(items))
    with open(dst, "w") as f:
        for d in items:
            edits, conf, note = G[d["gid"]]
            gold = apply(d["text"], edits)
            assert validate(d["text"], gold), d["gid"]
            for s in re.findall(r"\$[^$]+\$", gold):
                assert s[1] != " " and s[-2] != " ", (d["gid"], s)
            f.write(json.dumps({"gid": d["gid"], "gold": gold, "conf": conf, "note": note}, ensure_ascii=False) + "\n")
    print("ok", len(items))
