//! Bounded walk. Never enters Absorb names. Visit-budget stop is a `≥` verdict.

use std::collections::HashSet;
use std::fs::{self, Metadata};
use std::io;
use std::path::{Path, PathBuf};

use crate::absorb::{fate, is_absorb, partition, Fate};
use crate::ir::{Aspecta, Node};

#[derive(Debug, Clone)]
pub struct WalkOptions {
    /// Max directory entries processed (stat / classified). Not a line budget.
    pub visit_budget: usize,
    /// Stay on the starting filesystem (`-x`).
    pub one_filesystem: bool,
    /// `None` = default (do not open absorb). `Some("*")` = `--raw`.
    /// `Some(kind)` = `--inspect kind`.
    pub inspect: Option<String>,
}

impl Default for WalkOptions {
    fn default() -> Self {
        Self {
            visit_budget: 400,
            one_filesystem: true,
            inspect: None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct WalkStats {
    /// Directories we called `read_dir` on. Tests assert absorb trees are absent.
    pub dirs_opened: Vec<PathBuf>,
    pub visits: usize,
}

pub struct WalkResult {
    pub aspecta: Aspecta,
    pub stats: WalkStats,
}

struct Ctx {
    opts: WalkOptions,
    start_dev: Option<u64>,
    seen: HashSet<(u64, u64)>,
    stats: WalkStats,
}

/// Walk `root` into an `Aspecta`. Does not allocate a line budget — that is
/// render's job. Does not enter Absorb names unless `inspect` says so.
pub fn walk(root: impl AsRef<Path>, opts: WalkOptions) -> io::Result<WalkResult> {
    let root = root.as_ref().to_path_buf();
    let meta = fs::symlink_metadata(&root)?;
    let start_dev = device(&meta);
    let mut ctx = Ctx {
        opts,
        start_dev,
        seen: HashSet::new(),
        stats: WalkStats::default(),
    };
    if let Some(id) = inode_id(&meta) {
        ctx.seen.insert(id);
    }
    let name = root
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| root.display().to_string());
    let node = walk_dir(&root, name, &mut ctx)?;
    Ok(WalkResult {
        aspecta: Aspecta::new(root, node),
        stats: ctx.stats,
    })
}

fn walk_dir(path: &Path, name: String, ctx: &mut Ctx) -> io::Result<Node> {
    ctx.stats.dirs_opened.push(path.to_path_buf());
    let rd = match fs::read_dir(path) {
        Ok(rd) => rd,
        Err(e) => {
            let mut n = Node::dir(name);
            n.annotations.omitted.push(format!("({e})"));
            n.annotations.truncated = true;
            return Ok(n);
        }
    };

    let mut entries: Vec<(String, bool, PathBuf, Option<Metadata>)> = Vec::new();
    let mut leftover = Vec::new();
    for ent in rd {
        let ent = match ent {
            Ok(e) => e,
            Err(_) => continue,
        };
        let ename = ent.file_name().to_string_lossy().into_owned();
        if ctx.stats.visits >= ctx.opts.visit_budget {
            leftover.push(ename);
            continue;
        }
        ctx.stats.visits += 1;
        let ft = ent.file_type().ok();
        let is_dir = ft.as_ref().map(|t| t.is_dir()).unwrap_or(false);
        let is_symlink = ft.as_ref().map(|t| t.is_symlink()).unwrap_or(false);
        let meta = ent.metadata().ok();
        // Follow symlink-to-dir as a directory for descent, still record target.
        let mut dirish = is_dir;
        let p = ent.path();
        if is_symlink {
            if let Ok(m) = fs::metadata(&p) {
                dirish = m.is_dir();
            }
        }
        let _ = is_symlink;
        entries.push((ename, dirish, p, meta));
    }

    let classified: Vec<(String, bool)> = entries
        .iter()
        .map(|(n, d, _, _)| (n.clone(), *d))
        .collect();
    let inspect = ctx.opts.inspect.clone();
    let inspect_ref = inspect.as_deref();
    let part = partition(classified, inspect_ref);

    let mut kinds: Vec<String> = Vec::new();
    for (_, k) in &part.absorbed {
        if !kinds.iter().any(|x| x == k) {
            kinds.push((*k).to_string());
        }
    }
    for c in &part.children {
        if let Some(k) = c.witness_kind {
            if !kinds.iter().any(|x| x == k) {
                kinds.push(k.to_string());
            }
        }
    }

    let mut children = Vec::new();
    for spec in &part.children {
        let (ename, is_dir, p, meta) = entries
            .iter()
            .find(|(n, _, _, _)| n == &spec.name)
            .expect("partition name from entries");
        let f = fate(ename, *is_dir);
        let open_absorb = is_absorb(f) && inspect.is_some();

        if *is_dir && (!is_absorb(f) || open_absorb) {
            if ctx.opts.one_filesystem {
                if let (Some(start), Some(m)) = (ctx.start_dev, meta.as_ref()) {
                    if device(m) != Some(start) {
                        let mut leaf = Node::dir(ename.clone());
                        if let Some(r) = spec.role {
                            leaf.annotations.role = Some(r.to_string());
                        }
                        children.push(leaf);
                        continue;
                    }
                }
            }
            if let Some(id) = meta.as_ref().and_then(inode_id) {
                if !ctx.seen.insert(id) {
                    let mut leaf = Node::dir(ename.clone());
                    leaf.annotations.omitted.push("(cycle)".into());
                    children.push(leaf);
                    continue;
                }
            }
            let mut child = walk_dir(p, ename.clone(), ctx)?;
            if let Some(r) = spec.role {
                child.annotations.role = Some(r.to_string());
            }
            children.push(child);
        } else if meta
            .as_ref()
            .map(|m| m.file_type().is_symlink())
            .unwrap_or(false)
            || fs::symlink_metadata(p)
                .map(|m| m.file_type().is_symlink())
                .unwrap_or(false)
        {
            let target = fs::read_link(p)
                .map(|t| t.display().to_string())
                .unwrap_or_default();
            let mut n = Node::symlink(ename.clone(), target);
            if let Some(r) = spec.role {
                n.annotations.role = Some(r.to_string());
            }
            children.push(n);
        } else {
            let mut n = Node::file(ename.clone());
            if let Some(r) = spec.role {
                n.annotations.role = Some(r.to_string());
            }
            children.push(n);
        }
    }

    children.sort_by(|a, b| match (a.is_dir(), b.is_dir()) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.cmp(&b.name),
    });

    let mut node = Node::dir(name).with_kinds(kinds).with_children(children);
    if !leftover.is_empty() {
        node.annotations.truncated = true;
        for n in leftover {
            match fate(&n, true) {
                Fate::Omit => {}
                Fate::Absorb { .. } if inspect.is_none() => {}
                _ => node.annotations.omitted.push(n),
            }
        }
    }
    Ok(node)
}

pub fn same_filesystem(a: &Metadata, b: &Metadata) -> bool {
    device(a) == device(b)
}

#[cfg(unix)]
fn device(meta: &Metadata) -> Option<u64> {
    use std::os::unix::fs::MetadataExt;
    Some(meta.dev())
}

#[cfg(not(unix))]
fn device(_meta: &Metadata) -> Option<u64> {
    None
}

#[cfg(unix)]
fn inode_id(meta: &Metadata) -> Option<(u64, u64)> {
    use std::os::unix::fs::MetadataExt;
    Some((meta.dev(), meta.ino()))
}

#[cfg(not(unix))]
fn inode_id(_meta: &Metadata) -> Option<(u64, u64)> {
    None
}
