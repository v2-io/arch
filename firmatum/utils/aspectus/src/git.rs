//! Git as specialized furniture (design/furniture/git.md).
//!
//! The parent line says what git *is here*: local remote, branch, short
//! HEAD, porcelain dirt. **No network.** A submodule (`.git` gitlink file)
//! is the same furniture as a work tree. `(private)` is a later row.
//!
//! Branch / HEAD / remote come from reading the gitdir's own files — no
//! subprocess. Porcelain dirt is the one `git` invocation (local only);
//! when git is unavailable or fails, the dirt fact is simply not claimed —
//! a label is a claim, never a guess.

use std::fs;
use std::path::{Path, PathBuf};

/// Fill git facets across the visible tree in one bounded-parallel pass
/// (hardening 2026-08-14: the per-repo porcelain subprocess was a serial
/// ~2.4s on a multi-repo ~/src walk). A node gets the facet when its
/// kinds claim `git` and a real `.git` entry exists there; the facet
/// inserts at position 0 so the order stays [git, github, …].
pub fn annotate(node: &mut crate::n_level::Node, abs: &Path) {
    fn collect(n: &crate::n_level::Node, abs: &Path, out: &mut Vec<PathBuf>) {
        if n.is_dir && n.kinds.iter().any(|k| k == "git") && abs.join(".git").exists() {
            out.push(abs.to_path_buf());
        }
        for c in &n.children {
            collect(c, &abs.join(&c.name), out);
        }
    }
    fn assign(
        n: &mut crate::n_level::Node,
        abs: &Path,
        facets: &std::collections::HashMap<PathBuf, String>,
    ) {
        if let Some(f) = facets.get(abs) {
            n.facets.insert(0, f.clone());
        }
        for c in &mut n.children {
            let child_abs = abs.join(&c.name);
            assign(c, &child_abs, facets);
        }
    }
    let mut paths = Vec::new();
    collect(node, abs, &mut paths);
    if paths.is_empty() {
        return;
    }
    let facets: std::collections::HashMap<PathBuf, String> = std::thread::scope(|s| {
        let paths = &paths;
        let threads = crate::n_level::POOL_THREADS.min(paths.len());
        let handles: Vec<_> = (0..threads)
            .map(|t| {
                s.spawn(move || {
                    paths
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| i % threads == t)
                        .filter_map(|(_, p)| facet(p).map(|f| (p.clone(), f)))
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        handles
            .into_iter()
            .flat_map(|h| h.join().expect("git facet worker"))
            .collect()
    });
    assign(node, abs, &facets);
}

/// The facet for a directory whose children include `.git`, e.g.
/// `git: remote<github.com/v2-io/x> br<main> @1a2b3c4 dirty<3>`.
/// `None` when the `.git` there is not readable as a repo.
pub fn facet(dir: &Path) -> Option<String> {
    let gitdir = gitdir(dir)?;
    let common = commondir(&gitdir);
    let mut parts = Vec::new();
    if let Some(url) = remote_url(&common) {
        parts.push(format!("remote<{}>", trim_url(&url)));
    }
    let head = fs::read_to_string(gitdir.join("HEAD")).ok()?;
    let head = head.trim().to_string();
    let sha = if let Some(r) = head.strip_prefix("ref: ") {
        let branch = r.strip_prefix("refs/heads/").unwrap_or(r);
        parts.push(format!("br<{branch}>"));
        resolve_ref(&gitdir, &common, r)
    } else if !head.is_empty() {
        parts.push("detached".to_string());
        Some(head.clone())
    } else {
        None
    };
    if let Some(s) = sha {
        let short: String = s.chars().take(7).collect();
        parts.push(format!("@{short}"));
    }
    if let Some(n) = dirty(dir)
        && n > 0
    {
        parts.push(format!("dirty<{n}>"));
    }
    if parts.is_empty() {
        return None;
    }
    Some(format!("git: {}", parts.join(" ")))
}

/// `.git` directory, or the target of a `.git` gitlink file (submodule /
/// linked work tree).
fn gitdir(dir: &Path) -> Option<PathBuf> {
    let dotgit = dir.join(".git");
    let meta = fs::symlink_metadata(&dotgit).ok()?;
    if meta.is_dir() {
        return Some(dotgit);
    }
    let text = fs::read_to_string(&dotgit).ok()?;
    let target = text.trim().strip_prefix("gitdir:")?.trim();
    let p = Path::new(target);
    Some(if p.is_absolute() {
        p.to_path_buf()
    } else {
        dir.join(p)
    })
}

/// Linked work trees keep shared state under `commondir`.
fn commondir(gitdir: &Path) -> PathBuf {
    if let Ok(text) = fs::read_to_string(gitdir.join("commondir")) {
        let p = Path::new(text.trim());
        if p.is_absolute() {
            return p.to_path_buf();
        }
        return gitdir.join(p);
    }
    gitdir.to_path_buf()
}

fn resolve_ref(gitdir: &Path, common: &Path, r: &str) -> Option<String> {
    for base in [gitdir, common] {
        if let Ok(s) = fs::read_to_string(base.join(r)) {
            let s = s.trim();
            if !s.is_empty() {
                return Some(s.to_string());
            }
        }
    }
    for base in [gitdir, common] {
        if let Ok(packed) = fs::read_to_string(base.join("packed-refs")) {
            for line in packed.lines() {
                if line.starts_with('#') || line.starts_with('^') {
                    continue;
                }
                if let Some((sha, name)) = line.split_once(' ')
                    && name.trim() == r
                {
                    return Some(sha.trim().to_string());
                }
            }
        }
    }
    None
}

/// First remote URL, preferring `origin`. Local config only.
fn remote_url(common: &Path) -> Option<String> {
    let text = fs::read_to_string(common.join("config")).ok()?;
    let mut current: Option<String> = None;
    let mut first: Option<String> = None;
    let mut origin: Option<String> = None;
    for raw in text.lines() {
        let line = raw.trim();
        if line.starts_with('[') {
            current = line
                .strip_prefix("[remote \"")
                .and_then(|s| s.strip_suffix("\"]"))
                .map(str::to_string);
            continue;
        }
        if let Some(name) = &current
            && let Some((k, v)) = line.split_once('=')
            && k.trim() == "url"
        {
            let url = v.trim().to_string();
            if name == "origin" && origin.is_none() {
                origin = Some(url.clone());
            }
            if first.is_none() {
                first = Some(url);
            }
        }
    }
    origin.or(first)
}

/// `git@host:path`, `ssh://…`, `https://…` all render as `host/path`.
fn trim_url(url: &str) -> String {
    let mut s = url.trim().to_string();
    if let Some(rest) = s.strip_suffix(".git") {
        s = rest.to_string();
    }
    for scheme in ["ssh://", "https://", "http://", "git://"] {
        if let Some(rest) = s.strip_prefix(scheme) {
            s = rest.to_string();
        }
    }
    if let Some(rest) = s.strip_prefix("git@") {
        s = rest.replacen(':', "/", 1);
    }
    s
}

/// Count of porcelain-dirty paths. `None` = not obtained (no git, error) —
/// then the look claims nothing about dirt.
fn dirty(dir: &Path) -> Option<usize> {
    let out = std::process::Command::new("git")
        .args(["-C"])
        .arg(dir)
        .args(["status", "--porcelain", "--untracked-files=normal"])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    Some(
        out.stdout
            .split(|&b| b == b'\n')
            .filter(|l| !l.is_empty())
            .count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn urls_trim_to_host_path() {
        assert_eq!(trim_url("git@github.com:v2-io/x.git"), "github.com/v2-io/x");
        assert_eq!(trim_url("https://github.com/v2-io/x.git"), "github.com/v2-io/x");
        assert_eq!(trim_url("ssh://git@host/a/b"), "host/a/b");
    }
}
