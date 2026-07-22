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
//! `fmt-md $(find . -name '*.md')` — the tool has to be the thing that
//! remembers.

use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub const IGNORE_FILE: &str = ".fmt-mdignore";

/// Caches one compiled matcher per directory that holds a `.fmt-mdignore`.
#[derive(Default)]
pub struct Excluder {
    matchers: HashMap<PathBuf, Option<Gitignore>>,
}

impl Excluder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Is `path` excluded by any `.fmt-mdignore` at or above it?
    /// Returns the ignore file that made the call, for reporting.
    pub fn excluded(&mut self, path: &Path) -> Option<PathBuf> {
        let abs = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
        let mut dir = abs.parent()?.to_path_buf();
        loop {
            if let Some(gi) = self.matcher_for(&dir) {
                // is_dir=false: we only ever test files
                if gi.matched_path_or_any_parents(&abs, false).is_ignore() {
                    return Some(dir.join(IGNORE_FILE));
                }
            }
            if !dir.pop() {
                return None;
            }
        }
    }

    fn matcher_for(&mut self, dir: &Path) -> Option<&Gitignore> {
        let entry = self.matchers.entry(dir.to_path_buf()).or_insert_with(|| {
            let f = dir.join(IGNORE_FILE);
            if !f.is_file() {
                return None;
            }
            let mut b = GitignoreBuilder::new(dir);
            // add() returns Some(err) on failure; a malformed ignore file
            // should be loud rather than silently permissive
            if let Some(e) = b.add(&f) {
                eprintln!("fmt-md: {}: {e}", f.display());
                return None;
            }
            match b.build() {
                Ok(gi) => Some(gi),
                Err(e) => {
                    eprintln!("fmt-md: {}: {e}", f.display());
                    None
                }
            }
        });
        entry.as_ref()
    }
}
