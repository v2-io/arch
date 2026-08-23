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

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// One repo's porcelain: the dirty count (parent-line facet) and per-path
/// XY letters (far-left git-status cell). One subprocess yields both.
struct Porcelain {
    dirty: usize,
    /// Repo-relative path (no trailing slash) → worktree-wins glyph.
    letters: HashMap<String, char>,
}

/// Fill git facets *and* per-path status letters in one bounded-parallel
/// pass (hardening 2026-08-14: the per-repo porcelain subprocess was a
/// serial ~2.4s on a multi-repo ~/src walk). The enclosing work tree of
/// the look is obtained even when `.git` sits above the asked path, so a
/// look at `src/` still gets letters. A node gets the facet when a real
/// `.git` exists there; the facet inserts at position 0 so the order
/// stays [git, github, …].
pub fn annotate(node: &mut crate::n_level::Node, abs: &Path) {
    let mut roots: Vec<PathBuf> = Vec::new();
    if let Some(r) = crate::heat::enclosing_repo(abs) {
        roots.push(r);
    }
    collect_work_trees(node, abs, &mut roots);
    roots.sort();
    roots.dedup();
    if roots.is_empty() {
        return;
    }
    let obtained: HashMap<PathBuf, Obtained> = std::thread::scope(|s| {
        let roots = &roots;
        let threads = crate::n_level::POOL_THREADS.min(roots.len());
        let handles: Vec<_> = (0..threads)
            .map(|t| {
                s.spawn(move || {
                    roots
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| i % threads == t)
                        .map(|(_, p)| (p.clone(), obtain(p)))
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        handles
            .into_iter()
            .flat_map(|h| h.join().expect("git facet worker"))
            .collect()
    });
    assign(node, abs, &roots, &obtained);
}

struct Obtained {
    facet: Option<String>,
    porcelain: Option<Porcelain>,
}

fn obtain(dir: &Path) -> Obtained {
    let porcelain = porcelain(dir);
    Obtained {
        facet: facet_with(dir, porcelain.as_ref()),
        porcelain,
    }
}

fn collect_work_trees(n: &crate::n_level::Node, abs: &Path, out: &mut Vec<PathBuf>) {
    if n.is_dir && abs.join(".git").exists() && !out.contains(&abs.to_path_buf()) {
        out.push(abs.to_path_buf());
    }
    for c in &n.children {
        collect_work_trees(c, &abs.join(&c.name), out);
    }
}

/// Innermost work tree containing `abs`, if any of `roots` prefixes it.
fn innermost<'a>(abs: &Path, roots: &'a [PathBuf]) -> Option<&'a Path> {
    roots
        .iter()
        .filter(|r| abs == r.as_path() || abs.starts_with(r))
        .max_by_key(|r| r.components().count())
        .map(PathBuf::as_path)
}

fn assign(
    n: &mut crate::n_level::Node,
    abs: &Path,
    roots: &[PathBuf],
    obtained: &HashMap<PathBuf, Obtained>,
) {
    if let Some(got) = obtained.get(abs)
        && let Some(f) = &got.facet
    {
        n.facets.insert(0, f.clone());
    }
    if let Some(root) = innermost(abs, roots) {
        // A `.git` name is not a repo: furniture decoys (empty `.git/`)
        // must not conjure the far-left block. Live = facet or porcelain
        // actually obtained.
        let live = obtained
            .get(root)
            .is_some_and(|g| g.facet.is_some() || g.porcelain.is_some());
        if live {
            n.in_git = true;
            if let Some(got) = obtained.get(root)
                && let Some(p) = &got.porcelain
                && let Ok(rel) = abs.strip_prefix(root)
            {
                // Files and directories both: porcelain names an untracked
                // dir as `?? dir/` (the children are not listed under
                // `--untracked-files=normal`). Tracked-and-clean dirs are
                // absent from the map — the formatter blanks them.
                let key = rel.to_string_lossy().replace('\\', "/");
                if !key.is_empty() {
                    n.git_letter = p.letters.get(&key).copied();
                }
            }
        }
    }
    for c in &mut n.children {
        let child_abs = abs.join(&c.name);
        assign(c, &child_abs, roots, obtained);
    }
}

/// The facet for a directory whose children include `.git`, e.g.
/// `git: remote<github.com/v2-io/x> br<main> @1a2b3c4 dirty<3>`.
/// `None` when the `.git` there is not readable as a repo.
pub fn facet(dir: &Path) -> Option<String> {
    facet_with(dir, porcelain(dir).as_ref())
}

fn facet_with(dir: &Path, porcelain: Option<&Porcelain>) -> Option<String> {
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
    if let Some(p) = porcelain
        && p.dirty > 0
    {
        parts.push(format!("dirty<{}>", p.dirty));
    }
    if parts.is_empty() {
        return None;
    }
    Some(format!("git: {}", parts.join(" ")))
}

/// `.git` directory, or the target of a `.git` gitlink file (submodule /
/// linked work tree).
pub(crate) fn gitdir(dir: &Path) -> Option<PathBuf> {
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
pub(crate) fn commondir(gitdir: &Path) -> PathBuf {
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

/// The one porcelain subprocess (design/furniture/git.md; impl/git.md):
/// `git status --porcelain --untracked-files=normal`. The dirty *count*
/// and the per-path XY letters come from this call — never a second
/// subprocess. `dir` is **that repo's root** (the work tree `obtain`
/// was given), never the look's root or a subpath of the repo: a look
/// at `src/` of a repo still counts dirt for the whole repo, and a look
/// at a sibling repo does not inherit this one. `None` = not obtained
/// (no git, error) — then the look claims nothing about dirt or letters.
fn porcelain(dir: &Path) -> Option<Porcelain> {
    let out = std::process::Command::new("git")
        .args(["-C"])
        .arg(dir)
        .args(["status", "--porcelain", "--untracked-files=normal"])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let mut letters = HashMap::new();
    let mut dirty = 0usize;
    for raw in out.stdout.split(|&b| b == b'\n') {
        if raw.is_empty() {
            continue;
        }
        dirty += 1;
        if let Some((path, glyph)) = parse_porcelain_line(raw) {
            letters.insert(path, glyph);
        }
    }
    Some(Porcelain { dirty, letters })
}

/// Porcelain v1: `XY PATH` or `XY ORIG -> PATH`. XY are ASCII. The
/// worktree letter wins when it is not space; otherwise the index
/// letter (design/grid-cleanup.md: "dirty is the glance question").
/// Unmerged pairs (`DD AU UD UA DU AA UU`, or either column `U`)
/// render as `U`. `??` is `⁇` (U+2047).
fn parse_porcelain_line(raw: &[u8]) -> Option<(String, char)> {
    if raw.len() < 3 {
        return None;
    }
    let x = raw[0] as char;
    let y = raw[1] as char;
    let rest = std::str::from_utf8(&raw[3..]).ok()?.trim_end();
    let path = if let Some(i) = rest.find(" -> ") {
        &rest[i + 4..]
    } else {
        rest
    };
    let path = unquote(path).trim_end_matches('/').replace('\\', "/");
    if path.is_empty() {
        return None;
    }
    let glyph = status_glyph(x, y)?;
    Some((path, glyph))
}

fn status_glyph(x: char, y: char) -> Option<char> {
    let unmerged = x == 'U' || y == 'U' || matches!((x, y), ('D', 'D') | ('A', 'A'));
    if unmerged {
        return Some('U');
    }
    let letter = if y != ' ' { y } else { x };
    match letter {
        'M' => Some('M'),
        'A' => Some('A'),
        '?' => Some('\u{2047}'), // ⁇ — porcelain's `??` in one cell
        'R' => Some('R'),
        'D' => Some('D'),
        'C' => Some('C'),
        'T' => Some('T'),
        'U' => Some('U'),
        _ => None,
    }
}

/// Git C-style quoted paths (`"foo\nbar"`). Unquoted input is returned
/// as-is.
fn unquote(s: &str) -> String {
    let Some(inner) = s.strip_prefix('"').and_then(|x| x.strip_suffix('"')) else {
        return s.to_string();
    };
    let mut out = String::with_capacity(inner.len());
    let mut chars = inner.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('n') => out.push('\n'),
            Some('t') => out.push('\t'),
            Some('r') => out.push('\r'),
            Some('"') => out.push('"'),
            Some('\\') => out.push('\\'),
            Some(other) => {
                out.push('\\');
                out.push(other);
            }
            None => out.push('\\'),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn urls_trim_to_host_path() {
        assert_eq!(trim_url("git@github.com:v2-io/x.git"), "github.com/v2-io/x");
        assert_eq!(
            trim_url("https://github.com/v2-io/x.git"),
            "github.com/v2-io/x"
        );
        assert_eq!(trim_url("ssh://git@host/a/b"), "host/a/b");
    }

    fn glyph_line(s: &str) -> (String, char) {
        parse_porcelain_line(s.as_bytes()).expect(s)
    }

    #[test]
    fn porcelain_worktree_wins() {
        // Unstaged modify: worktree M.
        assert_eq!(glyph_line(" M src/a.rs"), ("src/a.rs".into(), 'M'));
        // Staged modify, worktree clean: index M still speaks.
        assert_eq!(glyph_line("M  src/a.rs"), ("src/a.rs".into(), 'M'));
        // Staged add then further modified: worktree M, not A.
        assert_eq!(glyph_line("AM src/a.rs"), ("src/a.rs".into(), 'M'));
        assert_eq!(glyph_line("A  new.rs"), ("new.rs".into(), 'A'));
        assert_eq!(glyph_line(" D gone.rs"), ("gone.rs".into(), 'D'));
        assert_eq!(glyph_line("R  old.rs -> new.rs"), ("new.rs".into(), 'R'));
        assert_eq!(glyph_line("C  old.rs -> copy.rs"), ("copy.rs".into(), 'C'));
        assert_eq!(glyph_line("T  flipped"), ("flipped".into(), 'T'));
        assert_eq!(
            glyph_line("?? untracked.rs"),
            ("untracked.rs".into(), '\u{2047}')
        );
        assert_eq!(glyph_line("?? newdir/"), ("newdir".into(), '\u{2047}'));
        // Unmerged: U even when a column is D or A.
        assert_eq!(glyph_line("UU conflict.rs"), ("conflict.rs".into(), 'U'));
        assert_eq!(glyph_line("DU conflict.rs"), ("conflict.rs".into(), 'U'));
        assert_eq!(glyph_line("DD both-deleted"), ("both-deleted".into(), 'U'));
        assert_eq!(glyph_line("AA both-added"), ("both-added".into(), 'U'));
    }

    #[test]
    fn porcelain_unquote() {
        let (p, g) = glyph_line(r#"?? "foo bar.rs""#);
        assert_eq!(p, "foo bar.rs");
        assert_eq!(g, '\u{2047}');
        let (p, _) = glyph_line(r#"M  "a\nb""#);
        assert_eq!(p, "a\nb");
    }
}
