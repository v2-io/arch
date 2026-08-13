//! Text render of an `Aspecta`. The picture is the teaching surface.

use crate::budget::{allocate_shares, Alloc};
use crate::ir::{Node, NodeKind};

/// Render `node` using `budget` lines including the node itself.
pub fn render_text(node: &Node, budget: usize) -> String {
    let mut lines = Vec::new();
    rec(node, budget, "", true, true, &mut lines);
    let mut s = lines.join("\n");
    if !s.is_empty() && !s.ends_with('\n') {
        s.push('\n');
    }
    s
}

fn rec(
    node: &Node,
    budget: usize,
    prefix: &str,
    is_root: bool,
    is_last: bool,
    out: &mut Vec<String>,
) {
    let branch = if is_root {
        ""
    } else if is_last {
        "└── "
    } else {
        "├── "
    };
    let alloc = allocate_shares(&node.children, budget);
    let listed_any = alloc.shares.iter().any(|&s| s > 0);
    out.push(format!(
        "{prefix}{branch}{}",
        format_node_line(node, !listed_any)
    ));

    let child_prefix = if is_root {
        String::new()
    } else if is_last {
        format!("{prefix}    ")
    } else {
        format!("{prefix}│   ")
    };

    emit(&node.children, &alloc, node, &child_prefix, out);
}

fn emit(children: &[Node], alloc: &Alloc, parent: &Node, prefix: &str, out: &mut Vec<String>) {
    let listed: Vec<usize> = (0..children.len())
        .filter(|&i| alloc.shares.get(i).copied().unwrap_or(0) > 0)
        .collect();
    let names = aggregate_names(children, alloc, parent);
    let has_agg = !names.is_empty();
    let total = listed.len() + usize::from(has_agg);

    for (k, &i) in listed.iter().enumerate() {
        let last = k + 1 == total;
        rec(&children[i], alloc.shares[i], prefix, false, last, out);
    }
    if has_agg && !listed.is_empty() {
        // Names already sit on the dir line when there are no listed children.
        out.push(format!(
            "{prefix}└── [{}{}]",
            if parent.annotations.truncated { "≥ " } else { "" },
            names.join(" · ")
        ));
    }
}

fn format_node_line(node: &Node, collapse_children: bool) -> String {
    let mut s = node.name.clone();
    if node.kind == NodeKind::Dir && !s.ends_with('/') {
        s.push('/');
    }
    if let Some(t) = &node.annotations.symlink_target {
        s.push_str(" -> ");
        s.push_str(t);
    }
    if let Some(role) = &node.annotations.role {
        s.push_str(&format!("  ({role})"));
    }
    if !node.annotations.kinds.is_empty() {
        s.push_str("  [");
        s.push_str(&node.annotations.kinds.join(", "));
        s.push(']');
    }
    if collapse_children {
        let mut names = node.annotations.omitted.clone();
        for c in &node.children {
            let mut n = c.name.clone();
            if c.is_dir() && !n.ends_with('/') {
                n.push('/');
            }
            if !names.iter().any(|x| x == &n || x.trim_end_matches('/') == n.trim_end_matches('/')) {
                names.push(n);
            }
        }
        if !names.is_empty() {
            let ge = if node.annotations.truncated { "≥ " } else { "" };
            s.push_str(&format!("  [{ge}{}]", names.join(" · ")));
        }
    }
    s
}

fn aggregate_names(children: &[Node], alloc: &Alloc, parent: &Node) -> Vec<String> {
    let mut names = Vec::new();
    for &i in &alloc.aggregated {
        let c = &children[i];
        let mut n = c.name.clone();
        if c.is_dir() && !n.ends_with('/') {
            n.push('/');
        }
        names.push(n);
    }
    for o in &parent.annotations.omitted {
        if !names.iter().any(|n| n == o || n.trim_end_matches('/') == o.trim_end_matches('/')) {
            names.push(o.clone());
        }
    }
    names
}
