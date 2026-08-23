//! The fact inventory — lattice-2 (design/lattice-2.md) as data, and the
//! discoverability surface `aspectus config` prints from it.
//!
//! One row per fact, carrying the seven fields of Joseph's refounded table:
//! **stat · fact · derived-from · position (+ office) · display · sort ·
//! width · formats**, plus what a caller needs to act on it — the config
//! key that turns it on/off/quiet and whether the obtain is built at all.
//! A fact absent from this table is a fact the machinery does not know.
//!
//! Two honesty rules this table lives under (it was rewritten 2026-08-22
//! because it broke the first): **`position` says where the fact paints
//! *today*** — not where a design wants it — and **`stat` says how settled
//! that is**, so a row the design moves later is visible as `↬` rather than
//! silently misfiled. The pre-2026-08-22 table's `place` words were
//! inverted against what rendered (Joseph: "very wrong").

use crate::config::Resolved;
use crate::ready::{Office, Position};

/// How settled a row is (lattice-2 §stat).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stat {
    /// Provisional — the design has not settled it.
    Provisional,
    /// Settled design, implemented to spec.
    Built,
    /// Settled design, needs (re)implementation against the table.
    Wants,
    /// Provisionally designed, deliberately deferred.
    Deferred,
}

impl Stat {
    pub fn glyph(self) -> &'static str {
        match self {
            Stat::Provisional => " ",
            Stat::Built => "✓",
            Stat::Wants => "↬",
            Stat::Deferred => "⇥",
        }
    }
}

/// Default display (lattice-2 §default-display): `always` is not quietable
/// and not configurable; `on` / `quiet` / `off` are the stack's business.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Display {
    Always,
    On,
    Quiet,
    Off,
}

impl Display {
    pub fn word(self) -> &'static str {
        match self {
            Display::Always => "always",
            Display::On => "on",
            Display::Quiet => "quiet",
            Display::Off => "off",
        }
    }
}

pub struct Fact {
    pub stat: Stat,
    /// The lattice-2 slug — the same string the ready-facts carry.
    pub key: &'static str,
    /// Raw facts or other rows this is computed from.
    pub from: &'static str,
    /// Where it lands on the line **today** (design/lattice-2.md's
    /// position vocabulary).
    pub position: Position,
    /// What kind of fact it is: of the line, of a census, of the look.
    pub office: Office,
    /// The lattice default before the caller stack speaks.
    pub display: Display,
    /// The `--sort KEY` that orders by this fact, or None. Only keys sort.rs
    /// actually accepts appear here — an aspiration would be a lie.
    pub sort: Option<&'static str>,
    /// Width in cells, as a word (`var`, `1`, `4×depth`, `cc`).
    pub width: &'static str,
    /// Config key that turns it on/off/quiet, or None (not overridable).
    pub toggle: Option<&'static str>,
    /// Format options, `*` marks the default; empty = none.
    pub formats: &'static str,
    /// Obtain implemented — a fact can be asked for only once built.
    pub built: bool,
}

// One row per fact, one line per row: the table is the artifact, so the
// constructor takes the fields positionally in the design table's order.
#[allow(clippy::too_many_arguments)]
const fn f(
    stat: Stat,
    key: &'static str,
    from: &'static str,
    position: Position,
    office: Office,
    display: Display,
    sort: Option<&'static str>,
    width: &'static str,
    toggle: Option<&'static str>,
    formats: &'static str,
    built: bool,
) -> Fact {
    Fact {
        stat,
        key,
        from,
        position,
        office,
        display,
        sort,
        width,
        toggle,
        formats,
        built,
    }
}

use Display::{Always, Off, On, Quiet};
use Office::{Census, Line, Look, Mark, Weight};
use Position::{
    AfterName, FarLeft, FarRight, InCell, LevelLocation, NameLocation, NameSuffix, NearRight,
    Unplaced,
};
use Stat::{Built, Deferred, Wants};

/// Every lattice-2 fact the machinery knows, grouped as the design table
/// groups them: the line's own identity, the quantities, time and
/// aliveness, identity/anomaly, the honesty marks, the claims cluster.
pub const FACTS: &[Fact] = &[
    // ── Initial (what a line *is*) ──────────────────────────────────────
    f(
        Built,
        "level-decorator",
        "depth-in-tree",
        LevelLocation,
        Line,
        Always,
        None,
        "4×depth",
        None,
        "box-light*",
        true,
    ),
    f(
        Built,
        "filename",
        "readdir",
        NameLocation,
        Line,
        Always,
        Some("name"),
        "var",
        None,
        "basename*",
        true,
    ),
    f(
        Built,
        "directory-glyph",
        "stat = dir",
        NameSuffix,
        Line,
        On,
        None,
        "1",
        None,
        "/*",
        true,
    ),
    f(
        Wants,
        "symlink-target",
        "lstat + readlink",
        AfterName,
        Line,
        On,
        None,
        "var",
        None,
        "target* (+ [broken])",
        true,
    ),
    f(
        Wants,
        "leaf-census",
        "census-agg(unlisted siblings)",
        NameLocation,
        Census,
        On,
        None,
        "var",
        None,
        "plus-census*",
        true,
    ),
    f(
        Built,
        "glob-template",
        "a real name-series at the level",
        NameLocation,
        Line,
        On,
        Some("name"),
        "var",
        Some("globify"),
        "range*",
        true,
    ),
    f(
        Wants,
        "glob-count",
        "glob-template members",
        NearRight,
        Line,
        On,
        None,
        "var",
        Some("globify"),
        "(N files)* — count cell pending",
        true,
    ),
    f(
        Deferred,
        "focus-match",
        "the focus set (a named path and its line)",
        Position::FarLeft,
        Mark,
        On,
        None,
        "1",
        None,
        "mark, on when focused (spelling open)",
        false,
    ),
    // ── Quantities ──────────────────────────────────────────────────────
    f(
        Built,
        "lines",
        "file: read; unexpanded dir: Σ subtree (mass)",
        FarRight,
        Line,
        On,
        Some("line-count"),
        "cc",
        Some("columns.line-count"),
        "count cell, unit 𝓁 (physical* / non-blank)",
        true,
    ),
    f(
        Built,
        "bytes",
        "file: st_size; dir: Σ descendants (unbuilt)",
        FarRight,
        Line,
        Quiet,
        Some("size"),
        "cc",
        Some("columns.size"),
        "count cell, unit B (1024)* / bytes",
        true,
    ),
    f(
        Wants,
        "dir-census",
        "an unexpanded dir's own children",
        NearRight,
        Census,
        Always,
        None,
        "var",
        None,
        "buckets*",
        true,
    ),
    f(
        Wants,
        "files",
        "dir: # descendant files",
        NearRight,
        Census,
        On,
        None,
        "var",
        None,
        "inside the census*",
        true,
    ),
    f(
        Wants,
        "dirs",
        "dir: # descendant dirs",
        NearRight,
        Census,
        On,
        None,
        "var",
        None,
        "inside the census*",
        true,
    ),
    f(
        Deferred,
        "tokens",
        "bytes × per-kind prior",
        FarRight,
        Line,
        Off,
        None,
        "cc",
        None,
        "count cell",
        false,
    ),
    // ── Time and aliveness ──────────────────────────────────────────────
    f(
        Wants,
        "mtime",
        "stat mtime (dir: its own inode)",
        FarRight,
        Line,
        Quiet,
        Some("recency"),
        "9",
        Some("columns.mtime"),
        "relative* / iso-8601 / epoch",
        true,
    ),
    f(
        Wants,
        "heat",
        "git log, commit-decay (half-life 7)",
        FarRight,
        Line,
        On,
        Some("heat"),
        "4",
        Some("columns.heat"),
        "score*",
        true,
    ),
    f(
        Built,
        "git-status",
        "git status --porcelain (worktree wins); ⊘ = gitignored",
        FarLeft,
        Line,
        On,
        None,
        "1",
        None,
        "{⊘ M A ⁇ R U D C T}*",
        true,
    ),
    f(
        Built,
        "initial-sha",
        "git log --name-status (A/C in window)",
        FarRight,
        Line,
        Off,
        None,
        "7",
        Some("columns.initial-sha"),
        "short* / h~n / full",
        true,
    ),
    f(
        Built,
        "latest-sha",
        "git log --name-status (last touch)",
        FarRight,
        Line,
        Off,
        None,
        "7",
        Some("columns.latest-sha"),
        "short* / h~n / full",
        true,
    ),
    f(
        Deferred,
        "created",
        "getattrlist / statx btime",
        FarRight,
        Line,
        Off,
        None,
        "9",
        Some("columns.created"),
        "iso-8601* / epoch",
        false,
    ),
    f(
        Deferred,
        "prior-name",
        "git log rename-follow",
        AfterName,
        Line,
        Quiet,
        None,
        "var",
        Some("columns.prior-name"),
        "was*",
        false,
    ),
    f(
        Deferred,
        "last-look-delta",
        "the caller's after-image",
        Position::FarLeft,
        Mark,
        Quiet,
        None,
        "1",
        None,
        "+ Δ −*",
        false,
    ),
    f(
        Deferred,
        "working-surface",
        "live working markers × recency",
        NearRight,
        Mark,
        Quiet,
        None,
        "1",
        None,
        "mark",
        false,
    ),
    // ── Identity and anomaly (quiet by nature) ──────────────────────────
    f(
        Built,
        "permissions",
        "stat mode (odd for level; special bits)",
        FarRight,
        Line,
        Quiet,
        None,
        "3–4",
        Some("columns.permissions"),
        "octal*",
        true,
    ),
    f(
        Built,
        "owner",
        "stat uid/gid (not you, not the plurality)",
        FarRight,
        Line,
        Quiet,
        None,
        "var",
        Some("columns.owner"),
        "name* / id",
        true,
    ),
    f(
        Built,
        "filetype",
        "stat type → magic → shebang → suffix",
        Unplaced,
        Line,
        On,
        None,
        "—",
        None,
        "suffix* / minor / major (feeds lines, census buckets, kind word)",
        true,
    ),
    f(
        Built,
        "filekind-word",
        "countable class vs the level's plurality, then the major",
        NearRight,
        Line,
        Quiet,
        None,
        "var",
        Some("columns.filekind"),
        "word*",
        true,
    ),
    f(
        Wants,
        "readme-title",
        "head-peek of the first important file",
        NearRight,
        Line,
        Off,
        None,
        "≤60",
        Some("readme-title"),
        "quoted*",
        true,
    ),
    f(
        Deferred,
        "linkcount",
        "stat nlink",
        FarRight,
        Line,
        Off,
        None,
        "2",
        Some("columns.linkcount"),
        "n*",
        false,
    ),
    f(
        Deferred,
        "cloud",
        "FS attribute (evicted / not hydrated)",
        AfterName,
        Mark,
        Quiet,
        None,
        "1",
        None,
        "evicted",
        false,
    ),
    // ── Honesty marks (facts about the look, loud by default) ───────────
    f(
        Wants,
        "count-marks",
        "walk/read/bounds state of the set",
        InCell,
        Mark,
        Always,
        None,
        "1",
        None,
        "≈ exact-grouped · ≥ floor · ~ estimated",
        true,
    ),
    f(
        Built,
        "denied",
        "readdir/stat EACCES/EPERM/io",
        NearRight,
        Mark,
        Always,
        None,
        "8",
        None,
        "[denied]* / [unreadable: io]",
        true,
    ),
    f(
        Built,
        "walk-bound",
        "the stat budget ran out here",
        NearRight,
        Mark,
        Always,
        None,
        "12",
        None,
        "[walk bound]*",
        true,
    ),
    f(
        Built,
        "cycle",
        "(dev,ino) already open on this path",
        NearRight,
        Mark,
        Always,
        None,
        "7",
        None,
        "[cycle]*",
        true,
    ),
    f(
        Built,
        "other-fs",
        "mount boundary, one-fs default",
        NearRight,
        Mark,
        Always,
        None,
        "10",
        None,
        "[other fs]*",
        true,
    ),
    f(
        Built,
        "gitignored",
        "the ignore stack (entry itself)",
        Unplaced,
        Mark,
        Always,
        None,
        "1",
        None,
        "JSON gitignored: true; glyph in git-status",
        true,
    ),
    f(
        Wants,
        "ignored-remainder",
        "gitignored files at an expanded level",
        NearRight,
        Mark,
        Always,
        None,
        "var",
        None,
        "[ignored×N]*",
        true,
    ),
    f(
        Deferred,
        "empty-dir",
        "readdir = ∅",
        NearRight,
        Mark,
        Always,
        None,
        "var",
        None,
        "spelling open (not ∅)",
        false,
    ),
    // ── Claims about a place (the cluster to refound next) ──────────────
    f(
        Wants,
        "has",
        "the furniture map's kind words + hidden mass",
        NearRight,
        Line,
        On,
        None,
        "var",
        Some("kinds"),
        "words*",
        true,
    ),
    f(
        Wants,
        "facet",
        "furniture plugins (.git, .github), verified",
        NearRight,
        Line,
        On,
        None,
        "var",
        Some("furniture"),
        "[git: …]* / [github: …]",
        true,
    ),
    // ── No place at all ─────────────────────────────────────────────────
    f(
        Built,
        "important",
        "the `important` config set (survival weight)",
        Unplaced,
        Weight,
        On,
        None,
        "0",
        Some("important"),
        "— (owns no column)",
        true,
    ),
    f(
        Built,
        "furniture-fate",
        "the furniture map's hide / omit / mark verdict",
        Unplaced,
        Weight,
        Always,
        None,
        "0",
        Some("furniture"),
        "— (decides child-slot vs parent-state)",
        true,
    ),
    // ── The look's own rows ─────────────────────────────────────────────
    f(
        Built,
        "headings",
        "the far-right column set",
        FarRight,
        Look,
        On,
        None,
        "var",
        None,
        "words*",
        true,
    ),
    f(
        Built,
        "root-facts",
        "the root node's own facts",
        NearRight,
        Look,
        On,
        None,
        "var",
        None,
        "same grammar as a node row*",
        true,
    ),
    f(
        Built,
        "config-drift",
        "the config caller stack",
        NearRight,
        Look,
        On,
        None,
        "var",
        None,
        "key = value (source) · --flag value (flag)*",
        true,
    ),
];

/// The state this caller's look gives a fact: config override, else the
/// lattice default.
pub fn state<'a>(fact: &'a Fact, cfg: &'a Resolved) -> &'a str {
    match fact.toggle.and_then(|k| cfg.won.get(k)) {
        Some((v, _)) => v.as_str(),
        None => fact.display.word(),
    }
}

/// The `--FACT` a caller might try, mapped to the honest refusal: no fact
/// has its own flag (design/aspect-lattice.md: almost no own flags); the
/// ask is the config path, or the fact is not built yet. A flag is matched
/// against the lattice slug *and* against the config key's own tail, so
/// `--size` still finds `bytes` (the fact the lattice now calls it).
pub fn flag_refusal(flag: &str) -> Option<String> {
    let bare = flag.trim_start_matches('-');
    let fact = FACTS.iter().find(|f| {
        f.key == bare
            || f.toggle
                .map(|k| k.rsplit('.').next() == Some(bare))
                .unwrap_or(false)
    })?;
    Some(match (fact.toggle, fact.built) {
        (Some(k), true) => {
            format!("facts have no own flags; ask via config: {k} = on (see `aspectus config`)")
        }
        (Some(k), false) => {
            format!("fact not built yet; when it lands the ask is config: {k} = on")
        }
        (None, _) => format!(
            "the {} fact is not toggleable (see `aspectus config` for the inventory)",
            fact.key
        ),
    })
}

/// The inventory block appended to `aspectus config` — lattice-2's fields,
/// with the live state of this caller's look folded in.
pub fn inventory(cfg: &Resolved) -> String {
    let mut out = String::from(
        "\nfacts (the aspect lattice, design/lattice-2.md; ask via config keys, e.g. columns.size = on)\n\
         position is where the fact paints *today*; stat: ✓ settled · ↬ settled, wants reimplementation · ⇥ deferred · (blank) provisional\n",
    );
    out.push_str(&format!(
        "  {:<1} {:<18} {:<9} {:<15} {:<7} {:<7} {:<10} {:<22} {}\n",
        "", "fact", "office", "position", "display", "state", "sort", "ask", "format"
    ));
    for fact in FACTS {
        let st = if fact.built {
            state(fact, cfg).to_string()
        } else {
            "unbuilt".into()
        };
        out.push_str(&format!(
            "  {:<1} {:<18} {:<9} {:<15} {:<7} {:<7} {:<10} {:<22} {}\n",
            fact.stat.glyph(),
            fact.key,
            fact.office.word(),
            fact.position.word(),
            fact.display.word(),
            st,
            fact.sort.unwrap_or("—"),
            fact.toggle.unwrap_or("—"),
            fmt_in_effect(fact, cfg),
        ));
    }
    out.push_str("\n  derived-from (what each fact is computed from):\n");
    for fact in FACTS {
        out.push_str(&format!(
            "    {:<18} {:<8} {}\n",
            fact.key, fact.width, fact.from
        ));
    }
    out
}

/// Formats with the one in effect starred (config `format.KEY` may move
/// it). The format key follows the *ask* — `columns.size` ⇒ `format.size` —
/// because the config vocabulary is a caller interface and does not rename
/// itself when the lattice re-slugs a fact (`size` ⇒ `bytes`).
fn fmt_in_effect(fact: &Fact, cfg: &Resolved) -> String {
    let tail = fact
        .toggle
        .and_then(|k| k.rsplit('.').next())
        .unwrap_or(fact.key);
    let chosen = crate::config::format_val(&cfg.won, &format!("format.{tail}"));
    match chosen {
        None => fact.formats.to_string(),
        Some(c) => fact
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
