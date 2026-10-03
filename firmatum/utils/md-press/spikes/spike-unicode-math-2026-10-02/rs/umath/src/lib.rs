//! Deterministic Unicode-math -> `$LaTeX$` converter for md-press prose.
//!
//! A behavior-identical Rust port of the spike's measured Python reference
//! (`py/frozen/umath_v7.py`; `convert_v6` … `convert_v3` / `convert_ver`
//! reproduce the earlier frozen versions). Read
//! PORT.md before changing anything: the measurements transfer only while
//! the behavior does, and the differential harness in `tools/` is how that
//! is checked.
//!
//! The output is always "the input with some regions replaced by `$...$`
//! spans"; nothing outside a span is touched.

// The structure mirrors the Python reference line by line (index loops,
// empty `pass` arms, un-simplified conditions) so the port can be read side
// by side with umath_v*.py; these lints would trade that for style.
#![allow(
    clippy::needless_range_loop,
    clippy::if_same_then_else,
    clippy::nonminimal_bool,
    clippy::explicit_counter_loop,
    clippy::needless_late_init,
    clippy::collapsible_if,
    clippy::needless_lifetimes
)]

mod consts;
mod convert;
pub mod lex;
mod spans;
pub(crate) mod tables;
mod term;
pub mod uni;

/// Which frozen reference to reproduce. Later versions include earlier rules.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Ver {
    V3,
    V4,
    V5,
    V6,
    V7,
}

/// One converted (or abstained-on) region. Offsets are in chars of the text
/// the span refers to. `latex: None` = a candidate the converter declined.
#[derive(Clone, Debug, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub latex: Option<String>,
    pub conf: f64,
}

/// Inputs on which the Python reference raises; reproduced, not fixed, so
/// the two agree everywhere. See PORT.md "reference bugs".
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    /// `mathalpha_latex` KeyError (U+1D7CA, bold capital digamma; v3-v5 only)
    MathalphaKeyError(char),
    /// a Python IndexError path
    IndexError(&'static str),
    /// Python RecursionError analogue: recursion deeper than Python's
    /// default 1000-frame limit (deeply nested brackets / prefix chains).
    /// Rust would otherwise overflow its stack and abort. See PORT.md.
    Recursion,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::MathalphaKeyError(c) => write!(f, "KeyError (mathalpha_latex U+{:04X})", *c as u32),
            Error::IndexError(w) => write!(f, "IndexError ({w})"),
            Error::Recursion => write!(f, "RecursionError"),
        }
    }
}

impl std::error::Error for Error {}

/// Python's default recursion limit. Every function that recurses in the
/// reference counts one level here; Python's frame stack holds those same
/// calls plus its caller's frames, so wherever the reference succeeds the
/// count stays below this.
const RECURSION_LIMIT: usize = 1000;

thread_local! {
    /// The frozen version being reproduced by the current `run` (some tables,
    /// e.g. OPS, differ by version and are consulted deep in the lexer).
    static VER: std::cell::Cell<Ver> = const { std::cell::Cell::new(Ver::V7) };
    static DEPTH: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static BLOWN: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Recursion guard: `let Some(_g) = Deep::enter() else { return None };`.
/// Once the limit is passed every guarded call bails, and `run` reports
/// `Error::Recursion` (the partial result is discarded).
pub(crate) struct Deep;

impl Deep {
    #[inline]
    pub(crate) fn enter() -> Option<Deep> {
        if BLOWN.get() {
            return None;
        }
        let d = DEPTH.get() + 1;
        if d > RECURSION_LIMIT {
            BLOWN.set(true);
            return None;
        }
        DEPTH.set(d);
        Some(Deep)
    }
}

impl Drop for Deep {
    fn drop(&mut self) {
        DEPTH.set(DEPTH.get() - 1);
    }
}

#[inline]
pub(crate) fn ver() -> Ver {
    VER.get()
}

fn run(text: &str, ver: Ver) -> Result<(String, Vec<Span>), Error> {
    VER.set(ver);
    DEPTH.set(0);
    BLOWN.set(false);
    let r = run_inner(text, ver);
    if BLOWN.get() {
        BLOWN.set(false);
        return Err(Error::Recursion);
    }
    r
}

fn run_inner(text: &str, ver: Ver) -> Result<(String, Vec<Span>), Error> {
    let s: Vec<char> = text.chars().collect();
    let (out, spans) = match ver {
        Ver::V3 => convert::convert_once(&s, Ver::V3)?,
        Ver::V4 | Ver::V5 | Ver::V6 | Ver::V7 => convert::convert_fixpoint(&s, ver)?,
    };
    Ok((out.into_iter().collect(), spans))
}

/// `convert(text)` of the latest frozen reference (umath_v7): (converted
/// text, spans).
pub fn convert(text: &str) -> Result<(String, Vec<Span>), Error> {
    run(text, Ver::V7)
}

/// umath_v6 `convert(text)`.
pub fn convert_v6(text: &str) -> Result<(String, Vec<Span>), Error> {
    run(text, Ver::V6)
}

/// umath_v5 `convert(text)`.
pub fn convert_v5(text: &str) -> Result<(String, Vec<Span>), Error> {
    run(text, Ver::V5)
}

/// umath_v4 `convert(text)`.
pub fn convert_v4(text: &str) -> Result<(String, Vec<Span>), Error> {
    run(text, Ver::V4)
}

/// Any frozen reference version.
pub fn convert_ver(text: &str, ver: Ver) -> Result<(String, Vec<Span>), Error> {
    run(text, ver)
}

/// umath_v3 `convert(text)`.
pub fn convert_v3(text: &str) -> Result<(String, Vec<Span>), Error> {
    run(text, Ver::V3)
}

/// The well-formedness check the converter applies to every span it writes.
pub fn tex_ok(lat: &str) -> bool {
    convert::tex_ok(lat).unwrap_or(false)
}
