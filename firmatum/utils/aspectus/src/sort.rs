//! Sort — display order of children (design/sort.md).
//!
//! Groupings (dirs-first, optionally dotfiles-first), then a lattice key,
//! then the name (bytewise) so the order is total. Default key: **recency**
//! (mtime, newest first — steward, 2026-08-14): re-calling the look shows
//! what moved, at the top, with no cache needed. `sort = name` restores
//! alphabetical.
//!
//! Each key has a natural direction (name: A→Z; recency: newest first;
//! size: largest first); a leading `-` reverses it. Groupings and the name
//! tiebreak are never reversed.

use crate::n_level::Node;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Name,
    Size,
    Mtime,
    LineCount,
    Heat,
}

#[derive(Debug, Clone, Copy)]
pub struct Order {
    pub key: Key,
    /// `-KEY`: the key's natural direction reversed.
    pub rev: bool,
    pub dotfiles_first: bool,
    /// Recency reads git last-touch where heat's log pass covered a path,
    /// mtime elsewhere (config `recency-source = git`; design/sort.md).
    pub recency_git: bool,
}

impl Default for Order {
    fn default() -> Self {
        Order {
            key: Key::Mtime,
            rev: false,
            dotfiles_first: false,
            recency_git: false,
        }
    }
}

/// Keys built today, as the refusal menu names them.
pub const BUILT: &[&str] = &[
    "recency (mtime, the default)",
    "name",
    "size",
    "line-count",
    "heat",
];

/// Lattice `sort = Y` facts whose obtain is not built yet — asking for one
/// is a refusal naming the class, never a silent fallback sort.
pub const UNBUILT: &[&str] = &[
    "created",
    "filetype",
    "filekind",
    "child-count",
    "initial-sha",
    "latest-sha",
    "last-look",
    "mentions",
    "preeminence",
    "working-surface",
];

pub enum KeyErr {
    /// A real lattice sort fact, not yet implemented.
    Unbuilt(String),
    /// Not a sortable fact at all.
    Unknown(String),
}

/// `recency`, `name`, `-size`, … A leading `-` reverses the natural
/// direction. Returns (key, reversed).
pub fn parse_key(s: &str) -> Result<(Key, bool), KeyErr> {
    let (rev, bare) = match s.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, s),
    };
    match bare {
        "name" => Ok((Key::Name, rev)),
        "size" => Ok((Key::Size, rev)),
        "recency" | "mtime" | "modified" => Ok((Key::Mtime, rev)),
        "line-count" | "lines" => Ok((Key::LineCount, rev)),
        "heat" => Ok((Key::Heat, rev)),
        k if UNBUILT.contains(&k) => Err(KeyErr::Unbuilt(k.to_string())),
        k => Err(KeyErr::Unknown(k.to_string())),
    }
}

/// Reorder every level's children in place. The leftover-siblings census is
/// not a child node and stays the last line of its level regardless.
pub fn apply(node: &mut Node, order: &Order) {
    node.children.sort_by(|a, b| cmp(a, b, order));
    for c in &mut node.children {
        apply(c, order);
    }
}

fn dot(n: &Node) -> bool {
    n.name.starts_with('.')
}

/// The full display comparator: groupings, key, name tiebreak.
pub fn cmp(a: &Node, b: &Node, o: &Order) -> std::cmp::Ordering {
    use std::cmp::Ordering;
    // Grouping 1: dirs first.
    match (a.is_dir, b.is_dir) {
        (true, false) => return Ordering::Less,
        (false, true) => return Ordering::Greater,
        _ => {}
    }
    // Grouping 2: dot-names first, when asked.
    if o.dotfiles_first {
        match (dot(a), dot(b)) {
            (true, false) => return Ordering::Less,
            (false, true) => return Ordering::Greater,
            _ => {}
        }
    }
    key_cmp(a, b, o).then_with(|| a.name.cmp(&b.name))
}

/// The key alone (no groupings, no tiebreak) — what the allocator folds
/// into its keep-choice (key-within-weights).
pub fn key_cmp(a: &Node, b: &Node, o: &Order) -> std::cmp::Ordering {
    use std::cmp::Ordering;
    match o.key {
        Key::Name => Ordering::Equal, // name is the tiebreak; nothing extra
        Key::Size => magnitude(a.size, b.size, o.rev),
        Key::Mtime => magnitude(recency(a, o), recency(b, o), o.rev),
        Key::LineCount => magnitude(a.lines, b.lines, o.rev),
        Key::Heat => magnitude(
            a.heat.map(f64::to_bits_ordered),
            b.heat.map(f64::to_bits_ordered),
            o.rev,
        ),
    }
}

fn recency(n: &Node, o: &Order) -> Option<i64> {
    if o.recency_git {
        n.git_ts.or(n.mtime)
    } else {
        n.mtime
    }
}

/// Order-preserving bits for non-negative heat scores.
trait BitsOrdered {
    fn to_bits_ordered(self) -> u64;
}
impl BitsOrdered for f64 {
    fn to_bits_ordered(self) -> u64 {
        self.max(0.0).to_bits()
    }
}

/// Natural direction: largest/newest first; `-KEY` reverses it. A node with
/// no value for the key sorts after those with one *whatever* the direction —
/// absence is not a rank, so it stays outside the reversal.
fn magnitude<T: Ord>(a: Option<T>, b: Option<T>, rev: bool) -> std::cmp::Ordering {
    use std::cmp::Ordering;
    match (a, b) {
        (Some(x), Some(y)) => {
            if rev {
                x.cmp(&y)
            } else {
                y.cmp(&x)
            }
        }
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    }
}
