//! Caller-stack config. Not a file in the project.
//!
//! defaults < global < user-home < agent-type < env < flags

use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

/// User-home file name (under `$XDG_CONFIG_HOME/aspectus/`).
pub const USER_HOME_FILENAME: &str = "aspectus.toml";
/// Directory under the XDG config home.
pub const USER_HOME_DIRNAME: &str = "aspectus";
/// Machine layer. Empty until we install that way.
pub const GLOBAL_PATH: &str = "/etc/aspectus/aspectus.toml";
/// Flag a tool-description can pass. Value is a key, not a taxonomy.
pub const CALLER_FLAG: &str = "--caller";

const DEFAULT_LINES: u32 = 80;
const DEFAULT_DEPTH: u32 = 2;
const DEFAULT_WALK: u64 = 10_000;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Layer {
    pub name: &'static str,
    pub path: Option<PathBuf>,
    pub existed: bool,
    pub values: BTreeMap<String, String>,
}

#[derive(Clone, Debug)]
pub struct Resolved {
    pub layers: Vec<Layer>,
    pub won: BTreeMap<String, (String, &'static str)>,
}

pub fn xdg_config_home() -> PathBuf {
    if let Ok(p) = env::var("XDG_CONFIG_HOME")
        && !p.is_empty() {
            return PathBuf::from(p);
        }
    let home = env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".config")
}

pub fn user_home_path() -> PathBuf {
    xdg_config_home()
        .join(USER_HOME_DIRNAME)
        .join(USER_HOME_FILENAME)
}

pub fn agent_type_path(caller: &str) -> PathBuf {
    xdg_config_home()
        .join(USER_HOME_DIRNAME)
        .join(format!("caller-{caller}.toml"))
}

pub fn defaults() -> BTreeMap<String, String> {
    let mut m = BTreeMap::new();
    m.insert("lines".into(), DEFAULT_LINES.to_string());
    m.insert("depth".into(), DEFAULT_DEPTH.to_string());
    m.insert("walk".into(), DEFAULT_WALK.to_string());
    // Sort (design/sort.md): recency = mtime, newest first, dirs first.
    m.insert("sort".into(), "recency".into());
    m.insert("dotfiles-first".into(), "off".into());
    // Lattice defaults for the built column facts (design/aspect-lattice.md):
    // quiet = only when it surprises; until the quiet law lands (its own
    // wave), quiet facts render nothing unless asked (`on`).
    m.insert("columns.size".into(), "quiet".into());
    m.insert("columns.mtime".into(), "quiet".into());
    m.insert("format.size".into(), "human".into());
    m.insert("format.mtime".into(), "iso-8601".into());
    m
}

pub fn parse_file(text: &str) -> BTreeMap<String, String> {
    let mut m = BTreeMap::new();
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with('[') {
            continue;
        }
        let Some((k, v)) = line.split_once('=') else {
            continue;
        };
        let k = k.trim();
        let v = v.trim().trim_matches('"').trim_matches('\'');
        if !k.is_empty() {
            m.insert(k.to_string(), v.to_string());
        }
    }
    m
}

fn load_path(path: &Path) -> (bool, BTreeMap<String, String>) {
    match fs::read_to_string(path) {
        Ok(s) => (true, parse_file(&s)),
        Err(_) => (false, BTreeMap::new()),
    }
}

pub fn env_values() -> BTreeMap<String, String> {
    let mut m = BTreeMap::new();
    if let Ok(v) = env::var("ASPECTUS_LINES")
        && !v.is_empty() {
            m.insert("lines".into(), v);
        }
    if let Ok(v) = env::var("ASPECTUS_DEPTH")
        && !v.is_empty() {
            m.insert("depth".into(), v);
        }
    if let Ok(v) = env::var("ASPECTUS_WALK")
        && !v.is_empty() {
            m.insert("walk".into(), v);
        }
    if let Ok(v) = env::var("ASPECTUS_FURNITURE")
        && !v.is_empty() {
            m.insert("furniture".into(), v);
        }
    // Same keys the config files speak, one env spelling each.
    for (var, key) in [
        ("ASPECTUS_SORT", "sort"),
        ("ASPECTUS_DOTFILES_FIRST", "dotfiles-first"),
        ("ASPECTUS_COLUMNS_SIZE", "columns.size"),
        ("ASPECTUS_COLUMNS_MTIME", "columns.mtime"),
        ("ASPECTUS_FORMAT_SIZE", "format.size"),
        ("ASPECTUS_FORMAT_MTIME", "format.mtime"),
    ] {
        if let Ok(v) = env::var(var)
            && !v.is_empty() {
                m.insert(key.into(), v);
            }
    }
    m
}

/// `user_home_override` is `--config=PATH` (substitutes for user-home).
/// `caller` is `--caller=KEY`.
/// `flag_vals` is argv (`--depth`, …).
pub fn resolve(
    user_home_override: Option<&Path>,
    caller: Option<&str>,
    flag_vals: BTreeMap<String, String>,
) -> Resolved {
    let mut layers = Vec::new();

    layers.push(Layer {
        name: "defaults",
        path: None,
        existed: true,
        values: defaults(),
    });

    let (g_ok, g_vals) = load_path(Path::new(GLOBAL_PATH));
    layers.push(Layer {
        name: "global",
        path: Some(PathBuf::from(GLOBAL_PATH)),
        existed: g_ok,
        values: g_vals,
    });

    let user_path = user_home_override
        .map(Path::to_path_buf)
        .unwrap_or_else(user_home_path);
    let (u_ok, u_vals) = load_path(&user_path);
    layers.push(Layer {
        name: "user-home",
        path: Some(user_path),
        existed: u_ok,
        values: u_vals,
    });

    match caller {
        Some(key) => {
            let p = agent_type_path(key);
            let (ok, vals) = load_path(&p);
            layers.push(Layer {
                name: "agent-type",
                path: Some(p),
                existed: ok,
                values: vals,
            });
        }
        None => layers.push(Layer {
            name: "agent-type",
            path: None,
            existed: false,
            values: BTreeMap::new(),
        }),
    }

    layers.push(Layer {
        name: "env",
        path: None,
        existed: !env_values().is_empty(),
        values: env_values(),
    });

    layers.push(Layer {
        name: "flags",
        path: None,
        existed: !flag_vals.is_empty(),
        values: flag_vals,
    });

    let mut won = BTreeMap::new();
    for layer in &layers {
        for (k, v) in &layer.values {
            won.insert(k.clone(), (v.clone(), layer.name));
        }
    }

    Resolved { layers, won }
}

pub fn render_show(res: &Resolved) -> String {
    let mut out = String::from("aspectus config\n\n");
    out.push_str("layer        status    source\n");
    for layer in &res.layers {
        let status = if layer.existed { "present" } else { "absent" };
        let source = match (&layer.path, layer.name, layer.existed) {
            (Some(p), _, _) => p.display().to_string(),
            (None, "defaults", _) => "built-in".into(),
            (None, "env", _) => "ASPECTUS_*".into(),
            (None, "flags", _) => "argv".into(),
            (None, "agent-type", _) => "--caller not set".into(),
            (None, _, _) => "—".into(),
        };
        out.push_str(&format!(
            "{:<12} {:<9} {}\n",
            layer.name, status, source
        ));
    }
    out.push_str("\nwon:\n");
    for (k, (v, from)) in &res.won {
        out.push_str(&format!("  {k} = {v}  ({from})\n"));
    }
    out
}
