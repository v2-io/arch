//! Focus set — several positional paths (design/focus.md §Multiple paths).
//!
//! Several positional paths are **one look**, not several: the locus is
//! their common ancestor (the perspective root the header prints and JSON
//! calls `root`), the named paths take the top survival tier, and
//! everything between and beside them is connective context.
//!
//! Three decided laws live here:
//!
//! 1. **Depth counts from each selected path**, not from the ancestor —
//!    `--depth 4` on four volumes means four generations under *each*.
//!    The connective chain down to them spends no depth (the caller named
//!    the places they want deep; counting from the root would make the ask
//!    unexpressible).
//! 2. **Unselected siblings fold**, at each connective level, into one
//!    typed leaf-census remainder (`[+ dir×9 ≈1.4Kf · md×27]`) — present,
//!    typed, one line. Never dropped: the fold is a compression, and the
//!    census is what keeps it from being a lie. (Joseph 2026-08-22: fold
//!    by default; a future `-q` would drop the remainders — not built.)
//! 3. **The ancestor is the root even when it is far up**, `/` included.
//!
//! One path is today's behavior exactly; the arity is the whole rule.

use std::path::{Path, PathBuf};

/// The selected set for one look.
#[derive(Debug, Clone)]
pub struct Focus {
    /// Absolute, deduped, non-nested — the paths the caller named.
    pub sel: Vec<PathBuf>,
    /// Generations below *each* selected path (`0` = no limit).
    pub depth: u32,
}

impl Focus {
    /// Is this exactly one of the named paths? (The walk restarts the
    /// depth count here.)
    pub fn is_selected(&self, p: &Path) -> bool {
        self.sel.iter().any(|s| s == p)
    }

    /// Is this a strict ancestor of a named path? (Connective: it renders
    /// as the chain down, spends no depth, and folds its other children.)
    pub fn is_connective(&self, p: &Path) -> bool {
        self.sel.iter().any(|s| s != p && s.starts_with(p))
    }
}

/// The deepest directory every path lies under. `/` when they share
/// nothing else — an honest root, not a refusal.
pub fn common_ancestor(paths: &[PathBuf]) -> PathBuf {
    let mut iter = paths.iter();
    let Some(first) = iter.next() else {
        return PathBuf::from("/");
    };
    let mut common: Vec<std::ffi::OsString> = first
        .parent()
        .unwrap_or(Path::new("/"))
        .components()
        .map(|c| c.as_os_str().to_os_string())
        .collect();
    for p in iter {
        let theirs: Vec<std::ffi::OsString> = p
            .parent()
            .unwrap_or(Path::new("/"))
            .components()
            .map(|c| c.as_os_str().to_os_string())
            .collect();
        let keep = common
            .iter()
            .zip(&theirs)
            .take_while(|(a, b)| a == b)
            .count();
        common.truncate(keep);
    }
    let mut out = PathBuf::new();
    for c in common {
        out.push(c);
    }
    if out.as_os_str().is_empty() {
        out.push("/");
    }
    out
}

/// Drop paths that lie inside another selected path: depth counts from the
/// outer one, so the inner ask is already served and a second selection
/// would only double-count it. Returns the dropped ones so the caller can
/// confess them (the look never silently edits the ask).
pub fn drop_nested(sel: &mut Vec<PathBuf>) -> Vec<PathBuf> {
    let mut dropped = Vec::new();
    let mut kept: Vec<PathBuf> = Vec::new();
    // Shortest first, so an outer path is always decided before its inner.
    let mut ordered = sel.clone();
    ordered.sort_by_key(|p| p.components().count());
    for p in ordered {
        if kept.iter().any(|k| p.starts_with(k)) {
            dropped.push(p);
        } else {
            kept.push(p);
        }
    }
    kept.sort();
    *sel = kept;
    dropped.sort();
    dropped
}

/// Fold the unselected siblings of every connective level into that
/// level's remainder census. Runs after the deep phase, so a folded
/// directory contributes its real mass (`dir×9 ≈1.4Kf`) rather than a
/// bare name-count — the whole point of folding rather than cutting.
pub fn fold_asides(node: &mut crate::n_level::Node) {
    for c in &mut node.children {
        fold_asides(c);
    }
    if !node.children.iter().any(|c| c.aside) {
        return;
    }
    let (asides, kept): (Vec<_>, Vec<_>) = std::mem::take(&mut node.children)
        .into_iter()
        .partition(|c| c.aside);
    node.children = kept;
    let mut om = crate::n_level::census_nodes(&asides);
    om.bounded |= asides.iter().any(|c| c.denied);
    if let Some(prev) = node.omitted.take() {
        om = crate::n_level::merge_census(om, prev);
    }
    node.omitted = Some(om);
}

/// Which selected paths did not survive to a line of their own. Leftover
/// matches must be typed, not silently cut (design/focus.md subfeature 6):
/// until the census grows a matched channel of its own — its spelling is
/// the row's open interface question, Joseph's to ratify — the honest
/// carrier is a stderr confession naming them.
pub fn unlisted(tree: &crate::n_level::Node, root: &Path, sel: &[PathBuf]) -> Vec<PathBuf> {
    fn walk(n: &crate::n_level::Node, at: &Path, out: &mut Vec<PathBuf>) {
        if n.matched {
            out.push(at.to_path_buf());
        }
        for c in &n.children {
            walk(c, &at.join(&c.name), out);
        }
    }
    let mut present = Vec::new();
    walk(tree, root, &mut present);
    sel.iter()
        .filter(|p| !present.contains(p))
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(s: &str) -> PathBuf {
        PathBuf::from(s)
    }

    #[test]
    fn ancestor_of_siblings_is_their_parent() {
        let sel = vec![p("/a/b/one"), p("/a/b/two"), p("/a/b/three")];
        assert_eq!(common_ancestor(&sel), p("/a/b"));
    }

    #[test]
    fn ancestor_climbs_when_they_diverge() {
        assert_eq!(common_ancestor(&[p("/a/b/c"), p("/a/x/y")]), p("/a"));
    }

    #[test]
    fn ancestor_is_root_when_they_share_nothing() {
        assert_eq!(common_ancestor(&[p("/a/b"), p("/x/y")]), p("/"));
    }

    #[test]
    fn nested_selections_collapse_to_the_outer() {
        let mut sel = vec![p("/a/b/c"), p("/a/b"), p("/a/z")];
        let dropped = drop_nested(&mut sel);
        assert_eq!(sel, vec![p("/a/b"), p("/a/z")]);
        assert_eq!(dropped, vec![p("/a/b/c")]);
    }

    #[test]
    fn roles_are_by_path_not_by_name() {
        let f = Focus {
            sel: vec![p("/a/b/one"), p("/a/b/two")],
            depth: 4,
        };
        assert!(f.is_connective(Path::new("/a/b")));
        assert!(f.is_connective(Path::new("/a")));
        assert!(f.is_selected(Path::new("/a/b/one")));
        assert!(!f.is_connective(Path::new("/a/b/one")));
        assert!(!f.is_selected(Path::new("/a/b/three")));
    }
}
