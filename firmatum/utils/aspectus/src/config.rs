//! Caller-stack config. Not a file in the project.
//!
//! defaults < global < user-home < agent-type < env < flags
//!
//! The `defaults` layer is `defaults.toml`, embedded at build
//! (`include_str!`) and parsed at startup (design/defaults.md).

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

/// User-home file name (under `$XDG_CONFIG_HOME/aspectus/`).
pub const USER_HOME_FILENAME: &str = "aspectus.toml";
/// Directory under the XDG config home.
pub const USER_HOME_DIRNAME: &str = "aspectus";
/// Machine layer. Empty until we install that way.
pub const GLOBAL_PATH: &str = "/etc/aspectus/aspectus.toml";
/// Flag a tool-description can pass. Value is a key, not a taxonomy.
pub const CALLER_FLAG: &str = "--caller";

/// The shipped file, verbatim. `aspectus config defaults` prints this.
pub const DEFAULTS_TOML: &str = include_str!("../defaults.toml");

/// Positions whose paint has not landed yet (design/lattice-2.md).
/// Far-left paints git-status (step 5); mtime/bytes compact forms in that
/// list are still unbuilt and named per-entry. Supplement has no tenant.
pub const UNBUILT_POSITIONS: &[&str] = &["supplement"];

/// Far-left facts this binary can paint. Listed members not in this set
/// show as `(unbuilt: …)` on `aspectus config`.
pub const FAR_LEFT_PAINTABLE: &[&str] = &["git-status"];

/// Layout list keys, in the order `aspectus config` prints them.
const LAYOUT_KEYS: &[&str] = &[
    "layout.far-left",
    "layout.near-right",
    "layout.supplement",
    "layout.far-right",
    "layout.quiet",
];

/// Fact slug in `[layout]` → the `columns.*` key today's renderer still
/// reads. `bytes` is the lattice name for what config still calls `size`.
const COLUMN_FACTS: &[(&str, &str)] = &[
    ("lines", "columns.line-count"),
    ("bytes", "columns.size"),
    ("mtime", "columns.mtime"),
    ("heat", "columns.heat"),
    ("permissions", "columns.permissions"),
    ("owner", "columns.owner"),
    ("filekind", "columns.filekind"),
    ("initial-sha", "columns.initial-sha"),
    ("latest-sha", "columns.latest-sha"),
];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Layer {
    pub name: &'static str,
    pub path: Option<PathBuf>,
    pub existed: bool,
    /// Scalar keys (dotted), including legacy comma-strings for
    /// `furniture` / `kinds` / `important`.
    pub values: BTreeMap<String, String>,
    /// Array keys (`layout.far-right`, `important`, …).
    pub arrays: BTreeMap<String, Vec<String>>,
    /// `[furniture]` table, file order. `"PATTERN" = "!"` is a drop.
    pub furniture: Vec<(String, String)>,
    /// `[kinds]` table, file order. `"SUFFIX" = "!"` is a drop.
    pub kinds: Vec<(String, String)>,
}

#[derive(Clone, Debug)]
pub struct Resolved {
    pub layers: Vec<Layer>,
    pub won: BTreeMap<String, (String, &'static str)>,
    /// Effective `[layout]` lists, each entry tagged with the layer that
    /// supplied the list it sits in.
    pub layout: BTreeMap<String, (Vec<String>, &'static str)>,
    /// Effective furniture rows, first-match order, with per-row source.
    pub furniture: Vec<Sourced>,
    /// Effective kinds rows, with per-row source.
    pub kinds: Vec<Sourced>,
    /// Effective important globs, list order, with per-row source.
    pub important: Vec<Sourced>,
    /// `columns.*` keys set above the defaults layer — accepted for one
    /// release, warned on stderr.
    pub old_column_keys: Vec<String>,
}

/// One map row (or important glob) and the layer that won it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sourced {
    pub key: String,
    pub value: String,
    pub source: &'static str,
}

/// Parsed TOML-subset (design/defaults.md §Parser): `[table]` headers,
/// dotted keys, arrays of strings, quoted keys. No crates.
#[derive(Clone, Debug, Default)]
pub struct Parsed {
    pub scalars: BTreeMap<String, String>,
    pub arrays: BTreeMap<String, Vec<String>>,
    pub furniture: Vec<(String, String)>,
    pub kinds: Vec<(String, String)>,
}

pub fn xdg_config_home() -> PathBuf {
    if let Ok(p) = env::var("XDG_CONFIG_HOME")
        && !p.is_empty()
    {
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

/// The embedded file, parsed once. Panics if the shipped file does not
/// parse — that is a build-time defect, not a runtime overlay miss.
pub fn embedded() -> &'static Parsed {
    static PARSED: OnceLock<Parsed> = OnceLock::new();
    PARSED.get_or_init(|| {
        let p = parse_file(DEFAULTS_TOML);
        assert!(
            !p.furniture.is_empty() && !p.kinds.is_empty(),
            "defaults.toml must parse a furniture map and a kinds map"
        );
        p
    })
}

/// on/true/1 · off/false/0. The shipped file uses TOML bools; the rest of
/// the stack still speaks on/off. Both are accepted.
pub fn boolish(s: &str) -> Option<bool> {
    match s {
        "on" | "true" | "1" => Some(true),
        "off" | "false" | "0" => Some(false),
        _ => None,
    }
}

fn layer_rank(name: &str) -> u8 {
    match name {
        "defaults" => 0,
        "global" => 1,
        "user-home" => 2,
        "agent-type" => 3,
        "env" => 4,
        "flags" => 5,
        _ => 0,
    }
}

/// Look up a format key under either its lattice name or the older
/// config name (`format.bytes` / `format.size`, `format.lines` /
/// `format.line-count`). The higher layer wins when both names are set.
pub fn format_val<'a>(
    won: &'a BTreeMap<String, (String, &'static str)>,
    key: &str,
) -> Option<&'a str> {
    let alias = match key {
        "format.size" => Some("format.bytes"),
        "format.bytes" => Some("format.size"),
        "format.line-count" => Some("format.lines"),
        "format.lines" => Some("format.line-count"),
        _ => None,
    };
    let a = won.get(key);
    let b = alias.and_then(|al| won.get(al));
    match (a, b) {
        (Some((v, sa)), Some((w, sb))) => {
            if layer_rank(sb) > layer_rank(sa) {
                Some(w.as_str())
            } else {
                Some(v.as_str())
            }
        }
        (Some((v, _)), None) => Some(v.as_str()),
        (None, Some((w, _))) => Some(w.as_str()),
        (None, None) => None,
    }
}

/// Display word for a layer name (design/defaults.md: `default` /
/// `user-home` / `caller` / `env`).
pub fn source_word(layer: &str) -> &'static str {
    match layer {
        "defaults" => "default",
        "agent-type" => "caller",
        "global" => "global",
        "user-home" => "user-home",
        "env" => "env",
        "flags" => "flags",
        _ => "default",
    }
}

/// A setting whose effective value differs from the built-in default.
/// `source` is the word a pasted look uses (`user-home`, `env`, `flag`,
/// `caller`, `global`) — `flag` not `flags`, matching the header example.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Drift {
    pub key: String,
    pub value: String,
    pub source: &'static str,
}

fn drift_source(layer: &str) -> &'static str {
    match layer {
        "flags" => "flag",
        other => source_word(other),
    }
}

fn same_value(a: &str, b: &str) -> bool {
    if a == b {
        return true;
    }
    match (boolish(a), boolish(b)) {
        (Some(x), Some(y)) => x == y,
        _ => false,
    }
}

/// Built-in defaults as `won` would see them (file scalars + columns
/// derived from the shipped `[layout]`). Overlay layers are not applied.
fn default_values(res: &Resolved) -> BTreeMap<String, String> {
    let Some(defaults) = res.layers.iter().find(|l| l.name == "defaults") else {
        return BTreeMap::new();
    };
    let mut m = defaults.values.clone();
    let mut layout = BTreeMap::new();
    for key in LAYOUT_KEYS {
        if let Some(v) = defaults.arrays.get(*key) {
            layout.insert((*key).to_string(), (v.clone(), "defaults"));
        }
    }
    for (k, v) in derive_columns(&layout) {
        m.entry(k).or_insert(v);
    }
    m
}

/// CLI form for a config key, when the source is the flags layer.
fn flag_form(key: &str, value: &str) -> Option<String> {
    match key {
        "depth" => Some(format!("--depth {value}")),
        "lines" => Some(format!("--lines {value}")),
        "walk" => Some(format!("--walk {value}")),
        "sort" => Some(format!("--sort {value}")),
        "format" => Some(format!("--format {value}")),
        "caller" => Some(format!("--caller {value}")),
        "dotfiles-first" if boolish(value) == Some(true) => Some("--dotfiles-first".into()),
        "one-fs" if boolish(value) == Some(false) => Some("--no-one-fs".into()),
        _ => None,
    }
}

/// Effective settings that differ from the built-in defaults, in key order.
/// `--caller` is included when set (it names whose eyes produced the look).
/// Empty when nothing differs — never a fake line.
pub fn drift(res: &Resolved, caller: Option<&str>) -> Vec<Drift> {
    let defaults = default_values(res);
    let mut out = Vec::new();
    for (k, (v, src)) in &res.won {
        if *src == "defaults" {
            continue;
        }
        // Serialization `format` (text/json) is the output channel, not
        // the eyes — a JSON look of the same tree must report the same
        // drift as the text look (design/json.md: same look).
        if k == "format" {
            continue;
        }
        // Maps are not one-line eye settings; they live in `aspectus config`.
        // A furniture overlay in the header would also collide with the
        // look (a `.mystery` in the drift line is not a child named that).
        if k == "furniture" || k == "kinds" || k == "important" {
            continue;
        }
        if defaults.get(k).is_some_and(|d| same_value(d, v)) {
            continue;
        }
        out.push(Drift {
            key: k.clone(),
            value: v.clone(),
            source: drift_source(src),
        });
    }
    if let Some(c) = caller {
        out.push(Drift {
            key: "caller".into(),
            value: c.to_string(),
            source: "flag",
        });
    }
    out.sort_by(|a, b| a.key.cmp(&b.key));
    out
}

/// One header line, or empty when nothing differs (absent, never faked).
pub fn drift_line(items: &[Drift]) -> String {
    if items.is_empty() {
        return String::new();
    }
    items
        .iter()
        .map(|d| {
            if d.source == "flag"
                && let Some(f) = flag_form(&d.key, &d.value)
            {
                return format!("{f} (flag)");
            }
            format!("{} = {} ({})", d.key, d.value, d.source)
        })
        .collect::<Vec<_>>()
        .join(" · ")
}

/// Units the `won:` listing prints beside a value (data carries its units).
/// `aspectus config defaults` prints the file verbatim — not this.
fn won_unit(key: &str) -> Option<&'static str> {
    match key {
        "heat.half-life" => Some("commits"),
        "reads" => Some("bytes"),
        "lines" => Some("lines"),
        "walk" => Some("names"),
        "depth" => Some("generations"),
        _ => None,
    }
}

fn defaults_layer() -> Layer {
    let p = embedded().clone();
    let mut values = p.scalars.clone();
    // The serialization key is not in the file (`[format]` is fact
    // formats). Keep today's default so `format = json` still overlays.
    values.entry("format".into()).or_insert("text".into());
    // Aliases so today's `format.size` / `format.line-count` lookups and
    // the inventory's starring still resolve.
    if let Some(v) = values.get("format.bytes").cloned() {
        values.entry("format.size".into()).or_insert(v);
    }
    if let Some(v) = values.get("format.lines").cloned() {
        values.entry("format.line-count".into()).or_insert(v);
    }
    Layer {
        name: "defaults",
        path: None,
        existed: true,
        values,
        arrays: p.arrays,
        furniture: p.furniture,
        kinds: p.kinds,
    }
}

pub fn parse_file(text: &str) -> Parsed {
    let mut out = Parsed::default();
    let mut section: Option<String> = None;
    for raw in text.lines() {
        let line = strip_comment(raw).trim();
        if line.is_empty() {
            continue;
        }
        if let Some(name) = parse_table_header(line) {
            section = Some(name);
            continue;
        }
        let Some((key, val)) = parse_key_val(line) else {
            continue;
        };
        match section.as_deref() {
            Some("furniture") => out.furniture.push((key, val.into_string())),
            Some("kinds") => out.kinds.push((key, val.into_string())),
            Some("format") => match val {
                Val::Array(a) => {
                    out.arrays.insert(format!("format.{key}"), a);
                }
                Val::Str(s) => {
                    out.scalars.insert(format!("format.{key}"), s);
                }
            },
            Some("layout") => match val {
                Val::Array(a) => {
                    out.arrays.insert(format!("layout.{key}"), a);
                }
                Val::Str(s) => {
                    out.scalars.insert(format!("layout.{key}"), s);
                }
            },
            Some(other) => match val {
                Val::Array(a) => {
                    out.arrays.insert(format!("{other}.{key}"), a);
                }
                Val::Str(s) => {
                    out.scalars.insert(format!("{other}.{key}"), s);
                }
            },
            None => match val {
                Val::Array(a) => {
                    out.arrays.insert(key, a);
                }
                Val::Str(s) => {
                    out.scalars.insert(key, s);
                }
            },
        }
    }
    out
}

enum Val {
    Str(String),
    Array(Vec<String>),
}

impl Val {
    fn into_string(self) -> String {
        match self {
            Val::Str(s) => s,
            Val::Array(a) => a.join(", "),
        }
    }
}

/// Drop a `#` comment that is not inside a quoted string.
fn strip_comment(line: &str) -> &str {
    let mut in_str = false;
    let mut quote = '\0';
    let mut bs = false;
    for (i, c) in line.char_indices() {
        if in_str {
            if bs {
                bs = false;
                continue;
            }
            if c == '\\' {
                bs = true;
                continue;
            }
            if c == quote {
                in_str = false;
            }
            continue;
        }
        if c == '"' || c == '\'' {
            in_str = true;
            quote = c;
            continue;
        }
        if c == '#' {
            return &line[..i];
        }
    }
    line
}

fn parse_table_header(line: &str) -> Option<String> {
    let line = line.trim();
    let inner = line.strip_prefix('[')?.strip_suffix(']')?;
    let name = inner.trim();
    if name.is_empty() || name.contains('[') {
        return None;
    }
    Some(name.to_string())
}

fn parse_key_val(line: &str) -> Option<(String, Val)> {
    let line = line.trim();
    let (key, rest) = if line.starts_with('"') || line.starts_with('\'') {
        let (k, r) = parse_quoted(line)?;
        let r = r.trim();
        let r = r.strip_prefix('=')?;
        (k, r.trim())
    } else {
        let (k, v) = line.split_once('=')?;
        (k.trim().to_string(), v.trim())
    };
    if key.is_empty() {
        return None;
    }
    Some((key, parse_val(rest)))
}

fn parse_val(v: &str) -> Val {
    let v = v.trim();
    if v.starts_with('[') {
        Val::Array(parse_str_array(v).unwrap_or_default())
    } else if v.starts_with('"') || v.starts_with('\'') {
        Val::Str(
            parse_quoted(v)
                .map(|(s, _)| s)
                .unwrap_or_else(|| unquote(v)),
        )
    } else {
        Val::Str(v.to_string())
    }
}

fn unquote(v: &str) -> String {
    let v = v.trim();
    if (v.starts_with('"') && v.ends_with('"') && v.len() >= 2)
        || (v.starts_with('\'') && v.ends_with('\'') && v.len() >= 2)
    {
        v[1..v.len() - 1].to_string()
    } else {
        v.to_string()
    }
}

fn parse_quoted(s: &str) -> Option<(String, &str)> {
    let s = s.trim_start();
    let bytes = s.as_bytes();
    if bytes.is_empty() {
        return None;
    }
    let q = bytes[0];
    if q != b'"' && q != b'\'' {
        return None;
    }
    let mut out = String::new();
    let mut i = 1;
    let mut bs = false;
    while i < bytes.len() {
        let c = bytes[i];
        if bs {
            out.push(c as char);
            bs = false;
            i += 1;
            continue;
        }
        if c == b'\\' {
            bs = true;
            i += 1;
            continue;
        }
        if c == q {
            return Some((out, &s[i + 1..]));
        }
        // TOML strings are UTF-8; fall back to char-width when non-ascii.
        let ch = s[i..].chars().next()?;
        out.push(ch);
        i += ch.len_utf8();
    }
    None
}

fn parse_str_array(s: &str) -> Option<Vec<String>> {
    let s = s.trim();
    let inner = s.strip_prefix('[')?.strip_suffix(']')?;
    let mut items = Vec::new();
    let mut rest = inner;
    loop {
        rest = rest.trim_start();
        if rest.starts_with(',') {
            rest = rest[1..].trim_start();
        }
        if rest.is_empty() {
            break;
        }
        if rest.starts_with('"') || rest.starts_with('\'') {
            let (item, r) = parse_quoted(rest)?;
            items.push(item);
            rest = r;
        } else {
            let (item, r) = match rest.split_once(',') {
                Some((a, b)) => (a.trim().to_string(), b),
                None => (rest.trim().to_string(), ""),
            };
            if !item.is_empty() {
                items.push(item);
            }
            rest = r;
        }
    }
    Some(items)
}

fn load_path(path: &Path) -> (bool, Parsed) {
    match fs::read_to_string(path) {
        Ok(s) => (true, parse_file(&s)),
        Err(_) => (false, Parsed::default()),
    }
}

fn layer_from_parsed(name: &'static str, path: Option<PathBuf>, existed: bool, p: Parsed) -> Layer {
    Layer {
        name,
        path,
        existed,
        values: p.scalars,
        arrays: p.arrays,
        furniture: p.furniture,
        kinds: p.kinds,
    }
}

pub fn env_values() -> BTreeMap<String, String> {
    let mut m = BTreeMap::new();
    if let Ok(v) = env::var("ASPECTUS_LINES")
        && !v.is_empty()
    {
        m.insert("lines".into(), v);
    }
    if let Ok(v) = env::var("ASPECTUS_DEPTH")
        && !v.is_empty()
    {
        m.insert("depth".into(), v);
    }
    if let Ok(v) = env::var("ASPECTUS_WALK")
        && !v.is_empty()
    {
        m.insert("walk".into(), v);
    }
    if let Ok(v) = env::var("ASPECTUS_FURNITURE")
        && !v.is_empty()
    {
        m.insert("furniture".into(), v);
    }
    // Same keys the config files speak, one env spelling each.
    for (var, key) in [
        ("ASPECTUS_SORT", "sort"),
        ("ASPECTUS_DOTFILES_FIRST", "dotfiles-first"),
        ("ASPECTUS_COLUMNS_SIZE", "columns.size"),
        ("ASPECTUS_COLUMNS_MTIME", "columns.mtime"),
        ("ASPECTUS_COLUMNS_LINE_COUNT", "columns.line-count"),
        ("ASPECTUS_COLUMNS_HEAT", "columns.heat"),
        ("ASPECTUS_COLUMNS_PERMISSIONS", "columns.permissions"),
        ("ASPECTUS_COLUMNS_OWNER", "columns.owner"),
        ("ASPECTUS_COLUMNS_FILEKIND", "columns.filekind"),
        ("ASPECTUS_COLUMNS_INITIAL_SHA", "columns.initial-sha"),
        ("ASPECTUS_COLUMNS_LATEST_SHA", "columns.latest-sha"),
        ("ASPECTUS_FORMAT_SIZE", "format.size"),
        ("ASPECTUS_FORMAT_MTIME", "format.mtime"),
        ("ASPECTUS_FORMAT_LINE_COUNT", "format.line-count"),
        ("ASPECTUS_FORMAT_CENSUS", "format.census"),
        ("ASPECTUS_FORMAT_OWNER", "format.owner"),
        ("ASPECTUS_FORMAT_INITIAL_SHA", "format.initial-sha"),
        ("ASPECTUS_FORMAT_LATEST_SHA", "format.latest-sha"),
        ("ASPECTUS_GLOBIFY", "globify"),
        ("ASPECTUS_GLOBIFY_MIN", "globify.min"),
        ("ASPECTUS_README_TITLE", "readme-title"),
        ("ASPECTUS_FORMAT", "format"),
        ("ASPECTUS_IMPORTANT", "important"),
        ("ASPECTUS_QUIET_SENSITIVITY", "quiet.sensitivity"),
        ("ASPECTUS_QUIET_SENSITIVITY_SIZE", "quiet.sensitivity.size"),
        (
            "ASPECTUS_QUIET_SENSITIVITY_MTIME",
            "quiet.sensitivity.mtime",
        ),
        ("ASPECTUS_KINDS", "kinds"),
        ("ASPECTUS_ONE_FS", "one-fs"),
        ("ASPECTUS_RECENCY_SOURCE", "recency-source"),
        ("ASPECTUS_HEAT_HALF_LIFE", "heat.half-life"),
        ("ASPECTUS_READS", "reads"),
    ] {
        if let Ok(v) = env::var(var)
            && !v.is_empty()
        {
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

    layers.push(defaults_layer());

    let (g_ok, g_vals) = load_path(Path::new(GLOBAL_PATH));
    layers.push(layer_from_parsed(
        "global",
        Some(PathBuf::from(GLOBAL_PATH)),
        g_ok,
        g_vals,
    ));

    let user_path = user_home_override
        .map(Path::to_path_buf)
        .unwrap_or_else(user_home_path);
    let (u_ok, u_vals) = load_path(&user_path);
    layers.push(layer_from_parsed(
        "user-home",
        Some(user_path),
        u_ok,
        u_vals,
    ));

    match caller {
        Some(key) => {
            let p = agent_type_path(key);
            let (ok, vals) = load_path(&p);
            layers.push(layer_from_parsed("agent-type", Some(p), ok, vals));
        }
        None => layers.push(Layer {
            name: "agent-type",
            path: None,
            existed: false,
            values: BTreeMap::new(),
            arrays: BTreeMap::new(),
            furniture: Vec::new(),
            kinds: Vec::new(),
        }),
    }

    let env_vals = env_values();
    layers.push(Layer {
        name: "env",
        path: None,
        existed: !env_vals.is_empty(),
        values: env_vals,
        arrays: BTreeMap::new(),
        furniture: Vec::new(),
        kinds: Vec::new(),
    });

    layers.push(Layer {
        name: "flags",
        path: None,
        existed: !flag_vals.is_empty(),
        values: flag_vals,
        arrays: BTreeMap::new(),
        furniture: Vec::new(),
        kinds: Vec::new(),
    });

    let mut won = BTreeMap::new();
    for layer in &layers {
        // File layers that restate a value keep the earlier source — a
        // copied defaults.toml is not an ask. Env and flags are this
        // invocation and always promote, even when restating (so
        // `--sort recency` still lifts the mtime column).
        let promote = layer.name == "env" || layer.name == "flags";
        for (k, v) in &layer.values {
            if !promote {
                if let Some((cur, _)) = won.get(k)
                    && cur == v
                {
                    continue;
                }
            }
            won.insert(k.clone(), (v.clone(), layer.name));
        }
    }

    let layout = merge_layout(&layers);
    let furniture = merge_furniture(&layers);
    let kinds = merge_kinds(&layers);
    let important = merge_important(&layers);

    // Column state is membership in `[layout]` + the quiet list, unless a
    // higher layer still sets `columns.*` (one-release compatibility).
    let explicit_columns: BTreeSet<String> = layers
        .iter()
        .filter(|l| l.name != "defaults")
        .flat_map(|l| l.values.keys().cloned())
        .filter(|k| k.starts_with("columns."))
        .collect();
    let old_column_keys: Vec<String> = {
        let mut v: Vec<String> = explicit_columns.iter().cloned().collect();
        v.sort();
        v
    };
    for (k, v) in derive_columns(&layout) {
        if !explicit_columns.contains(&k) {
            let src = layout_source(&layout, column_fact_for(&k));
            won.insert(k, (v, src));
        }
    }

    Resolved {
        layers,
        won,
        layout,
        furniture,
        kinds,
        important,
        old_column_keys,
    }
}

fn merge_layout(layers: &[Layer]) -> BTreeMap<String, (Vec<String>, &'static str)> {
    let mut m = BTreeMap::new();
    for layer in layers {
        for key in LAYOUT_KEYS {
            if let Some(v) = layer.arrays.get(*key) {
                m.insert((*key).to_string(), (v.clone(), layer.name));
            }
        }
    }
    m
}

fn layout_lists<'a>(
    layout: &'a BTreeMap<String, (Vec<String>, &'static str)>,
) -> impl Iterator<Item = &'a String> {
    LAYOUT_KEYS
        .iter()
        .filter(|k| **k != "layout.quiet")
        .filter_map(|k| layout.get(*k).map(|(v, _)| v))
        .flatten()
}

fn derive_columns(
    layout: &BTreeMap<String, (Vec<String>, &'static str)>,
) -> BTreeMap<String, String> {
    let quiet: BTreeSet<&str> = layout
        .get("layout.quiet")
        .map(|(v, _)| v.iter().map(String::as_str).collect())
        .unwrap_or_default();
    let listed: BTreeSet<&str> = layout_lists(layout).map(String::as_str).collect();
    let mut out = BTreeMap::new();
    for (fact, key) in COLUMN_FACTS {
        let state = if quiet.contains(fact) {
            "quiet"
        } else if listed.contains(fact) {
            "on"
        } else {
            "off"
        };
        out.insert((*key).to_string(), state.to_string());
    }
    out
}

fn column_fact_for(key: &str) -> &str {
    COLUMN_FACTS
        .iter()
        .find(|(_, k)| *k == key)
        .map(|(f, _)| *f)
        .unwrap_or("")
}

fn layout_source(
    layout: &BTreeMap<String, (Vec<String>, &'static str)>,
    fact: &str,
) -> &'static str {
    if fact.is_empty() {
        return "defaults";
    }
    for key in LAYOUT_KEYS {
        if let Some((facts, src)) = layout.get(*key)
            && facts.iter().any(|f| f == fact)
        {
            return *src;
        }
    }
    "defaults"
}

fn merge_furniture(layers: &[Layer]) -> Vec<Sourced> {
    let mut rows: Vec<Sourced> = Vec::new();
    for layer in layers {
        if !layer.furniture.is_empty() {
            overlay_pairs(&mut rows, &layer.furniture, layer.name);
        }
        if let Some(comma) = layer.values.get("furniture") {
            overlay_pairs(&mut rows, &parse_furniture_comma(comma), layer.name);
        }
    }
    rows
}

fn merge_kinds(layers: &[Layer]) -> Vec<Sourced> {
    let mut rows: Vec<Sourced> = Vec::new();
    for layer in layers {
        if !layer.kinds.is_empty() {
            overlay_kinds(&mut rows, &layer.kinds, layer.name);
        }
        if let Some(comma) = layer.values.get("kinds") {
            overlay_kinds(&mut rows, &parse_kinds_comma(comma), layer.name);
        }
    }
    rows
}

fn merge_important(layers: &[Layer]) -> Vec<Sourced> {
    let mut rows: Vec<Sourced> = Vec::new();
    for layer in layers {
        if let Some(list) = layer.arrays.get("important") {
            rows = list
                .iter()
                .map(|p| Sourced {
                    key: p.clone(),
                    value: String::new(),
                    source: layer.name,
                })
                .collect();
        }
        if let Some(comma) = layer.values.get("important") {
            overlay_important_comma(&mut rows, comma, layer.name);
        }
    }
    rows
}

fn overlay_pairs(rows: &mut Vec<Sourced>, incoming: &[(String, String)], source: &'static str) {
    let mut front = Vec::new();
    for (pat, val) in incoming {
        if val == "!" {
            let p = trim_slash(pat);
            rows.retain(|r| trim_slash(&r.key) != p);
            front.retain(|r: &Sourced| trim_slash(&r.key) != p);
            continue;
        }
        front.push(Sourced {
            key: pat.clone(),
            value: val.clone(),
            source,
        });
    }
    front.append(rows);
    *rows = front;
}

fn overlay_kinds(rows: &mut Vec<Sourced>, incoming: &[(String, String)], source: &'static str) {
    for (k, v) in incoming {
        let key = norm_kind_key(k);
        if v == "!" {
            rows.retain(|r| norm_kind_key(&r.key) != key);
            continue;
        }
        if let Some(existing) = rows.iter_mut().find(|r| norm_kind_key(&r.key) == key) {
            existing.key = k.clone();
            existing.value = v.clone();
            existing.source = source;
        } else {
            rows.push(Sourced {
                key: k.clone(),
                value: v.clone(),
                source,
            });
        }
    }
}

fn overlay_important_comma(rows: &mut Vec<Sourced>, comma: &str, source: &'static str) {
    let mut front = Vec::new();
    for item in comma.split(',') {
        let item = item.trim();
        if item.is_empty() {
            continue;
        }
        if let Some(pat) = item.strip_prefix('!') {
            rows.retain(|r| r.key != pat);
            front.retain(|r: &Sourced| r.key != pat);
            continue;
        }
        front.push(Sourced {
            key: item.to_string(),
            value: String::new(),
            source,
        });
    }
    front.append(rows);
    *rows = front;
}

fn trim_slash(s: &str) -> &str {
    s.trim_end_matches('/')
}

fn norm_kind_key(s: &str) -> String {
    s.trim().trim_start_matches('.').to_string()
}

/// Legacy `furniture` comma grammar → table pairs.
fn parse_furniture_comma(config: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for item in config.split(',') {
        let item = item.trim();
        if item.is_empty() {
            continue;
        }
        if let Some(pat) = item.strip_prefix('!') {
            out.push((pat.to_string(), "!".into()));
            continue;
        }
        let mut parts = item.splitn(3, ':');
        let pattern = parts.next().unwrap_or("");
        if pattern.is_empty() {
            continue;
        }
        let kinds = parts.next().unwrap_or("");
        let fate = parts.next();
        let value = match fate {
            Some(f) if !f.is_empty() => {
                if kinds.is_empty() {
                    format!(":{f}")
                } else {
                    format!("{kinds}:{f}")
                }
            }
            _ => kinds.to_string(),
        };
        out.push((pattern.to_string(), value));
    }
    out
}

/// Legacy `kinds` comma grammar → table pairs.
fn parse_kinds_comma(config: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for rule in config.split(',') {
        let rule = rule.trim();
        if rule.is_empty() {
            continue;
        }
        if let Some(dropped) = rule.strip_prefix('!') {
            out.push((dropped.to_string(), "!".into()));
            continue;
        }
        if let Some((pat, kind)) = rule.rsplit_once(':') {
            out.push((pat.to_string(), kind.trim().to_string()));
        }
    }
    out
}

/// Stderr note for the one-release `columns.*` compatibility window.
pub fn old_columns_note(res: &Resolved) -> Option<String> {
    if res.old_column_keys.is_empty() {
        return None;
    }
    Some(format!(
        "aspectus: {} accepted for this release; membership is now [layout] (see `aspectus config defaults`)",
        res.old_column_keys.join(", ")
    ))
}

pub fn render_show(res: &Resolved) -> String {
    let mut out = String::from("aspectus config\n\n");
    out.push_str("layer        status    source\n");
    for layer in &res.layers {
        let status = if layer.existed { "present" } else { "absent" };
        let source = match (&layer.path, layer.name, layer.existed) {
            (Some(p), _, _) => p.display().to_string(),
            (None, "defaults", _) => "built-in (defaults.toml, embedded)".into(),
            (None, "env", _) => "ASPECTUS_*".into(),
            (None, "flags", _) => "argv".into(),
            (None, "agent-type", _) => "--caller not set".into(),
            (None, _, _) => "—".into(),
        };
        out.push_str(&format!("{:<12} {:<9} {}\n", layer.name, status, source));
    }
    out.push_str("\nwon:\n");
    for (k, (v, from)) in &res.won {
        match won_unit(k) {
            Some(u) => out.push_str(&format!("  {k} = {v} {u}  ({from})\n")),
            None => out.push_str(&format!("  {k} = {v}  ({from})\n")),
        }
    }
    out.push_str(&render_layout(res));
    out.push_str(&render_map("furniture", &res.furniture, true));
    out.push_str(&render_map("kinds", &res.kinds, true));
    out.push_str(&render_important(res));
    out
}

fn render_layout(res: &Resolved) -> String {
    let mut out = String::from("\nlayout:\n");
    for key in LAYOUT_KEYS {
        let short = key.strip_prefix("layout.").unwrap_or(key);
        let unbuilt_pos = UNBUILT_POSITIONS.contains(&short);
        match res.layout.get(*key) {
            Some((facts, src)) => {
                let list = facts.join(", ");
                let mark = layout_unbuilt_mark(short, facts);
                out.push_str(&format!(
                    "  {short:<12} {list}{mark}  ({})\n",
                    source_word(src)
                ));
            }
            None if unbuilt_pos => {
                out.push_str(&format!("  {short:<12} —  (unbuilt position)\n"));
            }
            None => {
                out.push_str(&format!("  {short:<12} —\n"));
            }
        }
    }
    out
}

/// Whole-position unbuilt (`supplement`) vs per-entry (`far-left` once
/// git-status paints: `(unbuilt: mtime, bytes)`).
fn layout_unbuilt_mark(short: &str, facts: &[String]) -> String {
    if UNBUILT_POSITIONS.contains(&short) {
        return "  (unbuilt position)".to_string();
    }
    if short == "far-left" {
        let pending: Vec<&str> = facts
            .iter()
            .filter(|f| !FAR_LEFT_PAINTABLE.contains(&f.as_str()))
            .map(String::as_str)
            .collect();
        if !pending.is_empty() {
            return format!("  (unbuilt: {})", pending.join(", "));
        }
    }
    String::new()
}

fn render_map(title: &str, rows: &[Sourced], show_value: bool) -> String {
    let mut out = format!("\n{title}:\n");
    if rows.is_empty() {
        out.push_str("  (none)\n");
        return out;
    }
    for r in rows {
        let src = source_word(r.source);
        if show_value {
            out.push_str(&format!("  {} = {}  ({src})\n", r.key, r.value));
        } else {
            out.push_str(&format!("  {}  ({src})\n", r.key));
        }
    }
    out
}

fn render_important(res: &Resolved) -> String {
    let mut out = String::from("\nimportant:\n");
    if res.important.is_empty() {
        out.push_str("  (none)\n");
        return out;
    }
    for r in &res.important {
        out.push_str(&format!("  {}  ({})\n", r.key, source_word(r.source)));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shipped_file_parses() {
        let p = parse_file(DEFAULTS_TOML);
        assert_eq!(p.scalars.get("depth").map(String::as_str), Some("2"));
        assert_eq!(p.scalars.get("lines").map(String::as_str), Some("80"));
        assert_eq!(p.scalars.get("globify").map(String::as_str), Some("true"));
        assert_eq!(p.scalars.get("globify.min").map(String::as_str), Some("5"));
        assert_eq!(p.scalars.get("one-fs").map(String::as_str), Some("true"));
        assert_eq!(
            p.scalars.get("format.mtime").map(String::as_str),
            Some("signa")
        );
        assert_eq!(
            p.scalars.get("format.bytes").map(String::as_str),
            Some("human")
        );
        assert_eq!(
            p.arrays.get("layout.far-left").cloned().unwrap_or_default(),
            ["heat", "git-status", "mtime", "bytes"]
        );
        assert_eq!(
            p.arrays
                .get("layout.far-right")
                .cloned()
                .unwrap_or_default(),
            ["lines", "age"]
        );
        assert_eq!(
            p.arrays.get("important").cloned().unwrap_or_default(),
            ["README*", "AGENTS.md", "CLAUDE.md"]
        );
        assert!(
            p.furniture.iter().any(|(k, v)| k == ".git" && v == "git"),
            "furniture .git: {:?}",
            p.furniture
        );
        assert!(
            p.furniture
                .iter()
                .any(|(k, v)| k == "target/" && v == "build"),
            "{:?}",
            p.furniture
        );
        assert!(
            p.furniture
                .iter()
                .any(|(k, v)| k == ".DS_Store" && v == ":omit"),
            "{:?}",
            p.furniture
        );
        assert!(
            p.furniture
                .iter()
                .any(|(k, v)| k == "Cargo.toml" && v == "rust:mark"),
            "{:?}",
            p.furniture
        );
        assert!(
            p.kinds
                .iter()
                .any(|(k, v)| k == "md" && v == "text/markdown"),
            "{:?}",
            p.kinds
        );
        assert!(
            p.kinds.iter().any(|(k, v)| k == "7z" && v == "archive/7z"),
            "quoted key 7z: {:?}",
            p.kinds
        );
        assert!(
            p.kinds.iter().any(|(k, v)| k == "svg" && v == "image/svg"),
            "{:?}",
            p.kinds
        );
        // A comment after an array must not leak into the values.
        let quiet = p.arrays.get("layout.quiet").expect("quiet list");
        assert_eq!(
            quiet,
            &["mtime", "bytes", "permissions", "owner", "filekind"]
        );
    }

    #[test]
    fn quoted_key_and_inline_comment() {
        let p = parse_file(
            r#"
[furniture]
".git" = "git" # a comment with "quotes"
"target/" = "build"
"7z" = "archive/7z"
"#,
        );
        assert_eq!(
            p.furniture,
            vec![
                (".git".into(), "git".into()),
                ("target/".into(), "build".into()),
                ("7z".into(), "archive/7z".into()),
            ]
        );
    }

    #[test]
    fn array_and_dotted_keys() {
        let p = parse_file(
            r#"
globify.min = 5
important = ["README*", "AGENTS.md"]
[layout]
far-right = ["lines", "heat"]
"#,
        );
        assert_eq!(p.scalars.get("globify.min").unwrap(), "5");
        assert_eq!(
            p.arrays.get("important").unwrap(),
            &["README*", "AGENTS.md"]
        );
        assert_eq!(
            p.arrays.get("layout.far-right").unwrap(),
            &["lines", "heat"]
        );
    }

    #[test]
    fn derived_columns_match_today() {
        let p = parse_file(DEFAULTS_TOML);
        let mut layout = BTreeMap::new();
        for key in LAYOUT_KEYS {
            if let Some(v) = p.arrays.get(*key) {
                layout.insert((*key).to_string(), (v.clone(), "defaults"));
            }
        }
        let cols = derive_columns(&layout);
        let get = |k: &str| cols.get(k).map(String::as_str);
        assert_eq!(get("columns.line-count"), Some("on"));
        assert_eq!(get("columns.heat"), Some("on"));
        assert_eq!(get("columns.size"), Some("quiet"));
        assert_eq!(get("columns.mtime"), Some("quiet"));
        assert_eq!(get("columns.permissions"), Some("quiet"));
        assert_eq!(get("columns.owner"), Some("quiet"));
        assert_eq!(get("columns.filekind"), Some("quiet"));
        assert_eq!(get("columns.initial-sha"), Some("off"));
        assert_eq!(get("columns.latest-sha"), Some("off"));
        assert_eq!(p.scalars.get("lines").map(String::as_str), Some("80"));
        assert_eq!(
            p.scalars.get("format.bytes").map(String::as_str),
            Some("human")
        );
        assert_eq!(
            p.scalars.get("format.lines").map(String::as_str),
            Some("physical")
        );
    }

    #[test]
    fn drift_line_absent_when_empty() {
        assert_eq!(drift_line(&[]), "");
    }

    #[test]
    fn drift_line_flag_and_file_forms() {
        let items = vec![
            Drift {
                key: "depth".into(),
                value: "3".into(),
                source: "user-home",
            },
            Drift {
                key: "lines".into(),
                value: "200".into(),
                source: "flag",
            },
        ];
        assert_eq!(
            drift_line(&items),
            "depth = 3 (user-home) · --lines 200 (flag)"
        );
    }

    #[test]
    fn drift_line_caller_and_inverted_flag() {
        let items = vec![
            Drift {
                key: "caller".into(),
                value: "grok".into(),
                source: "flag",
            },
            Drift {
                key: "one-fs".into(),
                value: "off".into(),
                source: "flag",
            },
        ];
        assert_eq!(
            drift_line(&items),
            "--caller grok (flag) · --no-one-fs (flag)"
        );
    }
}
