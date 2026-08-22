//! Columns — selection, format, and the row grammar (design/columns.md,
//! design/grid-cleanup.md).
//!
//! Duties, in the pipeline's order:
//!
//! 1. **Selection / format** (`Cols`): which facts are in this look and in
//!    which spelling, resolved through the config caller stack against the
//!    lattice defaults (design/lattice-2.md is the inventory; `facts.rs`
//!    prints it).
//! 2. **Ready-facts**: each fact rendered by its own formatter, quiet
//!    applied, no neighbor knowledge — `ready.rs`.
//! 3. **Cells / rows** (`Slots`, `Row`): the ready-facts land in *named
//!    slots in a fixed order* — far-left · level-location · name-location ·
//!    name-suffix · after-name · near-right · supplement · far-right (the
//!    positions of design/lattice-2.md). Far-left and supplement have no
//!    tenants yet; a node emits exactly one row today (sub-rows are a later
//!    slice).
//! 4. **Paint**: the only layer that knows about other rows — the computed
//!    pseudo-tab-stop, per-column right edges, heading alignment, color.
//!    Stops are pure functions of the look's content, never terminal width
//!    (alignment decided 2026-08-14; git-heat's look is the prior art).

use crate::count_cell;
use crate::n_level::Node;
use crate::ready::{self, Position, Ready};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SizeFmt {
    Human,
    Bytes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeFmt {
    Iso8601,
    Epoch,
    /// Age relative to the look's stamp (`2.2h ago`) — the default text
    /// spelling, so mtime and the heat cluster's age read as one register
    /// (format-consistency steer, 2026-08-14; JSON stays iso-8601).
    Relative,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineFmt {
    Physical,
    NonBlank,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OwnerFmt {
    Name,
    Id,
}

/// The sha-fact spellings (lattice initial-sha / latest-sha row:
/// `short* / H~N / full`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaFmt {
    Short,
    /// Commits behind HEAD: `H` for HEAD itself, `H~4` behind.
    HN,
    Full,
}

/// One fact's selection state (config `columns.FACT`). `Quiet` renders a
/// value only on lines where the fact surprises (`Node.q`, quiet.rs); the
/// silent lines keep an empty cell so the other columns stay aligned.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    On,
    Off,
    Quiet,
}

impl State {
    pub fn parse(s: &str) -> Option<State> {
        match s {
            "on" => Some(State::On),
            "off" => Some(State::Off),
            "quiet" => Some(State::Quiet),
            _ => None,
        }
    }

    pub fn claims_column(self) -> bool {
        self != State::Off
    }
}

/// The selection for one look, already arbitrated by config.
#[derive(Debug, Clone, Copy)]
pub struct Cols {
    pub line_count: bool,
    pub size: State,
    pub mtime: State,
    pub perms: State,
    pub owner: State,
    /// The `score · age` right-cluster (design/heat.md); in-repo only by
    /// nature — outside git no node has heat and the column is silent.
    pub heat: bool,
    /// The sha facts (compose-only, lattice OFF): the commit that
    /// introduced a file / last touched it, from heat's log pass.
    pub intro_sha: bool,
    pub latest_sha: bool,
    pub size_fmt: SizeFmt,
    pub mtime_fmt: TimeFmt,
    pub line_fmt: LineFmt,
    pub owner_fmt: OwnerFmt,
    pub intro_fmt: ShaFmt,
    pub latest_fmt: ShaFmt,
    /// The look's own stamp time (secs) — ages are relative to the look,
    /// not to each render.
    pub now: i64,
}

impl Default for Cols {
    fn default() -> Self {
        Cols {
            line_count: true,
            size: State::Quiet,
            mtime: State::Quiet,
            perms: State::Quiet,
            owner: State::Quiet,
            heat: true,
            intro_sha: false,
            latest_sha: false,
            size_fmt: SizeFmt::Human,
            mtime_fmt: TimeFmt::Iso8601,
            line_fmt: LineFmt::Physical,
            owner_fmt: OwnerFmt::Name,
            intro_fmt: ShaFmt::Short,
            latest_fmt: ShaFmt::Short,
            now: 0,
        }
    }
}

impl Cols {
    pub fn active(&self) -> usize {
        ready::columns(self).len()
    }

    /// The heat cluster's cell index, for the paint path's sub-column
    /// alignment (score under `heat`, age under `age`).
    fn cluster_idx(&self) -> Option<usize> {
        self.heat.then(|| self.active() - 1)
    }
}

/// The named slots one node's ready-facts land in, in the fixed left-to-
/// right order of design/lattice-2.md. A slot with no tenant is empty, not
/// absent: the grammar is the same on every row, which is what lets paint
/// align across the look.
#[derive(Default)]
struct Slots {
    /// Fixed-width blocks before the hierarchy — no tenants yet (the
    /// git-status glyph is the first, a later slice).
    far_left: Vec<Ready>,
    /// The tree's indent and branch glyphs.
    level: String,
    /// Name **or** glob-template **or** leaf-census — mutually exclusive.
    name: String,
    /// Painted inside the name's color run (`/` on dirs).
    name_suffix: String,
    /// `-> target`, uncolored, still part of the name column's width.
    after_name: String,
    /// The typed near-right parts, in order — one string per part today,
    /// each one still its own ready-fact rather than a pre-joined blob.
    near_right: Vec<Ready>,
    /// Between near-right and far-right — no tenants yet (design/lattice-2
    /// names it; nothing has been assigned to it).
    supplement: Vec<Ready>,
    /// One cell per active far-right column, empty where the fact is silent
    /// on this line (the gap keeps the neighbors aligned; no placeholder).
    far_right: Vec<Ready>,
}

impl Slots {
    /// A node's slots, filled from its ready-facts.
    fn of(n: &Node, cols: &Cols, level: String) -> Slots {
        Slots {
            far_left: Vec::new(),
            level,
            name: ready::name(n).text,
            name_suffix: ready::dir_suffix(n).map(|r| r.text).unwrap_or_default(),
            after_name: ready::symlink_target(n).map(|r| r.text).unwrap_or_default(),
            near_right: ready::near_right(n),
            supplement: Vec::new(),
            far_right: ready::far_right(n, cols),
        }
    }

    /// A row that carries only text in the name slot (the stamp line, the
    /// root path line, the pre-joined root facts line, a leaf census).
    fn bare(name: String, ncols: usize) -> Slots {
        Slots {
            name,
            far_right: vec![blank(); ncols],
            ..Slots::default()
        }
    }

    fn near_right_text(&self) -> String {
        let parts: Vec<&str> = self
            .near_right
            .iter()
            .chain(self.supplement.iter())
            .filter(|r| !r.is_empty())
            .map(|r| r.text.as_str())
            .collect();
        parts.join("  ")
    }
}

fn blank() -> Ready {
    Ready {
        fact: "",
        text: String::new(),
        position: Position::FarRight,
        office: crate::ready::Office::Line,
    }
}

/// One printed line: the slots plus how this row wants to be painted.
struct Row {
    slots: Slots,
    color_dir: bool,
    /// This entry is gitignored: the name renders dimmed on a TTY (the
    /// glyph in near-right is the carrier; dim is the redundant overlay —
    /// steward ask 2026-08-14).
    dim: bool,
    /// The headings line (design/columns.md §Column headings): its cells
    /// name columns instead of carrying values; painted dim, dropped when
    /// no value column below it exists, excluded from value widths.
    heading: bool,
}

impl Row {
    fn plain(name: String, ncols: usize) -> Row {
        Row {
            slots: Slots::bare(name, ncols),
            color_dir: false,
            dim: false,
            heading: false,
        }
    }

    /// The width of everything left of the tab-stop.
    fn name_width(&self) -> usize {
        let s = &self.slots;
        s.far_left
            .iter()
            .map(|r| r.text.chars().count())
            .sum::<usize>()
            + s.level.chars().count()
            + s.name.chars().count()
            + s.name_suffix.chars().count()
            + s.after_name.chars().count()
    }

    fn cell(&self, i: usize) -> &str {
        self.slots
            .far_right
            .get(i)
            .map(|r| r.text.as_str())
            .unwrap_or("")
    }

    fn has_right(&self) -> bool {
        !self.slots.near_right_text().is_empty()
            || self.slots.far_right.iter().any(|r| !r.is_empty())
    }
}

/// Non-name material lands at a computed pseudo-tab-stop: a pure function
/// of the look's content (longest name column, capped), never terminal
/// width (design/columns.md, alignment decided 2026-08-14). A name past
/// the cap goes ragged on its own line only. Column cells are then aligned
/// right-edge per column across the look.
const STOP_CAP: usize = 48;
const GAP: usize = 2;

fn paint(mut rows: Vec<Row>, color: bool, ncols: usize, cluster: Option<usize>) -> String {
    // The heat cluster aligns as two sub-columns — score under `heat`,
    // age under `age`, the `·` at one column for every row — derived here,
    // in the same layout pass that positions the cells, so heading and
    // values cannot drift apart (heading-alignment fix, 2026-08-14).
    if let Some(ci) = cluster {
        let (mut lw, mut rw) = (0usize, 0usize);
        for r in rows.iter().filter(|r| !r.heading) {
            let c = r.cell(ci);
            if !c.is_empty() {
                match c.split_once(" · ") {
                    Some((l, a)) => {
                        lw = lw.max(l.chars().count());
                        rw = rw.max(a.chars().count());
                    }
                    None => lw = lw.max(c.chars().count()),
                }
            }
        }
        if lw > 0 && rw > 0 {
            for r in &mut rows {
                let Some(cell) = r.slots.far_right.get_mut(ci) else {
                    continue;
                };
                if cell.is_empty() {
                    continue;
                }
                cell.text = match cell.text.split_once(" · ") {
                    Some((l, a)) => format!("{l:>lw$} · {a:>rw$}"),
                    None => format!("{:>lw$}{}", cell.text.clone(), " ".repeat(3 + rw)),
                };
            }
        }
    }
    let stop = rows
        .iter()
        .filter(|r| r.has_right())
        .map(|r| r.name_width() + GAP)
        .max()
        .unwrap_or(0)
        .min(STOP_CAP);
    // A column exists only where a *value* row fills it; the headings line
    // never conjures a column (it teaches, it does not claim).
    let value_widths: Vec<usize> = (0..ncols)
        .map(|i| {
            rows.iter()
                .filter(|r| !r.heading)
                .map(|r| r.cell(i).chars().count())
                .max()
                .unwrap_or(0)
        })
        .collect();
    let any_column = value_widths.iter().any(|&w| w > 0);
    let widths: Vec<usize> = (0..ncols)
        .map(|i| {
            if value_widths[i] == 0 {
                return 0;
            }
            rows.iter()
                .filter(|r| r.heading)
                .map(|r| r.cell(i).chars().count())
                .max()
                .unwrap_or(0)
                .max(value_widths[i])
        })
        .collect();
    let mut out = String::new();
    for r in rows {
        if r.heading && !any_column {
            continue; // no fact columns in this look — no headings line
        }
        let mut line = String::new();
        for g in &r.slots.far_left {
            line.push_str(&g.text);
        }
        line.push_str(&r.slots.level);
        // The name and its suffix are one painted run: the `/` belongs to
        // the dir's name visually even though it is its own lattice row.
        let named = format!("{}{}", r.slots.name, r.slots.name_suffix);
        let painted = if r.dim {
            // Faint beats the dir blue here: the ignore claim is the
            // line's loudest fact, and bold-blue would fight it.
            crate::color::dim(&named, color)
        } else if r.color_dir {
            crate::color::dir(&named, color)
        } else {
            named
        };
        line.push_str(&painted);
        line.push_str(&r.slots.after_name);
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
                let cell = r.cell(i);
                let cw = cell.chars().count();
                line.push_str(&" ".repeat(width - cw));
                if r.heading {
                    line.push_str(&crate::color::dim(cell, color));
                } else {
                    line.push_str(cell);
                }
            }
            let rest = r.slots.near_right_text();
            if !rest.is_empty() {
                if !first {
                    line.push_str("  ");
                }
                line.push_str(&rest);
            }
        }
        // Alignment gaps for silent cells must not leave trailing bytes.
        out.push_str(line.trim_end());
        out.push('\n');
    }
    out
}

/// Heading words per active column, in `ready::columns` order
/// (design/columns.md §Column headings; spellings provisional until
/// Joseph ratifies).
fn heading_row(cols: &Cols) -> Row {
    let cells = ready::columns(cols)
        .into_iter()
        .map(|c| Ready {
            fact: c.fact,
            // Count-cell headings end at the `.` (cell 9); the field stays
            // 12 so heat does not move (design/grid-cleanup.md §The count
            // cell: a heading is aligned over the columns it names).
            text: match c.fact {
                "lines" | "bytes" => count_cell::heading(c.heading),
                _ => c.heading.to_string(),
            },
            position: Position::FarRight,
            office: crate::ready::Office::Look,
        })
        .collect();
    Row {
        slots: Slots {
            far_right: cells,
            ..Slots::default()
        },
        color_dir: false,
        dim: false,
        heading: true,
    }
}

/// Will this look carry a headings line? True when any node below the root
/// gives any fact column a value. The budget path calls this on the
/// *budgeted* tree (a second allocation pass pays for the line only when
/// it actually renders); paint applies the same predicate via widths.
pub fn headings_expected(tree: &Node, cols: &Cols) -> bool {
    fn any_cell(n: &Node, cols: &Cols) -> bool {
        n.children
            .iter()
            .any(|c| ready::far_right(c, cols).iter().any(|r| !r.is_empty()) || any_cell(c, cols))
    }
    cols.active() > 0 && any_cell(tree, cols)
}

/// The root's own facts, pre-joined for the header facts line. Empty when
/// the root has nothing to say. Count cells here show their unit (this
/// line sits *above* the headings; a file root has no headings line at
/// all) — the 2026-08-14 `"767 lines"` wrap is the unit slot now. Other
/// unitless columns still carry their word (`perms`, `heat`). The 12-cell
/// pad is trimmed: this line is not in the column grid.
pub fn root_facts_line(tree: &Node, cols: &Cols) -> String {
    let cells = ready::far_right_header(tree, cols);
    let mut parts: Vec<String> = if tree.is_dir {
        cells
            .into_iter()
            .filter(|c| !c.is_empty())
            .map(|c| {
                if c.fact == "lines" || c.fact == "bytes" {
                    c.text.trim().to_string()
                } else {
                    c.text
                }
            })
            .collect()
    } else {
        cells
            .into_iter()
            .zip(ready::columns(cols))
            .filter(|(c, _)| !c.is_empty())
            .map(|(c, col)| match col.heading {
                "lines" | "bytes" => c.text.trim().to_string(),
                "perms" => format!("perms {}", c.text),
                "heat · age" => format!("heat {}", c.text),
                _ => c.text,
            })
            .collect()
    };
    let rest = Slots {
        near_right: ready::near_right(tree),
        ..Slots::default()
    }
    .near_right_text();
    if !rest.is_empty() {
        parts.push(rest);
    }
    parts.join("  ")
}

/// The whole look. Header is up to three lines — stamp, the root's facts
/// (only when it has any), then the bare root path directly above its
/// children (design/overview-invariants.md, decided 2026-08-14; the path
/// line is "the path and nothing else" — Joseph's simple-header steer).
pub fn render(root_path: &str, tree: &Node, color: bool, stamp: &str, cols: &Cols) -> String {
    let mut root = root_path.to_string();
    if tree.is_dir && !root.ends_with('/') {
        root.push('/');
    }
    let ncols = cols.active();
    let mut rows = vec![Row::plain(stamp.to_string(), ncols)];
    let facts = root_facts_line(tree, cols);
    if !facts.is_empty() {
        rows.push(Row::plain(facts, ncols));
    }
    let mut root_row = Row::plain(root, ncols);
    // Decoration completes the name (a symlinked root's `-> target`).
    root_row.slots.after_name = ready::symlink_target(tree)
        .map(|r| r.text)
        .unwrap_or_default();
    root_row.color_dir = true;
    rows.push(root_row);
    // The headings line, directly above the children it teaches (design/
    // columns.md §Column headings). paint drops it when no fact column
    // below carries a value.
    if ncols > 0 && (!tree.children.is_empty() || tree.omitted.is_some()) {
        rows.push(heading_row(cols));
    }
    emit(&tree.children, tree.omitted.as_ref(), "", cols, &mut rows);
    // The steward's feedback footer rides on stderr (main.rs) — the tool
    // speaking about itself, not the picture (stdout is data; Joseph,
    // 2026-08-22).
    paint(rows, color, ncols, cols.cluster_idx())
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
        out.push(Row {
            slots: Slots::of(k, cols, format!("{prefix}{branch}")),
            color_dir: k.is_dir,
            dim: k.ignored,
            heading: false,
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
    if has_om && let Some(c) = omitted {
        // The allocator's remainder: a leaf census standing where a name
        // stands (design/lattice-2.md — name-location, three tenants).
        let mut row = Row::plain(ready::leaf_census(c).text, cols.active());
        row.slots.level = format!("{prefix}└── ");
        out.push(row);
    }
}
