//! N-level tree. Depth is generations *below* the root.
//! `1` = children only. `2` = children and grandchildren. `0` = no limit.

use std::collections::HashSet;
use std::fs;
use std::io;
use std::os::unix::fs::MetadataExt;
use std::path::Path;

/// Cumulative weight of a subtree (design/mass.md): descendant files and
/// text lines, furniture-hidden names excluded. `est` = some lines were
/// estimated from size (read budget); `bounded` = the count is a floor
/// (mass cap, denied dir, unreadable text, mount stop) and renders `≥`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Mass {
    pub files: u64,
    pub lines: u64,
    pub est: bool,
    pub bounded: bool,
}

impl Mass {
    pub fn absorb(&mut self, other: Mass) {
        self.files += other.files;
        self.lines += other.lines;
        self.est |= other.est;
        self.bounded |= other.bounded;
    }
}

/// Census, reworked form (design/dir-census.md, 2026-08-14): no total,
/// `kind×N` buckets, dirs separated as containers with mass prevalent,
/// `·` separators, `+` only in the leaf (unlisted-siblings) form. All
/// glyphs shipping-provisional — one render function, cheap to move.
#[derive(Debug, Clone, Default)]
pub struct Census {
    /// Container entries: subdirectories among the censused names.
    pub dirs: usize,
    /// The one dir's name when `dirs == 1` (name beats count at n=1).
    pub dir_name: Option<String>,
    /// Deep file-count beneath the censused dirs, when mass is known.
    pub dir_files: Option<Mass>,
    /// Suffix buckets for files, label without the dot (`md`, `other`),
    /// count-descending.
    pub buckets: Vec<(String, usize)>,
    /// The sole file's name when the census holds exactly one file and no
    /// dirs — a census that would conceal one cheap name shows the name.
    pub single: Option<String>,
    /// Gitignored files among the censused names — typed, never silent
    /// (design/gitignore-bodies.md); they are outside the buckets and
    /// outside every aggregate.
    pub ignored: usize,
    /// The counts are a floor (walk stopped early); renders `≥`.
    pub bounded: bool,
}

/// Names longer than this fall back to the count form (design open).
const NAME_FORM_CAP: usize = 24;

impl Census {
    pub fn is_empty(&self) -> bool {
        self.dirs == 0 && self.buckets.is_empty() && self.ignored == 0
    }

    pub fn render(&self) -> String {
        self.render_inner(false)
    }

    pub fn render_plus(&self) -> String {
        self.render_inner(true)
    }

    fn render_inner(&self, plus: bool) -> String {
        if self.is_empty() {
            return String::new();
        }
        let mut parts: Vec<String> = Vec::new();
        if self.dirs > 0 {
            let head = match &self.dir_name {
                Some(n) if self.dirs == 1 && n.chars().count() <= NAME_FORM_CAP => {
                    format!("{n}/")
                }
                _ => format!("dir×{}", self.dirs),
            };
            let mass = match self.dir_files {
                // An empty container's zero adds nothing the name lacks.
                // File counts are never budget-estimated (only lines are),
                // so the figure earns ≥ or ≈, never ~.
                Some(m) if m.files > 0 || m.bounded => {
                    let mark = if m.bounded { "≥" } else { "≈" };
                    format!(" {mark}{}f", m.files)
                }
                _ => String::new(),
            };
            parts.push(format!("{head}{mass}"));
        } else if self.buckets.iter().map(|(_, n)| n).sum::<usize>() == 1
            && self.ignored == 0
            && let Some(name) = &self.single
            && name.chars().count() <= NAME_FORM_CAP
        {
            let geq = if self.bounded { "≥ " } else { "" };
            let p = if plus { "+ " } else { "" };
            return format!("[{p}{geq}{name}]");
        }
        for (k, n) in &self.buckets {
            parts.push(format!("{k}×{n}"));
        }
        if self.ignored > 0 {
            // Spelling provisional until Joseph ratifies (design Open).
            parts.push(format!("ignored×{}", self.ignored));
        }
        let geq = if self.bounded { "≥ " } else { "" };
        let p = if plus { "+ " } else { "" };
        format!("[{p}{geq}{}]", parts.join(" · "))
    }
}

#[derive(Debug, Clone, Default)]
pub struct Node {
    pub name: String,
    pub is_dir: bool,
    pub children: Vec<Node>,
    /// Set when we stopped expanding this directory (depth cutoff).
    pub leftover: Option<Census>,
    /// Siblings not listed because the line budget ran out.
    pub omitted: Option<Census>,
    /// The walk could not read this dir (or stat this name): `[denied]`.
    pub denied: bool,
    /// The walk bound stopped enumeration here before every name was seen.
    pub cut: bool,
    /// An entry mid-iteration errored; this dir's listing may be missing names.
    pub iter_err: bool,
    /// Kinds claimed on this line's gathering spot: `[has: git, rust, …]`.
    pub kinds: Vec<String>,
    /// Hidden furniture dirs here, as (claiming kind, name) — walk-time
    /// record; the deep phase turns it into `has_counts`.
    pub hidden_dirs: Vec<(String, String)>,
    /// Deep file-count of hidden furniture per kind: (kind, files,
    /// bounded). Presence survives hiding — `[has: archive ≈127f, …]`
    /// (design/furniture.md leaning, implemented 2026-08-14).
    pub has_counts: Vec<(String, u64, bool)>,
    /// Specialized-furniture facets, already phrased (`git: br<main> @…`).
    pub facets: Vec<String>,
    /// Modification time, seconds since the epoch (symlinks: the target's).
    pub mtime: Option<i64>,
    /// `st_size`, plain files only (symlinked files: the target's).
    pub size: Option<u64>,
    /// Permission bits + special bits (`st_mode & 0o7777`); symlinks: the
    /// target's. Absent only where the stat itself failed.
    pub mode: Option<u32>,
    /// Owner uid / group gid (symlinks: the target's).
    pub uid: Option<u32>,
    pub gid: Option<u32>,
    /// Survival weight from the `important` config set (design/
    /// important-files.md): survives tight budgets, owns no column or order.
    pub important: bool,
    /// Which quiet facts surprise on this line (design/quiet-columns.md,
    /// cold baseline; set post-gather over full statted levels).
    pub q: crate::quiet::Speaks,
    /// The kind *word* for the near-right spot, when filekind speaks
    /// (a node's countable class among its level) or is asked `on`.
    pub kind_word: Option<&'static str>,
    /// Ladder type (design/filetype.md). Every node has one; unknown
    /// is the honest floor, never faked as text.
    pub filetype: crate::filetype::FileType,
    /// Census bucket key at the caller's grain (suffix default).
    pub census_key: String,
    /// Line count of a non-binary file (design/linecount.md); absent on
    /// binary, unreadable, or past the read budget — never a guess.
    pub lines: Option<u64>,
    /// Subtree weight, every dir (rendered only where a census is).
    pub mass: Option<Mass>,
    /// This dir's `(st_dev, st_ino)` was already aggregated elsewhere in
    /// this look (diamond symlink) — shown here, counted once in mass.
    pub mass_dup: bool,
    /// Commit-decay heat (design/heat.md); absent outside git or when
    /// history says nothing.
    pub heat: Option<f64>,
    /// Last git touch (unix secs), when heat's log pass covered this path.
    pub git_ts: Option<i64>,
    /// The commit that last touched this file: (full sha, commits behind
    /// HEAD) — from heat's log pass; absent outside git or past the cap.
    pub touch: Option<(String, u32)>,
    /// The commit that introduced this file (A/C in the log window, or the
    /// initial commit when the log reached it); never guessed past the cap.
    pub intro: Option<(String, u32)>,
    /// This entry is itself gitignored (design/gitignore-bodies.md):
    /// presence shows (dimmed + glyph in the far-left git-status cell),
    /// contents stay out of the look and out of every aggregate.
    pub ignored: bool,
    /// This node sits inside a git work tree (enclosing `.git` at or
    /// above it). The far-left git-status cell is positional look-wide
    /// on tree rows and the headings line once *any* node is in a work
    /// tree; this flag is the per-node membership that paints a letter
    /// vs a blank.
    pub in_git: bool,
    /// Porcelain worktree-wins letter for this path (`M A ⁇ R U D C T`),
    /// absent when clean or when porcelain did not name this path.
    /// Directories carry `⁇` when porcelain named them (`?? dir/`); the
    /// formatter blanks tracked-and-clean dirs. Gitignored paints `⊘`
    /// from `ignored`, not from here.
    pub git_letter: Option<char>,
    /// Gitignored files among this dir's direct children, not listed —
    /// the typed remainder on an expanded level.
    pub ignored_files: usize,
    /// A collapsed name-series (design/globify.md): this node is one
    /// listee standing for `count` members; never expanded.
    pub glob: Option<crate::globify::Glob>,
    /// The name this dir's README gives it (design/readme-title.md);
    /// absent unless asked (config `readme-title = on`) and truthful.
    pub title: Option<String>,
    /// Symlink target as the link states it (INFO, lattice `ON`).
    pub link: Option<String>,
    /// The target does not resolve to anything that exists.
    pub link_broken: bool,
    /// A symlink back to a directory already being expanded on this path:
    /// target shown, recursion refused, never a hang.
    pub cycle: bool,
    /// A mount point the one-fs default did not cross.
    pub other_fs: bool,
    /// This node is one of the paths the caller named (design/focus.md):
    /// top survival tier, `matched` in JSON, and the spot where the depth
    /// count restarts.
    pub matched: bool,
    /// An unselected sibling on a connective level of a focus look: walked
    /// for its census and mass, then folded into that level's remainder by
    /// `focus::fold_asides`. Never on a node the caller can see.
    pub aside: bool,
}

/// Counts names statted. `None` = unbounded.
#[derive(Debug, Default)]
pub struct WalkBudget {
    remaining: Option<u64>,
    pub tripped: bool,
    pub statted: u64,
    /// Names the furniture map kept off the child lists this walk.
    pub furniture_hidden: u64,
}

impl WalkBudget {
    /// `bound` of 0 means no bound.
    pub fn new(bound: u64) -> Self {
        WalkBudget {
            remaining: if bound == 0 { None } else { Some(bound) },
            tripped: false,
            statted: 0,
            furniture_hidden: 0,
        }
    }

    fn charge(&mut self) {
        self.statted += 1;
        if let Some(n) = &mut self.remaining {
            *n = n.saturating_sub(1);
        }
    }

    fn exhausted(&self) -> bool {
        self.remaining == Some(0)
    }
}

/// The reads the look may spend on file content (line counting). Past the
/// budget a file's lines are estimated (mass, `≈`) or absent (per-file
/// column) — the honest degraded form; the glance never silently slows.
#[derive(Debug)]
pub struct ReadMeter {
    remaining: Option<u64>,
}

impl ReadMeter {
    /// `budget` of 0 means unlimited.
    pub fn new(budget: u64) -> Self {
        ReadMeter {
            remaining: if budget == 0 { None } else { Some(budget) },
        }
    }

    /// From an already-optional remainder (`None` = unlimited) — the deep
    /// phase splits one budget into per-subtree shares this way.
    fn with(remaining: Option<u64>) -> Self {
        ReadMeter { remaining }
    }

    fn can_read(&self, size: u64) -> bool {
        match self.remaining {
            None => true,
            Some(r) => size <= r,
        }
    }

    fn charge(&mut self, bytes: u64) {
        if let Some(r) = &mut self.remaining {
            *r = r.saturating_sub(bytes);
        }
    }
}

/// Estimated lines for an unread text file. A constant bytes-per-line so
/// the estimate is a function of the file alone — the earlier look-observed
/// ratio made an unread directory's total depend on *what else* this walk
/// had read, which is exactly the flag-to-flag instability the hallway
/// testers caught (hardening 2026-08-14). `≈`→`~` marks it estimated.
/// 64 is calibrated against real estate trees (close audit 2026-08-14;
/// the prior 32 overestimated 2–3×): measured B/line — asf ≈100,
/// vivarium ≈110, firmatum ≈66, memorata ≈58 (md-press-unwrapped prose
/// runs ≈110–125; dense code ≈40–60). Basis recorded in design/mass.md.
const EST_BYTES_PER_LINE: u64 = 64;

pub fn est_lines(size: u64) -> u64 {
    size / EST_BYTES_PER_LINE
}

/// A visible file this small is counted even past the read budget — the
/// look's own lines deserve real numbers when they are cheap. Bigger
/// visible files degrade like the deep walk (count absent, mass `~`):
/// the old unconditional visible read slurped multi-GB files whole
/// (measured 2.8 GB resident on ~/src, hardening 2026-08-14).
const VISIBLE_READ_FLOOR: u64 = 256 * 1024;

/// Bytes sniffed to decide text-vs-binary for unknown suffixes. Shared
/// with magic/shebang (design/filetype.md); never a second classification
/// read. The constant lives in filetype so the ladder and the walk agree.
const SNIFF_BYTES: usize = crate::filetype::SNIFF_BYTES;

/// Names deep-mass will visit per look before declaring the floor (`≥`).
const MASS_NAME_CAP: u64 = 500_000;

/// Everything the walk consults beyond the tree itself.
pub struct LookCtx<'a> {
    pub map: &'a crate::furniture::Map,
    pub view: &'a crate::furniture::View,
    pub kinds: &'a crate::filetype::Map,
    /// Census bucket grain (`format.census`); default suffix.
    pub census_grain: crate::filetype::CensusGrain,
    /// Count non-blank instead of physical lines.
    pub non_blank: bool,
    /// Stay on the starting filesystem (default; `--no-one-fs` clears).
    pub one_fs: bool,
    pub reads: ReadMeter,
    /// Git-ignore state for the walk (design/gitignore-bodies.md); seeded
    /// from the root's enclosing repo, grown as repos/.gitignores appear.
    pub ignore: crate::ignore::Stack,
    /// When Some, dir lines borrow their README's title (config
    /// `readme-title = on`; the set is important-files', shared).
    pub titles: Option<&'a crate::important::Set>,
    /// The focus set, when several positional paths named one
    /// (design/focus.md §Multiple paths). It changes the walk only on
    /// *connective* levels — the chain from the common ancestor down to
    /// the selected paths; inside a selected subtree the ordinary depth
    /// law runs untouched.
    pub focus: Option<&'a crate::focus::Focus>,
    root_dev: Option<u64>,
    /// Dirs being expanded on the current path — the cycle guard.
    dir_stack: Vec<(u64, u64)>,
    /// Dirs whose mass was already aggregated — diamonds count once.
    seen: HashSet<(u64, u64)>,
    mass_names: u64,
}

impl<'a> LookCtx<'a> {
    pub fn new(
        map: &'a crate::furniture::Map,
        view: &'a crate::furniture::View,
        kinds: &'a crate::filetype::Map,
        non_blank: bool,
        one_fs: bool,
        read_budget: u64,
    ) -> Self {
        LookCtx {
            map,
            view,
            kinds,
            census_grain: crate::filetype::CensusGrain::Suffix,
            non_blank,
            one_fs,
            reads: ReadMeter::new(read_budget),
            ignore: crate::ignore::Stack::new(),
            titles: None,
            focus: None,
            root_dev: None,
            dir_stack: Vec::new(),
            seen: HashSet::new(),
            mass_names: MASS_NAME_CAP,
        }
    }
}

/// Line-count outcome for one file.
enum Count {
    Lines(u64),
    /// Text (or presumed text) not read: estimated for mass only.
    Est(u64),
    /// Binary — no count anywhere, never 0.
    Binary,
    /// Unreadable text: no count, mass floor.
    Denied,
}

fn count_file(
    path: &Path,
    size: u64,
    mode: u32,
    ctx: &mut LookCtx,
) -> (crate::filetype::FileType, Count) {
    count_file_at(path, size, mode, ctx, false)
}

/// `visible` files (lines of the look itself) get a small exemption from
/// the budget (`VISIBLE_READ_FLOOR`); past that everything degrades the
/// same honest way. Classification uses one ≤1 KiB window (magic, shebang,
/// sniff); line-count may then read the rest of a text file. Known-binary
/// suffixes are not opened just to verify magic — the read budget governs
/// whether a file is read at all.
fn count_file_at(
    path: &Path,
    size: u64,
    mode: u32,
    ctx: &mut LookCtx,
    visible: bool,
) -> (crate::filetype::FileType, Count) {
    use crate::filetype::{self, FileType, Major};
    use std::io::Read;
    let name = path
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default();
    // Specials (fifo/socket/…) must never be opened — a fifo read blocks
    // the glance until a writer appears.
    if let Ok(meta) = fs::symlink_metadata(path) {
        let ft = meta.file_type();
        if let Some(special) =
            crate::filetype::from_stat(ft).filter(|t| t.major == crate::filetype::Major::Special)
        {
            return (special, Count::Binary);
        }
    }
    let mapped = ctx.kinds.lookup(&name);
    let exec = mode & 0o111 != 0;

    if size == 0 {
        let mut ft = FileType::empty();
        if exec {
            ft.trait_ = Some("+x".into());
        }
        return (ft, Count::Lines(0));
    }

    // Known-binary by suffix: no read. Magic only runs when a read is
    // already happening (line-count or unknown sniff).
    if let Some(ft) = &mapped
        && !ft.counts_lines()
    {
        let mut ft = ft.clone();
        if exec && ft.trait_.is_none() && ft.major != Major::Exe {
            ft.trait_ = Some("+x".into());
        }
        return (ft, Count::Binary);
    }

    let can = ctx.reads.can_read(size) || (visible && size <= VISIBLE_READ_FLOOR);
    if !can {
        let ft = mapped.unwrap_or_else(FileType::unknown);
        if ft.counts_lines() && ft.major != Major::Unknown {
            return (ft, Count::Est(est_lines(size)));
        }
        return (ft, Count::Binary);
    }

    let unknown = mapped.is_none();
    if unknown {
        let mut head = [0u8; SNIFF_BYTES];
        let sniffed = fs::File::open(path)
            .and_then(|mut f| f.read(&mut head))
            .unwrap_or(0);
        if sniffed == 0 && size > 0 {
            return (FileType::unknown(), Count::Binary);
        }
        let ft = filetype::from_window(&head[..sniffed], None, exec);
        if !ft.counts_lines() {
            return (ft, Count::Binary);
        }
        if size as usize <= sniffed {
            let lines = if ctx.non_blank {
                filetype::non_blank_lines(&head[..sniffed])
            } else {
                filetype::physical_lines(&head[..sniffed])
            };
            ctx.reads.charge(sniffed as u64);
            return (ft, Count::Lines(lines));
        }
        return finish_count(path, ft, exec, ctx);
    }

    finish_count(path, mapped.unwrap_or_else(FileType::unknown), exec, ctx)
}

/// Full-file read for a type that counts (or that we just sniffed as
/// text). The first 1 KiB of this buffer is the ladder window — magic can
/// still override a suffix (a `.md` that starts `\x89PNG` becomes image).
fn finish_count(
    path: &Path,
    mapped: crate::filetype::FileType,
    exec: bool,
    ctx: &mut LookCtx,
) -> (crate::filetype::FileType, Count) {
    use crate::filetype;
    match fs::read(path) {
        Ok(bytes) => {
            ctx.reads.charge(bytes.len() as u64);
            let n = bytes.len().min(SNIFF_BYTES);
            let ft = filetype::from_window(&bytes[..n], Some(mapped.clone()), exec);
            if !ft.counts_lines() {
                return (ft, Count::Binary);
            }
            let lines = if ctx.non_blank {
                filetype::non_blank_lines(&bytes)
            } else {
                filetype::physical_lines(&bytes)
            };
            (ft, Count::Lines(lines))
        }
        Err(_) => {
            if mapped.counts_lines() {
                (mapped, Count::Denied)
            } else {
                (mapped, Count::Binary)
            }
        }
    }
}

/// `depth` is how many generations below the root to print. `0` = no limit.
pub fn gather(
    path: &Path,
    depth: u32,
    walk: &mut WalkBudget,
    ctx: &mut LookCtx,
) -> io::Result<Node> {
    let remain = if depth == 0 { None } else { Some(depth) };
    // Ignore rules from any repo *above* the root (the root's own .git and
    // .gitignore are met by gather_dir like every dir's). An explicitly
    // named locus is looked at even if it is itself ignored — the ask wins.
    if let Some(parent) = path.parent() {
        ctx.ignore = crate::ignore::Stack::for_path(parent);
    }
    gather_at(path, remain, walk, ctx)
}

/// The same walk, rooted at the focus set's common ancestor
/// (design/focus.md §Multiple paths). The root and the chain below it are
/// *connective*: they spend no depth and are not depth-cut, because every
/// one of their children is assigned a role explicitly — selected (the
/// depth count starts here), connective (keep descending), or aside
/// (census only, folded afterwards).
pub fn gather_focus<'a>(
    path: &Path,
    focus: &'a crate::focus::Focus,
    walk: &mut WalkBudget,
    ctx: &mut LookCtx<'a>,
) -> io::Result<Node> {
    if let Some(parent) = path.parent() {
        ctx.ignore = crate::ignore::Stack::for_path(parent);
    }
    ctx.focus = Some(focus);
    gather_at(path, None, walk, ctx)
}

/// One name as readdir reported it, before any stat.
struct Entry {
    name: String,
    /// From the readdir file-type hint — no stat spent. A symlink is not
    /// a dir here; the stat decides whether it expands like one.
    is_dir: bool,
}

fn enumerate(path: &Path) -> Option<(Vec<Entry>, bool)> {
    let rd = fs::read_dir(path).ok()?;
    let mut entries = Vec::new();
    let mut iter_err = false;
    for ent in rd {
        let Ok(ent) = ent else {
            iter_err = true;
            continue;
        };
        let os = ent.file_name();
        if os == "." || os == ".." {
            continue;
        }
        entries.push(Entry {
            name: os.to_string_lossy().into_owned(),
            is_dir: ent.file_type().map(|t| t.is_dir()).unwrap_or(false),
        });
    }
    entries.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.cmp(&b.name),
    });
    Some((entries, iter_err))
}

pub fn suffix_bucket(name: &str) -> String {
    match name.rsplit_once('.') {
        Some((stem, ext)) if !stem.is_empty() && !ext.is_empty() && !ext.contains('/') => {
            ext.to_string()
        }
        _ => "other".to_string(),
    }
}

fn bucketize(names: impl Iterator<Item = String>) -> Vec<(String, usize)> {
    let mut buckets: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
    for label in names {
        *buckets.entry(label).or_insert(0) += 1;
    }
    let mut buckets: Vec<(String, usize)> = buckets.into_iter().collect();
    buckets.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    buckets
}

fn census_entries(entries: &[Entry], bounded: bool, ctx: &LookCtx) -> Census {
    let dirs = entries.iter().filter(|e| e.is_dir).count();
    let dir_name = (dirs == 1)
        .then(|| entries.iter().find(|e| e.is_dir).map(|e| e.name.clone()))
        .flatten();
    let files: Vec<&Entry> = entries.iter().filter(|e| !e.is_dir).collect();
    let single = (dirs == 0 && files.len() == 1).then(|| files[0].name.clone());
    let grain = ctx.census_grain;
    let map = ctx.kinds;
    Census {
        dirs,
        dir_name,
        dir_files: None,
        buckets: bucketize(files.iter().map(|e| {
            let ft = map
                .lookup(&e.name)
                .unwrap_or_else(crate::filetype::FileType::unknown);
            crate::filetype::census_key(&e.name, &ft, grain)
        })),
        single,
        ignored: 0,
        bounded,
    }
}

pub(crate) fn census_nodes(nodes: &[Node]) -> Census {
    let dirs: usize = nodes
        .iter()
        .filter(|n| n.is_dir)
        .map(|n| n.glob.as_ref().map_or(1, |g| g.count))
        .sum();
    let dir_name = (dirs == 1)
        .then(|| nodes.iter().find(|n| n.is_dir).map(|n| n.name.clone()))
        .flatten();
    let mut dir_files: Option<Mass> = None;
    for n in nodes.iter().filter(|n| n.is_dir) {
        if n.ignored {
            continue; // exclusion is declared (the ignored mark), not a floor
        }
        match (n.mass, n.mass_dup) {
            (Some(m), false) => dir_files.get_or_insert_with(Mass::default).absorb(m),
            // A dup or massless dir under a known aggregate: floor, not lie.
            _ => {
                if let Some(df) = &mut dir_files {
                    df.bounded = true;
                }
            }
        }
    }
    let files: Vec<&Node> = nodes.iter().filter(|n| !n.is_dir).collect();
    let nfiles: usize = files
        .iter()
        .map(|n| n.glob.as_ref().map_or(1, |g| g.count))
        .sum();
    let single = (dirs == 0 && nfiles == 1).then(|| files[0].name.clone());
    // A collapsed group folds back as its members (count under its suffix
    // bucket), so a budget fold cannot shrink 44 files to one listee.
    let labels = files.iter().flat_map(|n| {
        let (label, count) = match &n.glob {
            Some(g) => (g.bucket.clone(), g.count),
            None => {
                let key = if n.census_key.is_empty() {
                    suffix_bucket(&n.name)
                } else {
                    n.census_key.clone()
                };
                (key, 1)
            }
        };
        std::iter::repeat_n(label, count)
    });
    let ignored = nodes.iter().map(|n| n.ignored_files).sum::<usize>()
        + nodes.iter().filter(|n| !n.is_dir && n.ignored).count();
    Census {
        dirs,
        dir_name,
        dir_files,
        buckets: bucketize(labels),
        single,
        ignored,
        bounded: false,
    }
}

/// Two censuses of disjoint name-sets become one. `bounded` is sticky.
pub(crate) fn merge_census(a: Census, b: Census) -> Census {
    let dirs = a.dirs + b.dirs;
    let dir_name = match (dirs, &a.dir_name, &b.dir_name) {
        (1, Some(n), _) | (1, _, Some(n)) => Some(n.clone()),
        _ => None,
    };
    let dir_files = match (a.dir_files, b.dir_files) {
        (Some(mut x), Some(y)) => {
            x.absorb(y);
            Some(x)
        }
        (Some(mut x), None) => {
            if b.dirs > 0 {
                x.bounded = true;
            }
            Some(x)
        }
        (None, Some(mut y)) => {
            if a.dirs > 0 {
                y.bounded = true;
            }
            Some(y)
        }
        (None, None) => None,
    };
    let mut buckets: std::collections::BTreeMap<String, usize> = a.buckets.into_iter().collect();
    for (k, n) in b.buckets {
        *buckets.entry(k).or_insert(0) += n;
    }
    let mut buckets: Vec<(String, usize)> = buckets.into_iter().collect();
    buckets.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    let nfiles: usize = buckets.iter().map(|(_, n)| n).sum();
    let single = if dirs == 0 && nfiles == 1 {
        a.single.or(b.single)
    } else {
        None
    };
    Census {
        dirs,
        dir_name,
        dir_files,
        buckets,
        single,
        ignored: a.ignored + b.ignored,
        bounded: a.bounded || b.bounded,
    }
}

/// Deep weight of an unexpanded subtree: recursive, furniture-aware,
/// cycle-guarded, one-fs, count-once. `path` is a directory already known
/// to be on the right filesystem and not a cycle.
fn deep_mass(path: &Path, ctx: &mut LookCtx) -> Mass {
    let mut m = Mass::default();
    let Some((entries, iter_err)) = enumerate(path) else {
        m.bounded = true;
        return m;
    };
    m.bounded |= iter_err;
    let entered = ctx
        .ignore
        .enter_dir(path, entries.iter().map(|e| e.name.as_str()));
    let (_reading, keep) = crate::furniture::read_names(
        ctx.map,
        ctx.view,
        entries.iter().map(|e| (e.name.as_str(), e.is_dir)),
    );
    let m = deep_mass_body(path, ctx, entries, keep, m);
    ctx.ignore.exit_dir(entered);
    m
}

fn deep_mass_body(
    path: &Path,
    ctx: &mut LookCtx,
    entries: Vec<Entry>,
    keep: Vec<bool>,
    mut m: Mass,
) -> Mass {
    for (e, listed) in entries.iter().zip(keep) {
        if !listed {
            continue;
        }
        // Ignored bodies stay out of mass (mass's own promise). Under
        // --show-all restored bodies join it — show-all means show all
        // (implemented leaning; design Open notes the ≈-mark question).
        if !ctx.view.show_all
            && ctx.ignore.active()
            && ctx.ignore.is_ignored(path, &e.name, e.is_dir)
        {
            continue;
        }
        if ctx.mass_names == 0 {
            m.bounded = true;
            return m;
        }
        ctx.mass_names -= 1;
        let p = path.join(&e.name);
        let Ok(meta) = fs::symlink_metadata(&p) else {
            m.bounded = true;
            continue;
        };
        let (meta, is_dir) = if meta.file_type().is_symlink() {
            match fs::metadata(&p) {
                Ok(t) => {
                    let d = t.is_dir();
                    (t, d)
                }
                Err(_) => continue, // broken link weighs nothing
            }
        } else {
            let d = meta.is_dir();
            (meta, d)
        };
        if is_dir {
            let key = (meta.dev(), meta.ino());
            if ctx.dir_stack.contains(&key) || ctx.seen.contains(&key) {
                continue; // cycle or diamond: counted once
            }
            if ctx.one_fs && ctx.root_dev.is_some_and(|d| d != meta.dev()) {
                m.bounded = true; // a mount the default did not cross
                continue;
            }
            ctx.seen.insert(key);
            ctx.dir_stack.push(key);
            m.absorb(deep_mass(&p, ctx));
            ctx.dir_stack.pop();
        } else if meta.is_file() {
            m.files += 1;
            match count_file(&p, meta.len(), meta.mode(), ctx).1 {
                Count::Lines(n) => m.lines += n,
                Count::Est(n) => {
                    m.lines += n;
                    m.est = true;
                }
                Count::Binary => {}
                Count::Denied => m.bounded = true,
            }
        }
    }
    m
}

/// A non-directory node: regular file, special, or a symlink whose target
/// is a file. The node's type is `link` when we got here via a symlink;
/// line-count still follows the target (today's behavior).
fn gather_non_dir(
    path: &Path,
    name: String,
    meta: &fs::Metadata,
    mtime: Option<i64>,
    mode: Option<u32>,
    uid: Option<u32>,
    gid: Option<u32>,
    link: Option<String>,
    link_broken: bool,
    ctx: &mut LookCtx,
) -> Node {
    use crate::filetype::{self, FileType};

    let ft = meta.file_type();
    let mut filetype = if link.is_some() {
        FileType::link(if link_broken { "broken" } else { "file" })
    } else if let Some(special) =
        filetype::from_stat(ft).filter(|t| t.major == filetype::Major::Special)
    {
        special
    } else {
        FileType::unknown()
    };

    let (size, lines) = if meta.is_file() {
        let bits = mode.unwrap_or(0);
        let (tgt, count) = count_file_at(path, meta.len(), bits, ctx, true);
        if link.is_some() && !link_broken {
            filetype.trait_ = tgt.kind_word().map(|s| s.to_string());
        } else if link.is_none() {
            filetype = tgt;
        }
        let lines = match count {
            Count::Lines(n) => Some(n),
            _ => None,
        };
        (Some(meta.len()), lines)
    } else {
        (None, None)
    };

    // A gitlink `.git` (submodule / linked work tree) only ever shows
    // under --inspect/--show-all; when it does, say where it points —
    // "see inside .git" cannot mean children here (grok, 2026-08-14).
    let facets = if name == ".git" && meta.is_file() {
        match fs::read_to_string(path)
            .ok()
            .as_deref()
            .and_then(|t| t.trim().strip_prefix("gitdir:"))
        {
            Some(target) => vec![format!("gitdir: {}", target.trim())],
            None => Vec::new(),
        }
    } else {
        Vec::new()
    };
    let census_key = filetype::census_key(&name, &filetype, ctx.census_grain);
    Node {
        name,
        is_dir: false,
        mtime,
        size,
        mode,
        uid,
        gid,
        lines,
        link,
        link_broken,
        facets,
        filetype,
        census_key,
        ..Node::default()
    }
}

fn gather_at(
    path: &Path,
    remain: Option<u32>,
    walk: &mut WalkBudget,
    ctx: &mut LookCtx,
) -> io::Result<Node> {
    let lmeta = fs::symlink_metadata(path)?;
    let name = path
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string());
    // A symlink expands like what it points at (design/links-and-fs.md);
    // facts are the target's, the `-> target` deco says how it got here.
    let (meta, link, link_broken) = if lmeta.file_type().is_symlink() {
        let target = fs::read_link(path)
            .ok()
            .map(|t| t.to_string_lossy().into_owned());
        match fs::metadata(path) {
            Ok(t) => (t, target, false),
            Err(_) => {
                let mtime = stamp_of(&lmeta);
                let (mode, uid, gid) = ids_of(&lmeta);
                let filetype = crate::filetype::FileType::link("broken");
                let census_key = crate::filetype::census_key(&name, &filetype, ctx.census_grain);
                return Ok(Node {
                    name,
                    is_dir: false,
                    mtime,
                    mode,
                    uid,
                    gid,
                    link: target,
                    link_broken: true,
                    filetype,
                    census_key,
                    ..Node::default()
                });
            }
        }
    } else {
        (lmeta, None, false)
    };
    let is_dir = meta.is_dir();
    let mtime = stamp_of(&meta);
    let (mode, uid, gid) = ids_of(&meta);
    if !is_dir {
        return Ok(gather_non_dir(
            path,
            name,
            &meta,
            mtime,
            mode,
            uid,
            gid,
            link,
            link_broken,
            ctx,
        ));
    }
    let key = (meta.dev(), meta.ino());
    if ctx.root_dev.is_none() {
        ctx.root_dev = Some(meta.dev());
    } else if ctx.one_fs && ctx.root_dev != Some(meta.dev()) {
        // A filesystem boundary, not an empty directory, stopped here.
        let mut n = Node {
            name,
            is_dir: true,
            mtime,
            mode,
            uid,
            gid,
            link,
            link_broken,
            other_fs: true,
            ..Node::default()
        };
        stamp_dir_type(&mut n, ctx.census_grain);
        return Ok(n);
    }
    if ctx.dir_stack.contains(&key) {
        let mut n = Node {
            name,
            is_dir: true,
            mtime,
            mode,
            uid,
            gid,
            link,
            link_broken,
            cycle: true,
            ..Node::default()
        };
        stamp_dir_type(&mut n, ctx.census_grain);
        return Ok(n);
    }
    let mass_dup = !ctx.seen.insert(key);
    ctx.dir_stack.push(key);
    let node = gather_dir(path, name, remain, walk, ctx, mtime, link, mass_dup);
    ctx.dir_stack.pop();
    node.map(|mut n| {
        n.mode = mode;
        n.uid = uid;
        n.gid = gid;
        stamp_dir_type(&mut n, ctx.census_grain);
        n
    })
}

/// Presence-only node for an ignored directory: the stat facts its line
/// may carry, nothing from inside (design/gitignore-bodies.md — the repo
/// disclaimed the innards; the look honors the declaration).
fn stat_only(path: &Path, name: String, grain: crate::filetype::CensusGrain) -> Node {
    let meta = fs::symlink_metadata(path).ok();
    let link = meta
        .as_ref()
        .filter(|m| m.file_type().is_symlink())
        .and_then(|_| fs::read_link(path).ok())
        .map(|t| t.to_string_lossy().into_owned());
    let meta = match (&link, fs::metadata(path)) {
        (Some(_), Ok(t)) => Some(t),
        (Some(_), Err(_)) => meta,
        (None, _) => meta,
    };
    let (mtime, mode, uid, gid) = match &meta {
        Some(m) => {
            let (mo, u, g) = ids_of(m);
            (stamp_of(m), mo, u, g)
        }
        None => (None, None, None, None),
    };
    let mut n = Node {
        name,
        is_dir: true,
        ignored: true,
        mtime,
        mode,
        uid,
        gid,
        link,
        ..Node::default()
    };
    stamp_dir_type(&mut n, grain);
    n
}

/// Head-peek constant for README titles (design/readme-title.md: a few KB,
/// because the fact multiplies across every visible directory).
const TITLE_PEEK: usize = 4096;
/// Rendered titles longer than this are cut with an ellipsis (provisional).
const TITLE_CAP: usize = 60;

/// The title a dir's README lends it: the important-files set picks the
/// file (config order breaks ties; names within a pattern tie
/// alphabetically — enumerate already sorted them), the head-peek yields
/// the first ATX heading, else the first non-empty line. Truthful or
/// silent: binary, unreadable, empty, or redundant (equal to the folder
/// name, case/punct-insensitive) lends nothing.
fn readme_title(
    dir: &Path,
    entries: &[Entry],
    dir_name: &str,
    set: &crate::important::Set,
) -> Option<String> {
    let source = set.first_match(
        entries
            .iter()
            .filter(|e| !e.is_dir)
            .map(|e| e.name.as_str()),
    )?;
    use std::io::Read;
    let mut head = vec![0u8; TITLE_PEEK];
    let n = fs::File::open(dir.join(source))
        .and_then(|mut f| f.read(&mut head))
        .ok()?;
    let head = &head[..n];
    if head.contains(&0) {
        return None; // binary lends nothing
    }
    let text = String::from_utf8_lossy(head);
    let mut fallback: Option<&str> = None;
    let mut title: Option<&str> = None;
    for line in text.lines() {
        let t = line.trim();
        if t.is_empty() {
            continue;
        }
        if fallback.is_none() {
            fallback = Some(t);
        }
        let hashes = t.bytes().take_while(|&b| b == b'#').count();
        if (1..=6).contains(&hashes) && t.as_bytes().get(hashes).is_none_or(|&b| b == b' ') {
            title = Some(t[hashes..].trim().trim_end_matches('#').trim_end());
            break;
        }
    }
    let raw = title.or(fallback)?;
    // Light decoration strip: surrounding emphasis only, no rendering.
    let clean = raw.trim_matches(|c| matches!(c, '*' | '_' | '`')).trim();
    if clean.is_empty() {
        return None;
    }
    let norm = |s: &str| -> String {
        s.chars()
            .filter(|c| c.is_alphanumeric())
            .flat_map(char::to_lowercase)
            .collect()
    };
    if norm(clean) == norm(dir_name) {
        return None; // would spend glyphs saying nothing
    }
    let mut out: String = clean.chars().take(TITLE_CAP).collect();
    if clean.chars().count() > TITLE_CAP {
        out.push('…');
    }
    Some(out)
}

/// Identity facts every stat already paid for (quiet-columns' substrate).
fn ids_of(meta: &fs::Metadata) -> (Option<u32>, Option<u32>, Option<u32>) {
    (
        Some(meta.mode() & 0o7777),
        Some(meta.uid()),
        Some(meta.gid()),
    )
}

fn stamp_dir_type(n: &mut Node, grain: crate::filetype::CensusGrain) {
    n.filetype = if n.link.is_some() {
        crate::filetype::FileType::link(if n.link_broken { "broken" } else { "dir" })
    } else {
        crate::filetype::FileType::dir()
    };
    n.census_key = crate::filetype::census_key(&n.name, &n.filetype, grain);
}

#[allow(clippy::too_many_arguments)]
fn gather_dir(
    path: &Path,
    name: String,
    remain: Option<u32>,
    walk: &mut WalkBudget,
    ctx: &mut LookCtx,
    mtime: Option<i64>,
    link: Option<String>,
    mass_dup: bool,
) -> io::Result<Node> {
    let Some((entries, iter_err)) = enumerate(path) else {
        return Ok(Node {
            name,
            is_dir: true,
            denied: true,
            mtime,
            link,
            mass_dup,
            ..Node::default()
        });
    };
    // Ignore state enters on the raw names (.git / .gitignore are furniture
    // and about to be hidden); the matching exit wraps every return path.
    let entered = ctx
        .ignore
        .enter_dir(path, entries.iter().map(|e| e.name.as_str()));
    let title = ctx
        .titles
        .and_then(|set| readme_title(path, &entries, &name, set));
    let mut node = gather_dir_inner(
        path, name, remain, walk, ctx, mtime, link, mass_dup, iter_err, entries,
    );
    ctx.ignore.exit_dir(entered);
    if let Ok(n) = &mut node {
        n.title = title;
    }
    node
}

#[allow(clippy::too_many_arguments)]
fn gather_dir_inner(
    path: &Path,
    name: String,
    remain: Option<u32>,
    walk: &mut WalkBudget,
    ctx: &mut LookCtx,
    mtime: Option<i64>,
    link: Option<String>,
    mass_dup: bool,
    iter_err: bool,
    entries: Vec<Entry>,
) -> io::Result<Node> {
    // Furniture folds into state on this line before any census or child
    // list (src/def-furniture.md).
    let (reading, keep) = crate::furniture::read_names(
        ctx.map,
        ctx.view,
        entries.iter().map(|e| (e.name.as_str(), e.is_dir)),
    );
    walk.furniture_hidden += reading.hidden as u64;
    // The git facet is filled by a parallel post-pass (git::annotate —
    // its porcelain subprocess was a serial cost on multi-repo walks);
    // it inserts at facets[0] so the order stays [git, github].
    let mut facets = Vec::new();
    let has = |n: &str| entries.iter().any(|e| e.name == n);
    if reading.kinds.iter().any(|k| k == "github")
        && has(".github")
        && let Some(f) = crate::github::facet(path)
    {
        facets.push(f);
    }
    let kinds = reading.kinds;
    let hidden_dirs = reading.hidden_dirs;
    let entries: Vec<Entry> = entries
        .into_iter()
        .zip(keep)
        .filter(|(_, listed)| *listed)
        .map(|(e, _)| e)
        .collect();
    // Ignored bodies (design/gitignore-bodies.md): furniture fates applied
    // first (its rendering wins; no double marks) — the map's leftovers are
    // what the ignore rules see. Ignored *files* leave the child list and
    // every census bucket, counted in the typed remainder; ignored *dirs*
    // keep their line (presence) but are never expanded or weighed. Under
    // --show-all everything lists again, still carrying the mark.
    let ig: Vec<bool> = if ctx.ignore.active() {
        entries
            .iter()
            .map(|e| ctx.ignore.is_ignored(path, &e.name, e.is_dir))
            .collect()
    } else {
        vec![false; entries.len()]
    };
    let show_all = ctx.view.show_all;
    let ignored_files = if show_all {
        0
    } else {
        entries
            .iter()
            .zip(&ig)
            .filter(|(e, i)| **i && !e.is_dir)
            .count()
    };
    let (entries, ig): (Vec<Entry>, Vec<bool>) = if show_all {
        (entries, ig)
    } else {
        entries
            .into_iter()
            .zip(ig)
            .filter(|(e, i)| !*i || e.is_dir)
            .unzip()
    };
    if remain == Some(0) {
        // Depth cutoff: census of the children now; the subtree's deep
        // mass comes from the parallel deep phase (deep_phase) after the
        // whole gather — the serial inline walk was the multi-repo cost.
        let mut census = census_entries(&entries, iter_err, ctx);
        census.ignored = ignored_files;
        return Ok(Node {
            name,
            is_dir: true,
            leftover: if census.is_empty() {
                None
            } else {
                Some(census)
            },
            iter_err,
            kinds,
            hidden_dirs,
            facets,
            mtime,
            link,
            mass_dup,
            ..Node::default()
        });
    }
    let child_remain = remain.map(|n| n.saturating_sub(1));
    // On a connective level of a focus look the depth law is per-child
    // (design/focus.md §Multiple paths): the selected restart the count,
    // the chain keeps descending for free, and everyone else is walked
    // one level for its census and folded after the deep phase.
    let connective = ctx
        .focus
        .is_some_and(|f| f.is_connective(path) && !f.is_selected(path));
    let mut children: Vec<Node> = Vec::new();
    let mut cut = false;
    let mut early: Option<Census> = None;
    for (i, ent) in entries.iter().enumerate() {
        if walk.exhausted() {
            walk.tripped = true;
            cut = true;
            early = Some(census_entries(&entries[i..], false, ctx));
            break;
        }
        walk.charge();
        let child_path = path.join(&ent.name);
        let (child_remain, matched, aside) = match ctx.focus {
            Some(f) if connective => {
                if f.is_selected(&child_path) {
                    (if f.depth == 0 { None } else { Some(f.depth) }, true, false)
                } else if f.is_connective(&child_path) {
                    (None, false, false)
                } else {
                    (Some(0), false, true)
                }
            }
            _ => (child_remain, false, false),
        };
        // An ignored dir keeps its line but its innards are not the
        // project's: no expansion, no census, no mass — presence only.
        // (Under --show-all it expands like anything, mark kept.)
        let kid = if ig[i] && !show_all {
            stat_only(&child_path, ent.name.clone(), ctx.census_grain)
        } else {
            match gather_at(&child_path, child_remain, walk, ctx) {
                Ok(mut kid) => {
                    kid.ignored = ig[i];
                    kid
                }
                Err(_) => {
                    let mut n = Node {
                        name: ent.name.clone(),
                        is_dir: ent.is_dir,
                        denied: true,
                        ignored: ig[i],
                        ..Node::default()
                    };
                    if n.is_dir {
                        stamp_dir_type(&mut n, ctx.census_grain);
                    }
                    n
                }
            }
        };
        let mut kid = kid;
        kid.matched = matched;
        kid.aside = aside;
        children.push(kid);
    }
    // Mass folds bottom-up in the deep phase (mass_up), once every
    // cutoff subtree has its parallel deep walk done.
    if let Some(rest) = early {
        // The walk bound cut this level: the un-statted remainder is a
        // typed census; its subtrees were never walked, so mass floors
        // (mass_up reads `cut`).
        if children.is_empty() {
            return Ok(Node {
                name,
                is_dir: true,
                leftover: Some(rest),
                cut,
                iter_err,
                kinds,
                hidden_dirs,
                facets,
                mtime,
                link,
                mass_dup,
                ignored_files,
                ..Node::default()
            });
        }
        return Ok(Node {
            name,
            is_dir: true,
            children,
            omitted: Some(rest),
            cut,
            iter_err,
            kinds,
            hidden_dirs,
            facets,
            mtime,
            link,
            mass_dup,
            ignored_files,
            ..Node::default()
        });
    }
    Ok(Node {
        name,
        is_dir: true,
        children,
        cut,
        iter_err,
        kinds,
        hidden_dirs,
        facets,
        mtime,
        link,
        mass_dup,
        ignored_files,
        ..Node::default()
    })
}

/// Threads any parallel post-pass may spend (deep mass, git facets, heat).
pub const POOL_THREADS: usize = 8;

/// Is this node a depth-cutoff dir awaiting its deep walk?
fn wants_deep(n: &Node) -> bool {
    n.is_dir
        && n.children.is_empty()
        && n.leftover.is_some()
        && !n.denied
        && !n.cycle
        && !n.other_fs
}

fn collect_deep_paths(n: &Node, abs: &Path, out: &mut Vec<std::path::PathBuf>) {
    if wants_deep(n) {
        out.push(abs.to_path_buf());
    }
    for c in &n.children {
        collect_deep_paths(c, &abs.join(&c.name), out);
    }
}

fn assign_deep(n: &mut Node, results: &[Mass], idx: &mut usize) {
    if wants_deep(n) {
        let mut m = results[*idx];
        *idx += 1;
        m.bounded |= n.iter_err;
        if let Some(census) = &mut n.leftover
            && census.dirs > 0
        {
            let direct = census.buckets.iter().map(|(_, k)| *k as u64).sum::<u64>();
            census.dir_files = Some(Mass {
                files: m.files.saturating_sub(direct),
                lines: 0,
                est: m.est,
                bounded: m.bounded,
            });
        }
        n.mass = Some(m);
    }
    for c in &mut n.children {
        assign_deep(c, results, idx);
    }
}

/// The deep phase (hardening 2026-08-14): every cutoff subtree's deep
/// walk, in parallel over a bounded pool, each with a *deterministic
/// share* of the remaining read budget and name cap and its own seen-set
/// — so a directory's printed total is a function of that directory and
/// its share, not of what a sibling (or an --inspect'ed .git) happened to
/// read first. A symlink diamond spanning two cutoff subtrees now counts
/// in each (each dir's total describes that dir); parent aggregates still
/// count an expanded dup once via `mass_dup`. Then mass folds bottom-up.
pub fn deep_phase(node: &mut Node, abs: &Path, ctx: &mut LookCtx) {
    let mut paths = Vec::new();
    collect_deep_paths(node, abs, &mut paths);
    if !paths.is_empty() {
        let n = paths.len() as u64;
        let read_share = ctx.reads.remaining.map(|r| r / n);
        let name_share = (ctx.mass_names / n).max(1);
        let results: Vec<Mass> = std::thread::scope(|s| {
            let paths = &paths;
            let map = ctx.map;
            let view = ctx.view;
            let kinds = ctx.kinds;
            let non_blank = ctx.non_blank;
            let one_fs = ctx.one_fs;
            let root_dev = ctx.root_dev;
            let census_grain = ctx.census_grain;
            let threads = POOL_THREADS.min(paths.len());
            let handles: Vec<_> = (0..threads)
                .map(|t| {
                    s.spawn(move || {
                        let mut out = Vec::new();
                        for (i, p) in paths.iter().enumerate() {
                            if i % threads != t {
                                continue;
                            }
                            let mut c = LookCtx::new(map, view, kinds, non_blank, one_fs, 0);
                            c.census_grain = census_grain;
                            c.reads = ReadMeter::with(read_share);
                            c.mass_names = name_share;
                            c.root_dev = root_dev;
                            // Ignore rules from repos above the cutoff dir;
                            // its own .gitignore/.git are met by deep_mass.
                            if let Some(parent) = p.parent() {
                                c.ignore = crate::ignore::Stack::for_path(parent);
                            }
                            // Guard against a symlink looping back to the
                            // cutoff dir itself.
                            if let Ok(meta) = fs::metadata(p) {
                                let key = (meta.dev(), meta.ino());
                                c.seen.insert(key);
                                c.dir_stack.push(key);
                            }
                            out.push((i, deep_mass(p, &mut c)));
                        }
                        out
                    })
                })
                .collect();
            let mut results = vec![Mass::default(); paths.len()];
            for h in handles {
                for (i, m) in h.join().expect("deep-mass worker") {
                    results[i] = m;
                }
            }
            results
        });
        let mut idx = 0;
        assign_deep(node, &results, &mut idx);
    }
    mass_up(node);
}

/// Names visited per hidden furniture dir before its count floors (`≥`).
const HIDDEN_COUNT_CAP: u64 = 20_000;

/// Deep file-count by readdir type-hints alone — no stats, cheap even for
/// a big `target/`. Symlinks are left uncounted and floor the figure.
fn count_names(path: &Path, cap: &mut u64, bounded: &mut bool) -> u64 {
    let Ok(rd) = fs::read_dir(path) else {
        *bounded = true;
        return 0;
    };
    let mut files = 0;
    for ent in rd {
        let Ok(ent) = ent else {
            *bounded = true;
            continue;
        };
        if *cap == 0 {
            *bounded = true;
            return files;
        }
        *cap -= 1;
        match ent.file_type() {
            Ok(t) if t.is_dir() => files += count_names(&ent.path(), cap, bounded),
            Ok(t) if t.is_file() => files += 1,
            _ => *bounded = true,
        }
    }
    files
}

/// Presence survives hiding (design/furniture.md, three testimonies
/// 2026-08-14): every hidden furniture dir gets a deep file-count so the
/// has-spot can say `archive ≈127f` — magnitude without a child slot.
/// Bounded-parallel like the other post-passes.
pub fn hidden_phase(node: &mut Node, abs: &Path) {
    fn collect(n: &Node, abs: &Path, out: &mut Vec<std::path::PathBuf>) {
        for (_, name) in &n.hidden_dirs {
            out.push(abs.join(name));
        }
        for c in &n.children {
            collect(c, &abs.join(&c.name), out);
        }
    }
    fn assign(n: &mut Node, results: &[(u64, bool)], idx: &mut usize) {
        if !n.hidden_dirs.is_empty() {
            let mut per: std::collections::BTreeMap<String, (u64, bool)> =
                std::collections::BTreeMap::new();
            for (kind, _) in &n.hidden_dirs {
                let (files, bounded) = results[*idx];
                *idx += 1;
                let e = per.entry(kind.clone()).or_insert((0, false));
                e.0 += files;
                e.1 |= bounded;
            }
            n.has_counts = per.into_iter().map(|(k, (f, b))| (k, f, b)).collect();
        }
        for c in &mut n.children {
            assign(c, results, idx);
        }
    }
    let mut paths = Vec::new();
    collect(node, abs, &mut paths);
    if paths.is_empty() {
        return;
    }
    let results: Vec<(u64, bool)> = std::thread::scope(|s| {
        let paths = &paths;
        let threads = POOL_THREADS.min(paths.len());
        let handles: Vec<_> = (0..threads)
            .map(|t| {
                s.spawn(move || {
                    let mut out = Vec::new();
                    for (i, p) in paths.iter().enumerate() {
                        if i % threads != t {
                            continue;
                        }
                        let mut cap = HIDDEN_COUNT_CAP;
                        let mut bounded = false;
                        let files = count_names(p, &mut cap, &mut bounded);
                        out.push((i, (files, bounded)));
                    }
                    out
                })
            })
            .collect();
        let mut results = vec![(0, false); paths.len()];
        for h in handles {
            for (i, r) in h.join().expect("hidden-count worker") {
                results[i] = r;
            }
        }
        results
    });
    let mut idx = 0;
    assign(node, &results, &mut idx);
}

/// Fold mass bottom-up over expanded dirs (children carry theirs already
/// — cutoff nodes from the deep phase, files from their counts).
fn mass_up(node: &mut Node) {
    for c in &mut node.children {
        mass_up(c);
    }
    if !node.is_dir || node.denied || node.cycle || node.other_fs || wants_deep(node) {
        return;
    }
    let mut mass = Mass::default();
    for c in &node.children {
        if c.is_dir {
            match (&c.mass, c.mass_dup) {
                (Some(m), false) => mass.absorb(*m),
                // A diamond's weight was aggregated at its first sighting;
                // a cycle's belongs to the ancestor being expanded.
                (Some(_), true) => {}
                (None, _) => {}
            }
            if c.denied || c.other_fs {
                mass.bounded = true;
            }
        } else {
            mass.files += 1;
            match c.lines {
                Some(n) => mass.lines += n,
                None => {
                    if let Some(s) = c.size
                        && c.filetype.counts_lines()
                    {
                        mass.lines += est_lines(s);
                        mass.est = true;
                    }
                }
            }
        }
    }
    mass.bounded |= node.iter_err || node.cut;
    node.mass = Some(mass);
}

fn stamp_of(meta: &fs::Metadata) -> Option<i64> {
    meta.modified()
        .ok()
        .map(|t| match t.duration_since(std::time::UNIX_EPOCH) {
            Ok(d) => d.as_secs() as i64,
            Err(e) => -(e.duration().as_secs() as i64),
        })
}

fn fold_children_into_dir_census(node: &mut Node) {
    if node.children.is_empty() && node.omitted.is_none() {
        return;
    }
    let mut census = census_nodes(&node.children);
    census.bounded = node.children.iter().any(|c| c.denied);
    if let Some(om) = node.omitted.take() {
        census = merge_census(census, om);
    }
    node.leftover = Some(census);
    node.children.clear();
}

/// Survival tiers (design/sort.md key-within-weights; design/
/// important-files.md adds the middle tier; design/focus.md puts the
/// caller's explicit ask above every standing default):
/// focus > dirs > important > plain.
fn weight(n: &Node) -> u32 {
    if n.matched {
        8
    } else if n.is_dir {
        4
    } else if n.important {
        2
    } else {
        1
    }
}

/// Lines this node would spend if fully rendered.
fn capacity(n: &Node) -> usize {
    let mut c = 1 + usize::from(n.omitted.is_some());
    for k in &n.children {
        c += capacity(k);
    }
    c
}

/// `budget` includes this node's own line. `0` means no line limit.
pub fn apply_budget(
    node: &mut Node,
    budget: usize,
    order: &crate::sort::Order,
    explain: &mut Vec<String>,
) {
    if budget == 0 {
        return;
    }
    let n = node.children.len();
    if n == 0 {
        return;
    }
    // A remainder census this node already carries (a focus fold, or a
    // walk-bound cut) costs a line of its own. Reserve it, or the look
    // overshoots --lines by one per such level — the budget is a promise.
    let reserved = usize::from(node.omitted.as_ref().is_some_and(|c| !c.is_empty()));
    let remain = budget.saturating_sub(1).saturating_sub(reserved);
    if remain == 0 {
        explain.push(format!(
            "{}: budget 1 — census on this line, not a child [+]",
            node.name
        ));
        fold_children_into_dir_census(node);
        return;
    }
    if remain < n {
        // One line goes to the leftover census — unless this node already
        // reserved one, in which case the drops merge into it for free.
        let show = if reserved == 1 {
            remain
        } else {
            remain.saturating_sub(1)
        };
        if show == 0 {
            explain.push(format!(
                "{}: one leftover line would only repeat dir census",
                node.name
            ));
            fold_children_into_dir_census(node);
            return;
        }
        let mut rank: Vec<usize> = (0..n).collect();
        rank.sort_by(|&a, &b| {
            weight(&node.children[b])
                .cmp(&weight(&node.children[a]))
                .then_with(|| crate::sort::key_cmp(&node.children[a], &node.children[b], order))
                .then_with(|| node.children[a].name.cmp(&node.children[b].name))
        });
        let keep: Vec<usize> = rank.iter().copied().take(show).collect();
        let drop: Vec<Node> = node
            .children
            .iter()
            .enumerate()
            .filter(|(i, _)| !keep.contains(i))
            .map(|(_, c)| c.clone())
            .collect();
        node.children = keep.iter().map(|&i| node.children[i].clone()).collect();
        let mut om = census_nodes(&drop);
        if let Some(prev) = node.omitted.take() {
            om = merge_census(om, prev);
        }
        node.omitted = Some(om);
        explain.push(format!(
            "{}: listed {} / {n}, omitted {}",
            node.name,
            node.children.len(),
            drop.len()
        ));
        for c in &mut node.children {
            apply_budget(c, 1, order, explain);
        }
        return;
    }
    let extra = remain - n;
    let mut shares = vec![1usize; n];
    let mut unspent = 0usize;
    if extra > 0 {
        let caps: Vec<usize> = node.children.iter().map(capacity).collect();
        let need2: Vec<bool> = node
            .children
            .iter()
            .map(|c| c.children.len() >= 2)
            .collect();
        let mut left = extra;
        let mut progressed = true;
        while left > 0 && progressed {
            progressed = false;
            for i in 0..n {
                if left == 0 {
                    break;
                }
                if shares[i] >= caps[i] {
                    continue;
                }
                if shares[i] == 1 && need2[i] {
                    if left >= 2 {
                        shares[i] += 2;
                        left -= 2;
                        progressed = true;
                    }
                } else {
                    shares[i] += 1;
                    left -= 1;
                    progressed = true;
                }
            }
        }
        unspent = left;
    }
    explain.push(format!(
        "{}: budget {budget}, remain {remain}, shares {shares:?}{}",
        node.name,
        if unspent > 0 {
            format!(", unspent {unspent} (tree exhausted)")
        } else {
            String::new()
        }
    ));
    for (c, s) in node.children.iter_mut().zip(shares) {
        apply_budget(c, s, order, explain);
    }
}
