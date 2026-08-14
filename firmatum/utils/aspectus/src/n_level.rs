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
        let n = if plus {
            format!("+{}", self.total)
        } else {
            self.total.to_string()
        };
        format!("[{n}: {}]", parts.join(", "))
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub is_dir: bool,
    pub children: Vec<Node>,
    /// Set when we stopped expanding this directory (depth cutoff).
    pub leftover: Option<Census>,
    /// Siblings not listed because the line budget ran out.
    pub omitted: Option<Census>,
}

/// `depth` is how many generations below the root to print. `0` = no limit.
pub fn gather(path: &Path, depth: u32) -> io::Result<Node> {
    let remain = if depth == 0 { None } else { Some(depth) };
    gather_at(path, remain)
}

fn gather_at(path: &Path, remain: Option<u32>) -> io::Result<Node> {
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
            children: Vec::new(),
            leftover: None,
            omitted: None,
        });
    }
    if remain == Some(0) {
        let leftover = census_dir(path)?;
        return Ok(Node {
            name,
            is_dir: true,
            children: Vec::new(),
            leftover: if leftover.total == 0 {
                None
            } else {
                Some(leftover)
            },
            omitted: None,
        });
    }
    let child_remain = remain.map(|n| n.saturating_sub(1));
    let mut children = Vec::new();
    if let Ok(rd) = fs::read_dir(path) {
        for ent in rd {
            let ent = ent?;
            let os = ent.file_name();
            if os == "." || os == ".." {
                continue;
            }
            let child_path = path.join(&os);
            let kid = gather_at(&child_path, child_remain)?;
            children.push(kid);
        }
    }
    children.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.cmp(&b.name),
    });
    Ok(Node {
        name,
        is_dir: true,
        children,
        leftover: None,
        omitted: None,
    })
}

fn label_node(n: &Node) -> String {
    if n.is_dir {
        "dir".into()
    } else {
        suffix_label(&n.name)
    }
}

fn fold_children_into_dir_census(node: &mut Node) {
    if node.children.is_empty() {
        return;
    }
    node.leftover = Some(census_nodes(&node.children));
    node.children.clear();
    node.omitted = None;
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
        node.omitted = Some(census_nodes(&drop));
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

fn census_dir(path: &Path) -> io::Result<Census> {
    let mut buckets: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
    let mut total = 0usize;
    let rd = match fs::read_dir(path) {
        Ok(rd) => rd,
        Err(_) => {
            return Ok(Census {
                total: 0,
                buckets: Vec::new(),
            });
        }
    };
    for ent in rd {
        let ent = ent?;
        let os = ent.file_name();
        if os == "." || os == ".." {
            continue;
        }
        total += 1;
        let is_dir = ent.file_type().map(|t| t.is_dir()).unwrap_or(false);
        let label = if is_dir {
            "dir".to_string()
        } else {
            suffix_label(&os.to_string_lossy())
        };
        *buckets.entry(label).or_insert(0) += 1;
    }
    let mut buckets: Vec<(String, usize)> = buckets.into_iter().collect();
    buckets.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    Ok(Census { total, buckets })
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

pub fn render(root_path: &str, tree: &Node, color: bool, stamp: &str) -> String {
    let mut root = root_path.to_string();
    if tree.is_dir && !root.ends_with('/') {
        root.push('/');
    }
    let root = crate::color::dir(&root, color);
    let header = format!("{root}  {stamp}");
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
