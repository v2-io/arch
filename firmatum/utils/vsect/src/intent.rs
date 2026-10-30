//! Intent: what an agent is about to do to a record, visible to every other agent now.
//!
//! A claim is not part of the record. It says nothing about truth and is not history, so it is
//! not committed and not hashed. It lives under git's *common* directory
//! (`git rev-parse --git-common-dir`), which every worktree of the repository shares, so an
//! integrator in its own worktree and a drain agent in the main tree see the same claims the
//! moment they are written — no commit, no rebase. When the store is not in a git repository,
//! claims live under `.vsect/.intents/` (a hidden directory, never a record).
//!
//! A claim expires (default six hours) and can be released by its holder, or by anyone with a
//! stated reason once it has expired. A second agent claiming a record another holds is refused
//! and shown the holder's note, unless it says `--alongside`, which records a visible co-intent
//! (two minds recording findings on one segment is fine; two minds editing one segment is the
//! collision the claim exists to surface).

use crate::store::{self, Store};
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claim {
    pub record: String,
    pub agent: String,
    #[serde(default)]
    pub role: String,
    pub intent: String,
    /// `edit` (exclusive by nature), `record` (additive: findings, items), or free text
    #[serde(default)]
    pub kind: String,
    pub worktree: String,
    #[serde(default)]
    pub branch: String,
    pub since: String,
    pub expires: String,
    #[serde(default)]
    pub alongside: bool,
}

impl Claim {
    pub fn expired(&self) -> bool {
        chrono::DateTime::parse_from_rfc3339(&self.expires)
            .map(|t| t.with_timezone(&chrono::Utc) < chrono::Utc::now())
            .unwrap_or(false)
    }
}

/// Where claims live for this store: the shared git common dir, else a hidden store dir.
pub fn intents_dir(store: &Store) -> PathBuf {
    match store::git(&store.root, &["rev-parse", "--git-common-dir"]) {
        Ok(d) => {
            let d = d.trim();
            let p = PathBuf::from(d);
            let p = if p.is_absolute() { p } else { store.root.join(p) };
            p.join("vsect-intents")
        }
        Err(_) => store.vsect_dir().join(".intents"),
    }
}

fn agent_key(agent: &str) -> String {
    crate::norm::sha256_hex(agent.as_bytes())[..12].to_string()
}

fn claim_path(store: &Store, record: &str, agent: &str) -> PathBuf {
    let (kind, slug) = record.split_once(':').unwrap_or(("_", record));
    intents_dir(store).join(kind).join(slug).join(format!("{}.yaml", agent_key(agent)))
}

pub fn parse_ttl(s: &str) -> Result<chrono::Duration> {
    let s = s.trim();
    let (num, unit) = s.split_at(s.trim_end_matches(|c: char| c.is_ascii_alphabetic()).len());
    let n: i64 = num.parse().with_context(|| format!("ttl `{s}`: a number then h or m, like 4h or 90m"))?;
    Ok(match unit {
        "h" | "" => chrono::Duration::hours(n),
        "m" => chrono::Duration::minutes(n),
        "d" => chrono::Duration::days(n),
        _ => bail!("ttl `{s}`: units are m, h, d"),
    })
}

pub fn all_claims(store: &Store) -> Vec<Claim> {
    let mut out = vec![];
    let pattern = intents_dir(store).join("*").join("*").join("*.yaml");
    if let Some(p) = pattern.to_str() {
        if let Ok(paths) = glob::glob(p) {
            for path in paths.flatten() {
                if let Ok(t) = std::fs::read_to_string(&path) {
                    if let Ok(c) = serde_yaml::from_str::<Claim>(&t) {
                        out.push(c);
                    }
                }
            }
        }
    }
    out.sort_by(|a, b| a.since.cmp(&b.since));
    out
}

pub fn claims_on(store: &Store, record: &str) -> Vec<Claim> {
    all_claims(store).into_iter().filter(|c| c.record == record).collect()
}

/// Write a claim. Refuses when another agent holds a live claim on the record unless `alongside`.
pub fn claim(store: &Store, record: &str, agent: &str, role: &str, intent: &str, kind: &str, ttl: chrono::Duration, alongside: bool) -> Result<(Claim, Vec<Claim>)> {
    let others: Vec<Claim> = claims_on(store, record).into_iter().filter(|c| c.agent != agent && !c.expired()).collect();
    if !others.is_empty() && !alongside {
        let who: Vec<String> = others.iter().map(|c| format!("{} ({}, in {}, since {}): {}", c.agent, if c.kind.is_empty() { "?" } else { &c.kind }, short_path(&c.worktree), c.since.get(11..16).unwrap_or(&c.since), c.intent)).collect();
        bail!("{record} is claimed by {}. Read their note; message them through your coordinator; or `--alongside` to record a visible co-intent (fine for recording findings, a collision for editing).", who.join("; "));
    }
    let now = chrono::Utc::now();
    let c = Claim {
        record: record.to_string(),
        agent: agent.to_string(),
        role: role.to_string(),
        intent: intent.to_string(),
        kind: kind.to_string(),
        worktree: store.root.display().to_string(),
        branch: store::git(&store.root, &["rev-parse", "--abbrev-ref", "HEAD"]).map(|s| s.trim().to_string()).unwrap_or_default(),
        since: now.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        expires: (now + ttl).format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        alongside: alongside && !others.is_empty(),
    };
    let path = claim_path(store, record, agent);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&path, serde_yaml::to_string(&c)?)?;
    Ok((c, others))
}

/// Remove a claim: the holder's own, or anyone's once expired, or anyone's with `force` and a reason.
pub fn release(store: &Store, record: &str, agent: &str, force: Option<&str>) -> Result<Vec<Claim>> {
    let mut released = vec![];
    for c in claims_on(store, record) {
        let mine = c.agent == agent;
        if mine || c.expired() || force.is_some() {
            let p = claim_path(store, record, &c.agent);
            let _ = std::fs::remove_file(&p);
            // drop the record's dir and the kind's dir when they are empty, so `ls` tells the truth
            if let Some(d) = p.parent() {
                let _ = std::fs::remove_dir(d);
                if let Some(k) = d.parent() {
                    let _ = std::fs::remove_dir(k);
                }
            }
            released.push(c);
        }
    }
    if released.is_empty() {
        let live: Vec<String> = claims_on(store, record).iter().map(|c| c.agent.clone()).collect();
        if live.is_empty() {
            bail!("no claim on {record}");
        }
        bail!("{record} is claimed by {} and not expired; only the holder releases it, or `--force --why \"…\"` with the reason printed", live.join(", "));
    }
    Ok(released)
}

pub fn short_path(p: &str) -> String {
    let home = std::env::var("HOME").unwrap_or_default();
    let s = if !home.is_empty() && p.starts_with(&home) { format!("~{}", &p[home.len()..]) } else { p.to_string() };
    // a worktree path says which agent's worktree in its last component
    match s.rsplit_once('/') {
        Some((_, last)) if s.contains("/worktrees/") => format!("worktree {last}"),
        _ => s,
    }
}

/// Records (kind:slug) that the given store paths resolve to, via the store's kind globs.
pub fn records_for_paths(store: &Store, paths: &[String]) -> Vec<(String, String)> {
    let scan = crate::record::scan(store);
    let mut out = vec![];
    for p in paths {
        let rel = std::path::Path::new(p);
        let rel = rel.strip_prefix(&store.root).map(|r| r.to_string_lossy().to_string()).unwrap_or_else(|_| p.clone());
        if let Some(r) = scan.records.values().find(|r| r.rel == rel) {
            out.push((rel.clone(), r.id.record_id()));
        }
    }
    out
}
