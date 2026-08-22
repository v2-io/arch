//! Ignored bodies (design/gitignore-bodies.md): git's own ignore semantics,
//! consulted during the walk so ignored contents stay out of the look and
//! out of mass while presence still shows.
//!
//! The contract is "agrees with `git check-ignore`": nested `.gitignore`
//! files (deepest wins, last match within a file wins), `$GIT_DIR/info/
//! exclude`, the global `core.excludesFile` (honored — the leaning recorded
//! in the design's Open; the look agrees with the user's git), negations,
//! dir-only patterns, `**`. **Tracked beats ignored**: a path in the index
//! is part of the project whatever the patterns say; for a directory, any
//! tracked path beneath it keeps the directory from being treated as wholly
//! ignored (git still shows those files, so hiding the dir would lie).
//!
//! Scope: only inside a git work tree; a nested repo (submodule or plain)
//! replaces the outer repo's rules entirely, as git does. A parent dir
//! already ignored is never descended into, so per-path checks only need
//! the direct decision (git's own rule: a file cannot be re-included when
//! a parent directory is excluded).

use std::fs;
use std::path::{Path, PathBuf};

/// One parsed pattern line.
#[derive(Debug, Clone)]
struct Pattern {
    neg: bool,
    dir_only: bool,
    /// Contains a `/` before the end: relative to the file's own dir.
    anchored: bool,
    /// Segments, `**` kept as its own segment.
    segs: Vec<String>,
}

/// One ignore file's patterns, in file order (last match wins).
#[derive(Debug, Clone, Default)]
pub struct File {
    patterns: Vec<Pattern>,
}

impl File {
    pub fn parse(text: &str) -> File {
        let mut patterns = Vec::new();
        for raw in text.lines() {
            if let Some(p) = parse_line(raw) {
                patterns.push(p);
            }
        }
        File { patterns }
    }

    /// Last matching pattern's verdict: `Some(true)` = ignored,
    /// `Some(false)` = negated (re-included), `None` = no opinion.
    /// `rel` is relative to this file's own directory.
    fn verdict(&self, rel: &str, is_dir: bool) -> Option<bool> {
        let mut v = None;
        let segs: Vec<&str> = rel.split('/').collect();
        for p in &self.patterns {
            if p.dir_only && !is_dir {
                continue;
            }
            let hit = if p.anchored {
                segs_match(&p.segs, &segs)
            } else {
                // No slash: the basename, at any depth below this file.
                seg_match(&p.segs[0], segs[segs.len() - 1])
            };
            if hit {
                v = Some(!p.neg);
            }
        }
        v
    }
}

fn parse_line(raw: &str) -> Option<Pattern> {
    // Trailing spaces are stripped unless backslash-escaped.
    let mut line = raw;
    while line.ends_with(' ') && !line.ends_with("\\ ") {
        line = &line[..line.len() - 1];
    }
    if line.is_empty() || line.starts_with('#') {
        return None;
    }
    let mut neg = false;
    if let Some(rest) = line.strip_prefix('!') {
        neg = true;
        line = rest;
    } else if let Some(rest) = line
        .strip_prefix("\\!")
        .or_else(|| line.strip_prefix("\\#"))
    {
        // Escaped leading ! or # is literal; put the char back.
        return Some(finish(false, &format!("{}{rest}", &line[1..2])));
    }
    if line.is_empty() {
        return None;
    }
    Some(finish(neg, line))
}

fn finish(neg: bool, line: &str) -> Pattern {
    let mut s = line.to_string();
    let dir_only = s.ends_with('/');
    if dir_only {
        s.pop();
    }
    // A separator at the beginning or middle anchors to the file's dir.
    let anchored = s.contains('/');
    let s = s.strip_prefix('/').unwrap_or(&s);
    let segs: Vec<String> = s.split('/').map(str::to_string).collect();
    Pattern {
        neg,
        dir_only,
        anchored,
        segs,
    }
}

/// Segment-list match with `**` (zero or more whole segments). A trailing
/// `**` means "everything *inside*" — it requires at least one segment.
fn segs_match(pat: &[String], name: &[&str]) -> bool {
    match pat.first() {
        None => name.is_empty(),
        Some(p) if p == "**" => {
            if pat.len() == 1 {
                return !name.is_empty();
            }
            segs_match(&pat[1..], name) || (!name.is_empty() && segs_match(pat, &name[1..]))
        }
        Some(p) => !name.is_empty() && seg_match(p, name[0]) && segs_match(&pat[1..], &name[1..]),
    }
}

/// fnmatch within one segment: `*` (never crosses `/` — segments already
/// split), `?`, `[...]` classes, `\` escapes. A `**` inside a segment
/// behaves as `*` (git: "other consecutive asterisks are regular").
fn seg_match(pat: &str, name: &str) -> bool {
    fn inner(p: &[char], n: &[char]) -> bool {
        match p.first() {
            None => n.is_empty(),
            Some('*') => inner(&p[1..], n) || (!n.is_empty() && inner(p, &n[1..])),
            Some('?') => !n.is_empty() && inner(&p[1..], &n[1..]),
            Some('\\') if p.len() > 1 => !n.is_empty() && p[1] == n[0] && inner(&p[2..], &n[1..]),
            Some('[') => {
                let Some((matched, rest)) = class_match(&p[1..], n.first().copied()) else {
                    // Unterminated class: literal '['.
                    return !n.is_empty() && n[0] == '[' && inner(&p[1..], &n[1..]);
                };
                !n.is_empty() && matched && inner(rest, &n[1..])
            }
            Some(c) => !n.is_empty() && *c == n[0] && inner(&p[1..], &n[1..]),
        }
    }
    let p: Vec<char> = pat.chars().collect();
    let n: Vec<char> = name.chars().collect();
    inner(&p, &n)
}

/// `[...]` after the opening bracket. Returns (matched, rest-after-`]`).
fn class_match(p: &[char], c: Option<char>) -> Option<(bool, &[char])> {
    let mut i = 0;
    let neg = matches!(p.first(), Some('!') | Some('^'));
    if neg {
        i += 1;
    }
    let mut matched = false;
    let mut first = true;
    loop {
        let ch = *p.get(i)?;
        if ch == ']' && !first {
            let hit = matched != neg;
            return Some((hit && c.is_some(), &p[i + 1..]));
        }
        first = false;
        // Range a-z (not when '-' is last before ']').
        if p.get(i + 1) == Some(&'-') && p.get(i + 2).is_some_and(|&x| x != ']') {
            let (lo, hi) = (ch, *p.get(i + 2)?);
            if let Some(c) = c
                && lo <= c
                && c <= hi
            {
                matched = true;
            }
            i += 3;
        } else {
            if c == Some(ch) {
                matched = true;
            }
            i += 1;
        }
    }
}

/// The tracked set: sorted repo-relative paths from the index.
#[derive(Debug, Default)]
pub struct Tracked {
    paths: Vec<String>,
}

impl Tracked {
    fn contains(&self, rel: &str) -> bool {
        self.paths.binary_search_by(|p| p.as_str().cmp(rel)).is_ok()
    }

    /// Any tracked path strictly beneath this directory?
    fn any_under(&self, rel_dir: &str) -> bool {
        let prefix = format!("{rel_dir}/");
        let i = self.paths.partition_point(|p| p.as_str() < prefix.as_str());
        self.paths.get(i).is_some_and(|p| p.starts_with(&prefix))
    }
}

/// Parse `.git/index` versions 2–4 for the path list. `None` on any
/// anomaly — then tracked-beats-ignored has no evidence to act on (the
/// patterns still apply; recorded limitation, impl note).
pub fn tracked_paths(gitdir: &Path) -> Option<Tracked> {
    let buf = fs::read(gitdir.join("index")).ok()?;
    if buf.len() < 12 || &buf[0..4] != b"DIRC" {
        return None;
    }
    let ver = u32::from_be_bytes(buf[4..8].try_into().ok()?);
    if !(2..=4).contains(&ver) {
        return None;
    }
    let count = u32::from_be_bytes(buf[8..12].try_into().ok()?) as usize;
    let mut off = 12usize;
    let mut prev = String::new();
    let mut paths = Vec::with_capacity(count);
    for _ in 0..count {
        if off + 62 > buf.len() {
            return None;
        }
        let flags = u16::from_be_bytes(buf[off + 60..off + 62].try_into().ok()?);
        let mut name_off = off + 62;
        if ver >= 3 && flags & 0x4000 != 0 {
            name_off += 2; // extended flags word
        }
        if ver < 4 {
            let name_len = (flags & 0x0FFF) as usize;
            let end = if name_len < 0x0FFF {
                name_off + name_len
            } else {
                let mut e = name_off;
                while *buf.get(e)? != 0 {
                    e += 1;
                }
                e
            };
            if end > buf.len() {
                return None;
            }
            paths.push(String::from_utf8_lossy(&buf[name_off..end]).into_owned());
            // Fixed part + path, padded with 1–8 NULs to a multiple of 8.
            let total = end - off;
            off += (total + 8) & !7;
        } else {
            // v4: varint prefix-strip + NUL-terminated suffix, no padding.
            let (strip, used) = varint(&buf[name_off..])?;
            let mut e = name_off + used;
            let start = e;
            while *buf.get(e)? != 0 {
                e += 1;
            }
            let keep = prev.len().checked_sub(strip as usize)?;
            let mut p = prev[..keep].to_string();
            p.push_str(&String::from_utf8_lossy(&buf[start..e]));
            prev = p.clone();
            paths.push(p);
            off = e + 1;
        }
    }
    paths.sort();
    Some(Tracked { paths })
}

/// Git's offset-style varint (index v4 prefix strip).
fn varint(buf: &[u8]) -> Option<(u64, usize)> {
    let mut i = 0;
    let mut c = *buf.get(i)?;
    let mut v = (c & 0x7f) as u64;
    while c & 0x80 != 0 {
        i += 1;
        c = *buf.get(i)?;
        v = ((v + 1) << 7) | (c & 0x7f) as u64;
    }
    Some((v, i + 1))
}

/// One repo's per-repo state: index-tracked paths and the repo-level
/// exclude files (info/exclude, then the global core.excludesFile).
#[derive(Debug, Default)]
pub struct Repo {
    pub tracked: Tracked,
    /// Checked after the `.gitignore` chain, in this order.
    excludes: Vec<File>,
}

impl Repo {
    pub fn load(worktree: &Path) -> Repo {
        let mut r = Repo::default();
        let Some(gitdir) = crate::git::gitdir(worktree) else {
            return r;
        };
        if let Some(t) = tracked_paths(&gitdir) {
            r.tracked = t;
        }
        let common = crate::git::commondir(&gitdir);
        if let Ok(text) = fs::read_to_string(common.join("info").join("exclude")) {
            r.excludes.push(File::parse(&text));
        }
        if let Some(g) = global_excludes(&common) {
            r.excludes.push(g);
        }
        r
    }
}

/// The global excludes file: `core.excludesFile` from repo config, else
/// user config, else the git default `$XDG_CONFIG_HOME/git/ignore`.
fn global_excludes(common: &Path) -> Option<File> {
    let from = |cfg: &Path| -> Option<PathBuf> {
        let text = fs::read_to_string(cfg).ok()?;
        let mut in_core = false;
        let mut found = None;
        for raw in text.lines() {
            let line = raw.trim();
            if line.starts_with('[') {
                in_core = line == "[core]";
                continue;
            }
            if in_core
                && let Some((k, v)) = line.split_once('=')
                && k.trim().eq_ignore_ascii_case("excludesfile")
            {
                found = Some(expand_home(v.trim()));
            }
        }
        found
    };
    let home = std::env::var("HOME").ok();
    let xdg = std::env::var("XDG_CONFIG_HOME")
        .ok()
        .filter(|s| !s.is_empty())
        .map(PathBuf::from)
        .or_else(|| home.as_ref().map(|h| Path::new(h).join(".config")));
    let path = from(&common.join("config"))
        .or_else(|| {
            home.as_ref()
                .and_then(|h| from(&Path::new(h).join(".gitconfig")))
        })
        .or_else(|| {
            xdg.as_ref()
                .and_then(|x| from(&x.join("git").join("config")))
        })
        .or_else(|| xdg.map(|x| x.join("git").join("ignore")));
    let text = fs::read_to_string(path?).ok()?;
    Some(File::parse(&text))
}

fn expand_home(p: &str) -> PathBuf {
    if let Some(rest) = p.strip_prefix("~/")
        && let Ok(home) = std::env::var("HOME")
    {
        return Path::new(&home).join(rest);
    }
    PathBuf::from(p)
}

/// One enclosing repo during the walk, with its `.gitignore` chain from
/// the repo root down to the current dir.
#[derive(Debug)]
struct Frame {
    repo: Repo,
    base: PathBuf,
    /// (rel dir the file lives in — `""` at the root — , parsed file).
    chain: Vec<(String, File)>,
}

/// What one `enter_dir` did, for the matching `exit_dir`.
#[derive(Debug, Default)]
pub struct Entered {
    pushed_frame: bool,
    pushed_chain: bool,
}

/// The walk-side context. Only the innermost repo's rules apply (a nested
/// repo replaces the outer one, as in git).
#[derive(Debug, Default)]
pub struct Stack {
    frames: Vec<Frame>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack::default()
    }

    /// Seed for a walk starting *below* possible repo roots: loads the
    /// nearest enclosing repo of `dir` (at or above it) and the
    /// `.gitignore` chain from that root down to `dir` inclusive.
    pub fn for_path(dir: &Path) -> Stack {
        let mut s = Stack::new();
        let Some(root) = crate::heat::enclosing_repo(dir) else {
            return s;
        };
        let mut frame = Frame {
            repo: Repo::load(&root),
            base: root.clone(),
            chain: Vec::new(),
        };
        // .gitignore files from the repo root down to `dir`.
        let rel = dir.strip_prefix(&root).unwrap_or(Path::new(""));
        let mut here = root.clone();
        let mut prefix = String::new();
        let load = |here: &Path, prefix: &str, chain: &mut Vec<(String, File)>| {
            if let Ok(text) = fs::read_to_string(here.join(".gitignore")) {
                chain.push((prefix.to_string(), File::parse(&text)));
            }
        };
        load(&here, &prefix, &mut frame.chain);
        for seg in rel.components() {
            let seg = seg.as_os_str().to_string_lossy();
            here = here.join(&*seg);
            if !prefix.is_empty() {
                prefix.push('/');
            }
            prefix.push_str(&seg);
            load(&here, &prefix, &mut frame.chain);
        }
        s.frames.push(frame);
        s
    }

    /// Called on entering `dir` whose raw entry names are `names` (before
    /// furniture filtering — `.git` and `.gitignore` are furniture).
    pub fn enter_dir<'a>(
        &mut self,
        dir: &Path,
        mut names: impl Iterator<Item = &'a str>,
    ) -> Entered {
        let mut e = Entered::default();
        let mut has_git = false;
        let mut has_ignore = false;
        for n in names.by_ref() {
            match n {
                ".git" => has_git = true,
                ".gitignore" => has_ignore = true,
                _ => {}
            }
            if has_git && has_ignore {
                break;
            }
        }
        if has_git {
            self.frames.push(Frame {
                repo: Repo::load(dir),
                base: dir.to_path_buf(),
                chain: Vec::new(),
            });
            e.pushed_frame = true;
        }
        if has_ignore
            && let Some(frame) = self.frames.last_mut()
            && let Ok(text) = fs::read_to_string(dir.join(".gitignore"))
        {
            let prefix = dir
                .strip_prefix(&frame.base)
                .map(|p| p.to_string_lossy().into_owned())
                .unwrap_or_default();
            frame.chain.push((prefix, File::parse(&text)));
            e.pushed_chain = true;
        }
        e
    }

    pub fn exit_dir(&mut self, e: Entered) {
        if e.pushed_chain
            && let Some(frame) = self.frames.last_mut()
        {
            frame.chain.pop();
        }
        if e.pushed_frame {
            self.frames.pop();
        }
    }

    pub fn active(&self) -> bool {
        !self.frames.is_empty()
    }

    /// Is `dir/name` gitignored? `dir` must be inside the innermost frame.
    pub fn is_ignored(&self, dir: &Path, name: &str, is_dir: bool) -> bool {
        let Some(frame) = self.frames.last() else {
            return false;
        };
        let Ok(rel_dir) = dir.strip_prefix(&frame.base) else {
            return false;
        };
        let rel_dir = rel_dir.to_string_lossy();
        let rel = if rel_dir.is_empty() {
            name.to_string()
        } else {
            format!("{rel_dir}/{name}")
        };
        // Deepest .gitignore first; within a file, last match already won.
        let mut verdict = None;
        for (prefix, file) in frame.chain.iter().rev() {
            let sub = if prefix.is_empty() {
                rel.as_str()
            } else if let Some(s) = rel
                .strip_prefix(prefix.as_str())
                .and_then(|s| s.strip_prefix('/'))
            {
                s
            } else {
                continue;
            };
            if let Some(v) = file.verdict(sub, is_dir) {
                verdict = Some(v);
                break;
            }
        }
        if verdict.is_none() {
            for file in &frame.repo.excludes {
                if let Some(v) = file.verdict(&rel, is_dir) {
                    verdict = Some(v);
                    break;
                }
            }
        }
        if verdict != Some(true) {
            return false;
        }
        // Tracked beats ignored (git's own rule); for a dir, tracked
        // content beneath keeps it from being treated as wholly ignored.
        if is_dir {
            !frame.repo.tracked.any_under(&rel) && !frame.repo.tracked.contains(&rel)
        } else {
            !frame.repo.tracked.contains(&rel)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(file: &str, rel: &str, is_dir: bool) -> Option<bool> {
        File::parse(file).verdict(rel, is_dir)
    }

    #[test]
    fn basenames_match_at_depth() {
        assert_eq!(v("*.log\n", "a/b/x.log", false), Some(true));
        assert_eq!(v("*.log\n", "x.txt", false), None);
    }

    #[test]
    fn negation_last_match_wins() {
        assert_eq!(v("*.log\n!keep.log\n", "keep.log", false), Some(false));
        assert_eq!(v("!keep.log\n*.log\n", "keep.log", false), Some(true));
    }

    #[test]
    fn anchored_and_dir_only() {
        assert_eq!(v("/build\n", "build", true), Some(true));
        assert_eq!(v("/build\n", "x/build", true), None);
        assert_eq!(v("logs/\n", "logs", false), None);
        assert_eq!(v("logs/\n", "logs", true), Some(true));
        assert_eq!(v("doc/frotz/\n", "doc/frotz", true), Some(true));
        assert_eq!(v("doc/frotz/\n", "a/doc/frotz", true), None);
    }

    #[test]
    fn double_star() {
        assert_eq!(v("**/foo\n", "foo", false), Some(true));
        assert_eq!(v("**/foo\n", "a/b/foo", false), Some(true));
        assert_eq!(
            v("abc/**\n", "abc", true),
            None,
            "inside, not the dir itself"
        );
        assert_eq!(v("abc/**\n", "abc/x", false), Some(true));
        assert_eq!(v("a/**/b\n", "a/b", false), Some(true));
        assert_eq!(v("a/**/b\n", "a/x/y/b", false), Some(true));
    }

    #[test]
    fn classes_and_escapes() {
        assert_eq!(v("[Dd]ebug\n", "debug", true), Some(true));
        assert_eq!(v("f[0-9].md\n", "f3.md", false), Some(true));
        assert_eq!(v("f[!0-9].md\n", "f3.md", false), None);
        assert_eq!(v("\\#literal\n", "#literal", false), Some(true));
    }
}
