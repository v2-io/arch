//! The fact inventory: the discoverability surface of the aspect lattice
//! (design/columns.md §Discoverability, design/aspect-lattice.md).
//!
//! One static table, rendered on the `aspectus config` surface, so an agent
//! learns which facts a line may carry, which this caller's look is showing,
//! and how to ask — without reading source. A fact absent from this table
//! is a fact the machinery does not know.

use crate::config::Resolved;

/// Placement classes (design/shorthand.md, superseding the lattice's
/// column/INFO binary). Only Decoration, NearRight, and FarRight render
/// today; the rest are declared so later facts plug in without
/// restructuring the paint path.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Placement {
    /// Fused to the name: `/` on dirs, `-> target`, README title.
    Decoration,
    /// Before the hierarchy indent (glyph blocks' leading candidate).
    FarLeft,
    /// Indented with the children, before names.
    NearLeft,
    /// After the name at the computed tab-stop: censuses, facets, marks.
    NearRight,
    /// Right-aligned full columns: sizes, times, counts.
    FarRight,
    /// Dense fixed-width symbolic column (permissions-style); vocabulary
    /// awaits ratification — nothing renders here yet.
    GlyphBlock,
}

impl Placement {
    pub fn word(self) -> &'static str {
        match self {
            Placement::Decoration => "decoration",
            Placement::FarLeft => "far-left",
            Placement::NearLeft => "near-left",
            Placement::NearRight => "near-right",
            Placement::FarRight => "far-right",
            Placement::GlyphBlock => "glyph-block",
        }
    }
}

pub struct Fact {
    pub key: &'static str,
    /// Where it lands on the line.
    pub place: Placement,
    /// Lattice default: "on", "off", "quiet".
    pub default: &'static str,
    /// Config key that turns it on/off/quiet, or None (not overridable).
    pub toggle: Option<&'static str>,
    /// Format options, `*` marks the default; empty = none.
    pub formats: &'static str,
    /// Obtain implemented — a fact can be asked for only once built.
    pub built: bool,
}

/// Every lattice fact the machinery knows, in lattice order. Unbuilt rows
/// are here so the inventory (and refusals) can name them honestly.
pub const FACTS: &[Fact] = &[
    Fact { key: "name", place: Placement::Decoration, default: "on", toggle: None, formats: "basename*", built: true },
    Fact { key: "child-count", place: Placement::NearRight, default: "on", toggle: None, formats: "n*", built: true },
    Fact { key: "line-count", place: Placement::FarRight, default: "on", toggle: Some("columns.line-count"), formats: "physical* / non-blank", built: true },
    Fact { key: "mtime", place: Placement::FarRight, default: "quiet", toggle: Some("columns.mtime"), formats: "relative* / iso-8601 / epoch", built: true },
    Fact { key: "size", place: Placement::FarRight, default: "quiet", toggle: Some("columns.size"), formats: "human* / bytes", built: true },
    Fact { key: "created", place: Placement::FarRight, default: "off", toggle: Some("columns.created"), formats: "iso-8601* / epoch", built: false },
    Fact { key: "filetype", place: Placement::NearRight, default: "on", toggle: None, formats: "suffix*", built: true },
    Fact { key: "filekind", place: Placement::NearRight, default: "quiet", toggle: Some("columns.filekind"), formats: "word*", built: true },
    Fact { key: "has", place: Placement::NearRight, default: "on", toggle: None, formats: "facet*", built: true },
    Fact { key: "git-letter", place: Placement::FarRight, default: "on", toggle: None, formats: "letter*", built: false },
    Fact { key: "heat", place: Placement::FarRight, default: "on", toggle: Some("columns.heat"), formats: "score*", built: true },
    Fact { key: "prior-name", place: Placement::NearRight, default: "quiet", toggle: Some("columns.prior-name"), formats: "was*", built: false },
    Fact { key: "symlink-target", place: Placement::Decoration, default: "on", toggle: None, formats: "target*", built: true },
    Fact { key: "filesystem", place: Placement::NearRight, default: "on", toggle: None, formats: "mark*", built: true },
    Fact { key: "denied", place: Placement::NearRight, default: "on", toggle: None, formats: "denied*", built: true },
    Fact { key: "walk-marks", place: Placement::NearRight, default: "on", toggle: None, formats: "mark*", built: true },
    Fact { key: "permissions", place: Placement::FarRight, default: "quiet", toggle: Some("columns.permissions"), formats: "octal*", built: true },
    Fact { key: "owner", place: Placement::FarRight, default: "quiet", toggle: Some("columns.owner"), formats: "name* / id", built: true },
    Fact { key: "linkcount", place: Placement::FarRight, default: "off", toggle: Some("columns.linkcount"), formats: "n*", built: false },
];

/// The state this caller's look gives a fact: config override, else default.
pub fn state<'a>(f: &'a Fact, cfg: &'a Resolved) -> &'a str {
    match f.toggle.and_then(|k| cfg.won.get(k)) {
        Some((v, _)) => v.as_str(),
        None => f.default,
    }
}

/// The `--FACT` a caller might try, mapped to the honest refusal: no fact
/// has its own flag (design/aspect-lattice.md: almost no own flags); the
/// ask is the config path, or the fact is not built yet.
pub fn flag_refusal(flag: &str) -> Option<String> {
    let bare = flag.trim_start_matches('-');
    let f = FACTS.iter().find(|f| f.key == bare)?;
    Some(match (f.toggle, f.built) {
        (Some(k), true) => format!(
            "facts have no own flags; ask via config: {k} = on (see `aspectus config`)"
        ),
        (Some(k), false) => format!(
            "fact not built yet; when it lands the ask is config: {k} = on"
        ),
        (None, _) => format!(
            "the {} fact is not toggleable (see `aspectus config` for the inventory)",
            f.key
        ),
    })
}

/// The inventory block appended to `aspectus config`.
pub fn inventory(cfg: &Resolved) -> String {
    let mut out = String::from(
        "\nfacts (the aspect lattice; ask via config keys, e.g. columns.size = on):\n",
    );
    out.push_str(&format!(
        "  {:<16} {:<11} {:<7} {:<22} {}\n",
        "fact", "place", "state", "ask", "format"
    ));
    for f in FACTS {
        let st = if f.built { state(f, cfg).to_string() } else { "unbuilt".into() };
        let ask = f.toggle.unwrap_or("—");
        let fmt = fmt_in_effect(f, cfg);
        out.push_str(&format!(
            "  {:<16} {:<11} {:<7} {:<22} {}\n",
            f.key,
            f.place.word(),
            st,
            ask,
            fmt
        ));
    }
    out
}

/// Formats with the one in effect starred (config `format.KEY` may move it).
fn fmt_in_effect(f: &Fact, cfg: &Resolved) -> String {
    let chosen = cfg
        .won
        .get(&format!("format.{}", f.key))
        .map(|(v, _)| v.as_str());
    match chosen {
        None => f.formats.to_string(),
        Some(c) => f
            .formats
            .split(" / ")
            .map(|opt| {
                let bare = opt.trim_end_matches('*');
                if bare == c {
                    format!("{bare}*")
                } else {
                    bare.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join(" / "),
    }
}
