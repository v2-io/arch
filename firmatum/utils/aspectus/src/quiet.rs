//! Quiet columns, cold baseline (design/quiet-columns.md).
//!
//! A fact appears only when it surprises. Two kinds of expectation feed
//! cold surprise: **convention** (world-level priors — `644` is normal,
//! root-owner in your tree is not) and **sibling norms** (the level's own
//! statistics). Sibling norms are computed over the **full statted set of
//! a level** — pre-budget, so `--lines` cannot flicker quiet — and facts
//! unobtainable for a node are absent from the statistics, not zeros.
//! Convention legs (special permission bits, root-owner-when-not-you) are
//! laws, not thresholds: the sensitivity dial never scales them.
//!
//! This module only *decides* (annotates `Node.q` / `Node.kind_word`);
//! rendering stays with columns.rs, and JSON carries the underlying facts
//! regardless — quiet is a text-rendering law, not a data cut.

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::n_level::{Node, suffix_bucket};

/// Which quiet facts surprise on this line.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Speaks {
    pub size: bool,
    pub mtime: bool,
    pub mode: bool,
    pub owner: bool,
    /// The group leg of the owner/group fact.
    pub group: bool,
}

/// Channel tuning, from the caller stack only (config `quiet.sensitivity`,
/// default 1.0, plus per-fact overrides `quiet.sensitivity.size` /
/// `.mtime`). Scales the statistical thresholds; convention legs never.
#[derive(Debug, Clone, Copy)]
pub struct Dials {
    pub base: f64,
    pub size: Option<f64>,
    pub mtime: Option<f64>,
}

impl Default for Dials {
    fn default() -> Self {
        Dials {
            base: 1.0,
            size: None,
            mtime: None,
        }
    }
}

impl Dials {
    fn size_sens(&self) -> f64 {
        self.size.unwrap_or(self.base).max(f64::EPSILON)
    }
    fn mtime_sens(&self) -> f64 {
        self.mtime.unwrap_or(self.base).max(f64::EPSILON)
    }
}

/// Absolute floor under the size-outlier law: tiny trees don't cry outlier.
/// (Default constant awaiting ratification, design Open.)
pub const SIZE_FLOOR: u64 = 256 * 1024;
/// Cold recency window (seconds): recent vs *now*, SIGNA-scale rationale.
pub const RECENCY_WINDOW: f64 = 86_400.0;
/// Same-suffix cohorts need this many members to be a norm.
const COHORT_MIN: usize = 3;

/// The caller the surprise is *to*: euid/egid. (Supplementary group list
/// is a recorded gap — the group leg uses egid only today.)
pub struct Caller {
    pub euid: u32,
    pub egid: u32,
}

unsafe extern "C" {
    fn geteuid() -> u32;
    fn getegid() -> u32;
}

impl Caller {
    pub fn detect() -> Caller {
        Caller {
            euid: unsafe { geteuid() },
            egid: unsafe { getegid() },
        }
    }
}

/// How the filekind column was asked (`columns.filekind`): quiet decides
/// by plurality; `on` claims the word wherever kind is known.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KindAsk {
    On,
    Off,
    Quiet,
}

/// Annotate the whole (pre-budget) tree: every level's statistics are over
/// its full statted children, so the decision is budget-independent.
pub fn annotate(node: &mut Node, dials: &Dials, caller: &Caller, kind_ask: KindAsk, now: i64) {
    level(&mut node.children, dials, caller, kind_ask, now);
    for c in &mut node.children {
        annotate(c, dials, caller, kind_ask, now);
    }
}

fn level(kids: &mut [Node], dials: &Dials, caller: &Caller, kind_ask: KindAsk, now: i64) {
    if kids.is_empty() {
        return;
    }
    permissions(kids);
    owner(kids, caller);
    size(kids, dials);
    filekind(kids, kind_ask);
    mtime(kids, dials, now);
}

/// Plurality value (unique max count), or none.
fn plurality<T: Eq + std::hash::Hash + Copy>(vals: impl Iterator<Item = T>) -> Option<T> {
    let mut counts: HashMap<T, usize> = HashMap::new();
    for v in vals {
        *counts.entry(v).or_insert(0) += 1;
    }
    let max = counts.values().copied().max()?;
    let mut at_max = counts.iter().filter(|&(_, &n)| n == max);
    let winner = *at_max.next()?.0;
    at_max.next().is_none().then_some(winner)
}

/// Mode differs from *both* the conventional pair for its kind and the
/// level's same-kind majority. Special bits always speak.
fn permissions(kids: &mut [Node]) {
    let perm = |m: u32| m & 0o777;
    let file_majority = plurality(
        kids.iter()
            .filter(|n| !n.is_dir)
            .filter_map(|n| n.mode)
            .map(perm),
    );
    let dir_majority = plurality(
        kids.iter()
            .filter(|n| n.is_dir)
            .filter_map(|n| n.mode)
            .map(perm),
    );
    // `755` is also usual for files under a bin-like majority-executable
    // level (design table, permissions row).
    let files: Vec<u32> = kids
        .iter()
        .filter(|n| !n.is_dir)
        .filter_map(|n| n.mode)
        .collect();
    let exec_majority =
        !files.is_empty() && files.iter().filter(|m| *m & 0o111 != 0).count() * 2 > files.len();
    for n in kids.iter_mut() {
        let Some(mode) = n.mode else { continue };
        if mode & 0o7000 != 0 {
            n.q.mode = true; // setuid/setgid/sticky: no majority normalizes
            continue;
        }
        let p = perm(mode);
        let conventional = if n.is_dir {
            p == 0o755
        } else {
            p == 0o644 || (exec_majority && p == 0o755)
        };
        let majority = if n.is_dir {
            dir_majority
        } else {
            file_majority
        };
        let matches_majority = majority == Some(p);
        n.q.mode = !conventional && !matches_majority;
    }
}

/// Owner speaks when not the caller's euid *and* not the level's majority
/// owner; group mirrors with egid.
fn owner(kids: &mut [Node], caller: &Caller) {
    let uid_majority = plurality(kids.iter().filter_map(|n| n.uid));
    let gid_majority = plurality(kids.iter().filter_map(|n| n.gid));
    for n in kids.iter_mut() {
        if let Some(uid) = n.uid {
            n.q.owner = uid != caller.euid && uid_majority != Some(uid);
        }
        if let Some(gid) = n.gid {
            n.q.group = gid != caller.egid && gid_majority != Some(gid);
        }
    }
}

/// Log-scale deviation from the cohort median ≥ sensitivity, over an
/// absolute floor. Cohort = same-suffix siblings when ≥ COHORT_MIN exist,
/// else all files at the level; < COHORT_MIN files total → floor only.
/// Median, not mean — one whale must not silence itself.
fn size(kids: &mut [Node], dials: &Dials) {
    let sizes: Vec<(usize, String, u64)> = kids
        .iter()
        .enumerate()
        .filter(|(_, n)| !n.is_dir)
        .filter_map(|(i, n)| n.size.map(|s| (i, suffix_bucket(&n.name), s)))
        .collect();
    let total = sizes.len();
    let median = |mut v: Vec<u64>| -> u64 {
        v.sort_unstable();
        v[v.len() / 2]
    };
    let level_median = (total >= COHORT_MIN).then(|| median(sizes.iter().map(|t| t.2).collect()));
    for (i, suffix, s) in &sizes {
        if *s < SIZE_FLOOR {
            continue;
        }
        if total < COHORT_MIN {
            kids[*i].q.size = true; // a lone 400 MB file still speaks
            continue;
        }
        let cohort: Vec<u64> = sizes
            .iter()
            .filter(|t| t.1 == *suffix)
            .map(|t| t.2)
            .collect();
        let med = if cohort.len() >= COHORT_MIN {
            median(cohort)
        } else {
            level_median.unwrap()
        };
        let dev = (*s as f64).max(1.0).log10() - (med as f64).max(1.0).log10();
        kids[*i].q.size = dev >= dials.size_sens();
    }
}

/// Countable class for the kind-word law (design/filetype.md): counts
/// lines vs. does not. Structural types (dir/link/special) and unknown
/// sit in neither class and are absent from the statistics.
fn countable_class(n: &Node) -> Option<bool> {
    use crate::filetype::Major;
    match n.filetype.major {
        Major::Unknown | Major::Dir | Major::Link | Major::Special => None,
        _ => Some(n.filetype.counts_lines()),
    }
}

/// The kind word appears when the node's *countable class* differs from
/// its level's plurality, and then says the *major* (design/filetype.md).
/// A `.toml` among `.md` is the same class — silent; a PDF among `.md`
/// says `doc`. No plurality (≤ 2 classed files, or a class tie) → no
/// norm, no surprise. `on` claims the word wherever the major is known.
fn filekind(kids: &mut [Node], ask: KindAsk) {
    if ask == KindAsk::Off {
        return;
    }
    if ask == KindAsk::On {
        for n in kids.iter_mut() {
            if !n.is_dir {
                n.kind_word = n.filetype.kind_word();
            }
        }
        return;
    }
    let known: Vec<(usize, bool, &'static str)> = kids
        .iter()
        .enumerate()
        .filter(|(_, n)| !n.is_dir)
        .filter_map(|(i, n)| {
            let class = countable_class(n)?;
            let w = n.filetype.kind_word()?;
            Some((i, class, w))
        })
        .collect();
    if known.len() <= 2 {
        return;
    }
    let Some(plural) = plurality(known.iter().map(|(_, c, _)| *c)) else {
        return;
    };
    for (i, class, w) in known {
        if class != plural {
            kids[i].kind_word = Some(w);
        }
    }
}

/// Recent in absolute perceived terms: age below the window (scaled down
/// by sensitivity — a higher dial is harder to surprise). Truthfully
/// wall-clock-relative: the fact *is* about now; the header timestamps
/// the look.
fn mtime(kids: &mut [Node], dials: &Dials, now: i64) {
    let window = RECENCY_WINDOW / dials.mtime_sens();
    for n in kids.iter_mut() {
        if let Some(t) = n.mtime {
            let age = (now - t) as f64;
            n.q.mtime = age >= 0.0 && age < window;
        }
    }
}

/// uid → login name via /etc/passwd (root et al.); numeric fallback stays
/// honest where the table doesn't say. One parse per process.
pub fn user_name(uid: u32) -> Option<&'static str> {
    static TABLE: OnceLock<HashMap<u32, String>> = OnceLock::new();
    let t = TABLE.get_or_init(|| passwd_like("/etc/passwd"));
    t.get(&uid).map(|s| s.as_str())
}

/// gid → group name via /etc/group.
pub fn group_name(gid: u32) -> Option<&'static str> {
    static TABLE: OnceLock<HashMap<u32, String>> = OnceLock::new();
    let t = TABLE.get_or_init(|| passwd_like("/etc/group"));
    t.get(&gid).map(|s| s.as_str())
}

/// `name:*:id:...` lines — the shared shape of passwd and group files.
fn passwd_like(path: &str) -> HashMap<u32, String> {
    let mut m = HashMap::new();
    let Ok(text) = std::fs::read_to_string(path) else {
        return m;
    };
    for line in text.lines() {
        let mut f = line.split(':');
        let (Some(name), _, Some(id)) = (f.next(), f.next(), f.next()) else {
            continue;
        };
        if let Ok(id) = id.parse::<u32>() {
            m.entry(id).or_insert_with(|| name.to_string());
        }
    }
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    fn file(name: &str, mode: u32, uid: u32, size: u64) -> Node {
        Node {
            name: name.into(),
            mode: Some(mode),
            uid: Some(uid),
            gid: Some(20),
            size: Some(size),
            mtime: Some(1_000_000),
            ..Node::default()
        }
    }

    /// Root-owned speaks (subfeature 5 — the law, via the decision layer,
    /// since a real root-owned fixture needs privilege).
    #[test]
    fn root_owned_speaks_among_your_own() {
        let caller = Caller {
            euid: 501,
            egid: 20,
        };
        let mut kids = vec![
            file("a.md", 0o644, 501, 10),
            file("b.md", 0o644, 501, 10),
            file("evil", 0o644, 0, 10),
        ];
        owner(&mut kids, &caller);
        assert!(!kids[0].q.owner);
        assert!(kids[2].q.owner, "root ≠ you and ≠ majority");
    }

    /// In a shared tree the majority owner may not be you and is still
    /// unsurprising.
    #[test]
    fn majority_other_owner_is_silent() {
        let caller = Caller {
            euid: 501,
            egid: 20,
        };
        let mut kids = vec![
            file("a", 0o644, 1000, 1),
            file("b", 0o644, 1000, 1),
            file("c", 0o644, 501, 1),
        ];
        owner(&mut kids, &caller);
        assert!(
            !kids[0].q.owner,
            "majority owner silent even though not you"
        );
        assert!(!kids[2].q.owner, "you are always silent");
    }

    #[test]
    fn special_bits_always_speak() {
        let mut kids = vec![
            file("a", 0o4755, 501, 1),
            file("b", 0o4755, 501, 1),
            file("c", 0o4755, 501, 1),
        ];
        permissions(&mut kids);
        assert!(kids.iter().all(|n| n.q.mode), "even in an all-setuid level");
    }

    #[test]
    fn bin_dir_of_755_files_is_silent() {
        let mut kids = vec![
            file("x", 0o755, 501, 1),
            file("y", 0o755, 501, 1),
            file("z", 0o755, 501, 1),
        ];
        permissions(&mut kids);
        assert!(kids.iter().all(|n| !n.q.mode), "sibling-usual and bin-like");
    }

    #[test]
    fn two_whales_both_speak() {
        let mut kids: Vec<Node> = (0..40)
            .map(|i| file(&format!("f{i}.md"), 0o644, 501, 2048))
            .collect();
        kids.push(file("whale1.md", 0o644, 501, 80 << 20));
        kids.push(file("whale2.md", 0o644, 501, 80 << 20));
        size(&mut kids, &Dials::default());
        assert!(kids[40].q.size && kids[41].q.size, "median guards the norm");
        assert!(!kids[0].q.size);
    }
}
