#!/usr/bin/env python3
"""Substitute hand-written plain-Unicode renderings into each $...$ span, in order.

Only span contents are replaced; all text outside spans is copied verbatim from the
task line. Span counts are checked so a miscount fails loudly instead of misaligning.
"""
import json, re, sys
from pathlib import Path

BASE = Path(__file__).resolve().parent.parent
TASK = BASE / "tasks" / "batch-2.jsonl"
OUT = BASE / "out" / "batch-2.jsonl"

SPAN = re.compile(r"\$([^$]+)\$")

R = {
"S2000": ["t_handle^unsupervised ≫ t_handle^supervised"],
"S2001": ["ε*", "ε*·ν_c", "ρ/α"],
"S2002": ["B"],
"S2003": ["S", "U"],
"S2004": ["t_c"],
"S2005": ["β"],
"S2006": ["< 0"],
"S2007": ["(Q, R)", "K̃ = 0", "α"],
"S2008": ["n_future"],
"S2009": ["do(a)"],
"S2010": ["R"],
"S2011": ["n₂"],
"S2012": ["√·"],
"S2013": ["G(x, y)"],
"S2014": ["α'"],
"S2015": ["C_i"],
"S2016": ["V_max · TV"],
"S2017": ["`alignment_component` ≈ 0.75"],
"S2018": ["do(·)"],
"S2019": ["Ω", "T"],
"S2020": ["t_deploy ≈ t_compile", "t_deploy = t_restart + t_recovery"],
"S2021": ["R* = ρ/α", "𝓑_{R*}", "𝓑_{R*}"],
"S2022": ["T_isolated = t_d + t_r"],
"S2023": ["Σ_t"],
"S2024": ["α'/β'", "H_b", "α/β", "α'/β'"],
"S2025": ["α"],
"S2026": ["T_recovery"],
"S2027": ["W"],
"S2028": ["≈ 1 − η*·c_min"],
"S2029": ["π_cont = π_current"],
"S2030": ["t_s"],
"S2031": ["κ = 0"],
"S2032": ["η_k"],
"S2033": ["Δt_i"],
"S2034": ["M_t ∈ ℳ"],
"S2035": ["M_t = φ(𝒞_t)"],
"S2036": ["W₁", "W₂"],
"S2037": ["P_catch^dialyzer"],
"S2038": ["L_i/|F_i|"],
"S2039": ["C", "C"],
"S2040": ["m"],
"S2041": ["𝓑_R", "E[‖w(t)‖²] = σ_w²"],
"S2042": ["β ≈ 0.15", "τ = 180"],
"S2043": ["C", "(2K+1)"],
"S2044": ["C", "A", "B", "Cov(r_A, r_B) > 0"],
"S2045": ["‖δ‖_new ≈ ρ/α'", "α'"],
"S2046": ["C = 1"],
"S2047": ["ρ/𝒯", "B", "ρ_B", "A", "𝒯_A", "(𝒯_A/𝒯_B)·(𝒯_A/𝒯_B) = (𝒯_A/𝒯_B)²"],
"S2048": ["W₂² ≤ (2/ρ_LSI)·KL", "d_FR² ≤ 2·KL", "C_FR = √2"],
"S2049": ["n", "n₁ > n₂", "ε* = (n₂+1)/(n₁+n₂+2) < 1/2", "(1−ε) > 1/2", "n"],
"S2050": ["A_O^(i) = sup_{π∈Π} V_O^(i)(M_t, π; N_h)", "i"],
"S2051": ["→", "→", "→", "→", "→"],
"S2052": ["α_u < 1"],
"S2053": ["α > ρ/R"],
"S2054": ["θ"],
"S2055": ["H̃_t"],
"S2056": ["r_{one-for-one}(i) = {A_i}"],
"S2057": ["a_t = π(M_t)", "π(M_t, G_t)", "M_t"],
"S2058": ["U_M = 1", "U_Σ = 0"],
"S2059": ["Q", "R", "K*", "(Q*, R*)", "Q̂_t, R̂_t", "K_t = K(Q̂_t, R̂_t)"],
"S2060": ["β ≈ 0.2"],
"S2061": ["L_A"],
"S2062": ["δ_epistemic", "δ_strategic"],
"S2063": ["t_d"],
"S2064": ["k"],
"S2065": ["p_dep", "j ≠ i"],
"S2066": ["α₂"],
"S2067": ["C"],
"S2068": ["X_t", "G_t = ∅"],
"S2069": ["I(a)"],
"S2070": ["α_{PID-Lur'e} = min(α_linear, k₁·c_abs)/κ(P)"],
"S2071": ["p_f"],
"S2072": ["S=1", "S=1", "T"],
"S2073": ["𝒯_Σ = ∑ ν_Σ·η_Σ*"],
"S2074": ["P_restart^supervision"],
"S2075": ["(p₁, p₂)", "y_G", "B"],
"S2076": ["T_act"],
"S2077": ["n_affected = 3", "d_base = 0.1·t_b", "t_coord = 0.5·t_b", "p_cross = 0.5"],
"S2078": ["ℐ(e_τ)", "ℐ"],
"S2079": ["Ṙ_min,i = α_i/2", "∑_i α_i/2"],
"S2080": ["λ_J", "Q_J"],
"S2081": ["𝒳_M"],
"S2082": ["ν_eff", "η^(k)*"],
"S2083": ["∑_k ν^(k)·η^(k)*", "𝒯"],
"S2084": ["M_t = φ(𝒞_t)"],
"S2085": ["r > 3.5"],
"S2086": ["(ν, U_o)"],
"S2087": ["T_traditional − T_meta"],
"S2088": ["`confusion_events`"],
"S2089": ["I(M_t; o_{t+1:∞})", "I(M_t; identity_{t+1})"],
"S2090": ["κ"],
"S2091": ["M_t"],
"S2092": ["ε*"],
"S2093": ["U_o", "k"],
"S2094": ["|𝒜| ≥ 2", "O_t", "Σ_t"],
"S2095": ["t_c"],
"S2096": ["θ ← θ + K(t)(y_t − ŷ_t)"],
"S2097": ["PRT ≡ SMD ∅", "PRV ≡ SMC ∅ ≡ MOD ∅", "SMD ∅", "SMC ∅", "MOD ∅"],
"S2098": ["C", "C", "Ψ_α", "(2K+1)"],
"S2099": ["H = I", "A = I", "Q = diag(q₁, …, qₙ)", "R_obs = diag(r₁, …, rₙ)", "n"],
"S2100": ["s'_c"],
"S2101": ["identity_{t+1:}"],
"S2102": ["𝔼[ΔV] < 0"],
"S2103": ["α_Σ^ss = (1−λ)/(2−λ)", "α = 1/(n+1)", "n_eff = 1/(1−λ)"],
"S2104": ["𝒜", "O(1)", "∂", "Σ", "∂ ≈ Σ", "𝒜"],
"S2105": ["Δ"],
"S2106": ["C", "Ψ_post = Ψ_prior + G(observation)"],
"S2107": ["α ∈ [0.05, 0.15]"],
"S2108": ["I"],
"S2109": ["𝒞_t"],
"S2110": ["T_opportunity"],
"S2111": ["λ₂"],
"S2112": ["w_c = 0.3"],
"S2113": ["λ_info ∝ U_M", "λ_surv ∝ 1/U_M"],
"S2114": ["ρ_B ≈ γ·T_A", "ρ_A ≈ γ·T_B", "‖δ‖_B / ‖δ‖_A = (γT_A/T_B) / (γT_B/T_A) = (T_A/T_B)²"],
"S2115": ["δ_regret", "A_O", "V_O(M_t, π_current; N_h)", "δ_sat"],
"S2116": ["σ_age"],
"S2117": ["φ*"],
"S2118": ["τ_critical ≈ 0.7"],
"S2119": ["I", "I = log N"],
"S2120": ["R", "𝟙[s_T ∈ R]", "V_{O'}"],
"S2121": ["K ≥ 2", "ν_A/K", "ν_A"],
"S2122": ["t_c^task ≈ 0.1ms"],
"S2123": ["(A₁, A₂, …, A_ℓ)", "k", "κ_k", "∏_ℓ κ_ℓ", "∑_ℓ log κ_ℓ", "∑_ℓ log(ν_ℓ/α_ℓ)"],
"S2124": ["t_i"],
"S2125": ["γ^adv·𝒯_j", "i"],
"S2126": ["n = 13", "x"],
"S2127": ["t_refactor < n_future × t̄_saved"],
"S2128": ["ε ≈ 0"],
"S2129": ["𝒯 = ν·α = 0.091", "ρ = k·√q = 0.4", "η*"],
"S2130": ["t_m"],
"S2131": ["𝒯 = ν · η*", "𝒯 = Σ_k ν^(k) · η^(k)*"],
"S2132": ["s"],
"S2133": ["λ", "λ(M_t)", "λ"],
"S2134": ["G_t"],
"S2135": ["ε_x, ε_a, ε_o"],
"S2136": ["d"],
"S2137": ["n", "n"],
"S2138": ["δ", "η*", "ρ"],
"S2139": ["P_survive"],
"S2140": ["α = 0.2"],
"S2141": ["ρ_i^surprise = √q/σ = 0.1", "σ = 1", "σ ≠ 1"],
"S2142": ["T_restart", "ρ_child", "δ_failure", "δ_critical^wrapper"],
"S2143": ["L", "M", "M"],
"S2144": ["C"],
"S2145": ["𝓜 ≻ 0", "κ > 0", "κ = 0"],
"S2146": ["n × p_f > 0.1", "m > 10"],
"S2147": ["α_k = α₀·e^(−λk)", "λ"],
"S2148": ["t_comp = t_base"],
"S2149": ["β ≈ 2"],
}

NOTES = {
"S2017": "Backticked the subscripted name because it reads like a code identifier to me; the ≈ 0.75 stays outside the backticks.",
"S2088": "Wrapped in backticks: `confusion_events` is an identifier-looking name and I would code-span it rather than treat it as math.",
"S2048": "\\frac{2}{ρ_LSI} flattened to (2/ρ_LSI)·KL, which is how I'd type it inline.",
"S2051": "Each bare \\to became a bare →, which reads as prose punctuation rather than math in this sentence.",
"S2097": "This line is PDF-extracted paper text, not an agent note; I just swapped ≡ and ∅ in place.",
"S2006": "The span is only an operator plus number ('LHS < 0'), so nothing marks it as math once the $ is gone.",
"S2028": "Used U+2212 minus here; most other places I used it too, but ASCII '-' stays in (2K+1) and a few identifiers.",
"S2073": "Used ∑ (U+2211) for the sum so it doesn't collide with the Σ subscript; elsewhere (S2131) I used Σ_k for the sum, which is the ambiguous form agents also write.",
"S2131": "Σ_k (Greek capital sigma) as the summation sign, deliberately: this is a common real-world spelling and it is ambiguous with Σ_t (strategy state) used elsewhere in the corpus.",
"S2104": "Here Σ is a type name (accumulation) not a sum; ∂ likewise is a type tag. Context-only disambiguation.",
"S2021": "\\mathcal B -> 𝓑 (bold script, U+1D4D1). I mixed script styles across lines (ℳ vs 𝓜, 𝒞, ℐ) the way copy-paste from different sources tends to.",
"S2120": "Kept braces in V_{O'} because V_O' would read as the derivative/prime of V_O.",
"S2070": "Braces kept around the hyphenated/apostrophe subscript; without them the subscript extent is unrecoverable.",
"S2079": "Ṙ uses a combining dot (R + U+0307). The comma subscript 'min,i' is left unbraced, which is how I'd type it but is ambiguous.",
"S2147": "Exponent written e^(−λk) with parentheses rather than braces.",
"S2050": "Mixed conventions in one span: ^(i) with parens, sup_{π∈Π} with braces.",
"S2089": "Kept braces on o_{t+1:∞}; o_t+1:∞ would be unreadable.",
}


def main():
    out_lines = []
    errs = []
    for raw in TASK.read_text(encoding="utf-8").splitlines():
        if not raw.strip():
            continue
        rec = json.loads(raw)
        rid, latex = rec["id"], rec["latex"]
        spans = SPAN.findall(latex)
        reps = R.get(rid)
        if reps is None:
            errs.append(f"{rid}: no replacements")
            continue
        if len(reps) != len(spans):
            errs.append(f"{rid}: {len(spans)} spans, {len(reps)} replacements: {spans}")
            continue
        it = iter(reps)
        uni = SPAN.sub(lambda m: next(it), latex)
        # independent check: every outside segment appears, in order, in the output
        pos = 0
        for seg in SPAN.split(latex)[0::2]:
            k = uni.find(seg, pos)
            if k < 0:
                errs.append(f"{rid}: outside segment lost: {seg!r}")
                break
            pos = k + len(seg)
        out_lines.append({"id": rid, "unicode": uni, "note": NOTES.get(rid, "")})
    extra = set(R) - {json.loads(l)["id"] for l in TASK.read_text().splitlines() if l.strip()}
    if extra:
        errs.append(f"unused ids: {sorted(extra)}")
    if errs:
        print("\n".join(errs), file=sys.stderr)
        sys.exit(1)
    OUT.parent.mkdir(parents=True, exist_ok=True)
    with OUT.open("w", encoding="utf-8") as f:
        for o in out_lines:
            f.write(json.dumps(o, ensure_ascii=False) + "\n")
    print(f"wrote {len(out_lines)} lines to {OUT}")


if __name__ == "__main__":
    main()
