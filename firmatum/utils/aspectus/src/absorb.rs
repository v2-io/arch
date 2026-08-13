//! Four fates of a name, as data. Partition is recognition, not a display toggle.

/// What a directory entry *is* to the snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fate {
    /// Never listed as a child; becomes parent state (`kind`).
    Absorb { kind: &'static str },
    /// Listed if it survives the budget; may contribute a parent kind.
    Witness { kind: Option<&'static str> },
    /// Neither child nor kind.
    Omit,
    /// Ordinary child; optional role tag (archive, trash, …).
    Child { role: Option<&'static str> },
}

#[derive(Debug, Clone, Copy)]
enum Match {
    Exact,
    /// Name starts with this stem (`README`, `README.md`, `README-auditor.md`).
    Prefix,
}

struct Rule {
    name: &'static str,
    how: Match,
    fate: Fate,
}

/// Default mapping. First match wins. Unknown names — including unknown
/// hidden names — are `Child`. Estate overlays do not belong here.
static RULES: &[Rule] = &[
    Rule { name: ".DS_Store", how: Match::Exact, fate: Fate::Omit },
    Rule { name: ".git", how: Match::Exact, fate: Fate::Absorb { kind: "git" } },
    Rule { name: ".gitignore", how: Match::Exact, fate: Fate::Absorb { kind: "git" } },
    Rule { name: ".gitmodules", how: Match::Exact, fate: Fate::Absorb { kind: "git" } },
    Rule { name: ".gitattributes", how: Match::Exact, fate: Fate::Absorb { kind: "git" } },
    Rule { name: ".github", how: Match::Exact, fate: Fate::Absorb { kind: "git" } },
    Rule { name: ".obsidian", how: Match::Exact, fate: Fate::Absorb { kind: "obsidian-vault" } },
    Rule { name: ".obsidian.vimrc", how: Match::Exact, fate: Fate::Absorb { kind: "obsidian-vault" } },
    Rule { name: ".claude", how: Match::Exact, fate: Fate::Absorb { kind: "agents" } },
    Rule { name: "target", how: Match::Exact, fate: Fate::Absorb { kind: "build" } },
    Rule { name: "node_modules", how: Match::Exact, fate: Fate::Absorb { kind: "build" } },
    Rule { name: "__pycache__", how: Match::Exact, fate: Fate::Absorb { kind: "build" } },
    Rule { name: ".build", how: Match::Exact, fate: Fate::Absorb { kind: "build" } },
    Rule { name: ".ruby-lsp", how: Match::Exact, fate: Fate::Absorb { kind: "build" } },
    Rule { name: ".mise.toml", how: Match::Exact, fate: Fate::Absorb { kind: "mise" } },
    Rule { name: "Cargo.toml", how: Match::Exact, fate: Fate::Witness { kind: Some("rust") } },
    Rule { name: "Cargo.lock", how: Match::Exact, fate: Fate::Witness { kind: Some("rust") } },
    Rule { name: "pyproject.toml", how: Match::Exact, fate: Fate::Witness { kind: Some("python") } },
    Rule { name: "Gemfile", how: Match::Exact, fate: Fate::Witness { kind: Some("ruby") } },
    Rule { name: "mise.toml", how: Match::Exact, fate: Fate::Witness { kind: Some("mise") } },
    Rule { name: "package.json", how: Match::Exact, fate: Fate::Witness { kind: Some("js") } },
    Rule { name: "AGENTS.md", how: Match::Exact, fate: Fate::Witness { kind: Some("agents") } },
    Rule { name: "CLAUDE.md", how: Match::Exact, fate: Fate::Witness { kind: Some("agents") } },
    Rule { name: "GEMINI.md", how: Match::Exact, fate: Fate::Witness { kind: Some("agents") } },
    Rule { name: "README", how: Match::Prefix, fate: Fate::Witness { kind: None } },
    Rule { name: ".archive", how: Match::Exact, fate: Fate::Child { role: Some("archive") } },
    Rule { name: ".super-archive", how: Match::Exact, fate: Fate::Child { role: Some("archive") } },
    Rule { name: ".trash", how: Match::Exact, fate: Fate::Child { role: Some("trash") } },
    Rule { name: "archive", how: Match::Exact, fate: Fate::Child { role: Some("archive") } },
];

/// Classify one name. `is_dir` is reserved for future rules that distinguish
/// a `.git` directory from a gitlink file — both absorb as `git` today.
pub fn fate(name: &str, _is_dir: bool) -> Fate {
    for rule in RULES {
        let hit = match rule.how {
            Match::Exact => name == rule.name,
            Match::Prefix => name == rule.name || name.starts_with(&format!("{}.", rule.name)),
        };
        if hit {
            return rule.fate;
        }
    }
    Fate::Child { role: None }
}

/// Whether this fate is furniture we must not descend into (unless raw/inspect).
pub fn is_absorb(f: Fate) -> bool {
    matches!(f, Fate::Absorb { .. })
}

pub fn absorb_kind(f: Fate) -> Option<&'static str> {
    match f {
        Fate::Absorb { kind } => Some(kind),
        _ => None,
    }
}

/// Inspect filter: `None` means all absorbed names become children (`--raw`
/// / bare `--inspect`). `Some(k)` means only that kind is opened.
pub fn absorb_is_visible(f: Fate, inspect: Option<&str>) -> bool {
    match f {
        Fate::Absorb { kind } => match inspect {
            Some("*") => true,
            Some(k) => k == kind,
            None => false,
        },
        _ => false,
    }
}

#[derive(Debug, Clone)]
pub struct ChildSpec {
    pub name: String,
    pub is_dir: bool,
    pub role: Option<&'static str>,
    pub witness_kind: Option<&'static str>,
}

#[derive(Debug, Clone, Default)]
pub struct Partition {
    pub absorbed: Vec<(String, &'static str)>,
    pub omitted: Vec<String>,
    pub children: Vec<ChildSpec>,
}

/// Split a directory listing into the four fates.
/// `inspect`: `None` = default (absorb stays parent state); `Some("*")` =
/// `--raw`; `Some(kind)` = `--inspect kind`.
pub fn partition(
    entries: impl IntoIterator<Item = (String, bool)>,
    inspect: Option<&str>,
) -> Partition {
    let mut out = Partition::default();
    for (name, is_dir) in entries {
        match fate(&name, is_dir) {
            Fate::Omit => out.omitted.push(name),
            Fate::Absorb { kind } if !absorb_is_visible(Fate::Absorb { kind }, inspect) => {
                out.absorbed.push((name, kind));
            }
            Fate::Absorb { kind } => out.children.push(ChildSpec {
                name,
                is_dir,
                role: None,
                witness_kind: Some(kind),
            }),
            Fate::Witness { kind } => out.children.push(ChildSpec {
                name,
                is_dir,
                role: None,
                witness_kind: kind,
            }),
            Fate::Child { role } => out.children.push(ChildSpec {
                name,
                is_dir,
                role,
                witness_kind: None,
            }),
        }
    }
    out
}
