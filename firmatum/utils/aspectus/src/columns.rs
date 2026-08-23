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
//!    positions of design/lattice-2.md). Far-left takes git-status (step 5);
//!    supplement has no tenants yet. A node emits exactly one row today
//!    (sub-rows are a later slice).
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
#[derive(Debug, Clone)]
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
    /// `[layout] far-left` membership, in list order. The renderer paints
    /// entries it has formatters for (`git-status`); the rest stay
    /// unbuilt (mtime/bytes compact forms are not designed).
    pub far_left: Vec<String>,
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
            far_left: Vec::new(),
        }
    }
}

impl Cols {
    pub fn active(&self) -> usize {
        ready::columns(self).len()
    }

    /// True when `[layout] far-left` names a fact this slice can paint.
    pub fn paints_far_left(&self) -> bool {
        self.far_left.iter().any(|f| f == "git-status")
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
    /// Fixed-width blocks before the hierarchy — git-status is the first
    /// tenant (step 5). Empty when the look contains no repo, or when
    /// `[layout] far-left` has no paintable entry: absent, never faked.
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
    fn of(n: &Node, cols: &Cols, level: String, far_left_on: bool) -> Slots {
        Slots {
            far_left: if far_left_on {
                ready::far_left(n, cols)
            } else {
                Vec::new()
            },
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
    /// far-left `⊘` is the carrier; dim is the redundant overlay —
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

    /// The width of everything left of the tab-stop. Far-left is a look
    /// prefix, not part of the name column — including it would steal from
    /// STOP_CAP and shift everything right of the tree prefix.
    fn name_width(&self) -> usize {
        let s = &self.slots;
        s.level.chars().count()
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
fn heading_row(cols: &Cols, far_left_on: bool) -> Row {
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
            far_left: if far_left_on {
                ready::far_left_blank(cols)
            } else {
                Vec::new()
            },
            far_right: cells,
            ..Slots::default()
        },
        color_dir: false,
        dim: false,
        heading: true,
    }
}

/// Look-wide: any node inside a work tree means the far-left block is
/// present on tree rows and the headings line. Stamp, root-facts, and
/// root-path do not pay it. Outside any repo the block is absent — the
/// width of the look must not change for trees with no git.
fn tree_has_git(n: &Node) -> bool {
    n.in_git || n.children.iter().any(tree_has_git)
}

fn pad_far_left(row: &mut Row, cols: &Cols, far_left_on: bool) {
    if far_left_on {
        row.slots.far_left = ready::far_left_blank(cols);
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
/// all, so `𝓁` stays) — the 2026-08-14 `"767 lines"` wrap is the unit
/// slot now. A file root uses the same count-cell grammar, and the same
/// `heat · age` cluster form as a row, labelled `heat …` (no heading
/// follows). Other unitless columns still carry their word (`perms`).
/// The 12-cell pad is trimmed: this line is not in the column grid.
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

/// The whole look. Header is stamp, then config-drift (when anything
/// differs from the built-in defaults), then the root's facts (only when
/// it has any), then the bare root path directly above its children
/// (design/overview-invariants.md). The path line is "the path and nothing
/// else" — Joseph's simple-header steer. `drift` is empty when nothing
/// differs (absent, never faked).
pub fn render(
    root_path: &str,
    tree: &Node,
    color: bool,
    stamp: &str,
    cols: &Cols,
    drift: &str,
) -> String {
    let mut root = root_path.to_string();
    if tree.is_dir && !root.ends_with('/') {
        root.push('/');
    }
    let ncols = cols.active();
    let far_left_on = cols.paints_far_left() && tree_has_git(tree);
    // Stamp, config-drift, root-facts, and root-path do not pay the
    // far-left cell — a copied root path must not start with a space
    // (design/grid-cleanup.md §Step 5 landed). Headings and every tree
    // row do.
    let stamp_row = Row::plain(stamp.to_string(), ncols);
    let mut rows = vec![stamp_row];
    if !drift.is_empty() {
        rows.push(Row::plain(drift.to_string(), ncols));
    }
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
    // below carries a value. The far-left block gets no heading word
    // (one cell; help teaches the pack).
    if ncols > 0 && (!tree.children.is_empty() || tree.omitted.is_some()) {
        rows.push(heading_row(cols, far_left_on));
    }
    emit(
        &tree.children,
        tree.omitted.as_ref(),
        "",
        cols,
        far_left_on,
        &mut rows,
    );
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
    far_left_on: bool,
    out: &mut Vec<Row>,
) {
    let has_om = omitted.map(|c| !c.is_empty()).unwrap_or(false);
    let total = kids.len() + usize::from(has_om);
    for (i, k) in kids.iter().enumerate() {
        let last = i + 1 == total;
        let branch = if last { "└── " } else { "├── " };
        out.push(Row {
            slots: Slots::of(k, cols, format!("{prefix}{branch}"), far_left_on),
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
            emit(
                &k.children,
                k.omitted.as_ref(),
                &next,
                cols,
                far_left_on,
                out,
            );
        }
    }
    if has_om && let Some(c) = omitted {
        // The allocator's remainder: a leaf census standing where a name
        // stands (design/lattice-2.md — name-location, three tenants).
        let mut row = Row::plain(ready::leaf_census(c).text, cols.active());
        row.slots.level = format!("{prefix}└── ");
        pad_far_left(&mut row, cols, far_left_on);
        out.push(row);
    }
}
