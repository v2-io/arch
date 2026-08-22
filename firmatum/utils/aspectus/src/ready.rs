//! Ready-facts — stage 2 of the grid pipeline (design/grid-cleanup.md):
//!
//! ```text
//! raw-facts ──▶ ready-facts ──▶ cells ──▶ rows ──▶ paint
//! (Node)        (this module)   (columns.rs Slots/Row)     (columns.rs)
//! ```
//!
//! A ready-fact is one fact of one node, rendered to text by **its own**
//! small formatter, with quiet already applied and **no knowledge of its
//! neighbors** — no tab-stop, no width, no sibling. It carries where it
//! wants to land (`Position`, the vocabulary of design/lattice-2.md) and
//! what kind of fact it is (`Office`). Everything about *other* rows —
//! stops, widths, right edges, padding — belongs to paint, one layer down.
//!
//! This layer is deliberately output-identical to the strings the pre-grid
//! renderer built inline (prefactor, 2026-08-22): the formatters moved, the
//! bytes did not. New forms (the count cell, far-left glyphs, sub-rows)
//! land here, one fact at a time, in the slices after this one.

use crate::columns::{Cols, OwnerFmt, ShaFmt, SizeFmt, State, TimeFmt};
use crate::n_level::Node;

/// Where a ready-fact lands on the line (design/lattice-2.md §default
/// position). `FarLeft` and `Supplement` have no tenants yet; they are
/// declared so the slices that give them one plug in rather than restructure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Position {
    /// Stackable fixed-width columns before the hierarchy (no tenants yet).
    FarLeft,
    /// The tree's indent and branch glyphs.
    LevelLocation,
    /// Where a name stands — filename **or** glob-template **or**
    /// leaf-census, mutually exclusive.
    NameLocation,
    /// Fused to the name and painted with it (`/` on dirs).
    NameSuffix,
    /// Right of the name, before the aligned columns (`-> target`).
    AfterName,
    /// Stable stops after the name: censuses, facets, has, marks.
    NearRight,
    /// Between near-right and far-right (no tenants yet).
    Supplement,
    /// Right-aligned columns: counts, sizes, times, shas, the heat cluster.
    FarRight,
    /// A mark *inside* another fact's cell (lattice-2's own ninth word):
    /// the honesty marks `≈ ≥ ~` live in a count cell's `m` slot.
    InCell,
    /// Renders nothing of its own — it is consumed by other facts
    /// (filetype feeds census buckets and the kind word) or it is a weight
    /// the allocator reads (important). The row exists so the inventory can
    /// say so instead of inventing a place for it.
    Unplaced,
}

impl Position {
    pub fn word(self) -> &'static str {
        match self {
            Position::FarLeft => "far-left",
            Position::LevelLocation => "level-location",
            Position::NameLocation => "name-location",
            Position::NameSuffix => "name-suffix",
            Position::AfterName => "after-name",
            Position::NearRight => "near-right",
            Position::Supplement => "supplement",
            Position::FarRight => "far-right",
            Position::InCell => "in-cell",
            Position::Unplaced => "—",
        }
    }
}

/// What kind of fact this is (design/grid-cleanup.md §legend): a fact of
/// the line's own inode, a bucket in a census, a fact about the *look*
/// rather than the inode, or a weight that owns no place at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Office {
    Line,
    Census,
    Mark,
    Look,
    Weight,
}

impl Office {
    pub fn word(self) -> &'static str {
        match self {
            Office::Line => "line",
            Office::Census => "census",
            Office::Mark => "mark",
            Office::Look => "look",
            Office::Weight => "weight",
        }
    }
}

/// One fact of one node, ready to place. `text` is empty only for a
/// far-right column that is silent on this line (the cell keeps its slot so
/// the columns beside it stay aligned); near-right parts are absent, not
/// empty, when they have nothing to say.
#[derive(Debug, Clone)]
pub struct Ready {
    /// The lattice-2 fact this came from (`facts::FACTS[..].fact`).
    pub fact: &'static str,
    pub text: String,
    pub position: Position,
    pub office: Office,
}

impl Ready {
    fn new(fact: &'static str, position: Position, office: Office, text: String) -> Ready {
        Ready { fact, text, position, office }
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }
}

// ── name-location, name-suffix, after-name ──────────────────────────────

/// The name as it stands in the name-location: the basename, or the glob
/// template that collapses a series (globify writes the template into
/// `name`), or — on a remainder row — a leaf census standing where a name
/// stands. `dir_suffix` carries the `/` separately (it is painted with the
/// name, but it is its own lattice row).
pub fn name(n: &Node) -> Ready {
    let fact = if n.glob.is_some() { "glob-template" } else { "filename" };
    Ready::new(fact, Position::NameLocation, Office::Line, n.name.clone())
}

/// The dir glyph (lattice-2 `directory-glyph`, name-suffix). Painted inside
/// the name's color run, which is why it is a suffix and not after-name.
pub fn dir_suffix(n: &Node) -> Option<Ready> {
    (n.is_dir && !n.name.ends_with('/')).then(|| {
        Ready::new("directory-glyph", Position::NameSuffix, Office::Line, "/".to_string())
    })
}

/// A leaf census standing in the name-location (`[+ md×5 · txt×2]`) — the
/// allocator's remainder row for names this look did not list.
pub fn leaf_census(c: &crate::n_level::Census) -> Ready {
    Ready::new("leaf-census", Position::NameLocation, Office::Census, c.render_plus())
}

/// The symlink target completing the name (`-> target`, `[broken]` when it
/// resolves to nothing).
pub fn symlink_target(n: &Node) -> Option<Ready> {
    let target = n.link.as_ref()?;
    let broken = if n.link_broken { " [broken]" } else { "" };
    Some(Ready::new(
        "symlink-target",
        Position::AfterName,
        Office::Line,
        format!(" -> {target}{broken}"),
    ))
}

// ── near-right ──────────────────────────────────────────────────────────

/// The README's name for the place (design/readme-title.md; off unless
/// asked). Position is still open in the design — after-name vs near-right;
/// the implementation chose near-right and this row records that choice.
fn readme_title(n: &Node) -> Option<Ready> {
    let t = n.title.as_ref()?;
    Some(Ready::new(
        "readme-title",
        Position::NearRight,
        Office::Line,
        format!("\"{t}\""),
    ))
}

/// The filekind word — quiet: it speaks only where kind surprises (the
/// binary among the `.md`) or was asked on. `Node.kind_word` is already the
/// arbitrated answer.
fn filekind_word(n: &Node) -> Option<Ready> {
    let w = n.kind_word?;
    Some(Ready::new("filekind-word", Position::NearRight, Office::Line, w.to_string()))
}

/// What one collapsed series line stands for (design/globify.md). The
/// count-cell grammar of design/grid-cleanup.md is a later slice; this is
/// today's form.
fn glob_count(n: &Node) -> Option<Ready> {
    let g = n.glob.as_ref()?;
    let what = if n.is_dir { "dirs" } else { "files" };
    Some(Ready::new(
        "glob-count",
        Position::NearRight,
        Office::Line,
        format!("({} {what})", g.count),
    ))
}

/// An unexpanded dir's own children, by subject (`[dir×3 ≈120f · md×31]`).
fn dir_census(n: &Node) -> Option<Ready> {
    let c = n.leftover.as_ref()?;
    let text = c.render();
    (!text.is_empty()).then(|| Ready::new("dir-census", Position::NearRight, Office::Census, text))
}

/// Mass's headline number on an unexpanded dir's line: subtree text lines
/// (the files already live in the census's dir bucket). Rides the census —
/// a dir this look expanded says its lines through its children.
fn mass_lines(n: &Node) -> Option<Ready> {
    n.leftover.as_ref()?;
    let m = n.mass.as_ref()?;
    (m.lines > 0).then(|| {
        Ready::new(
            "lines",
            Position::NearRight,
            Office::Line,
            format!("{}{} lines", m.mark(), crate::n_level::group_lines(m.lines)),
        )
    })
}

/// Gitignored files at an *expanded* level: the typed remainder, so the
/// level never pretends they are not there. (A cut level's count rides
/// inside its census instead — hence the `leftover.is_none()` gate.)
fn ignored_remainder(n: &Node) -> Option<Ready> {
    (n.leftover.is_none() && n.ignored_files > 0).then(|| {
        Ready::new(
            "ignored-remainder",
            Position::NearRight,
            Office::Mark,
            format!("[ignored×{}]", n.ignored_files),
        )
    })
}

/// Specialized furniture facets, already phrased by their plugins
/// (`[git: br<main> @sha]`, `[github: N workflows]`) — an identity claim,
/// verified, so it leads the claims block.
fn facets(n: &Node) -> Vec<Ready> {
    n.facets
        .iter()
        .map(|f| Ready::new("facet", Position::NearRight, Office::Line, format!("[{f}]")))
        .collect()
}

/// The contents claim: what the furniture map says this place holds. A
/// hidden dir's magnitude rides its word (`archive ≈127f`) so presence
/// survives hiding.
fn has_block(n: &Node) -> Option<Ready> {
    if n.kinds.is_empty() {
        return None;
    }
    let words: Vec<String> = n
        .kinds
        .iter()
        .map(|k| match n.has_counts.iter().find(|(hk, _, _)| hk == k) {
            Some((_, files, bounded)) => {
                let mark = if *bounded { "≥" } else { "≈" };
                format!("{k} {mark}{files}f")
            }
            None => k.clone(),
        })
        .collect();
    Some(Ready::new(
        "has",
        Position::NearRight,
        Office::Line,
        format!("[has: {}]", words.join(", ")),
    ))
}

/// The gitignored glyph — the no-color carrier of the per-entry status
/// (TTY adds dim as the redundant overlay). Spelling provisional
/// (design/gitignore-bodies.md); design/grid-cleanup.md retires this
/// marks-column glyph into the far-left git-status cell in a later slice.
pub const IGNORED_GLYPH: &str = "⊘";

/// The walk's confessions about this line — facts about the *look*, not the
/// inode, and never quietable: each one says where the picture stops.
fn look_marks(n: &Node) -> Vec<Ready> {
    let mut parts = Vec::new();
    let mark = |fact, text: &str| Ready::new(fact, Position::NearRight, Office::Mark, text.to_string());
    if n.ignored {
        parts.push(mark("gitignored", IGNORED_GLYPH));
    }
    if n.denied {
        parts.push(mark("denied", "[denied]"));
    }
    if n.cycle {
        // A link back to a place already being expanded on this path —
        // recursion refused, never a hang (spelling awaits ratification).
        parts.push(mark("cycle", "[cycle]"));
    }
    if n.other_fs {
        // A filesystem boundary, not an empty dir, stopped the walk here.
        parts.push(mark("other-fs", "[other fs]"));
    }
    if n.iter_err {
        parts.push(mark("denied", "[unreadable: io]"));
    }
    // Membership stays exact under a cut, so the census carries no `≥`;
    // this mark is what says the dir was cut short of the asked depth.
    if n.cut {
        parts.push(mark("walk-bound", "[walk bound]"));
    }
    parts
}

/// Every near-right part of one node, in the fixed order the paint joins
/// them in: title · kind-word · glob-count · census · mass · ignored
/// remainder · facets · has · look-marks. (design/grid-cleanup.md proposes
/// a per-part-kind sub-column order for a later slice; today the order is
/// the join order, made explicit here instead of implicit in a string.)
pub fn near_right(n: &Node) -> Vec<Ready> {
    let mut parts = Vec::new();
    parts.extend(readme_title(n));
    parts.extend(filekind_word(n));
    parts.extend(glob_count(n));
    parts.extend(dir_census(n));
    parts.extend(mass_lines(n));
    parts.extend(ignored_remainder(n));
    parts.extend(facets(n));
    parts.extend(has_block(n));
    parts.extend(look_marks(n));
    parts
}

/// Facts that hold no place on the line (lattice-2 keeps the row so the
/// inventory can say so). `important` is survival weight, not a column.
pub fn weights(n: &Node) -> Vec<Ready> {
    let mut v = Vec::new();
    if n.important {
        v.push(Ready::new("important", Position::Unplaced, Office::Weight, String::new()));
    }
    v
}

// ── far-right ───────────────────────────────────────────────────────────

/// One far-right column of this look: which fact it carries and the word
/// that heads it. Cells and headings are generated from the same list, in
/// the same order, so the two cannot drift apart.
pub struct Column {
    pub fact: &'static str,
    pub heading: &'static str,
}

/// The far-right columns this look carries, left to right.
pub fn columns(cols: &Cols) -> Vec<Column> {
    let mut v = Vec::new();
    let mut push = |fact, heading| v.push(Column { fact, heading });
    if cols.line_count {
        push("lines", "lines");
    }
    if cols.perms.claims_column() {
        push("permissions", "perms");
    }
    if cols.owner.claims_column() {
        push("owner", "owner");
    }
    if cols.size.claims_column() {
        push("bytes", "size");
    }
    if cols.mtime.claims_column() {
        push("mtime", "mtime");
    }
    if cols.intro_sha {
        // Heading spellings provisional until Joseph ratifies.
        push("initial-sha", "initial-sha");
    }
    if cols.latest_sha {
        push("latest-sha", "latest-sha");
    }
    if cols.heat {
        push("heat", "heat · age");
    }
    v
}

/// A quiet fact speaks only where it surprises (quiet.rs' cold law); a
/// silent line still yields the empty cell, so the columns beside it keep
/// their places.
fn quiet(st: State, speaks: bool, val: Option<String>) -> String {
    match st {
        State::On => val.unwrap_or_default(),
        State::Quiet if speaks => val.unwrap_or_default(),
        _ => String::new(),
    }
}

/// The far-right cells of one node, in `columns()` order — one per column,
/// empty where the fact has nothing to say on this line.
pub fn far_right(n: &Node, cols: &Cols) -> Vec<Ready> {
    let mut cells = Vec::with_capacity(cols.active());
    let mut push = |fact, text: String| {
        cells.push(Ready::new(fact, Position::FarRight, Office::Line, text))
    };
    if cols.line_count {
        push("lines", n.lines.map(|l| l.to_string()).unwrap_or_default());
    }
    if cols.perms.claims_column() {
        push("permissions", quiet(cols.perms, n.q.mode, n.mode.map(fmt_mode)));
    }
    if cols.owner.claims_column() {
        push(
            "owner",
            quiet(cols.owner, n.q.owner || n.q.group, fmt_owner(n, cols.owner_fmt)),
        );
    }
    if cols.size.claims_column() {
        push(
            "bytes",
            quiet(cols.size, n.q.size, n.size.map(|s| fmt_size(s, cols.size_fmt))),
        );
    }
    if cols.mtime.claims_column() {
        // The heat cluster already carries this line's mtime as `· age`;
        // a quiet mtime speaking beside it would say the same thing twice
        // (format-consistency steer, 2026-08-14). An explicit `on` still
        // renders everywhere — a full column asked for is a full column.
        let redundant = cols.heat && n.mtime.is_some() && (n.heat.is_some() || n.git_ts.is_some());
        push(
            "mtime",
            quiet(
                cols.mtime,
                n.q.mtime && !redundant,
                n.mtime.map(|t| fmt_mtime(t, cols.mtime_fmt, cols.now)),
            ),
        );
    }
    if cols.intro_sha {
        push("initial-sha", fmt_sha(&n.intro, cols.intro_fmt));
    }
    if cols.latest_sha {
        push("latest-sha", fmt_sha(&n.touch, cols.latest_fmt));
    }
    if cols.heat {
        push("heat", heat_cluster(n, cols.now));
    }
    cells
}

/// Two aliveness facts as one glance-stop: `1.01 · 13.6d ago`. A git-known
/// line without a score (noise basename, history says nothing hot) still
/// carries its age here — score honestly absent, the age in the one time
/// register — so quiet's mtime never re-speaks it in a stray column (close
/// audit 2026-08-14: the silenced Cargo.toml had become the loudest row).
fn heat_cluster(n: &Node, now: i64) -> String {
    match (n.heat, n.mtime) {
        (Some(h), Some(t)) => format!("{h:.2} · {}", rel_age(now, t)),
        (Some(h), None) => format!("{h:.2}"),
        (None, Some(t)) if n.git_ts.is_some() => format!(" · {}", rel_age(now, t)),
        _ => String::new(),
    }
}

// ── the individual formatters ───────────────────────────────────────────

/// `human` size: ≤4 glyphs of number+unit, deterministic. 1024-base.
fn human_size(n: u64) -> String {
    const UNITS: [&str; 5] = ["B", "K", "M", "G", "T"];
    let mut v = n as f64;
    let mut u = 0;
    while v >= 1000.0 && u + 1 < UNITS.len() {
        v /= 1024.0;
        u += 1;
    }
    if u == 0 {
        format!("{n}B")
    } else if v < 10.0 {
        format!("{v:.1}{}", UNITS[u])
    } else {
        format!("{v:.0}{}", UNITS[u])
    }
}

fn fmt_size(n: u64, f: SizeFmt) -> String {
    match f {
        SizeFmt::Human => human_size(n),
        SizeFmt::Bytes => n.to_string(),
    }
}

/// Octal, three digits — four when a special bit is set (`4755`).
fn fmt_mode(mode: u32) -> String {
    if mode & 0o7000 != 0 {
        format!("{mode:04o}")
    } else {
        format!("{:03o}", mode & 0o777)
    }
}

/// Owner cell: name (id fallback / `format.owner = id`); the group leg
/// appends `:group` only when it is the surprising half.
fn fmt_owner(n: &Node, f: OwnerFmt) -> Option<String> {
    let uid = n.uid?;
    let user = match f {
        OwnerFmt::Id => uid.to_string(),
        OwnerFmt::Name => crate::quiet::user_name(uid)
            .map(str::to_string)
            .unwrap_or_else(|| uid.to_string()),
    };
    if n.q.group
        && let Some(gid) = n.gid
    {
        let group = match f {
            OwnerFmt::Id => gid.to_string(),
            OwnerFmt::Name => crate::quiet::group_name(gid)
                .map(str::to_string)
                .unwrap_or_else(|| gid.to_string()),
        };
        return Some(format!("{user}:{group}"));
    }
    Some(user)
}

fn fmt_mtime(secs: i64, f: TimeFmt, now: i64) -> String {
    match f {
        TimeFmt::Relative => rel_age(now, secs),
        TimeFmt::Epoch => secs.to_string(),
        TimeFmt::Iso8601 => {
            if secs < 0 {
                // Pre-epoch mtimes exist in the wild; epoch form is exact.
                return secs.to_string();
            }
            let t = std::time::UNIX_EPOCH + std::time::Duration::from_secs(secs as u64);
            crate::overview::stamp_utc(t)
        }
    }
}

fn fmt_sha(fact: &Option<(String, u32)>, f: ShaFmt) -> String {
    match fact {
        None => String::new(),
        Some((sha, n)) => match f {
            ShaFmt::Full => sha.clone(),
            ShaFmt::Short => sha.chars().take(7).collect(),
            ShaFmt::HN => {
                if *n == 0 {
                    "H".to_string()
                } else {
                    format!("H~{n}")
                }
            }
        },
    }
}

/// Human-relative age (`13.6d ago`) — the delta register for aliveness
/// (design/heat.md affordances; git-heat's shipped look).
pub fn rel_age(now: i64, then: i64) -> String {
    let d = now - then;
    if d < 0 {
        return "future".to_string();
    }
    let d = d as f64;
    if d < 3600.0 {
        format!("{}m ago", (d / 60.0) as u64)
    } else if d < 172_800.0 {
        format!("{:.1}h ago", d / 3600.0)
    } else if d < 1_209_600.0 {
        format!("{:.1}d ago", d / 86_400.0)
    } else if d < 4_838_400.0 {
        format!("{:.1}w ago", d / 604_800.0)
    } else if d < 63_113_904.0 {
        format!("{:.1}mo ago", d / 2_629_746.0)
    } else {
        format!("{:.1}y ago", d / 31_556_952.0)
    }
}
