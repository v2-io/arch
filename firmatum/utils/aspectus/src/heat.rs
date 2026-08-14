//! Heat — git-heat's tuned commit-decay model, adopted verbatim
//! (design/heat.md; reference `~/.local/bin/git-heat`).
//!
//! `raw = Σ over non-initial touches of exp(-age/τ)`, age in commits behind
//! HEAD, `τ = half_life / ln 2`, `heat = 2·(1−exp(−1/τ))·raw` — touched
//! every commit converges to ~2. Dir heat = max of non-noise leaves.
//! Initial commit excluded; noise basenames get heat 0 and stay out of the
//! rollup. Obtain is one `git log --name-only` pass per repo, visible set
//! only — never a reason to widen the walk. The log is capped at
//! `LOG_CAP` commits: at τ≈10 a touch 400 commits back contributes ~e⁻⁴⁰,
//! zero to double precision, so the cap changes no printed score; last-touch
//! times past the cap simply are not claimed (git-recency falls back to
//! mtime there).

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;

pub const DEFAULT_HALF_LIFE: f64 = 7.0;
const LOG_CAP: usize = 400;
/// Bytes of `git log` output read per repo before the tail is dropped.
const LOG_BYTE_CAP: u64 = 4 * 1024 * 1024;
const NOISE_BASENAMES: &[&str] = &["Cargo.toml", "SOURCE_REV"];

fn is_noise(path: &str) -> bool {
    let base = path.rsplit('/').next().unwrap_or(path);
    NOISE_BASENAMES.contains(&base)
}

/// A commit reference: (full sha, commits behind HEAD).
pub type ShaAt = (String, u32);

/// Per-file facts from the one log pass: heat, last-touch time, the
/// touching/introducing commits (design/heat.md; the lattice's
/// initial-sha/latest-sha row is the same obtain).
#[derive(Debug, Clone, Default)]
pub struct FileFact {
    pub heat: f64,
    pub ts: i64,
    /// Newest commit touching this path: (full sha, commits behind HEAD).
    pub touch: Option<ShaAt>,
    /// The commit that introduced this path (A/C in the window, an R's new
    /// name, or the initial commit when the log reached it). Never guessed
    /// past the log cap.
    pub intro: Option<ShaAt>,
}

/// One repo's heat map: repo-relative path → facts.
/// Dirs carry max-of-non-noise-leaves and newest touch beneath.
pub struct RepoHeat {
    pub root: PathBuf,
    files: HashMap<String, FileFact>,
    dirs: HashMap<String, (f64, i64)>,
    /// The whole repo as one container: max non-noise heat, newest touch.
    root_agg: (f64, i64),
}

impl RepoHeat {
    /// `None` when git is unavailable or the log fails — then the look
    /// claims no heat (a fact absent, never faked).
    pub fn obtain(root: &Path, half_life: f64) -> Option<RepoHeat> {
        // Spawn with a byte cap instead of an unbounded slurp: a repo
        // whose log emits pathological volume (import commits touching
        // huge trees) gets its oldest tail dropped at the cap — ages are
        // newest-first, so what's lost is what the decay already zeroed
        // (hardening 2026-08-14; largest measured local log ≈1.2 MB).
        use std::io::Read;
        let mut child = Command::new("git")
            .arg("-C")
            .arg(root)
            .args([
                "log",
                &format!("-n{LOG_CAP}"),
                "--pretty=format:%H %ct",
                "--name-status",
                "--diff-filter=ACMR",
            ])
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::null())
            .spawn()
            .ok()?;
        let mut buf = Vec::new();
        let capped = {
            let stdout = child.stdout.take()?;
            let mut limited = stdout.take(LOG_BYTE_CAP);
            limited.read_to_end(&mut buf).ok()?;
            buf.len() as u64 >= LOG_BYTE_CAP
        };
        let ok = if capped {
            let _ = child.kill();
            let _ = child.wait();
            // Drop the possibly-torn last line.
            if let Some(i) = buf.iter().rposition(|&b| b == b'\n') {
                buf.truncate(i);
            }
            true
        } else {
            child.wait().ok()?.success()
        };
        if !ok {
            return None;
        }
        let text = String::from_utf8_lossy(&buf);
        // Newest first: age = commit index. If the log reached the initial
        // commit (fewer than LOG_CAP returned), its touches are inventory
        // only — no heat contribution (the model excludes it) — but they
        // do claim introduction (everything there was born by then).
        // `--name-status` lines: `M\tpath`, `A\tpath`, `R100\told\tnew`.
        type Commit = (String, i64, Vec<(u8, String)>);
        let mut commits: Vec<Commit> = Vec::new();
        for line in text.lines() {
            let line = line.trim_end();
            if line.is_empty() {
                continue;
            }
            let header = line.split_once(' ').and_then(|(h, ts)| {
                (h.len() == 40 && h.bytes().all(|b| b.is_ascii_hexdigit()))
                    .then(|| ts.parse::<i64>().ok().map(|t| (h.to_string(), t)))
                    .flatten()
            });
            match header {
                Some((sha, ts)) => commits.push((sha, ts, Vec::new())),
                None => {
                    if let Some((_, _, paths)) = commits.last_mut() {
                        let mut it = line.split('\t');
                        let (Some(status), Some(first)) = (it.next(), it.next()) else {
                            continue;
                        };
                        let letter = status.as_bytes().first().copied().unwrap_or(b'M');
                        // Renames/copies list old\tnew: the touch (and the
                        // birth of the *name*) belongs to the new path.
                        let path = it.next().unwrap_or(first);
                        paths.push((letter, path.to_string()));
                    }
                }
            }
        }
        if commits.is_empty() {
            return None;
        }
        let saw_initial = commits.len() < LOG_CAP;
        let tau = half_life / std::f64::consts::LN_2;
        let scale = 2.0 * (1.0 - (-1.0 / tau).exp());
        struct Raw {
            score: f64,
            ts: i64,
            touch: Option<ShaAt>,
            intro: Option<ShaAt>,
        }
        let mut raw: HashMap<String, Raw> = HashMap::new();
        for (age, (sha, ts, paths)) in commits.iter().enumerate() {
            let initial = saw_initial && age == commits.len() - 1;
            for (letter, p) in paths {
                let e = raw.entry(p.clone()).or_insert(Raw {
                    score: 0.0,
                    ts: 0,
                    touch: None,
                    intro: None,
                });
                if !initial {
                    e.score += (-(age as f64) / tau).exp();
                }
                if *ts > e.ts {
                    e.ts = *ts;
                }
                if e.touch.is_none() {
                    // Newest-first: the first sighting is the last touch.
                    e.touch = Some((sha.clone(), age as u32));
                }
                if matches!(letter, b'A' | b'C' | b'R') || initial {
                    // Overwritten as we walk older commits: the oldest
                    // creation in the window wins.
                    e.intro = Some((sha.clone(), age as u32));
                }
            }
        }
        let mut files: HashMap<String, FileFact> = HashMap::new();
        let mut dirs: HashMap<String, (f64, i64)> = HashMap::new();
        let mut root_agg = (0.0f64, 0i64);
        for (p, r) in raw {
            let heat = if is_noise(&p) { 0.0 } else { scale * r.score };
            if heat > root_agg.0 {
                root_agg.0 = heat;
            }
            if r.ts > root_agg.1 {
                root_agg.1 = r.ts;
            }
            for anc in ancestors(&p) {
                let d = dirs.entry(anc).or_insert((0.0, 0));
                if !is_noise(&p) && heat > d.0 {
                    d.0 = heat;
                }
                if r.ts > d.1 {
                    d.1 = r.ts;
                }
            }
            files.insert(
                p,
                FileFact {
                    heat,
                    ts: r.ts,
                    touch: r.touch,
                    // Claimed only when the window actually saw a creation
                    // (or reached the initial commit) — never guessed.
                    intro: r.intro,
                },
            );
        }
        Some(RepoHeat {
            root: root.to_path_buf(),
            files,
            dirs,
            root_agg,
        })
    }

    /// Heat and last touch for a repo-relative path. Noise and unhistoried
    /// paths claim nothing (`heat` None); last-touch may still be known.
    pub fn lookup(&self, rel: &str, is_dir: bool) -> (Option<f64>, Option<i64>) {
        if is_dir {
            match self.dirs.get(rel) {
                Some(&(h, ts)) => ((h > 0.0).then_some(h), (ts > 0).then_some(ts)),
                None => (None, None),
            }
        } else {
            match self.files.get(rel) {
                Some(f) => ((f.heat > 0.0).then_some(f.heat), (f.ts > 0).then_some(f.ts)),
                None => (None, None),
            }
        }
    }

    /// The sha facts for a file (lattice initial-sha / latest-sha row):
    /// (introducing commit, last-touching commit), each (full sha, H~N).
    pub fn shas(&self, rel: &str) -> (Option<ShaAt>, Option<ShaAt>) {
        match self.files.get(rel) {
            Some(f) => (f.intro.clone(), f.touch.clone()),
            None => (None, None),
        }
    }
}

fn ancestors(path: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut acc = String::new();
    let mut parts = path.split('/').peekable();
    while let Some(p) = parts.next() {
        if parts.peek().is_none() {
            break; // the leaf itself
        }
        if !acc.is_empty() {
            acc.push('/');
        }
        acc.push_str(p);
        out.push(acc.clone());
    }
    out
}

/// The repo enclosing `path` (a `.git` dir or gitlink at or above it).
pub fn enclosing_repo(path: &Path) -> Option<PathBuf> {
    let mut p = path;
    loop {
        if p.join(".git").exists() {
            return Some(p.to_path_buf());
        }
        p = p.parent()?;
    }
}

/// Annotate the visible tree with heat and last-touch, one log pass per
/// repo (the root's enclosing repo, plus nested repos as their kind mark
/// surfaces them). The passes are independent subprocesses, so they run in
/// parallel — a program root full of submodules would otherwise pay them
/// serially (measured 2026-08-14: ~1.2s over ~/src/arch).
pub fn annotate(node: &mut crate::n_level::Node, abs: &Path, half_life: f64) {
    let mut roots: Vec<PathBuf> = Vec::new();
    if let Some(r) = enclosing_repo(abs) {
        roots.push(r);
    }
    collect_repo_roots(node, abs, &mut roots);
    roots.dedup();
    // Bounded pool: a 30-repo ~/src walk used to fork 30+ concurrent
    // gits (hardening 2026-08-14).
    let obtained: HashMap<PathBuf, RepoHeat> = std::thread::scope(|s| {
        let roots = &roots;
        let threads = crate::n_level::POOL_THREADS.min(roots.len().max(1));
        let handles: Vec<_> = (0..threads)
            .map(|t| {
                s.spawn(move || {
                    roots
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| i % threads == t)
                        .filter_map(|(_, r)| {
                            RepoHeat::obtain(r, half_life).map(|h| (r.clone(), h))
                        })
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        handles
            .into_iter()
            .flat_map(|h| h.join().expect("heat worker"))
            .collect()
    });
    let repo = enclosing_repo(abs).and_then(|r| obtained.get(&r));
    go(node, abs, repo, &obtained);
}

/// The `git` kind alone is not proof of a repo root (a lone .gitignore
/// claims it too); only a real `.git` entry roots a log pass — otherwise
/// the log would resolve to the enclosing repo and every relative path
/// would miss.
fn is_repo_root(node: &crate::n_level::Node, abs: &Path) -> bool {
    node.is_dir && node.kinds.iter().any(|k| k == "git") && abs.join(".git").exists()
}

fn collect_repo_roots(node: &crate::n_level::Node, abs: &Path, out: &mut Vec<PathBuf>) {
    if is_repo_root(node, abs) && !out.contains(&abs.to_path_buf()) {
        out.push(abs.to_path_buf());
    }
    for c in &node.children {
        collect_repo_roots(c, &abs.join(&c.name), out);
    }
}

fn go(
    node: &mut crate::n_level::Node,
    abs: &Path,
    repo: Option<&RepoHeat>,
    obtained: &HashMap<PathBuf, RepoHeat>,
) {
    let own = if is_repo_root(node, abs) {
        obtained.get(abs).filter(|r| repo.is_none_or(|c| c.root != r.root))
    } else {
        None
    };
    let repo = own.or(repo);
    if let Some(r) = repo
        && let Ok(rel) = abs.strip_prefix(&r.root)
    {
        let rel = rel.to_string_lossy();
        if !rel.is_empty() {
            let (h, ts) = r.lookup(&rel, node.is_dir);
            node.heat = h;
            node.git_ts = ts;
            if !node.is_dir {
                let (intro, touch) = r.shas(&rel);
                node.intro = intro;
                node.touch = touch;
            }
        } else if node.is_dir {
            // The repo root itself: the repo as one container.
            let (h, ts) = r.root_agg;
            node.heat = (h > 0.0).then_some(h);
            node.git_ts = (ts > 0).then_some(ts);
        }
    }
    for c in &mut node.children {
        let child_abs = abs.join(&c.name);
        go(c, &child_abs, repo, obtained);
    }
}
