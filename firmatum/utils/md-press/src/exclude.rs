//! Frozen-region exclusions (requirements R10/R33).
//!
//! Some markdown under these trees is *verbatim provenance*, not prose:
//! raw session transcripts, provenanced copies of source material, frozen
//! archaeology. Reformatting it is render-equivalent and still wrong — the
//! line structure is part of what is being preserved. Render-equality
//! cannot detect that, because nothing about the rendered document changed;
//! only a human (or an agent reading the directory's purpose) can say
//! "these bytes are evidence."
//!
//! So exclusion is declared, not inferred: a `.fmt-mdignore` file, with
//! gitignore syntax, anywhere at or above the file being formatted. Files
//! it matches are left alone even when named explicitly on the command
//! line, because the realistic accident is an agent running
//! `md-press $(find . -name '*.md')` — the tool has to be the thing that
//! remembers.
//!
//! This module carries a second guard of the same family, keyed on the file
//! extension rather than an ignore file. That is still a declaration and not
//! an inference: whoever named a file `.udon` declared its language, and the
//! extension is the most durable form that declaration takes — it travels
//! with the file into trees that have no `.fmt-mdignore` and cannot be
//! forgotten when a new directory is added. See `foreign_language`.

use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Both names are honored: `.md-pressignore` is the current one; trees
/// marked up before the 2026-08-06 rename carry `.fmt-mdignore`, and an
/// exclusion silently expiring because a tool changed its name would be the
/// exact accident this file exists to prevent.
pub const IGNORE_FILES: &[&str] = &[".md-pressignore", ".fmt-mdignore"];

/// Extensions whose language md-press has no model of, and whose line
/// structure carries meaning the render-equality gate cannot see.
///
/// UDON is the founding member, and the reasoning is worth keeping next to
/// the list because the instinct to accept `.udon` as "markdown enough" has
/// already come up once (`UDON-ASSESSMENT-2026-07-29.md`, which reproduced
/// each failure rather than arguing it). Three facts, each from UDON's own
/// spec: the newline is *literal text content* under the text law, not
/// collapsible whitespace — so joining two prose lines edits the
/// reconstructed value rather than preserving it; a bare attribute value runs
/// to end of line, so joining `:author X` onto the line above silently
/// swallows every following `:key` into one attribute holding garbage; and
/// `!:lang:` verbatim blocks are invisible to comrak, which reads them as
/// ordinary paragraphs and flattens working source into one line.
///
/// None of that is visible to the render-equality gate, and that is the
/// point: the gate compares *CommonMark renders*, and a corrupted UDON
/// attribute line renders as unremarkable Markdown text. The tool's central
/// safety claim has no UDON referent to be true or false about, so the guard
/// has to sit in front of the gate rather than relying on it.
pub const FOREIGN_EXTENSIONS: &[&str] = &["udon"];

/// Does `path`'s own name declare a language from `FOREIGN_EXTENSIONS`?
/// Returns the matched extension, for reporting.
///
/// Filename-keyed, so it cannot help in stdin mode (`md-press - < f.udon`) —
/// there is no name to read. That is a real hole and is documented in
/// `--help` rather than papered over; stdin mode writes to stdout, so the
/// accident it guards against (in-place corruption of a tree) is not
/// reachable that way.
pub fn foreign_language(path: &Path) -> Option<&'static str> {
    let ext = path.extension()?.to_str()?.to_ascii_lowercase();
    FOREIGN_EXTENSIONS.iter().copied().find(|e| *e == ext)
}

/// Caches one compiled matcher per directory that holds an ignore file,
/// tagged with the ignore filename actually found there (for reporting).
#[derive(Default)]
pub struct Excluder {
    matchers: HashMap<PathBuf, Option<(Gitignore, &'static str)>>,
}

impl Excluder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Is `path` excluded by any `.md-pressignore` / `.fmt-mdignore` at or
    /// above it? Returns the ignore file that made the call, for reporting.
    pub fn excluded(&mut self, path: &Path) -> Option<PathBuf> {
        let abs = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
        let mut dir = abs.parent()?.to_path_buf();
        loop {
            if let Some((gi, name)) = self.matcher_for(&dir) {
                // is_dir=false: we only ever test files
                if gi.matched_path_or_any_parents(&abs, false).is_ignore() {
                    return Some(dir.join(name));
                }
            }
            if !dir.pop() {
                return None;
            }
        }
    }

    fn matcher_for(&mut self, dir: &Path) -> Option<&(Gitignore, &'static str)> {
        let entry = self.matchers.entry(dir.to_path_buf()).or_insert_with(|| {
            let mut b = GitignoreBuilder::new(dir);
            let mut found: Option<&'static str> = None;
            for name in IGNORE_FILES {
                let f = dir.join(name);
                if !f.is_file() {
                    continue;
                }
                found.get_or_insert(name);
                // add() returns Some(err) on failure; a malformed ignore
                // file should be loud rather than silently permissive
                if let Some(e) = b.add(&f) {
                    eprintln!("md-press: {}: {e}", f.display());
                }
            }
            let name = found?;
            match b.build() {
                Ok(gi) => Some((gi, name)),
                Err(e) => {
                    eprintln!("md-press: {}/{}: {e}", dir.display(), name);
                    None
                }
            }
        });
        entry.as_ref()
    }
}
