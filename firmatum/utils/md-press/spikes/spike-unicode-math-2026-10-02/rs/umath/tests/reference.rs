//! Spot checks against outputs of the Python reference (computed with
//! py/frozen/umath_v7.py, umath_v6.py, umath_v5.py and umath_v3.py). The real evidence is the full
//! differential (tools/differential.sh); these keep `cargo test` meaningful.
//! The v5 tests pin *reference bugs* on purpose (see PORT.md): the port
//! reproduces each version exactly so its measurements transfer; v6 fixed
//! four of them, and the v6 tests pin the fixes.

fn v5(t: &str) -> String {
    umath::convert_v5(t).unwrap().0
}

fn v6(t: &str) -> String {
    umath::convert_v6(t).unwrap().0
}

fn v7(t: &str) -> String {
    umath::convert(t).unwrap().0
}

#[test]
fn ordinary_conversions() {
    assert_eq!(v5("Let ‖δ‖ ≤ R and M_τ⁺ hold"), r"Let $\lVert \delta\rVert \leq R$ and $M_{\tau^+}$ hold");
    assert_eq!(v5("κ_processing scales as O(n²)"), r"$\kappa_{\text{processing}}$ scales as $O(n^2)$");
    assert_eq!(v5("Appendix E shows x = 3"), "Appendix E shows $x = 3$");
    assert_eq!(v5("ρ(consistency, −IQR) ≈ 0.4"), r"$\rho(\text{consistency}, - \text{IQR}) \approx 0.4$");
    assert_eq!(v5("√(2η − η²)"), r"$\sqrt{2\eta - \eta^2}$");
    assert_eq!(v5("the value \"κ_t is"), "the value \"$\\kappa_t$ is");
}

#[test]
fn leaves_alone() {
    for t in [
        "plain prose with no math at all.",
        "W₀/W₁/W₂ regimes",
        "$5 / unit · fee: $25",
    ] {
        assert_eq!(v5(t), t);
    }
}

#[test]
fn spans_report_char_offsets() {
    let (_, sp) = umath::convert("Let ‖δ‖ ≤ R ok").unwrap();
    assert_eq!(sp.len(), 1);
    assert_eq!((sp[0].start, sp[0].end), (4, 11));
    assert_eq!(sp[0].latex.as_deref(), Some(r"\lVert \delta\rVert \leq R"));
}

#[test]
fn v3_matches_on_shared_behavior() {
    assert_eq!(umath::convert_v3("κ_processing scales as O(n²)").unwrap().0, v5("κ_processing scales as O(n²)"));
}

#[test]
fn v6_matches_v5_on_ordinary_text() {
    for t in ["Let ‖δ‖ ≤ R and M_τ⁺ hold", "κ_processing scales as O(n²)", "√(2η − η²)", "W₀/W₁/W₂ regimes"] {
        assert_eq!(v6(t), v5(t));
    }
}

// ---- v6 fixes

#[test]
fn v6_fixes() {
    assert_eq!(v6("let 𝟊 = 1"), "let 𝟊 = 1");
    assert_eq!(v6("A†ⁱ = B"), r"$A^{\dagger i} = B$");
    assert_eq!(v6("Ξ*^T = 1"), r"$\Xi^{\ast T} = 1$");
    assert_eq!(v6("x†ᵀ³ = y"), r"$x^{\dagger T3} = y$");
    assert_eq!(v6("the value \"κ_t"), "the value \"$\\kappa_t$");
    assert_eq!(v6("⋃_i A_i = X"), r"$\bigcup_iA_i = X$");
    // an author's own command inside a merged existing span is allowed
    assert_eq!(v6(r"$\foo{x}$ and η = 1"), r"$\foo{x}$ and $\eta = 1$");
}

// ---- v7 changes (hyphen compounds, `$` in inline code)

#[test]
fn v7_changes() {
    assert_eq!(v6("an $n$-dim space"), r"an $n - \dim$ space");
    assert_eq!(v7("an $n$-dim space"), "an $n$-dim space");
    assert_eq!(v6("the γ-sign flip"), r"the $\gamma - \operatorname{sign}$ flip");
    assert_eq!(v7("the γ-sign flip"), r"the $\gamma$-sign flip");
    assert_eq!(v7("Δ(near-boundary − elsewhere)"), r"$\Delta(\text{near-boundary} - \text{elsewhere})$");
    assert_eq!(v6("set `$HOME` and η = 1"), r"set `$HOME` and $\eta = 1$");
    assert_eq!(v7("set `$HOME` and η = 1"), "set `$HOME` and η = 1");
    assert_eq!(v7("1-exp(-x) for x ≥ 0"), r"1-exp(-x) for $x \geq 0$");
    // as frozen: the compound is split at the math boundary (see PORT.md)
    assert_eq!(v7("log-det/λ grows"), r"log-$\det/\lambda$ grows");
}

#[test]
fn v7_matches_v6_on_ordinary_text() {
    for t in ["Let ‖δ‖ ≤ R and M_τ⁺ hold", "κ_processing scales as O(n²)", "Ξ*^T = 1", "⋃_i A_i = X"] {
        assert_eq!(v7(t), v6(t));
    }
}

// ---- reference bugs in v3-v5, reproduced deliberately

#[test]
fn bug_digamma_raises() {
    assert!(matches!(umath::convert_v5("let 𝟊 = 1"), Err(umath::Error::MathalphaKeyError('𝟊'))));
    assert!(umath::convert_v5("`𝟊`").is_ok()); // protected: never lexed
}

#[test]
fn bug_glued_command_in_superscript() {
    assert_eq!(v5("A†ⁱ = B"), r"$A^{\daggeri} = B$");
    assert_eq!(v5("Ξ*^T = 1"), r"$\Xi^{\astT} = 1$");
}

#[test]
fn bug_open_quote_at_end_abstains() {
    assert_eq!(v5("the value \"κ_t"), "the value \"κ_t");
}
