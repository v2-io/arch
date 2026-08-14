//! Two-level look: the place and its immediate children. No furniture rule.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Entry {
    pub name: String,
    pub is_dir: bool,
}

pub fn list(path: impl AsRef<Path>) -> io::Result<(String, Vec<Entry>)> {
    let path = path.as_ref();
    let meta = fs::symlink_metadata(path)?;
    if !meta.is_dir() && !meta.file_type().is_symlink() {
        // A file locus: name only, no children.
        let name = display_name(path);
        return Ok((name, Vec::new()));
    }
    let name = display_name(path);
    let mut kids = Vec::new();
    for ent in fs::read_dir(path)? {
        let ent = ent?;
        let os = ent.file_name();
        if os == "." || os == ".." {
            continue;
        }
        let fname = os.to_string_lossy().into_owned();
        let is_dir = ent.file_type().map(|t| t.is_dir()).unwrap_or(false);
        kids.push(Entry {
            name: fname,
            is_dir,
        });
    }
    kids.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.cmp(&b.name),
    });
    Ok((name, kids))
}

pub fn render(name: &str, kids: &[Entry]) -> String {
    let mut root = name.to_string();
    if !root.ends_with('/') {
        root.push('/');
    }
    if kids.is_empty() {
        return format!("{root}\n");
    }
    let mut lines = vec![root];
    for (i, k) in kids.iter().enumerate() {
        let last = i + 1 == kids.len();
        let branch = if last { "└── " } else { "├── " };
        let mut n = k.name.clone();
        if k.is_dir && !n.ends_with('/') {
            n.push('/');
        }
        lines.push(format!("{branch}{n}"));
    }
    let mut s = lines.join("\n");
    s.push('\n');
    s
}

fn display_name(path: &Path) -> String {
    if path.as_os_str() == "." || path.as_os_str().is_empty() {
        return std::env::current_dir()
            .ok()
            .and_then(|p| p.file_name().map(|s| s.to_string_lossy().into_owned()))
            .unwrap_or_else(|| ".".into());
    }
    path.file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| path.display().to_string())
}

/// Resolve `.` against cwd for tests that pass a real fixture path.
pub fn resolve_locus(path: &Path) -> PathBuf {
    if path.as_os_str() == "." {
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
    } else {
        path.to_path_buf()
    }
}
