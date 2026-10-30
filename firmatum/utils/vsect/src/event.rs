//! Events (one file per act, content-addressed) and objects (one file per version seen).

use crate::norm;
use crate::store::Store;
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::collections::BTreeMap;
use std::path::PathBuf;

pub const SCHEMA: &str = "vsect-event/0";

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Actor {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub read: Vec<String>,
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

/// Store the canonical serialization of a statement version; returns the version.
pub fn write_object(store: &Store, kind: &str, form: &str, text: &str, dry_run: bool) -> Result<String> {
    let canon = norm::canonical_object(kind, form, text);
    let version = norm::sha256_hex(canon.as_bytes());
    let path = object_path(store, &version);
    if !dry_run && !path.exists() {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&path, canon)?;
    }
    Ok(version)
}

pub fn read_object(store: &Store, version: &str) -> Result<String> {
    let path = object_path(store, version);
    std::fs::read_to_string(&path)
        .map_err(|_| anyhow!("no object for version {}: vsect never saw it", norm::short(version)))
}

/// The normalized text part of a stored object.
pub fn object_text(canon: &str) -> &str {
    match canon.find("\n---\n") {
        Some(i) => &canon[i + 5..],
        None => canon,
    }
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
