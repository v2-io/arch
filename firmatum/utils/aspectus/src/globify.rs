//! Globify (design/globify.md): a real sequence of names collapses to its
//! pattern — `output-[001-047].bak  (44 files)` — one listee, one line.
//!
//! The sin is false-positive collapse; every guard leans toward *not*
//! collapsing: minimum count (config `globify.min`, default 5), exactly one
//! varying numeric run (same prefix, same suffix, same position), width
//! honesty (all members the same digit width — mixed widths stay listed),
//! same kind (all files or all dirs, symlinks never join), important files
//! exempt. Members are *listed* (compactly), not leftover: they never join
//! the leaf census, and a collapsed dir is never expanded (an exemplar
//! would be a guess dressed as a look). `--show-all` and `globify = off`
//! restore the names.
//!
//! Runs on the pre-budget tree, after facts are annotated, so the one
//! listee costs one line and carries honest aggregates: size as the sum,
//! mtime/heat as the newest/max.

use crate::n_level::Node;

/// A collapsed group's identity — JSON gets these as fields so members are
/// recoverable in principle, never a lossy string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Glob {
    pub lo: u64,
    pub hi: u64,
    pub width: usize,
    pub count: usize,
    /// Suffix bucket of a member name (for censuses, should the listee
    /// itself fall to a fold).
    pub bucket: String,
}

/// Candidate key: name split around one maximal digit run.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Key {
    prefix: String,
    suffix: String,
    width: usize,
    dir: bool,
}

fn runs(name: &str) -> Vec<(usize, usize)> {
    let b = name.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < b.len() {
        if b[i].is_ascii_digit() {
            let start = i;
            while i < b.len() && b[i].is_ascii_digit() {
                i += 1;
            }
            out.push((start, i));
        } else {
            i += 1;
        }
    }
    out
}

/// May this node join a series at all? Plain nodes only — anything
/// carrying its own claims or marks stays listed by name.
fn plain(n: &Node) -> bool {
    !n.important
        && n.link.is_none()
        && !n.denied
        && !n.cycle
        && !n.other_fs
        && !n.ignored
        && !n.cut
        && n.kinds.is_empty()
        && n.facets.is_empty()
        && n.glob.is_none()
}

/// Collapse real sequences at every level of the (pre-budget) tree.
pub fn apply(node: &mut Node, min: usize) {
    level(node, min);
    for c in &mut node.children {
        apply(c, min);
    }
}

fn level(node: &mut Node, min: usize) {
    use std::collections::HashMap;
    if node.children.len() < min {
        return;
    }
    // Candidate groups: (key -> member indices with their numeric value).
    let mut groups: HashMap<Key, Vec<(usize, u64)>> = HashMap::new();
    for (i, c) in node.children.iter().enumerate() {
        if !plain(c) {
            continue;
        }
        for (s, e) in runs(&c.name) {
            let Ok(v) = c.name[s..e].parse::<u64>() else {
                continue; // wider than u64: too strange to fuse
            };
            groups
                .entry(Key {
                    prefix: c.name[..s].to_string(),
                    suffix: c.name[e..].to_string(),
                    width: e - s,
                    dir: c.is_dir,
                })
                .or_default()
                .push((i, v));
        }
    }
    // Deterministic order: biggest first, then key order; a member joins
    // at most one group (first claim wins), and a group re-checks the
    // threshold after losses.
    let mut order: Vec<(Key, Vec<(usize, u64)>)> = groups.into_iter().collect();
    order.sort_by(|a, b| b.1.len().cmp(&a.1.len()).then_with(|| a.0.cmp(&b.0)));
    let mut taken = vec![false; node.children.len()];
    let mut collapsed: Vec<(Key, Vec<(usize, u64)>)> = Vec::new();
    for (key, members) in order {
        let members: Vec<(usize, u64)> = members.into_iter().filter(|(i, _)| !taken[*i]).collect();
        if members.len() < min {
            continue;
        }
        for (i, _) in &members {
            taken[*i] = true;
        }
        collapsed.push((key, members));
    }
    if collapsed.is_empty() {
        return;
    }
    let mut keep: Vec<Node> = Vec::new();
    let mut consumed = vec![false; node.children.len()];
    let mut synthetic: Vec<Node> = Vec::new();
    for (key, members) in collapsed {
        let lo = members.iter().map(|(_, v)| *v).min().unwrap_or(0);
        let hi = members.iter().map(|(_, v)| *v).max().unwrap_or(0);
        let w = key.width;
        let name = format!("{}[{:0w$}-{:0w$}]{}", key.prefix, lo, hi, key.suffix);
        let mut mtime: Option<i64> = None;
        let mut size: Option<u64> = None;
        let mut lines: Option<u64> = Some(0);
        let mut heat: Option<f64> = None;
        let mut git_ts: Option<i64> = None;
        let mut bucket = String::new();
        for (i, _) in &members {
            let m = &node.children[*i];
            consumed[*i] = true;
            if bucket.is_empty() {
                bucket = if m.census_key.is_empty() {
                    crate::n_level::suffix_bucket(&m.name)
                } else {
                    m.census_key.clone()
                };
            }
            if let Some(t) = m.mtime {
                mtime = Some(mtime.map_or(t, |x: i64| x.max(t)));
            }
            if let Some(s) = m.size {
                size = Some(size.unwrap_or(0) + s);
            }
            match (m.is_dir, m.lines) {
                (false, Some(l)) => lines = lines.map(|x| x + l),
                (false, None) => lines = None, // any unknown: no sum claimed
                (true, _) => lines = None,
            }
            if let Some(h) = m.heat {
                heat = Some(heat.map_or(h, |x: f64| x.max(h)));
            }
            if let Some(t) = m.git_ts {
                git_ts = Some(git_ts.map_or(t, |x: i64| x.max(t)));
            }
        }
        synthetic.push(Node {
            name,
            is_dir: key.dir,
            mtime,
            size: (!key.dir).then_some(size.unwrap_or(0)),
            lines: if key.dir { None } else { lines },
            heat,
            git_ts,
            glob: Some(Glob {
                lo,
                hi,
                width: w,
                count: members.len(),
                bucket,
            }),
            ..Node::default()
        });
    }
    for (i, c) in node.children.drain(..).enumerate() {
        if !consumed[i] {
            keep.push(c);
        }
    }
    keep.append(&mut synthetic);
    node.children = keep;
}
