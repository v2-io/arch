//! The store: where it is, what it declares, and the moment it is read at.

use anyhow::{anyhow, bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

/// What one record kind declares in `.vsect/kinds.yaml`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KindDecl {
    /// `record` (single-statement) or `container`. Only `record` is handled now.
    #[serde(default = "default_grain")]
    pub grain: String,
    /// Globs, relative to the store root, that find this kind's records.
    #[serde(default)]
    pub find: Vec<String>,
    /// Globs to exclude (e.g. `*/src/old-*.md`).
    #[serde(default)]
    pub exclude: Vec<String>,
    /// Alternate names accepted on input and in links.
    #[serde(default)]
    pub aliases: Vec<String>,
    /// Frontmatter fields that are edges, with the edge name they declare.
    /// e.g. `depends: premise` means `depends:` lists premise targets.
    #[serde(default)]
    pub edges: BTreeMap<String, String>,
    /// One sentence a reader sees in `vsect kinds`.
    #[serde(default)]
    pub note: String,
}

fn default_grain() -> String {
    "record".into()
}

/// What an edge means, declared in `.vsect/kinds.yaml` under `edges:`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EdgeDecl {
    /// Is the target part of the basis a check is bound to?
    #[serde(default)]
    pub in_basis: bool,
    /// Does a change of meaning at the target lapse checks here, transitively?
    #[serde(default)]
    pub meaning: bool,
    /// What happens here when the target is refuted: `broken`, `flag`, or `none`.
    #[serde(default = "default_on_refute")]
    pub on_refute: String,
    #[serde(default)]
    pub note: String,
}

fn default_on_refute() -> String {
    "flag".into()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kinds {
    pub name: String,
    #[serde(default)]
    pub steward: String,
    #[serde(default)]
    pub decider: String,
    #[serde(default)]
    pub stores: BTreeMap<String, String>,
    #[serde(default)]
    pub kinds: BTreeMap<String, KindDecl>,
    #[serde(default)]
    pub edges: BTreeMap<String, EdgeDecl>,
    /// Fields vsect accepts on records but never computes from (shown raw).
    #[serde(default)]
    pub declared_fields: Vec<String>,
    /// Lines evidence can be recorded on, with their rungs lowest first.
    #[serde(default)]
    pub lines: BTreeMap<String, Vec<String>>,
    /// Where the store says which not-yet-written records are planned, so an edge naming one is
    /// not dangling. e.g. ASF's OUTLINE `missing` rows.
    #[serde(default)]
    pub planned: Option<PlannedDecl>,
}

/// Table rows in the named files whose cells include `cell` name planned records by `#slug`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlannedDecl {
    #[serde(default)]
    pub find: Vec<String>,
    #[serde(default = "default_planned_cell")]
    pub cell: String,
}

fn default_planned_cell() -> String {
    "missing".into()
}

impl Store {
    /// The paths a record of this kind and slug could live at, from the kind's `find` globs with
    /// the file name fixed to `<slug>.md` (directories only organize, so every glob's directory
    /// is a candidate). Used to search git history for a record's past versions.
    pub fn pathspecs_for(&self, kind: &str, slug: &str) -> Vec<String> {
        let globs: Vec<String> = match self.kinds.kinds.get(kind) {
            Some(d) => d.find.clone(),
            None => {
                let dir = self.builtin_dir(kind);
                let rel = dir.strip_prefix(&self.root).map(|p| p.to_string_lossy().to_string()).unwrap_or_default();
                let mut g = vec![format!("{rel}/*.md")];
                if kind == "item" {
                    g.push(format!("{rel}/imported/*.md"));
                }
                g
            }
        };
        let mut out = vec![];
        for g in globs {
            let dir = match g.rsplit_once('/') {
                Some((d, _)) => d.to_string(),
                None => String::new(),
            };
            // expand a glob in the directory part against the working tree and git's HEAD tree
            let dir_pattern = if dir.is_empty() { ".".to_string() } else { dir.clone() };
            let mut dirs: Vec<String> = vec![];
            if dir_pattern.contains('*') {
                if let Some(full) = self.root.join(&dir_pattern).to_str() {
                    if let Ok(paths) = glob::glob(full) {
                        for p in paths.flatten() {
                            if let Ok(rel) = p.strip_prefix(&self.root) {
                                dirs.push(rel.to_string_lossy().to_string());
                            }
                        }
                    }
                }
            } else {
                dirs.push(dir);
            }
            for d in dirs {
                out.push(if d.is_empty() || d == "." { format!("{slug}.md") } else { format!("{d}/{slug}.md") });
            }
        }
        out.sort();
        out.dedup();
        out
    }

    /// Records whose working-tree content differs from HEAD, as paths relative to the root.
    pub fn dirty_paths(&self) -> std::collections::BTreeSet<String> {
        git(&self.root, &["status", "--porcelain", "--untracked-files=all"])
            .map(|s| s.lines().filter_map(|l| l.get(3..).map(|p| p.trim().to_string())).collect())
            .unwrap_or_default()
    }

    /// When a path first entered git, as `<short sha> <date>`, if it did.
    pub fn first_commit_of(&self, rel: &str) -> Option<String> {
        git(&self.root, &["log", "--diff-filter=A", "--format=%h %ad", "--date=short", "--follow", "--", rel])
            .ok()
            .and_then(|s| s.lines().last().map(|l| l.to_string()))
            .filter(|s| !s.is_empty())
    }

    /// Slugs the store declares as planned: `#slug` tokens on table rows carrying the planned cell.
    pub fn planned_slugs(&self) -> std::collections::BTreeSet<String> {
        let mut out = std::collections::BTreeSet::new();
        let Some(p) = &self.kinds.planned else { return out };
        let slug_re = regex::Regex::new(r"#([a-z0-9]+(?:-[a-z0-9]+)*)").unwrap();
        for g in &p.find {
            let Some(full) = self.root.join(g).to_str().map(|s| s.to_string()) else { continue };
            let Ok(paths) = glob::glob(&full) else { continue };
            for path in paths.flatten() {
                let Ok(text) = std::fs::read_to_string(&path) else { continue };
                for line in text.lines() {
                    if !line.trim_start().starts_with('|') {
                        continue;
                    }
                    let is_planned = line.split('|').any(|c| c.trim() == p.cell);
                    if !is_planned {
                        continue;
                    }
                    if let Some(c) = slug_re.captures(line) {
                        out.insert(c[1].to_string());
                    }
                }
            }
        }
        out
    }
}

impl Kinds {
    /// Resolve an alias or canonical name to the canonical kind name.
    pub fn canonical_kind(&self, name: &str) -> Option<String> {
        if self.kinds.contains_key(name) || BUILTIN_KINDS.contains(&name) {
            return Some(name.to_string());
        }
        for (k, d) in &self.kinds {
            if d.aliases.iter().any(|a| a == name) {
                return Some(k.clone());
            }
        }
        None
    }
}

/// Kinds vsect's own verbs write. They need no declaration.
pub const BUILTIN_KINDS: &[&str] = &["question", "decision", "purpose", "review", "item"];

pub struct Store {
    pub root: PathBuf,
    pub kinds: Kinds,
}

impl Store {
    /// The nearest directory at or above `from` that holds `.vsect/`.
    pub fn find(from: &Path) -> Result<Store> {
        let mut dir = from.canonicalize().unwrap_or_else(|_| from.to_path_buf());
        loop {
            if dir.join(".vsect").is_dir() {
                return Store::open(&dir);
            }
            if !dir.pop() {
                bail!(
                    "no store here: no `.vsect/` at or above {}. `vsect init` founds one.",
                    from.display()
                );
            }
        }
    }

    pub fn open(root: &Path) -> Result<Store> {
        let path = root.join(".vsect").join("kinds.yaml");
        let text = std::fs::read_to_string(&path)
            .with_context(|| format!("reading {}", path.display()))?;
        let kinds: Kinds =
            serde_yaml::from_str(&text).with_context(|| format!("parsing {}", path.display()))?;
        Ok(Store { root: root.to_path_buf(), kinds })
    }

    pub fn vsect_dir(&self) -> PathBuf {
        self.root.join(".vsect")
    }
    pub fn events_dir(&self) -> PathBuf {
        self.vsect_dir().join("events")
    }
    pub fn objects_dir(&self) -> PathBuf {
        self.vsect_dir().join("objects")
    }

    /// Where readable records of a built-in kind are written by vsect's verbs.
    pub fn builtin_dir(&self, kind: &str) -> PathBuf {
        if let Some(d) = self.kinds.kinds.get(kind) {
            if let Some(g) = d.find.first() {
                // first glob's directory part
                let p = Path::new(g);
                if let Some(parent) = p.parent() {
                    return self.root.join(parent);
                }
            }
        }
        match kind {
            "review" => self.root.join("reviews"),
            "question" => self.root.join("questions"),
            "item" => self.root.join("items"),
            "decision" => self.root.join("decisions"),
            "purpose" => self.root.join("purposes"),
            _ => self.root.join(kind),
        }
    }

    /// The moment: the HEAD commit, with `+dirty` when tracked files differ.
    pub fn moment(&self) -> String {
        let head = git(&self.root, &["rev-parse", "--short", "HEAD"])
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|_| "no-git".into());
        let dirty = git(&self.root, &["status", "--porcelain", "--untracked-files=no"])
            .map(|s| !s.trim().is_empty())
            .unwrap_or(false);
        if dirty {
            format!("{head}+dirty")
        } else {
            head
        }
    }

    /// A file's content at a git revision, by path relative to the repo root.
    pub fn file_at(&self, rev: &str, rel: &str) -> Result<String> {
        git(&self.root, &["show", &format!("{rev}:{rel}")])
            .map_err(|e| anyhow!("git show {rev}:{rel}: {e}"))
    }

    /// Every path tracked at a revision.
    pub fn tree_at(&self, rev: &str) -> Result<Vec<String>> {
        let out = git(&self.root, &["ls-tree", "-r", "--name-only", rev])?;
        Ok(out.lines().map(|s| s.to_string()).collect())
    }

    /// Whether a path is in git history (for deleted-trail detection).
    pub fn tracked_in_head(&self, rel: &str) -> bool {
        git(&self.root, &["cat-file", "-e", &format!("HEAD:{rel}")]).is_ok()
    }
}

pub fn git(root: &Path, args: &[&str]) -> Result<String> {
    let out = Command::new("git").arg("-C").arg(root).args(args).output()?;
    if !out.status.success() {
        bail!(String::from_utf8_lossy(&out.stderr).trim().to_string());
    }
    Ok(String::from_utf8_lossy(&out.stdout).to_string())
}

/// The `kinds.yaml` `init` writes: a name, a steward, a decider, and no kinds of the store's own.
pub fn founding_kinds(name: &str, steward: &str, decider: &str) -> String {
    let mut s = String::new();
    s.push_str("# vsect store declarations. `vsect init` wrote this with no kinds of the store's own:\n");
    s.push_str("# kinds are admitted when real records show they fail and are repaired differently\n");
    s.push_str("# from every other kind. vsect's own kinds (question, decision, purpose, review) are built in.\n");
    s.push_str(&format!("name: {name}\nsteward: {steward}\ndecider: {decider}\n"));
    s.push_str("stores: {}\nkinds: {}\nedges: {}\ndeclared_fields: []\n");
    s.push_str("lines:\n");
    s.push_str("  derivation: [drafted, premises-checked, independently-rederived]\n");
    s.push_str("  simulation: [prediction-committed, run-once, swept, reproduced-by-another]\n");
    s.push_str("  instance: [mapping-proposed, mapping-checked, prediction-checked]\n");
    s.push_str("  imported: [named, recalled, found-by-search, checked-at-secondary, checked-at-primary]\n");
    s.push_str("  measured: [anecdotal, measured-once, robust, calibrated]\n");
    s.push_str("  testimony: [captured, corroborated, corroborated-by-another-kind]\n");
    s
}
