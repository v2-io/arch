//! N-level tree. Depth is generations *below* the root.
//! `1` = children only. `2` = children and grandchildren. `0` = no limit.

use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug, Clone, Default)]
pub struct Census {
    pub total: usize,
    /// (label, count), already sorted for display.
    pub buckets: Vec<(String, usize)>,
    /// The count is a floor, not a fact: the walk stopped (bound or denied
    /// child) before every name was seen. Renders as `≥`.
    pub bounded: bool,
}

impl Census {
    pub fn render(&self) -> String {
        self.render_inner(false)
    }

    pub fn render_plus(&self) -> String {
        self.render_inner(true)
    }

    fn render_inner(&self, plus: bool) -> String {
        if self.total == 0 {
            return String::new();
        }
        let parts: Vec<String> = self
            .buckets
            .iter()
            .map(|(k, n)| format!("{n} {k}"))
            .collect();
        let geq = if self.bounded { "≥" } else { "" };
        let n = if plus {
            format!("+{geq}{}", self.total)
        } else {
            format!("{geq}{}", self.total)
        };
        format!("[{n}: {}]", parts.join(", "))
    }
}

#[derive(Debug, Clone, Default)]
pub struct Node {
    pub name: String,
    pub is_dir: bool,
    pub children: Vec<Node>,
    /// Set when we stopped expanding this directory (depth cutoff).
    pub leftover: Option<Census>,
    /// Siblings not listed because the line budget ran out.
    pub omitted: Option<Census>,
    /// The walk could not read this dir (or stat this name): `[denied]`.
    pub denied: bool,
    /// The walk bound stopped enumeration here before every name was seen.
    pub cut: bool,
    /// An entry mid-iteration errored; this dir's listing may be missing names.
    pub iter_err: bool,
}

/// Counts names statted. `None` = unbounded.
///
/// The bound is on stat+recurse — "how many names we will even stat"
/// (design/walk-bound.md) — never on name enumeration: a readdir of names
/// is nearly free, and level membership must be total, not readdir-order
/// roulette. Censuses are built from readdir type-hints and cost nothing,
/// so the honesty device cannot drain the budget it reports on.
#[derive(Debug, Default)]
pub struct WalkBudget {
    remaining: Option<u64>,
    pub tripped: bool,
    pub statted: u64,
}

impl WalkBudget {
    /// `bound` of 0 means no bound.
    pub fn new(bound: u64) -> Self {
        WalkBudget {
            remaining: if bound == 0 { None } else { Some(bound) },
            tripped: false,
            statted: 0,
        }
    }

    /// Charge one stat.
    fn charge(&mut self) {
        self.statted += 1;
        if let Some(n) = &mut self.remaining {
            *n = n.saturating_sub(1);
        }
    }

    /// Nothing left to spend on stat/recurse; a cut it causes sets `tripped`.
    fn exhausted(&self) -> bool {
        self.remaining == Some(0)
    }
}

/// `depth` is how many generations below the root to print. `0` = no limit.
pub fn gather(path: &Path, depth: u32, walk: &mut WalkBudget) -> io::Result<Node> {
    let remain = if depth == 0 { None } else { Some(depth) };
    gather_at(path, remain, walk)
}

/// One name as readdir reported it, before any stat.
struct Entry {
    name: String,
    /// From the readdir file-type hint — no stat spent.
    is_dir: bool,
}

/// All of a directory's names, free of the budget and never cut short:
/// level membership is total, or the whole dir is denied (`None`).
fn enumerate(path: &Path) -> Option<(Vec<Entry>, bool)> {
    let rd = fs::read_dir(path).ok()?;
    let mut entries = Vec::new();
    let mut iter_err = false;
    for ent in rd {
        let Ok(ent) = ent else {
            iter_err = true;
            continue;
        };
        let os = ent.file_name();
        if os == "." || os == ".." {
            continue;
        }
        entries.push(Entry {
            name: os.to_string_lossy().into_owned(),
            is_dir: ent.file_type().map(|t| t.is_dir()).unwrap_or(false),
        });
    }
    entries.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.cmp(&b.name),
    });
    Some((entries, iter_err))
}

fn census_entries(entries: &[Entry], iter_err: bool) -> Census {
    let mut buckets: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
    for e in entries {
        let label = if e.is_dir {
            "dir".to_string()
        } else {
            suffix_label(&e.name)
        };
        *buckets.entry(label).or_insert(0) += 1;
    }
    let mut buckets: Vec<(String, usize)> = buckets.into_iter().collect();
    buckets.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    Census {
        total: entries.len(),
        buckets,
        bounded: iter_err,
    }
}

fn gather_at(path: &Path, remain: Option<u32>, walk: &mut WalkBudget) -> io::Result<Node> {
    let meta = fs::symlink_metadata(path)?;
    let is_dir = meta.is_dir();
    let name = path
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string());
    if !is_dir {
        return Ok(Node {
            name,
            is_dir: false,
            ..Node::default()
        });
    }
    let Some((entries, iter_err)) = enumerate(path) else {
        return Ok(Node {
            name,
            is_dir: true,
            denied: true,
            ..Node::default()
        });
    };
    if remain == Some(0) {
        let census = census_entries(&entries, iter_err);
        return Ok(Node {
            name,
            is_dir: true,
            leftover: if census.total == 0 { None } else { Some(census) },
            iter_err,
            ..Node::default()
        });
    }
    // Spend the remaining budget on stat+recurse, in sorted order — which
    // subtrees get expanded is deterministic, not readdir roulette.
    let child_remain = remain.map(|n| n.saturating_sub(1));
    let mut children = Vec::new();
    let mut cut = false;
    for (i, ent) in entries.iter().enumerate() {
        if walk.exhausted() {
            walk.tripped = true;
            cut = true;
            let rest = census_entries(&entries[i..], false);
            if children.is_empty() {
                // Nothing was listed at this level: the whole membership
                // sits on this dir's own line, like a depth cutoff.
                return Ok(Node {
                    name,
                    is_dir: true,
                    leftover: Some(rest),
                    cut,
                    iter_err,
                    ..Node::default()
                });
            }
            return Ok(Node {
                name,
                is_dir: true,
                children,
                omitted: Some(rest),
                cut,
                iter_err,
                ..Node::default()
            });
        }
        walk.charge();
        let child_path = path.join(&ent.name);
        let kid = match gather_at(&child_path, child_remain, walk) {
            Ok(kid) => kid,
            Err(_) => Node {
                name: ent.name.clone(),
                is_dir: ent.is_dir,
                denied: true,
                ..Node::default()
            },
        };
        children.push(kid);
    }
    Ok(Node {
        name,
        is_dir: true,
        children,
        cut,
        iter_err,
        ..Node::default()
    })
}

fn label_node(n: &Node) -> String {
    if n.is_dir {
        "dir".into()
    } else {
        suffix_label(&n.name)
    }
}

/// Two censuses of disjoint name-sets become one. `bounded` is sticky.
fn merge_census(a: Census, b: Census) -> Census {
    let mut buckets: std::collections::BTreeMap<String, usize> = a.buckets.into_iter().collect();
    for (k, n) in b.buckets {
        *buckets.entry(k).or_insert(0) += n;
    }
    let mut buckets: Vec<(String, usize)> = buckets.into_iter().collect();
    buckets.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    Census {
        total: a.total + b.total,
        buckets,
        bounded: a.bounded || b.bounded,
    }
}

fn fold_children_into_dir_census(node: &mut Node) {
    if node.children.is_empty() && node.omitted.is_none() {
        return;
    }
    let mut census = census_nodes(&node.children);
    // A denied dir folded out of sight: the count of names is exact, but the
    // aggregate hides an unreadable place — the parent's census is marked
    // incomplete (design/denied.md).
    census.bounded = node.children.iter().any(|c| c.denied);
    if let Some(om) = node.omitted.take() {
        census = merge_census(census, om);
    }
    node.leftover = Some(census);
    node.children.clear();
}

fn census_nodes(nodes: &[Node]) -> Census {
    let mut buckets: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
    for n in nodes {
        *buckets.entry(label_node(n)).or_insert(0) += 1;
    }
    let mut buckets: Vec<(String, usize)> = buckets.into_iter().collect();
    buckets.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    Census {
        total: nodes.len(),
        buckets,
        bounded: false,
    }
}

fn weight(n: &Node) -> u32 {
    if n.is_dir {
        4
    } else {
        1
    }
}

/// `budget` includes this node's own line. `0` means no line limit.
pub fn apply_budget(node: &mut Node, budget: usize, explain: &mut Vec<String>) {
    if budget == 0 {
        return;
    }
    let n = node.children.len();
    if n == 0 {
        return;
    }
    let remain = budget.saturating_sub(1);
    if remain == 0 {
        explain.push(format!(
            "{}: budget 1 — census on this line, not a child [+]",
            node.name
        ));
        fold_children_into_dir_census(node);
        return;
    }
    if remain < n {
        let show = remain.saturating_sub(1);
        if show == 0 {
            explain.push(format!(
                "{}: one leftover line would only repeat dir census",
                node.name
            ));
            fold_children_into_dir_census(node);
            return;
        }
        let mut order: Vec<usize> = (0..n).collect();
        order.sort_by(|&a, &b| {
            weight(&node.children[b])
                .cmp(&weight(&node.children[a]))
                .then_with(|| node.children[a].name.cmp(&node.children[b].name))
        });
        let keep: Vec<usize> = order.iter().copied().take(show).collect();
        let drop: Vec<Node> = node
            .children
            .iter()
            .enumerate()
            .filter(|(i, _)| !keep.contains(i))
            .map(|(_, c)| c.clone())
            .collect();
        node.children = keep.iter().map(|&i| node.children[i].clone()).collect();
        let mut om = census_nodes(&drop);
        if let Some(prev) = node.omitted.take() {
            om = merge_census(om, prev);
        }
        node.omitted = Some(om);
        explain.push(format!(
            "{}: listed {} / {n}, omitted {}",
            node.name,
            node.children.len(),
            drop.len()
        ));
        for c in &mut node.children {
            apply_budget(c, 1, explain);
        }
        return;
    }
    let extra = remain - n;
    let mut shares = vec![1usize; n];
    if extra > 0 {
        let mut dirs: Vec<usize> = (0..n).filter(|&i| node.children[i].is_dir).collect();
        if dirs.is_empty() {
            dirs = (0..n).collect();
        }
        for i in 0..extra {
            shares[dirs[i % dirs.len()]] += 1;
        }
    }
    explain.push(format!("{}: budget {budget}, remain {remain}, shares {shares:?}", node.name));
    for (c, s) in node.children.iter_mut().zip(shares) {
        apply_budget(c, s, explain);
    }
}

fn suffix_label(name: &str) -> String {
    let base = name.rsplit('/').next().unwrap_or(name);
    match base.rsplit_once('.') {
        Some((stem, ext)) if !stem.is_empty() && !ext.is_empty() && !ext.contains('/') => {
            format!(".{ext}")
        }
        _ => "other".to_string(),
    }
}

/// INFO to hang on a name: the walk's confession about this line.
fn info_marks(n: &Node) -> String {
    let mut s = String::new();
    if n.denied {
        s.push_str("  [denied]");
    }
    if n.iter_err {
        s.push_str("  [unreadable: io]");
    }
    // Membership stays exact under a cut, so the census carries no `≥`;
    // the mark is what says this dir was cut short of the requested depth.
    if n.cut {
        s.push_str("  [walk bound]");
    }
    s
}

pub fn render(root_path: &str, tree: &Node, color: bool, stamp: &str) -> String {
    let mut root = root_path.to_string();
    if tree.is_dir && !root.ends_with('/') {
        root.push('/');
    }
    let root = crate::color::dir(&root, color);
    let mut root_census = String::new();
    if let Some(c) = &tree.leftover {
        let extra = c.render();
        if !extra.is_empty() {
            root_census = format!("  {extra}");
        }
    }
    let header = format!("{root}  {stamp}{root_census}{}", info_marks(tree));
    if tree.children.is_empty() && tree.omitted.is_none() {
        return format!("{header}\n");
    }
    let mut lines = vec![header];
    emit(&tree.children, tree.omitted.as_ref(), "", color, &mut lines);
    let mut s = lines.join("\n");
    s.push('\n');
    s
}

fn emit(
    kids: &[Node],
    omitted: Option<&Census>,
    prefix: &str,
    color: bool,
    out: &mut Vec<String>,
) {
    let has_om = omitted.map(|c| c.total > 0).unwrap_or(false);
    let total = kids.len() + usize::from(has_om);
    for (i, k) in kids.iter().enumerate() {
        let last = i + 1 == total;
        let branch = if last { "└── " } else { "├── " };
        let mut n = k.name.clone();
        if k.is_dir && !n.ends_with('/') {
            n.push('/');
        }
        if k.is_dir {
            n = crate::color::dir(&n, color);
        }
        if let Some(c) = &k.leftover {
            let extra = c.render();
            if !extra.is_empty() {
                n = format!("{n}  {extra}");
            }
        }
        n.push_str(&info_marks(k));
        out.push(format!("{prefix}{branch}{n}"));
        if !k.children.is_empty() || k.omitted.is_some() {
            let next = if last {
                format!("{prefix}    ")
            } else {
                format!("{prefix}│   ")
            };
            emit(&k.children, k.omitted.as_ref(), &next, color, out);
        }
    }
    if has_om {
        if let Some(c) = omitted {
            out.push(format!("{prefix}└── {}", c.render_plus()));
        }
    }
}
