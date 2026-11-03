//! Events (one file per act, content-addressed) and objects (one file per version seen).

use crate::norm;
use crate::store::Store;
use anyhow::{anyhow, bail, Context, Result};
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::collections::BTreeMap;
use std::path::PathBuf;

pub const SCHEMA: &str = "vsect-event/0";

/// Who acted. `name` is a label (typed with `--by`, or defaulted from the harness identity);
/// `agent` is the identity acts are compared on when the harness supplied one. `identity` says
/// what the record of who acted rests on, and is always set by vsect, never read from a file:
///
/// - `harness-agent`: `agent` is the Claude Code agent key (`<session>` or `<session>/<agent-id>`),
///   exported into the shell as `CC_AGENT` by the agent-identity hook from the harness's own ids;
/// - `harness-session`: only the root session id was available (`CLAUDE_CODE_SESSION_ID`), which
///   every subagent of that session shares, so it cannot tell a lead from its subagents;
/// - `declared`: nothing but what was typed.
///
/// Fields absent on events written before 2026-11-02 are absent, not empty, so those events hash
/// as they always did.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Actor {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    #[serde(default, rename = "agent-type", skip_serializing_if = "Option::is_none")]
    pub agent_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub read: Vec<String>,
    /// Actor fields this build does not know, carried through unchanged so that an event written
    /// by a newer vsect still hashes to its id here. Without it, the 2026-10-30 build read the
    /// first events carrying `agent:` as edited (the unknown keys were dropped before re-hashing).
    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extra: BTreeMap<String, Value>,
}

impl Actor {
    /// The identity acts are compared on: the agent key when there is one, else the name.
    pub fn mind(&self) -> &str {
        self.agent.as_deref().filter(|s| !s.is_empty()).unwrap_or(&self.name)
    }
    /// Same mind: equal agent keys when both acts carry one; otherwise equal names (the only
    /// evidence older or undecorated records have). Agent keys answer the 74-spellings problem
    /// both ways: one agent typing two names is one mind; two agents typing one name are two.
    pub fn same_mind(&self, other: &Actor) -> bool {
        match (self.agent.as_deref().filter(|s| !s.is_empty()), other.agent.as_deref().filter(|s| !s.is_empty())) {
            (Some(a), Some(b)) => a == b,
            _ => self.name == other.name,
        }
    }
    /// The name, with the agent's short id when the name was typed and does not show it.
    pub fn label(&self) -> String {
        match self.agent.as_deref() {
            Some(a) => {
                let short = a.rsplit('/').next().unwrap_or(a);
                let short = &short[..short.len().min(8)];
                if self.name.contains(short) { self.name.clone() } else { format!("{} [{}]", self.name, short) }
            }
            None => self.name.clone(),
        }
    }
}

/// The identity the harness supplies to this process, if any (see `Actor`).
#[derive(Debug, Clone, Default)]
pub struct HarnessIdentity {
    pub identity: &'static str,
    pub session: Option<String>,
    pub agent: Option<String>,
    pub agent_id: Option<String>,
    pub agent_type: Option<String>,
    pub parent: Option<String>,
}

/// Read the harness identity from the environment. `CC_AGENT` is written into every Bash command
/// by the agent-identity hook (estate install, 2026-11-02): the agent key first, then `key=value`
/// fields, e.g. `<session>/<agent-id> parent=<session> type=general-purpose depth=1
/// src=agent-identity-hook/2`. The key counts as harness-supplied only when `src=` names that
/// hook; an agent can still export a value by hand, so this guards against mistakes, not fraud.
pub fn harness_identity() -> HarnessIdentity {
    harness_identity_from(std::env::var("CC_AGENT").ok().as_deref(), std::env::var("CLAUDE_CODE_SESSION_ID").ok().as_deref())
}

pub fn harness_identity_from(cc_agent: Option<&str>, session_env: Option<&str>) -> HarnessIdentity {
    let session = session_env.map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
    if let Some(v) = cc_agent {
        let mut words = v.split_whitespace();
        let key = words.next().unwrap_or("").to_string();
        let fields: BTreeMap<&str, &str> = words.filter_map(|w| w.split_once('=')).collect();
        if !key.is_empty() && !key.contains('=') && fields.get("src").map(|s| s.starts_with("agent-identity-hook/")).unwrap_or(false) {
            let agent_id = key.split_once('/').map(|(_, a)| a.to_string());
            let get = |k: &str| fields.get(k).map(|s| s.to_string()).filter(|s| !s.is_empty());
            return HarnessIdentity {
                identity: "harness-agent",
                session: Some(key.split('/').next().unwrap_or(&key).to_string()),
                agent: Some(key.clone()),
                agent_id: Some(agent_id.unwrap_or_else(|| "main".into())),
                agent_type: get("type"),
                parent: get("parent"),
            };
        }
    }
    if session.is_some() {
        return HarnessIdentity { identity: "harness-session", session, ..Default::default() };
    }
    HarnessIdentity { identity: "declared", ..Default::default() }
}

impl HarnessIdentity {
    /// A readable default name: `main 0903e35e` or `general-purpose a4d8232a`.
    pub fn default_name(&self) -> Option<String> {
        let key = self.agent.as_deref()?;
        let id = self.agent_id.as_deref().unwrap_or("main");
        Some(if id == "main" {
            format!("main {}", &key[..key.len().min(8)])
        } else {
            format!("{} {}", self.agent_type.as_deref().unwrap_or("agent"), &id[..id.len().min(8)])
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub schema: String,
    #[serde(default)]
    pub id: String,
    pub at: String,
    pub store: String,
    pub act: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub atom: Option<String>,
    pub actor: Actor,
    pub normalizer: String,
    #[serde(flatten)]
    pub payload: BTreeMap<String, Value>,
}

/// The id an event hashes to: SHA-256 of the canonical JSON (sorted keys) of every field but `id`.
pub fn compute_id(ev: &Event) -> Result<String> {
    let mut v = serde_json::to_value(ev)?;
    if let serde_json::Value::Object(m) = &mut v {
        m.remove("id");
    }
    let canon = serde_json::to_string(&v)?; // serde_json's Map sorts keys
    Ok(norm::sha256_hex(canon.as_bytes()))
}

pub fn now() -> String {
    chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

impl Event {
    pub fn new(store: &Store, act: &str, about: Option<&str>, actor: Actor) -> Event {
        Event {
            schema: SCHEMA.into(),
            id: String::new(),
            at: now(),
            store: store.kinds.name.clone(),
            act: act.into(),
            about: about.map(|s| s.to_string()),
            atom: None,
            actor,
            normalizer: norm::NORMALIZER.into(),
            payload: BTreeMap::new(),
        }
    }
    pub fn with(mut self, key: &str, v: Value) -> Event {
        self.payload.insert(key.into(), v);
        self
    }
    pub fn str(&self, key: &str) -> Option<&str> {
        self.payload.get(key).and_then(|v| v.as_str())
    }
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.payload.get(key)
    }
    /// The event's directory under `.vsect/events/`, from `about`.
    pub fn dir(&self) -> String {
        match &self.about {
            Some(a) => {
                let rec = a.split('#').next().unwrap_or(a);
                match rec.split_once(':') {
                    Some((k, s)) => format!("{k}/{s}"),
                    None => format!("_/{rec}"),
                }
            }
            None => "_store".into(),
        }
    }
    pub fn rel_path(&self) -> String {
        format!(".vsect/events/{}/{}.yaml", self.dir(), self.id)
    }
}

thread_local! {
    /// Every file a verb wrote in this run, relative to the store root, so the verb can end by
    /// naming them: "commit your own paths only" needs no guessing about where `take` puts a finding.
    pub static WRITTEN: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) };
}

pub fn note_written(store: &Store, path: &std::path::Path) {
    let rel = path.strip_prefix(&store.root).map(|p| p.to_string_lossy().to_string()).unwrap_or_else(|_| path.display().to_string());
    WRITTEN.with(|w| {
        let mut w = w.borrow_mut();
        if !w.contains(&rel) {
            w.push(rel);
        }
    });
}

pub fn drain_written() -> Vec<String> {
    WRITTEN.with(|w| std::mem::take(&mut *w.borrow_mut()))
}

pub fn write_event(store: &Store, ev: &mut Event, dry_run: bool) -> Result<PathBuf> {
    ev.id = compute_id(ev)?;
    let path = store.root.join(ev.rel_path());
    if dry_run {
        return Ok(path);
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    if path.exists() {
        return Ok(path); // identical content already recorded
    }
    let text = to_yaml(ev)?;
    std::fs::write(&path, text).with_context(|| format!("writing {}", path.display()))?;
    note_written(store, &path);
    Ok(path)
}

/// YAML with the envelope fields first, in a fixed order, then the payload.
pub fn to_yaml(ev: &Event) -> Result<String> {
    let mut m = serde_yaml::Mapping::new();
    let put = |m: &mut serde_yaml::Mapping, k: &str, v: Value| {
        m.insert(Value::String(k.into()), v);
    };
    put(&mut m, "schema", Value::String(ev.schema.clone()));
    put(&mut m, "id", Value::String(ev.id.clone()));
    put(&mut m, "at", Value::String(ev.at.clone()));
    put(&mut m, "store", Value::String(ev.store.clone()));
    put(&mut m, "act", Value::String(ev.act.clone()));
    if let Some(a) = &ev.about {
        put(&mut m, "about", Value::String(a.clone()));
    }
    if let Some(a) = &ev.atom {
        put(&mut m, "atom", Value::String(a.clone()));
    }
    put(&mut m, "actor", serde_yaml::to_value(&ev.actor)?);
    put(&mut m, "normalizer", Value::String(ev.normalizer.clone()));
    for (k, v) in &ev.payload {
        put(&mut m, k, v.clone());
    }
    Ok(serde_yaml::to_string(&Value::Mapping(m))?)
}

pub struct Trail {
    pub events: Vec<Event>,
    /// (path, problem)
    pub problems: Vec<(String, String)>,
}

pub fn read_all(store: &Store) -> Trail {
    let mut events = vec![];
    let mut problems = vec![];
    let dir = store.events_dir();
    let pattern = dir.join("**").join("*.yaml");
    if let Some(p) = pattern.to_str() {
        if let Ok(paths) = glob::glob(p) {
            for path in paths.flatten() {
                let rel = path
                    .strip_prefix(&store.root)
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|_| path.display().to_string());
                let text = match std::fs::read_to_string(&path) {
                    Ok(t) => t,
                    Err(e) => {
                        problems.push((rel, format!("unreadable: {e}")));
                        continue;
                    }
                };
                let ev: Event = match serde_yaml::from_str(&text) {
                    Ok(e) => e,
                    Err(e) => {
                        problems.push((rel, format!("not an event: {e}")));
                        continue;
                    }
                };
                match compute_id(&ev) {
                    Ok(id) if id == ev.id => {}
                    Ok(id) => problems.push((
                        rel.clone(),
                        format!("edited: content hashes to {} not {}", norm::short(&id), norm::short(&ev.id)),
                    )),
                    Err(e) => problems.push((rel.clone(), format!("cannot hash: {e}"))),
                }
                let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
                if stem != ev.id {
                    problems.push((rel.clone(), "file name disagrees with the event id".into()));
                }
                events.push(ev);
            }
        }
    }
    events.sort_by(|a, b| a.at.cmp(&b.at).then(a.id.cmp(&b.id)));
    Trail { events, problems }
}

// ---- objects ----

pub fn object_path(store: &Store, version: &str) -> PathBuf {
    store.objects_dir().join(&version[..2.min(version.len())]).join(version)
}

/// Keep a statement version as an object unless the durable ref (origin/main, by default) can
/// reproduce it. A version held only in the working tree, or only on a worktree branch that a
/// rebase will leave behind, is kept; `gc` drops it once the durable ref carries it. Returns the
/// version either way.
pub fn write_object(store: &Store, rel: &str, kind: &str, form: &str, text: &str, dry_run: bool) -> Result<String> {
    let canon = norm::canonical_object(kind, form, text);
    let version = norm::sha256_hex(canon.as_bytes());
    if durable_holds(store, rel, kind, &version)? {
        return Ok(version);
    }
    let path = object_path(store, &version);
    if !dry_run && !path.exists() {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&path, canon)?;
        note_written(store, &path);
    }
    Ok(version)
}

/// Does the durable ref carry this version of the record? Fast path: the file at the durable tip
/// (and at HEAD, when HEAD is on it); slow path: every commit on the durable ref that touched the
/// record's paths.
fn durable_holds(store: &Store, rel: &str, kind: &str, version: &str) -> Result<bool> {
    let d = store.durable_ref();
    let version_of = |content: &str| -> String {
        let (fm, body) = crate::record::split_frontmatter(content);
        let f = fm.and_then(|f| crate::record::parse_frontmatter(&f).ok()).and_then(|m| m.get("form").and_then(|v| v.as_str()).map(|s| s.to_string())).unwrap_or_default();
        norm::own_version(kind, &f, &crate::record::statement_text(&body))
    };
    if let Ok(c) = store.file_at(&d, rel) {
        if version_of(&c) == version {
            return Ok(true);
        }
    }
    let slug = std::path::Path::new(rel).file_stem().and_then(|s| s.to_str()).unwrap_or("");
    let rid = format!("{kind}:{slug}");
    Ok(find_in_git_in(store, &rid, version, &d)?.is_some())
}

/// The canonical object for a version: from `.vsect/objects/` if kept there, else from git — the
/// record's file at each commit that touched it, normalized, until one hashes to the version.
pub fn read_object(store: &Store, version: &str, record: Option<&str>) -> Result<String> {
    let path = object_path(store, version);
    if let Ok(c) = std::fs::read_to_string(&path) {
        return Ok(c);
    }
    if let Some(rid) = record {
        if let Some(c) = find_in_git(store, rid, version)? {
            return Ok(c);
        }
    }
    bail!("no text for version {}: not in .vsect/objects/ and not reproducible from git{}", norm::short(version), record.map(|r| format!(" for {r}")).unwrap_or_default())
}

/// Search every ref for the record's file content whose normalized version is `version`.
pub fn find_in_git(store: &Store, record_id: &str, version: &str) -> Result<Option<String>> {
    find_in_git_in(store, record_id, version, "--all")
}

/// Search the commits reachable from `refspec` (a ref, or `--all`) that touched the record's
/// paths for the content whose normalized version is `version`.
pub fn find_in_git_in(store: &Store, record_id: &str, version: &str, refspec: &str) -> Result<Option<String>> {
    let (kind, slug) = record_id.split_once(':').ok_or_else(|| anyhow!("{record_id} is not kind:slug"))?;
    let pathspecs = store.pathspecs_for(kind, slug);
    if pathspecs.is_empty() {
        return Ok(None);
    }
    let mut args = vec!["log", "--format=%H"];
    args.push(refspec);
    args.push("--");
    let specs: Vec<&str> = pathspecs.iter().map(|s| s.as_str()).collect();
    args.extend(specs.iter());
    let out = match crate::store::git(&store.root, &args) {
        Ok(o) => o,
        Err(_) => return Ok(None),
    };
    for commit in out.lines() {
        for spec in &pathspecs {
            let Ok(content) = store.file_at(commit, spec) else { continue };
            let (fm, body) = crate::record::split_frontmatter(&content);
            let form = fm.and_then(|f| crate::record::parse_frontmatter(&f).ok()).and_then(|m| m.get("form").and_then(|v| v.as_str()).map(|s| s.to_string())).unwrap_or_default();
            let canon = norm::canonical_object(kind, &form, &crate::record::statement_text(&body));
            if norm::sha256_hex(canon.as_bytes()) == version {
                return Ok(Some(canon));
            }
        }
    }
    Ok(None)
}

/// Where a cited version can be found: kept as an object, on the durable ref, on some other ref
/// only (a worktree branch a rebase may leave behind), or nowhere.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Anchor {
    Object,
    Durable,
    ElsewhereOnly,
    Lost,
}

pub fn anchor_of(store: &Store, record: &str, version: &str) -> Result<Anchor> {
    if object_path(store, version).exists() {
        return Ok(Anchor::Object);
    }
    let d = store.durable_ref();
    if find_in_git_in(store, record, version, &d)?.is_some() {
        return Ok(Anchor::Durable);
    }
    if find_in_git(store, record, version)?.is_some() {
        return Ok(Anchor::ElsewhereOnly);
    }
    Ok(Anchor::Lost)
}

/// The normalized text part of a stored object.
pub fn object_text(canon: &str) -> &str {
    match canon.find("\n---\n") {
        Some(i) => &canon[i + 5..],
        None => canon,
    }
}

/// Every version the trail cites, with the record it belongs to (when the event says).
pub fn cited_versions(trail: &Trail) -> BTreeMap<String, Option<String>> {
    let mut out: BTreeMap<String, Option<String>> = BTreeMap::new();
    for ev in &trail.events {
        let rec = ev.about.as_ref().map(|a| a.split('#').next().unwrap_or(a).to_string());
        for k in ["own-version", "target-version", "from-version", "to-version"] {
            if let Some(v) = ev.str(k) {
                out.entry(v.to_string()).or_insert_with(|| rec.clone());
            }
        }
        if let Some(Value::Mapping(m)) = ev.get("basis") {
            for (k, v) in m {
                if let (Some(k), Some(v)) = (k.as_str(), v.as_str()) {
                    out.entry(v.to_string()).or_insert_with(|| Some(k.to_string()));
                }
            }
        }
    }
    out
}

/// Drop every object the durable ref can reproduce, or that no event cites. A version held only
/// on a worktree branch stays kept: a rebase may leave that branch behind and `git gc` take it.
/// Returns (removed, kept, kept-uncited).
pub fn gc_objects(store: &Store, trail: &Trail, dry_run: bool) -> Result<(usize, usize, Vec<String>)> {
    let cited = cited_versions(trail);
    let mut removed = 0;
    let mut kept = 0;
    let mut kept_uncited = vec![];
    let pattern = store.objects_dir().join("*").join("*");
    let Some(p) = pattern.to_str() else { return Ok((0, 0, vec![])) };
    for path in glob::glob(p)?.flatten() {
        let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("").to_string();
        let drop = match cited.get(&name) {
            None => true, // nothing points at it
            Some(Some(rec)) => find_in_git_in(store, rec, &name, &store.durable_ref())?.is_some(),
            Some(None) => false,
        };
        if drop {
            if !dry_run {
                std::fs::remove_file(&path)?;
                if let Some(parent) = path.parent() {
                    let _ = std::fs::remove_dir(parent); // only if empty
                }
            }
            removed += 1;
        } else {
            kept += 1;
            if cited.get(&name) == Some(&None) {
                kept_uncited.push(name);
            }
        }
    }
    Ok((removed, kept, kept_uncited))
}

pub fn check_objects(store: &Store) -> Vec<(String, String)> {
    let mut problems = vec![];
    let pattern = store.objects_dir().join("*").join("*");
    if let Some(p) = pattern.to_str() {
        if let Ok(paths) = glob::glob(p) {
            for path in paths.flatten() {
                let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("").to_string();
                let rel = path
                    .strip_prefix(&store.root)
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_default();
                match std::fs::read(&path) {
                    Ok(bytes) => {
                        if norm::sha256_hex(&bytes) != name {
                            problems.push((rel, "edited: content no longer hashes to its name".into()));
                        }
                    }
                    Err(e) => problems.push((rel, format!("unreadable: {e}"))),
                }
            }
        }
    }
    problems
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ev(actor_yaml: &str) -> Event {
        let y = format!("schema: vsect-event/0\nid: ''\nat: 2026-11-02T00:00:00Z\nstore: t\nact: rederive\nactor:\n{actor_yaml}normalizer: vsect-norm/0\noutcome: confirmed\n");
        serde_yaml::from_str(&y).unwrap()
    }

    #[test]
    fn events_without_identity_fields_hash_as_before() {
        // the id of an event written before 2026-11-02 (name/role only) must not move
        let e = ev("  name: A\n  role: verifier\n");
        let json = serde_json::to_string(&serde_json::to_value(&e.actor).unwrap()).unwrap();
        assert_eq!(json, r#"{"name":"A","role":"verifier"}"#);
    }

    #[test]
    fn unknown_actor_fields_survive_a_round_trip_and_count_in_the_id() {
        let with = ev("  name: A\n  agent: s/a1\n  future-field: x\n");
        let without = ev("  name: A\n  agent: s/a1\n");
        assert_eq!(with.actor.extra.get("future-field").and_then(|v| v.as_str()), Some("x"));
        let again: Event = serde_yaml::from_str(&serde_yaml::to_string(&with).unwrap()).unwrap();
        assert_eq!(compute_id(&with).unwrap(), compute_id(&again).unwrap());
        assert_ne!(compute_id(&with).unwrap(), compute_id(&without).unwrap());
    }

    #[test]
    fn cc_agent_is_parsed_and_needs_the_hook_marker() {
        let h = harness_identity_from(Some("s1/a2b3 parent=s1 type=general-purpose depth=1 src=agent-identity-hook/2"), Some("s1"));
        assert_eq!(h.identity, "harness-agent");
        assert_eq!(h.agent.as_deref(), Some("s1/a2b3"));
        assert_eq!(h.session.as_deref(), Some("s1"));
        assert_eq!(h.parent.as_deref(), Some("s1"));
        assert_eq!(h.agent_type.as_deref(), Some("general-purpose"));
        assert_eq!(h.default_name().as_deref(), Some("general-purpose a2b3"));
        let main = harness_identity_from(Some("0903e35e-b03e depth=0 src=agent-identity-hook/2"), None);
        assert_eq!(main.default_name().as_deref(), Some("main 0903e35e"));
        assert_eq!(main.parent, None);
        assert_eq!(harness_identity_from(Some("s1/a2b3 type=x"), Some("s1")).identity, "harness-session");
        assert_eq!(harness_identity_from(None, None).identity, "declared");
    }

    #[test]
    fn same_mind_prefers_agent_keys_and_falls_back_to_names() {
        let a = |name: &str, agent: Option<&str>| Actor { name: name.into(), agent: agent.map(|s| s.into()), ..Default::default() };
        assert!(a("Lane A", Some("s/1")).same_mind(&a("Lane B", Some("s/1"))));
        assert!(!a("Lane A", Some("s/1")).same_mind(&a("Lane A", Some("s/2"))));
        assert!(a("Lane A", None).same_mind(&a("Lane A", Some("s/2"))));
        assert!(!a("Lane A", None).same_mind(&a("Lane B", None)));
    }
}
