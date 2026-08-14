//! N-level tree. Depth is generations *below* the root.
//! `1` = children only. `2` = children and grandchildren. `0` = no limit.

use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub is_dir: bool,
    pub children: Vec<Node>,
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
    if !is_dir || remain == Some(0) {
        return Ok(Node {
            name,
            is_dir,
            children: Vec::new(),
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
    })
}

pub fn render(root_path: &str, tree: &Node, color: bool, stamp: &str) -> String {
    let mut root = root_path.to_string();
    if tree.is_dir && !root.ends_with('/') {
        root.push('/');
    }
    let root = crate::color::dir(&root, color);
    let header = format!("{root}  {stamp}");
    if tree.children.is_empty() {
        return format!("{header}\n");
    }
    let mut lines = vec![header];
    emit(&tree.children, "", color, &mut lines);
    let mut s = lines.join("\n");
    s.push('\n');
    s
}

fn emit(kids: &[Node], prefix: &str, color: bool, out: &mut Vec<String>) {
    for (i, k) in kids.iter().enumerate() {
        let last = i + 1 == kids.len();
        let branch = if last { "└── " } else { "├── " };
        let mut n = k.name.clone();
        if k.is_dir && !n.ends_with('/') {
            n.push('/');
        }
        if k.is_dir {
            n = crate::color::dir(&n, color);
        }
        out.push(format!("{prefix}{branch}{n}"));
        if !k.children.is_empty() {
            let next = if last {
                format!("{prefix}    ")
            } else {
                format!("{prefix}│   ")
            };
            emit(&k.children, &next, color, out);
        }
    }
}
