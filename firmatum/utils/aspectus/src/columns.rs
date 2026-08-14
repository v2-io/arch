//! Columns — how facts land on a line (design/columns.md).
//!
//! Three duties: selection (which facts are in this look — resolved through
//! the config caller stack against the lattice defaults), placement (a
//! column beside the name vs INFO hanging right of it — the lattice cell is
//! the law), and format (lattice options, starred default, `format.FACT`
//! overrides). Placement follows the classes of design/shorthand.md
//! (decoration / near-right / far-right render today; far-left, near-left,
//! and glyph-block are declared in `facts::Placement` and plug in here).
//! This module also owns the paint path: all non-name material
//! lands at computed pseudo-tab-stops — pure functions of the look's
//! content, never terminal width (alignment decided 2026-08-14). Column
//! values are additionally aligned per-column, right-edge (position carries
//! meaning; git-heat's look is the prior art).

use crate::n_level::Node;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SizeFmt {
    Human,
    Bytes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeFmt {
    Iso8601,
    Epoch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineFmt {
    Physical,
    NonBlank,
}

/// The selection for one look, already arbitrated by config.
#[derive(Debug, Clone, Copy)]
pub struct Cols {
    pub line_count: bool,
    pub size: bool,
    pub mtime: bool,
    /// The `score · age` right-cluster (design/heat.md); in-repo only by
    /// nature — outside git no node has heat and the column is silent.
    pub heat: bool,
    pub size_fmt: SizeFmt,
    pub mtime_fmt: TimeFmt,
    pub line_fmt: LineFmt,
    /// The look's own stamp time (secs) — ages are relative to the look,
    /// not to each render.
    pub now: i64,
}

impl Default for Cols {
    fn default() -> Self {
        Cols {
            line_count: true,
            size: false,
            mtime: false,
            heat: true,
            size_fmt: SizeFmt::Human,
            mtime_fmt: TimeFmt::Iso8601,
            line_fmt: LineFmt::Physical,
            now: 0,
        }
    }
}

impl Cols {
    fn active(&self) -> usize {
        usize::from(self.line_count)
            + usize::from(self.size)
            + usize::from(self.mtime)
            + usize::from(self.heat)
    }
}

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

fn fmt_mtime(secs: i64, f: TimeFmt) -> String {
    match f {
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

/// One printed line, before alignment. Fields follow the placement classes
/// of design/shorthand.md: decoration is fused to the name, `cells` are the
/// far-right aligned columns, `rest` is the near-right material at the
/// tab-stop. Far-left / near-left / glyph-block have no tenants yet and
/// plug in here when they do.
struct Row {
    /// Tree glyphs and branch, uncolored.
    left_prefix: String,
    /// The name itself (trailing `/` on dirs), uncolored.
    name: String,
    /// Decoration fused to the name (`-> target`), uncolored, part of the
    /// name column for tab-stop purposes.
    deco: String,
    color_dir: bool,
    /// One cell per active column, empty when the fact has nothing to say
    /// (a quiet gap keeps the other columns aligned; no placeholder glyph).
    cells: Vec<String>,
    /// Near-right: censuses, facets, kind spot, look-marks.
    rest: String,
}

impl Row {
    fn name_width(&self) -> usize {
        self.left_prefix.chars().count() + self.name.chars().count() + self.deco.chars().count()
    }

    fn has_right(&self) -> bool {
        !self.rest.is_empty() || self.cells.iter().any(|c| !c.is_empty())
    }
}

/// Non-name material lands at a computed pseudo-tab-stop: a pure function
/// of the look's content (longest name column, capped), never terminal
/// width (design/columns.md, alignment decided 2026-08-14). A name past
/// the cap goes ragged on its own line only. Column cells are then aligned
/// right-edge per column across the look.
const STOP_CAP: usize = 48;
const GAP: usize = 2;

fn paint(rows: Vec<Row>, color: bool, ncols: usize) -> String {
    let stop = rows
        .iter()
        .filter(|r| r.has_right())
        .map(|r| r.name_width() + GAP)
        .max()
        .unwrap_or(0)
        .min(STOP_CAP);
    let widths: Vec<usize> = (0..ncols)
        .map(|i| {
            rows.iter()
                .map(|r| r.cells.get(i).map(|c| c.chars().count()).unwrap_or(0))
                .max()
                .unwrap_or(0)
        })
        .collect();
    let mut out = String::new();
    for r in rows {
        let mut line = String::new();
        line.push_str(&r.left_prefix);
        let painted = if r.color_dir {
            crate::color::dir(&r.name, color)
        } else {
            r.name.clone()
        };
        line.push_str(&painted);
        line.push_str(&r.deco);
        if r.has_right() {
            let w = r.name_width();
            let pad = if w + GAP <= stop { stop - w } else { GAP };
            line.push_str(&" ".repeat(pad));
            let mut first = true;
            for (i, width) in widths.iter().enumerate() {
                if *width == 0 {
                    continue;
                }
                if !first {
                    line.push_str("  ");
                }
                first = false;
                let cell = r.cells.get(i).map(String::as_str).unwrap_or("");
                let cw = cell.chars().count();
                line.push_str(&" ".repeat(width - cw));
                line.push_str(cell);
            }
            if !r.rest.is_empty() {
                if !first {
                    line.push_str("  ");
                }
                line.push_str(&r.rest);
            }
        }
        // Alignment gaps for silent cells must not leave trailing bytes.
        out.push_str(line.trim_end());
        out.push('\n');
    }
    out
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

/// The column cells for one node, in lattice order (line-count, size,
/// mtime, then the heat cluster at the far right). A fact with nothing to
/// say for this line yields an empty cell.
fn cells_of(n: &Node, cols: &Cols) -> Vec<String> {
    let mut cells = Vec::with_capacity(cols.active());
    if cols.line_count {
        cells.push(n.lines.map(|l| l.to_string()).unwrap_or_default());
    }
    if cols.size {
        cells.push(n.size.map(|s| fmt_size(s, cols.size_fmt)).unwrap_or_default());
    }
    if cols.mtime {
        cells.push(
            n.mtime
                .map(|t| fmt_mtime(t, cols.mtime_fmt))
                .unwrap_or_default(),
        );
    }
    if cols.heat {
        // Two aliveness facts as one glance-stop: `1.01 · 13.6d ago`.
        cells.push(match (n.heat, n.mtime) {
            (Some(h), Some(t)) => format!("{h:.2} · {}", rel_age(cols.now, t)),
            (Some(h), None) => format!("{h:.2}"),
            _ => String::new(),
        });
    }
    cells
}

/// The gathering spot for what this place *is*: specialized facets first,
/// then the kind list (src/def-furniture.md). Empty claims print nothing.
fn state_marks(n: &Node) -> Vec<String> {
    let mut parts = Vec::new();
    for f in &n.facets {
        parts.push(format!("[{f}]"));
    }
    if !n.kinds.is_empty() {
        parts.push(format!("[kind: {}]", n.kinds.join(", ")));
    }
    parts
}

/// INFO to hang on a name: the walk's confession about this line.
fn info_marks(n: &Node) -> Vec<String> {
    let mut parts = Vec::new();
    if n.denied {
        parts.push("[denied]".to_string());
    }
    if n.cycle {
        // A link back to a place already being expanded on this path —
        // recursion refused, never a hang (spelling awaits ratification).
        parts.push("[cycle]".to_string());
    }
    if n.other_fs {
        // A filesystem boundary, not an empty dir, stopped the walk here.
        parts.push("[other fs]".to_string());
    }
    if n.iter_err {
        parts.push("[unreadable: io]".to_string());
    }
    // Membership stays exact under a cut, so the census carries no `≥`;
    // the mark is what says this dir was cut short of the requested depth.
    if n.cut {
        parts.push("[walk bound]".to_string());
    }
    parts
}

/// Decoration fused to the name: the symlink target completes the name
/// (design/shorthand.md — decoration class).
fn deco_of(n: &Node) -> String {
    match &n.link {
        Some(target) => {
            let broken = if n.link_broken { " [broken]" } else { "" };
            format!(" -> {target}{broken}")
        }
        None => String::new(),
    }
}

/// The near-right material for one node: dir census, facets, kind spot,
/// look-marks — at the computed tab-stop.
fn rest_of(n: &Node) -> String {
    let mut parts = Vec::new();
    if let Some(c) = &n.leftover {
        let extra = c.render();
        if !extra.is_empty() {
            parts.push(extra);
        }
        // Mass's headline number: subtree text lines on the unexpanded
        // dir's line (files already live in the census's dir bucket).
        if let Some(m) = &n.mass
            && m.lines > 0
        {
            let mark = if m.bounded { "≥" } else { "≈" };
            parts.push(format!(
                "{mark}{} lines",
                crate::n_level::group_lines(m.lines)
            ));
        }
    }
    parts.extend(state_marks(n));
    parts.extend(info_marks(n));
    parts.join("  ")
}

/// The whole look. Header is two lines — stamp, then root — so the root
/// line sits directly above its children (design/overview-invariants.md,
/// decided 2026-08-14).
pub fn render(root_path: &str, tree: &Node, color: bool, stamp: &str, cols: &Cols) -> String {
    let mut root = root_path.to_string();
    if tree.is_dir && !root.ends_with('/') {
        root.push('/');
    }
    let ncols = cols.active();
    let mut rows = vec![
        Row {
            left_prefix: String::new(),
            name: stamp.to_string(),
            deco: String::new(),
            color_dir: false,
            cells: vec![String::new(); ncols],
            rest: String::new(),
        },
        Row {
            left_prefix: String::new(),
            name: root,
            deco: deco_of(tree),
            color_dir: true,
            cells: cells_of(tree, cols),
            rest: rest_of(tree),
        },
    ];
    emit(&tree.children, tree.omitted.as_ref(), "", cols, &mut rows);
    paint(rows, color, ncols)
}

fn emit(
    kids: &[Node],
    omitted: Option<&crate::n_level::Census>,
    prefix: &str,
    cols: &Cols,
    out: &mut Vec<Row>,
) {
    let has_om = omitted.map(|c| !c.is_empty()).unwrap_or(false);
    let total = kids.len() + usize::from(has_om);
    for (i, k) in kids.iter().enumerate() {
        let last = i + 1 == total;
        let branch = if last { "└── " } else { "├── " };
        let mut n = k.name.clone();
        if k.is_dir && !n.ends_with('/') {
            n.push('/');
        }
        out.push(Row {
            left_prefix: format!("{prefix}{branch}"),
            name: n,
            deco: deco_of(k),
            color_dir: k.is_dir,
            cells: cells_of(k, cols),
            rest: rest_of(k),
        });
        if !k.children.is_empty() || k.omitted.is_some() {
            let next = if last {
                format!("{prefix}    ")
            } else {
                format!("{prefix}│   ")
            };
            emit(&k.children, k.omitted.as_ref(), &next, cols, out);
        }
    }
    if has_om
        && let Some(c) = omitted
    {
        out.push(Row {
            left_prefix: format!("{prefix}└── "),
            name: c.render_plus(),
            deco: String::new(),
            color_dir: false,
            cells: vec![String::new(); cols.active()],
            rest: String::new(),
        });
    }
}
