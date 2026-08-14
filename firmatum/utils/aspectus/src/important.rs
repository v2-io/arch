//! Important files — pure survival weight (design/important-files.md).
//!
//! One ordered config list of basename globs. Under a tight budget they
//! fall into the middle survival tier (`dirs > important > plain`,
//! key-within-tier — `n_level::weight`). No column, no glyph, no order
//! change: with budget to spare an important file sits wherever the sort
//! puts it, and a cold reader cannot tell it survived by weight rather
//! than by luck. Per level, among siblings — no ancestor rescue: a README
//! inside an unexpanded dir is simply part of that dir's census/mass.
//!
//! The list's *order* is load-bearing beyond survival: README title
//! borrows this exact set and breaks ties by list order — defined here,
//! once, so the two rows cannot drift.

use crate::furniture::glob_match;
use crate::n_level::Node;

/// Shipped default (design leaning, Joseph ratifies): the file that says
/// what the place *is*, then the agent-instruction names. Everything else
/// is config. (`Cargo.toml` etc. are furniture mark rows — a different
/// office.)
pub const DEFAULT: &[&str] = &["README*", "AGENTS.md", "CLAUDE.md"];

#[derive(Debug, Clone)]
pub struct Set {
    /// Ordered; first match is also the readme-title tiebreak someday.
    patterns: Vec<String>,
}

impl Set {
    pub fn shipped() -> Self {
        Set {
            patterns: DEFAULT.iter().map(|s| s.to_string()).collect(),
        }
    }

    /// Config key `important`: comma-separated basename globs, same grammar
    /// family as the furniture map. Config entries go in front of the
    /// shipped defaults (they outrank in list order); `!PATTERN` drops a
    /// row (shipped or earlier-config) entirely.
    pub fn with_config(config: &str) -> Self {
        let mut set = Set::shipped();
        let mut front = Vec::new();
        for item in config.split(',') {
            let item = item.trim();
            if item.is_empty() {
                continue;
            }
            if let Some(pat) = item.strip_prefix('!') {
                set.patterns.retain(|p| p != pat);
                front.retain(|p: &String| p != pat);
                continue;
            }
            front.push(item.to_string());
        }
        front.append(&mut set.patterns);
        Set { patterns: front }
    }

    pub fn matches(&self, name: &str) -> bool {
        self.patterns.iter().any(|p| glob_match(p, name))
    }

    /// The file that lends a dir its README title (design/readme-title.md
    /// borrows this exact set): first pattern in config-list order wins;
    /// names within a pattern tie in the caller's (sorted) order.
    pub fn first_match<'n, I>(&self, names: I) -> Option<&'n str>
    where
        I: Iterator<Item = &'n str> + Clone,
    {
        for p in &self.patterns {
            for n in names.clone() {
                if glob_match(p, n) {
                    return Some(n);
                }
            }
        }
        None
    }

    /// Mark every matching file in the (pre-budget) tree. Dirs already
    /// outrank the important tier, so only files carry the mark.
    pub fn annotate(&self, node: &mut Node) {
        for c in &mut node.children {
            if !c.is_dir && self.matches(&c.name) {
                c.important = true;
            }
            self.annotate(c);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_match_readme_family() {
        let s = Set::shipped();
        assert!(s.matches("README.md"));
        assert!(s.matches("README"));
        assert!(s.matches("AGENTS.md"));
        assert!(!s.matches("NOTES.md"));
    }

    #[test]
    fn config_extends_in_front_and_drops() {
        let s = Set::with_config("DESIGN.md, !README*");
        assert!(s.matches("DESIGN.md"));
        assert!(!s.matches("README.md"));
        assert!(s.matches("CLAUDE.md"));
    }
}
