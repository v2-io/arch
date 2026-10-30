//! Records: files with YAML frontmatter, found by the kinds' globs.

use crate::norm;
use crate::store::Store;
use anyhow::{anyhow, bail, Result};
use regex::Regex;
use serde_yaml::Value;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// `kind:slug` or `kind:slug#id`. Only the record grain is handled now, so `#id` is carried
/// but not resolved to a span.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id {
    pub kind: String,
    pub slug: String,
    pub anchor: Option<String>,
}

impl Id {
    pub fn parse(store: &Store, s: &str) -> Result<Id> {
        let (rest, anchor) = match s.split_once('#') {
            Some((a, b)) => (a, Some(b.to_string())),
            None => (s, None),
        };
        let (kind, slug) = rest
            .split_once(':')
            .ok_or_else(|| anyhow!("`{s}` is not an id: write it as kind:slug (e.g. segment:def-strategy-dag)"))?;
        let canon = store
            .kinds
            .canonical_kind(kind)
            .ok_or_else(|| anyhow!("`{kind}` is not a declared kind (see `vsect kinds`)"))?;
        if slug.is_empty() {
            bail!("`{s}` has no slug");
        }
        Ok(Id { kind: canon, slug: slug.to_string(), anchor })
    }
    pub fn record_id(&self) -> String {
        format!("{}:{}", self.kind, self.slug)
    }
    pub fn without_anchor(&self) -> Id {
        Id { kind: self.kind.clone(), slug: self.slug.clone(), anchor: None }
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.anchor {
            Some(a) => write!(f, "{}:{}#{}", self.kind, self.slug, a),
            None => write!(f, "{}:{}", self.kind, self.slug),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Record {
    pub id: Id,
    /// Path relative to the store root.
    pub rel: String,
    pub frontmatter: BTreeMap<String, Value>,
    /// The body after frontmatter, as written.
    pub body: String,
    /// The statement text: body minus Working Notes and generated marks.
    pub text: String,
    pub own_version: String,
    /// Edge name -> target ids as written (unresolved).
    pub edges: BTreeMap<String, Vec<String>>,
}

pub fn split_frontmatter(content: &str) -> (Option<String>, String) {
    let c = content.strip_prefix('\u{feff}').unwrap_or(content);
    if let Some(rest) = c.strip_prefix("---\n") {
        if let Some(end) = rest.find("\n---\n") {
            let fm = &rest[..end];
            let body = &rest[end + 5..];
            return (Some(fm.to_string()), body.to_string());
        }
        if let Some(fm) = rest.strip_suffix("\n---") {
            return (Some(fm.to_string()), String::new());
        }
    }
    (None, c.to_string())
}

pub fn parse_frontmatter(fm: &str) -> Result<BTreeMap<String, Value>> {
    let v: Value = serde_yaml::from_str(fm)?;
    match v {
        Value::Mapping(m) => Ok(m
            .into_iter()
            .filter_map(|(k, v)| k.as_str().map(|k| (k.to_string(), v)))
            .collect()),
        Value::Null => Ok(BTreeMap::new()),
        _ => bail!("frontmatter is not a mapping"),
    }
}

/// Strip the final `## Working Notes` section (any heading level 2 with that title) and marks.
pub fn statement_text(body: &str) -> String {
    let mut out = String::new();
    let mut in_wn = false;
    let mut in_mark = false;
    for line in body.lines() {
        let trimmed = line.trim_end();
        if trimmed.starts_with("## ") {
            let title = trimmed[3..].trim().to_lowercase();
            in_wn = title == "working notes";
            if in_wn {
                continue;
            }
        } else if in_wn && trimmed.starts_with("# ") {
            in_wn = false;
        }
        if in_wn {
            continue;
        }
        // generated marks: a block quote whose first line opens `> [!vsect `
        if trimmed.starts_with("> [!vsect ") {
            in_mark = true;
            continue;
        }
        if in_mark {
            if trimmed.starts_with('>') {
                continue;
            }
            in_mark = false;
        }
        out.push_str(line);
        out.push('\n');
    }
    out
}

fn yaml_list(v: &Value) -> Vec<String> {
    match v {
        Value::Sequence(s) => s
            .iter()
            .filter_map(|x| match x {
                Value::String(s) => Some(s.clone()),
                Value::Number(n) => Some(n.to_string()),
                _ => None,
            })
            .collect(),
        Value::String(s) => vec![s.clone()],
        _ => vec![],
    }
}

pub fn load_record(store: &Store, kind: &str, rel: &str, content: &str) -> Result<Record> {
    let (fm, body) = split_frontmatter(content);
    let fm = match fm {
        Some(f) => parse_frontmatter(&f).map_err(|e| anyhow!("{rel}: {e}"))?,
        None => BTreeMap::new(),
    };
    let stem = Path::new(rel)
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| anyhow!("{rel}: no file stem"))?
        .to_string();
    let slug = fm
        .get("slug")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| stem.clone());
    let decl = store.kinds.kinds.get(kind);
    let mut edges = BTreeMap::new();
    if let Some(d) = decl {
        for (field, edge) in &d.edges {
            if let Some(v) = fm.get(field) {
                edges.entry(edge.clone()).or_insert_with(Vec::new).extend(yaml_list(v));
            }
        }
    }
    // built-in record kinds carry core edges at top level
    for core in ["concerns", "blocks", "supersedes", "per", "about"] {
        if let Some(v) = fm.get(core) {
            edges.entry(core.to_string()).or_insert_with(Vec::new).extend(yaml_list(v));
        }
    }
    let text = statement_text(&body);
    let form = fm.get("form").and_then(|v| v.as_str()).unwrap_or("");
    let own_version = norm::own_version(kind, form, &text);
    Ok(Record {
        id: Id { kind: kind.to_string(), slug, anchor: None },
        rel: rel.to_string(),
        frontmatter: fm,
        body,
        text,
        own_version,
        edges,
    })
}

/// Expand a glob relative to the store root into relative paths.
fn expand(store: &Store, pattern: &str) -> Vec<String> {
    let full = store.root.join(pattern);
    let Some(s) = full.to_str() else { return vec![] };
    let mut out = vec![];
    if let Ok(paths) = glob::glob(s) {
        for p in paths.flatten() {
            if let Ok(rel) = p.strip_prefix(&store.root) {
                out.push(rel.to_string_lossy().to_string());
            }
        }
    }
    out
}

pub struct Scan {
    pub records: BTreeMap<String, Record>, // keyed by record id "kind:slug"
    /// Two files claiming one id.
    pub collisions: Vec<(String, Vec<String>)>,
    /// Files that could not be read as records, with the reason.
    pub unreadable: Vec<(String, String)>,
}

pub fn scan(store: &Store) -> Scan {
    let mut records: BTreeMap<String, Record> = BTreeMap::new();
    let mut seen: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut unreadable = vec![];
    let mut kinds: Vec<(String, Vec<String>, Vec<String>)> = store
        .kinds
        .kinds
        .iter()
        .map(|(k, d)| (k.clone(), d.find.clone(), d.exclude.clone()))
        .collect();
    for b in crate::store::BUILTIN_KINDS {
        if !store.kinds.kinds.contains_key(*b) {
            let dir = store.builtin_dir(b);
            if let Ok(rel) = dir.strip_prefix(&store.root).map(|p| p.to_string_lossy().to_string()) {
                let mut globs = vec![format!("{rel}/*.md")];
                if *b == "item" {
                    // imported items wait in their own directory until a mind re-checks them
                    globs.push(format!("{rel}/imported/*.md"));
                }
                kinds.push((b.to_string(), globs, vec![]));
            }
        }
    }
    for (kind, finds, excludes) in kinds {
        let excluded: std::collections::HashSet<String> =
            excludes.iter().flat_map(|e| expand(store, e)).collect();
        for f in &finds {
            for rel in expand(store, f) {
                if excluded.contains(&rel) {
                    continue;
                }
                let content = match std::fs::read_to_string(store.root.join(&rel)) {
                    Ok(c) => c,
                    Err(e) => {
                        unreadable.push((rel, e.to_string()));
                        continue;
                    }
                };
                match load_record(store, &kind, &rel, &content) {
                    Ok(r) => {
                        let id = r.id.record_id();
                        seen.entry(id.clone()).or_default().push(rel.clone());
                        records.entry(id).or_insert(r);
                    }
                    Err(e) => unreadable.push((rel, e.to_string())),
                }
            }
        }
    }
    let collisions = seen.into_iter().filter(|(_, v)| v.len() > 1).collect();
    Scan { records, collisions, unreadable }
}

/// Load a record of `kind` as it was at a git revision, by path.
pub fn record_at(store: &Store, kind: &str, rel: &str, rev: &str) -> Result<Record> {
    let content = store.file_at(rev, rel)?;
    load_record(store, kind, rel, &content)
}

/// Find, at a git revision, the path of the record with this id (by the kind's globs).
pub fn path_at(store: &Store, id: &Id, rev: &str) -> Result<Option<String>> {
    let decl = store.kinds.kinds.get(&id.kind);
    let finds: Vec<String> = match decl {
        Some(d) => d.find.clone(),
        None => {
            let dir = store.builtin_dir(&id.kind);
            let rel = dir.strip_prefix(&store.root).map(|p| p.to_string_lossy().to_string());
            rel.map(|r| vec![format!("{r}/*.md")]).unwrap_or_default()
        }
    };
    let tree = store.tree_at(rev)?;
    let patterns: Vec<glob::Pattern> =
        finds.iter().filter_map(|f| glob::Pattern::new(f).ok()).collect();
    for p in tree {
        if patterns.iter().any(|pat| pat.matches(&p)) {
            let stem = Path::new(&p).file_stem().and_then(|s| s.to_str()).unwrap_or("");
            if stem == id.slug {
                return Ok(Some(p));
            }
        }
    }
    Ok(None)
}

pub fn slug_re() -> Regex {
    Regex::new(r"^[a-z0-9]+(?:-[a-z0-9]+)*$").unwrap()
}

pub fn abs(store: &Store, rel: &str) -> PathBuf {
    store.root.join(rel)
}
