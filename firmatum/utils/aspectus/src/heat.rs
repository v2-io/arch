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

/// One repo's heat map: repo-relative path → (heat, last-touch unix secs).
/// Dirs carry max-of-non-noise-leaves and newest touch beneath.
pub struct RepoHeat {
    pub root: PathBuf,
    files: HashMap<String, (f64, i64)>,
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
                "--name-only",
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
        // only — no heat contribution (the model excludes it).
        let mut commits: Vec<(i64, Vec<String>)> = Vec::new();
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let header = line.split_once(' ').and_then(|(h, ts)| {
                (h.len() == 40 && h.bytes().all(|b| b.is_ascii_hexdigit()))
                    .then(|| ts.parse::<i64>().ok())
                    .flatten()
            });
            match header {
                Some(ts) => commits.push((ts, Vec::new())),
                None => {
                    if let Some((_, paths)) = commits.last_mut() {
                        paths.push(line.to_string());
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
        let mut raw: HashMap<String, (f64, i64)> = HashMap::new();
        for (age, (ts, paths)) in commits.iter().enumerate() {
            let initial = saw_initial && age == commits.len() - 1;
            for p in paths {
                let e = raw.entry(p.clone()).or_insert((0.0, 0));
                if !initial {
                    e.0 += (-(age as f64) / tau).exp();
                }
                if *ts > e.1 {
                    e.1 = *ts;
                }
            }
        }
        let mut files: HashMap<String, (f64, i64)> = HashMap::new();
        let mut dirs: HashMap<String, (f64, i64)> = HashMap::new();
        let mut root_agg = (0.0f64, 0i64);
        for (p, (r, ts)) in raw {
            let heat = if is_noise(&p) { 0.0 } else { scale * r };
            if heat > root_agg.0 {
                root_agg.0 = heat;
            }
            if ts > root_agg.1 {
                root_agg.1 = ts;
            }
            for anc in ancestors(&p) {
                let d = dirs.entry(anc).or_insert((0.0, 0));
                if !is_noise(&p) && heat > d.0 {
                    d.0 = heat;
                }
                if ts > d.1 {
                    d.1 = ts;
                }
            }
            files.insert(p, (heat, ts));
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
        let hit = if is_dir {
            self.dirs.get(rel)
        } else {
            self.files.get(rel)
        };
        match hit {
            Some(&(h, ts)) => {
                let heat = (h > 0.0).then_some(h);
                let touched = (ts > 0).then_some(ts);
                (heat, touched)
            }
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
