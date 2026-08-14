//! Furniture: a well-known name is state on its parent line, not a child.
//!
//! The positive concept is the parent line's **kind** gathering spot
//! (`[has: git, rust, …]`); hiding is the consequence, not the point
//! (src/def-furniture.md). Most furniture is a **map**: glob → name, and a
//! fate. A name the map does not know is just a child.
//!
//! Fates:
//! - `hide` — not listed as a child; its kind folds onto the parent line.
//! - `omit` — not listed, not mentioned (`.DS_Store`).
//! - `mark` — listed as an ordinary child, *and* its kind is claimed on the
//!   parent (`Cargo.toml` → `rust`). Mark rows are the Labels feed, not
//!   furniture proper (design/labels.md).
//!
//! Hidden and omitted names never join a census silently: they are not
//! children of the look, so they are not counted as children — the kind spot
//! is what says they are here (mass/census law, ASPECTUS.outline row Mass).

use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fate {
    Hide,
    Omit,
    Mark,
}

#[derive(Clone, Debug)]
pub struct Rule {
    /// Basename glob (`*`, `?`). Trailing `/` in the source restricts to dirs.
    pub pattern: String,
    pub dir_only: bool,
    /// The kinds this name claims. Empty claims nothing (pure omit).
    pub kinds: Vec<String>,
    pub fate: Fate,
}

#[derive(Clone, Debug)]
pub struct Map {
    /// First match wins. Config rules sit in front of the defaults.
    rules: Vec<Rule>,
}

/// How this invocation wants furniture treated.
#[derive(Clone, Debug, Default)]
pub struct View {
    /// `--show-all`: every mapped name is an ordinary child again.
    pub show_all: bool,
    /// `--inspect KIND`: names of these kinds are ordinary children again.
    pub inspect: Vec<String>,
}

impl View {
    /// Does this view neutralize hiding for a rule with these kinds?
    fn opened(&self, kinds: &[String]) -> bool {
        self.show_all
            || kinds
                .iter()
                .any(|k| self.inspect.iter().any(|i| i == k))
    }
}

fn rule(pattern: &str, kinds: &[&str], fate: Fate) -> Rule {
    let dir_only = pattern.ends_with('/');
    Rule {
        pattern: pattern.trim_end_matches('/').to_string(),
        dir_only,
        kinds: kinds.iter().map(|s| s.to_string()).collect(),
        fate,
    }
}

/// The shipped map. Rows reviewed against the first snapshot's table
/// (impl/absorb.md) and the origin's own kind list.
///
/// `target/` (also maven's) stays kind `build`, not `rust` — the name alone
/// claims build output; the ecosystem claim comes from `Cargo.toml`
/// (impl/furniture.md records the choice).
pub fn default_rules() -> Vec<Rule> {
    use Fate::*;
    vec![
        // Specialized plugins hang on these two (src/git.rs, src/github.rs).
        rule(".git", &["git"], Hide),
        rule(".github", &["github"], Hide),
        // The ignore-file family claims its own words, not `git` — a
        // `.gitignore` inside `.pytest_cache/` is not a repo, and one bad
        // `git` row poisoned the whole facet's credibility (both hallway
        // testers, 2026-08-14). Where a real `.git` also claims `git`,
        // these words fold away as subsumed (read_names).
        rule(".gitignore", &["gitignore"], Hide),
        rule(".gitmodules", &["gitmodules"], Hide),
        rule(".gitattributes", &["gitattributes"], Hide),
        // Build debris.
        rule("target/", &["build"], Hide),
        rule("node_modules/", &["build", "js"], Hide),
        rule("__pycache__/", &["build", "python"], Hide),
        rule("*.egg-info/", &["build", "python"], Hide),
        rule(".pytest_cache/", &["build", "python"], Hide),
        rule(".mypy_cache/", &["build", "python"], Hide),
        rule(".ruff_cache/", &["build", "python"], Hide),
        rule(".tox/", &["build", "python"], Hide),
        rule(".build/", &["build"], Hide),
        rule(".ruby-lsp/", &["build", "ruby"], Hide),
        // Recognized machinery of a place.
        rule(".obsidian/", &["obsidian-vault"], Hide),
        rule(".obsidian.vimrc", &["obsidian-vault"], Hide),
        rule(".claude/", &["agents"], Hide),
        rule(".mise.toml", &["mise"], Hide),
        rule(".archive/", &["archive"], Hide),
        rule(".trash/", &["trash"], Hide),
        // Noise: not listed, not mentioned.
        rule(".DS_Store", &[], Omit),
        // Mark rows: still children; their kind is claimed on the parent.
        rule("Cargo.toml", &["rust"], Mark),
        rule("Cargo.lock", &["rust"], Mark),
        rule("pyproject.toml", &["python"], Mark),
        rule("Gemfile", &["ruby"], Mark),
        rule("mise.toml", &["mise"], Mark),
        rule("package.json", &["js"], Mark),
        rule("AGENTS.md", &["agents"], Mark),
        rule("CLAUDE.md", &["agents"], Mark),
        rule("GEMINI.md", &["agents"], Mark),
    ]
}

impl Map {
    pub fn shipped() -> Self {
        Map {
            rules: default_rules(),
        }
    }

    /// Extend with config rules (`furniture` key on the caller stack).
    ///
    /// Comma-separated, each `PATTERN[:KINDS[:FATE]]` — kinds `+`-separated
    /// (`__pycache__/:build+python`), fate `hide` when absent — or
    /// `!PATTERN` to drop every rule with that exact pattern.
    /// Config rules match before the defaults, so a config row on a default
    /// name overrides it. Straight globs — no regex engine.
    pub fn with_config(config: &str) -> Self {
        let mut map = Map::shipped();
        let mut front = Vec::new();
        for item in config.split(',') {
            let item = item.trim();
            if item.is_empty() {
                continue;
            }
            if let Some(pat) = item.strip_prefix('!') {
                let pat = pat.trim_end_matches('/');
                map.rules.retain(|r| r.pattern != pat);
                front.retain(|r: &Rule| r.pattern != pat);
                continue;
            }
            let mut parts = item.splitn(3, ':');
            let pattern = parts.next().unwrap_or("");
            if pattern.is_empty() {
                continue;
            }
            let kinds: Vec<&str> = parts
                .next()
                .unwrap_or("")
                .split('+')
                .filter(|k| !k.is_empty())
                .collect();
            let fate = match parts.next() {
                Some("omit") => Fate::Omit,
                Some("mark") => Fate::Mark,
                _ => Fate::Hide,
            };
            front.push(rule(pattern, &kinds, fate));
        }
        front.append(&mut map.rules);
        Map { rules: front }
    }

    /// Every kind this map can claim, sorted — the `--inspect` refusal menu.
    pub fn known_kinds(&self) -> Vec<String> {
        let mut set = BTreeSet::new();
        for r in &self.rules {
            for k in &r.kinds {
                set.insert(k.clone());
            }
        }
        set.into_iter().collect()
    }

    /// First matching rule, or `None` — an unknown name is just a child.
    pub fn classify(&self, name: &str, is_dir: bool) -> Option<&Rule> {
        self.rules
            .iter()
            .find(|r| (!r.dir_only || is_dir) && glob_match(&r.pattern, name))
    }
}

/// What one directory's names told the map.
#[derive(Debug, Default)]
pub struct Reading {
    /// Kinds claimed on the parent line, sorted, deduplicated.
    pub kinds: Vec<String>,
    /// Names hidden or omitted from the child list.
    pub hidden: usize,
    /// Hidden *directories*, as (claiming kind, name) — the deep phase
    /// counts them so presence survives hiding (design/furniture.md,
    /// three testimonies 2026-08-14). Specialized kinds (git, github)
    /// carry their own facets and are not listed here.
    pub hidden_dirs: Vec<(String, String)>,
}

/// Kinds whose presence is already spoken by a specialized facet — their
/// hidden mass would be noise (`.git`'s object store is not the repo's
/// working weight).
fn speaks_for_itself(kind: &str) -> bool {
    kind == "git" || kind == "github"
}

/// Fold a directory's names through the map. Returns the reading; the
/// caller drops names for which `keep` came back false.
pub fn read_names<'a, I>(map: &Map, view: &View, names: I) -> (Reading, Vec<bool>)
where
    I: IntoIterator<Item = (&'a str, bool)>,
{
    let mut kinds = BTreeSet::new();
    let mut keep = Vec::new();
    let mut hidden = 0usize;
    let mut hidden_dirs = Vec::new();
    for (name, is_dir) in names {
        match map.classify(name, is_dir) {
            None => keep.push(true),
            Some(r) => {
                for k in &r.kinds {
                    kinds.insert(k.clone());
                }
                let listed = r.fate == Fate::Mark || view.opened(&r.kinds);
                if !listed {
                    hidden += 1;
                    if is_dir
                        && let Some(k) = r.kinds.iter().find(|k| !speaks_for_itself(k))
                    {
                        hidden_dirs.push((k.clone(), name.to_string()));
                    }
                }
                keep.push(listed);
            }
        }
    }
    // A real `.git` subsumes the ignore-file family's words — the repo
    // claim covers its own config files; `[has: git, gitignore]` at every
    // repo root would be noise.
    if kinds.contains("git") {
        for k in ["gitignore", "gitmodules", "gitattributes"] {
            kinds.remove(k);
        }
    }
    (
        Reading {
            kinds: kinds.into_iter().collect(),
            hidden,
            hidden_dirs,
        },
        keep,
    )
}

/// Basename glob: `*` any span, `?` one char. No `/`, no regex.
pub fn glob_match(pattern: &str, name: &str) -> bool {
    fn inner(p: &[u8], n: &[u8]) -> bool {
        match (p.first(), n.first()) {
            (None, None) => true,
            (Some(b'*'), _) => inner(&p[1..], n) || (!n.is_empty() && inner(p, &n[1..])),
            (Some(b'?'), Some(_)) => inner(&p[1..], &n[1..]),
            (Some(a), Some(b)) if a == b => inner(&p[1..], &n[1..]),
            _ => false,
        }
    }
    inner(pattern.as_bytes(), name.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn glob_basics() {
        assert!(glob_match("*.bak", "x.bak"));
        assert!(glob_match("target", "target"));
        assert!(!glob_match("target", "targets"));
        assert!(glob_match("?ar", "car"));
    }

    #[test]
    fn dir_only_rules_skip_files() {
        let m = Map::shipped();
        assert!(m.classify("target", true).is_some());
        assert!(m.classify("target", false).is_none());
    }

    #[test]
    fn config_overrides_and_removes() {
        let m = Map::with_config(".mystery/:lab, !target/");
        assert!(m.classify("target", true).is_none());
        let r = m.classify(".mystery", true).unwrap();
        assert_eq!(r.kinds, vec!["lab".to_string()]);
        assert_eq!(r.fate, Fate::Hide);
    }
}
