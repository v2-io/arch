//! The normalizer `vsect-norm/0` and the own version.
//!
//! This is a documented approximation of the design's `vsect-norm/1` (core.md §3.1), not that
//! normalizer: it does not parse CommonMark. Its name is recorded on every event that cites a
//! version, so a later normalizer never silently invalidates earlier checks. What it does:
//!
//! 1. NFC; line endings to `\n`; a trailing newline on every line.
//! 2. Fenced code blocks (``` or ~~~) and display math (`$$` … `$$` on their own lines) are kept
//!    byte for byte.
//! 3. Everywhere else, a paragraph is a run of non-blank lines; its soft line breaks become one
//!    space, runs of spaces and tabs become one space, leading and trailing whitespace goes.
//!    A line that opens a block (heading, list item, block quote, table row, horizontal rule)
//!    starts a new paragraph, so lists and tables keep one item or row per line.
//! 4. Blocks are joined with one blank line.
//! 5. md-press's own markers, HTML comments opening `<!--md-press:` and closing at the next `-->`,
//!    are removed first, wherever they occur. They record a formatting decision (a declined math
//!    promotion), not anything the statement says, so they belong with the design's "markers are
//!    not text" rule (core §1.9) and never change a version. (Added 2026-10-30, after every
//!    version in the asf store was computed and before md-press wrote any marker: on every text
//!    ever hashed the function is unchanged, which is why the rule ships under the same name.)
//!
//! Any change to math is a change. False staleness is preferred to undocumented equivalence.
//! `vsect-norm/1` is reserved for the design's CommonMark normalizer.
//!
//! Known defect in the object format: the normalizer's name is written into the canonical object
//! and so is part of every version hash. Renaming the normalizer therefore rehashes the whole
//! store, which is why the name cannot be bumped for a rule that changes no existing value. The
//! next object format should carry the name on events only.

use sha2::{Digest, Sha256};
use unicode_normalization::UnicodeNormalization;

pub const NORMALIZER: &str = "vsect-norm/0";

/// Remove md-press's markers (`<!--md-press:…-->`), which record formatting decisions, not text.
fn strip_tool_markers(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(i) = rest.find("<!--md-press:") {
        out.push_str(&rest[..i]);
        match rest[i..].find("-->") {
            Some(j) => rest = &rest[i + j + 3..],
            None => {
                rest = "";
            }
        }
    }
    out.push_str(rest);
    out
}

fn opens_block(line: &str) -> bool {
    let t = line.trim_start();
    t.starts_with('#')
        || t.starts_with("- ")
        || t.starts_with("* ")
        || t.starts_with("+ ")
        || t.starts_with('>')
        || t.starts_with('|')
        || t == "---"
        || t == "***"
        || t.chars().take_while(|c| c.is_ascii_digit()).count() > 0
            && t.trim_start_matches(|c: char| c.is_ascii_digit()).starts_with(". ")
}

fn collapse(line: &str) -> String {
    line.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn normalize(text: &str) -> String {
    let text = strip_tool_markers(text);
    let text: String = text.nfc().collect();
    let text = text.replace("\r\n", "\n").replace('\r', "\n");
    let mut blocks: Vec<String> = vec![];
    let mut para: Vec<String> = vec![];
    let mut fence: Option<String> = None; // the fence token that opened the block
    let mut verbatim: Vec<String> = vec![];
    let mut in_math = false;

    let flush_para = |para: &mut Vec<String>, blocks: &mut Vec<String>| {
        if !para.is_empty() {
            blocks.push(para.join("\n"));
            para.clear();
        }
    };

    for line in text.lines() {
        if let Some(tok) = &fence {
            verbatim.push(line.to_string());
            if line.trim_start().starts_with(tok.as_str()) {
                blocks.push(verbatim.join("\n"));
                verbatim.clear();
                fence = None;
            }
            continue;
        }
        if in_math {
            verbatim.push(line.to_string());
            if line.trim() == "$$" || line.trim().ends_with("$$") {
                blocks.push(verbatim.join("\n"));
                verbatim.clear();
                in_math = false;
            }
            continue;
        }
        let t = line.trim_start();
        if t.starts_with("```") || t.starts_with("~~~") {
            flush_para(&mut para, &mut blocks);
            let tok: String = t.chars().take_while(|&c| c == '`' || c == '~').collect();
            fence = Some(tok);
            verbatim.push(line.to_string());
            continue;
        }
        if t.trim_end() == "$$" || (t.starts_with("$$") && !t[2..].contains("$$")) {
            flush_para(&mut para, &mut blocks);
            in_math = true;
            verbatim.push(line.to_string());
            continue;
        }
        if line.trim().is_empty() {
            flush_para(&mut para, &mut blocks);
            continue;
        }
        if opens_block(line) {
            flush_para(&mut para, &mut blocks);
            para.push(collapse(line));
            flush_para(&mut para, &mut blocks);
            continue;
        }
        // continuation of a paragraph
        match para.last_mut() {
            Some(last) if !last.is_empty() => {
                last.push(' ');
                last.push_str(&collapse(line));
            }
            _ => para.push(collapse(line)),
        }
    }
    if fence.is_some() || in_math {
        blocks.push(verbatim.join("\n"));
    }
    flush_para(&mut para, &mut blocks);
    let mut out = blocks.join("\n\n");
    out.push('\n');
    out
}

pub fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    hex::encode(h.finalize())
}

/// The canonical serialization an own version hashes: kind, declared form, normalized text.
pub fn canonical_object(kind: &str, form: &str, text: &str) -> String {
    format!("kind: {kind}\nform: {form}\nnormalizer: {NORMALIZER}\n---\n{}", normalize(text))
}

pub fn own_version(kind: &str, form: &str, text: &str) -> String {
    sha256_hex(canonical_object(kind, form, text).as_bytes())
}

pub fn short(v: &str) -> String {
    v.chars().take(8).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reflow_invariant() {
        let a = "Alpha beta\ngamma delta.\n\nNext para.";
        let b = "Alpha beta gamma\ndelta.\n\n\nNext   para.";
        assert_eq!(normalize(a), normalize(b));
    }

    #[test]
    fn math_change_stales() {
        assert_ne!(normalize("x $a \\lt b$ y"), normalize("x $a < b$ y"));
    }

    #[test]
    fn fenced_kept() {
        let s = "p\n\n```\na   b\n  c\n```\n";
        assert!(normalize(s).contains("a   b\n  c"));
    }

    #[test]
    fn md_press_markers_are_not_text() {
        let a = "The gain is η here.<!--md-press:not-math:η--> Next.\n";
        let b = "The gain is η here. Next.\n";
        assert_eq!(normalize(a), normalize(b));
        let c = "| cell η<!--md-press:not-math:η--><!--md-press:not-math:δ-->   |\n";
        let d = "| cell η |\n";
        assert_eq!(normalize(c), normalize(d));
        // a marker inside display math would still be stripped; md-press never writes one there
        assert_eq!(normalize("x\n"), "x\n");
    }

    #[test]
    fn display_math_kept() {
        let s = "p\n\n$$\nH(S_t \\mid C)   = 0\n$$\n";
        assert!(normalize(s).contains("H(S_t \\mid C)   = 0"));
    }
}
