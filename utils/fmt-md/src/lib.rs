//! fmt-md — markdown canonicalizer for the archema program's house standards.
//!
//! Phase 1–2 scope: deterministic, structure-aware unwrap of hard-wrapped
//! prose (PLAN.md §Policy, ratified 2026-07-22): every prose paragraph
//! becomes one logical line. The decision of *which* newlines join is made
//! by the CommonMark parse (comrak, with math/wikilink/frontmatter
//! extensions), never by line-shape heuristics. Everything outside a joined
//! paragraph is emitted byte-identically.

use comrak::nodes::{AstNode, NodeValue};
use comrak::{Arena, Options, parse_document};

pub mod classify;
pub mod exclude;
pub mod math;

/// comrak options matching the corpus's construct inventory (requirements
/// R1–R4): tables, math-dollars, wikilinks, footnotes, YAML frontmatter,
/// strikethrough, task lists. Sourcepos drives the join engine.
pub fn options() -> Options<'static> {
    let mut o = Options::default();
    o.extension.table = true;
    o.extension.strikethrough = true;
    o.extension.tasklist = true;
    o.extension.footnotes = true;
    o.extension.math_dollars = true;
    o.extension.wikilinks_title_after_pipe = true;
    o.extension.front_matter_delimiter = Some("---".into());
    o.render.sourcepos = true;
    o
}

/// A paragraph eligible for joining: 1-based inclusive source-line span plus
/// its blockquote nesting depth (number of `>` prefixes a continuation line
/// may carry).
#[derive(Debug)]
struct ProsePara {
    start: usize,
    end: usize,
    quote_depth: usize,
}

fn quote_depth<'a>(node: &'a AstNode<'a>) -> usize {
    let mut d = 0;
    let mut cur = node.parent();
    while let Some(p) = cur {
        if matches!(p.data.borrow().value, NodeValue::BlockQuote) {
            d += 1;
        }
        cur = p.parent();
    }
    d
}

fn collect_paras<'a>(root: &'a AstNode<'a>, out: &mut Vec<ProsePara>) {
    for node in root.descendants() {
        if let NodeValue::Paragraph = node.data.borrow().value {
            let sp = node.data.borrow().sourcepos;
            if sp.end.line > sp.start.line {
                out.push(ProsePara {
                    start: sp.start.line,
                    end: sp.end.line,
                    quote_depth: quote_depth(node),
                });
            }
        }
    }
}

// Deliberately absent: a rule making inline `$…$` / `` ` `` spans opaque to
// joining. It was implemented and reverted the same day (2026-07-22), and
// the reasoning is worth keeping because the instinct to add it is strong.
//
// A column-wrapper splits mid-expression all the time — `$\mathcal` /
// `M_{adm}$` is a real case from the asf corpus — and those breaks are
// exactly the wraps this tool exists to repair. Refusing to join them would
// sacrifice the primary use case. Joining is safe: LaTeX and MathJax ignore
// whitespace in math mode, and CommonMark already converts a line ending
// inside a code span to a space, so the rendered document is unchanged
// either way. `random_wrap_recovery` fails the moment the rule goes in.
//
// The case that motivated trying it — 20 raw session transcripts whose
// pasted shell (`"$UDON_SESS"` … `"$d"`) parses as accidental math — is not
// a rendering problem at all. Those files render identically before and
// after; what the join destroys is their value *as evidence*. No render
// check can see that, which is why `exclude.rs` exists: verbatim material is
// declared, never inferred.

/// True if `line` carries a CommonMark hard line break at its end
/// (two+ trailing spaces, or a trailing backslash). Such breaks are
/// semantic (R8) and are never join points.
fn has_hard_break(line: &str) -> bool {
    line.ends_with("  ") || line.ends_with('\\')
}

/// A continuation line that *looks like* a footnote or link-reference
/// definition (`[^label]: …` / `[label]: …`). CommonMark parses it as lazy
/// paragraph continuation (definitions cannot interrupt a paragraph), but
/// the author almost certainly intended a definition — joining it would
/// bury that intent irrecoverably (R15e: current-parse ≠ author-intent).
/// Such breaks are preserved.
fn looks_like_definition(content: &str) -> bool {
    let s = content.trim_start();
    let Some(rest) = s.strip_prefix('[') else {
        return false;
    };
    if let Some(close) = rest.find(']') {
        close > 0 && rest[close + 1..].starts_with(':')
    } else {
        false
    }
}

/// A standalone equation-level epistemic tag per the asf house convention
/// (`*[Definition (slug)]*`, `*[Derived (…, from …)]*`, …). These are
/// block-level markers for the equation that follows; they keep their own
/// line even though CommonMark sees them as paragraph text.
fn is_standalone_eq_tag(content: &str) -> bool {
    let s = content.trim();
    s.starts_with("*[") && s.ends_with("]*") && !s[2..s.len() - 2].contains("]*")
}

/// A line that is entirely one `$…$` math expression. Authors write these
/// as display equations (the house fix is promotion to `$$…$$`, a later
/// phase); absorbing one into prose would bury an equation. It keeps its
/// own line.
fn is_standalone_math(content: &str) -> bool {
    let s = content.trim();
    s.len() > 2
        && s.starts_with('$')
        && s.ends_with('$')
        && !s.starts_with("$$")
        && !s[1..s.len() - 1].contains('$')
}

/// Strip a continuation line's container prefix: leading whitespace, then up
/// to `quote_depth` `>` markers (each optionally followed by one space),
/// then remaining indentation. What's left is the prose content.
fn strip_continuation_prefix(line: &str, quote_depth: usize) -> &str {
    let mut rest = line.trim_start_matches([' ', '\t']);
    for _ in 0..quote_depth {
        if let Some(r) = rest.strip_prefix('>') {
            rest = r.strip_prefix(' ').unwrap_or(r);
            rest = rest.trim_start_matches([' ', '\t']);
        } else {
            break; // lazy continuation: no marker present
        }
    }
    rest
}

/// A classifier verdict the caller may want to tell the user about.
pub struct Note {
    pub kept: bool,      // true = kept + marked; false = joined
    pub p: f64,          // calibrated P(phb)
    pub line_a: String,  // the two sides of the break, for the stderr note
    pub line_b: String,
}

/// Output of a classified format run: the final text (with any added `"  "`
/// hard-break markers), a gate variant (identical but without the added
/// markers — marker insertion deliberately changes rendering, so the
/// render-equality gate must compare against this), and the mid-band notes.
pub struct Classified {
    pub output: String,
    pub gate_output: String,
    pub notes: Vec<Note>,
}

/// Canonicalize: join every multi-line prose paragraph to one logical line.
/// All other bytes pass through unchanged. Idempotent by construction
/// (single-line paragraphs are not touched).
pub fn format(input: &str) -> String {
    format_impl(input, None).output
}

/// Canonicalize with the break classifier consulted on every candidate join
/// (the ratified band design, STATUS.md): p ≥ 0.85 keep + mark silently;
/// 0.5–0.85 keep + mark + note; 0.15–0.5 join + note; < 0.15 join silently.
/// Deterministic preserved-break categories still take precedence — the
/// model only judges breaks the engine would otherwise join.
pub fn format_classified(input: &str, clf: &classify::Classifier) -> Classified {
    format_impl(input, Some(clf))
}

fn format_impl(input: &str, clf: Option<&classify::Classifier>) -> Classified {
    let arena = Arena::new();
    let opts = options();
    let root = parse_document(&arena, input, &opts);
    let mut paras = Vec::new();
    collect_paras(root, &mut paras);
    // comrak hoists footnote definitions in the AST, so traversal order is
    // not source order; sort by span and drop any overlap defensively
    // (not-joining is always the safe direction).
    paras.sort_by_key(|p| p.start);
    let mut last_end = 0;
    paras.retain(|p| {
        let ok = p.start > last_end;
        if ok {
            last_end = p.end;
        }
        ok
    });

    // line index -> the paragraph it continues (if it is a continuation)
    let lines: Vec<&str> = input.split_inclusive('\n').collect();
    let bare_lines: Vec<&str> = lines.iter().map(|l| split_eol(l).0).collect();
    let fstats = clf.map(|_| classify::file_stats(&bare_lines));
    let mut notes: Vec<Note> = Vec::new();
    let mut out = String::with_capacity(input.len());
    let mut gate = String::with_capacity(input.len());
    let mut joined_until: usize = 0; // 1-based line already consumed through

    macro_rules! emit {
        ($s:expr) => {{
            out.push_str($s);
            gate.push_str($s);
        }};
    }

    for para in &paras {
        // emit untouched lines before this paragraph
        for l in joined_until..para.start - 1 {
            emit!(lines[l]);
        }
        // Partition the paragraph's lines into runs terminated by hard
        // line breaks (which are semantic, R8, and stay). Each run joins
        // to one physical line. A run's first line keeps its own raw
        // prefix (paragraph-initial or post-break continuation — either
        // way its prefix is structural); later lines in the run get their
        // container prefix stripped before joining.
        let mut run_first = true;
        let mut acc = String::new();
        let mut acc_eol = "\n";
        for idx in para.start - 1..para.end {
            let (content, eol) = split_eol(lines[idx]);
            let stripped_probe = strip_continuation_prefix(content, para.quote_depth);
            let solo = is_standalone_eq_tag(stripped_probe) || is_standalone_math(stripped_probe);
            // Equation-after-colon idiom: a line ending `:` announces the
            // math line that follows (house pseudo-display form) — the
            // break is authorial, not a wrap.
            let after_colon_math = acc.trim_end().ends_with(':') && stripped_probe.starts_with('$');
            let own_line = looks_like_definition(stripped_probe) || solo || after_colon_math;
            if !run_first && own_line {
                // intent-preserving break: flush the run before this line
                emit!(&acc);
                emit!(acc_eol);
                acc.clear();
                run_first = true;
            }
            // Classifier hook: about to JOIN this line into the run — the
            // model may overrule with keep(+mark). Deterministic keeps above
            // never reach here. Scope: plain paragraphs only (quote_depth 0),
            // matching the training distribution.
            if !run_first {
                if let (Some(c), Some(fs)) = (clf, fstats.as_ref()) {
                    if para.quote_depth == 0 && idx > 0 {
                        let p = c.p_phb(&bare_lines, para.start - 1, idx - 1, para.end, fs);
                        if p >= 0.5 {
                            // keep + write the marker nobody remembers
                            let marked = !acc.ends_with("  ");
                            out.push_str(&acc);
                            if marked {
                                out.push_str("  ");
                            }
                            out.push_str(acc_eol);
                            gate.push_str(&acc);
                            gate.push_str(acc_eol);
                            if p < 0.85 {
                                notes.push(Note {
                                    kept: true,
                                    p,
                                    line_a: acc.clone(),
                                    line_b: content.trim().to_string(),
                                });
                            }
                            acc.clear();
                            run_first = true;
                        } else if p >= 0.15 {
                            notes.push(Note {
                                kept: false,
                                p,
                                line_a: split_eol(lines[idx - 1]).0.trim().to_string(),
                                line_b: content.trim().to_string(),
                            });
                        }
                    }
                }
            }
            if run_first {
                acc.push_str(content);
                run_first = false;
            } else {
                acc.push(' ');
                acc.push_str(strip_continuation_prefix(content, para.quote_depth));
            }
            acc_eol = if eol.is_empty() { "" } else { eol };
            let is_para_last = idx + 1 == para.end;
            // eq-tags are fully solo; definition-looking lines may still
            // absorb their own wrapped continuations
            if solo && !is_para_last {
                emit!(&acc);
                emit!(acc_eol);
                acc.clear();
                run_first = true;
            } else if has_hard_break(content) && !is_para_last {
                // flush run, preserving the trailing break marker bytes
                emit!(&acc);
                emit!(acc_eol);
                acc.clear();
                run_first = true;
            } else if is_para_last {
                let t = acc.trim_end().to_string();
                emit!(&t);
                emit!(acc_eol);
            }
        }
        joined_until = para.end;
    }
    for l in joined_until..lines.len() {
        emit!(lines[l]);
    }
    Classified { output: out, gate_output: gate, notes }
}

fn split_eol(raw: &str) -> (&str, &str) {
    if let Some(s) = raw.strip_suffix("\r\n") {
        (s, "\r\n")
    } else if let Some(s) = raw.strip_suffix('\n') {
        (s, "\n")
    } else {
        (raw, "")
    }
}

/// Render-equality oracle (R12): comrak HTML with whitespace runs collapsed
/// outside `<pre>` blocks.
///
/// Collapsing is what makes the check usable: a soft break renders as a
/// newline and our join renders as a space, and outside preformatted blocks
/// those are the same rendered document. That includes inline code spans and
/// `$…$` math — CommonMark converts a line ending in a code span to a space,
/// and LaTeX ignores whitespace in math mode — so collapsing there is
/// correct, not lax. (Comparing math content as literal text, as a raw
/// pandoc-HTML diff does, reports differences that no reader could see.)
///
/// What this cannot check is whether a file *should* have been reformatted
/// at all. Joining pasted shell in a raw transcript is render-identical and
/// still destroys it. That is `exclude.rs`'s job, not this one's.
pub fn render_fingerprint(md: &str) -> String {
    let mut opts = options();
    opts.render.sourcepos = false; // positions necessarily shift on join
    let html = comrak::markdown_to_html(md, &opts);
    let mut out = String::with_capacity(html.len());
    let mut in_pre = false;
    let mut ws = false;
    for (i, c) in html.char_indices() {
        if html[i..].starts_with("<pre") {
            in_pre = true;
        } else if html[i..].starts_with("</pre>") {
            in_pre = false;
        }
        if !in_pre && (c == ' ' || c == '\n' || c == '\t') {
            if !ws {
                out.push(' ');
                ws = true;
            }
        } else {
            out.push(c);
            ws = false;
        }
    }
    out
}
