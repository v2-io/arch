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

/// Group a line total for the census channel: `61234` → `61k`.
pub fn group_lines(n: u64) -> String {
    if n < 1000 {
        n.to_string()
    } else if n < 10_000 {
        format!("{:.1}k", n as f64 / 1000.0)
    } else if n < 1_000_000 {
        format!("{}k", n / 1000)
    } else {
        format!("{:.1}M", n as f64 / 1_000_000.0)
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
    /// The counts are a floor (walk stopped early); renders `≥`.
    pub bounded: bool,
}

/// Names longer than this fall back to the count form (design open).
const NAME_FORM_CAP: usize = 24;

impl Census {
    pub fn is_empty(&self) -> bool {
        self.dirs == 0 && self.buckets.is_empty()
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
                Some(m) if m.files > 0 || m.bounded => {
                    let mark = if m.bounded { "≥" } else { "≈" };
                    format!(" {mark}{}f", m.files)
                }
                _ => String::new(),
            };
            parts.push(format!("{head}{mass}"));
        } else if self.buckets.iter().map(|(_, n)| n).sum::<usize>() == 1
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
    /// (`binary` among `.md`) or is asked `on`.
    pub kind_word: Option<&'static str>,
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
    /// Symlink target as the link states it (INFO, lattice `ON`).
    pub link: Option<String>,
    /// The target does not resolve to anything that exists.
    pub link_broken: bool,
    /// A symlink back to a directory already being expanded on this path:
    /// target shown, recursion refused, never a hang.
    pub cycle: bool,
    /// A mount point the one-fs default did not cross.
    pub other_fs: bool,
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
    bytes_counted: u64,
    lines_counted: u64,
}

impl ReadMeter {
    /// `budget` of 0 means unlimited.
    pub fn new(budget: u64) -> Self {
        ReadMeter {
            remaining: if budget == 0 { None } else { Some(budget) },
            bytes_counted: 0,
            lines_counted: 0,
        }
    }

    fn can_read(&self, size: u64) -> bool {
        match self.remaining {
            None => true,
            Some(r) => size <= r,
        }
    }

    fn charge(&mut self, bytes: u64, lines: u64) {
        if let Some(r) = &mut self.remaining {
            *r = r.saturating_sub(bytes);
        }
        self.bytes_counted += bytes;
        self.lines_counted += lines;
    }

    /// Estimated lines for an unread file, from the look's own observed
    /// bytes-per-line (fallback 32).
    fn estimate(&self, size: u64) -> u64 {
        let bpl = self
            .bytes_counted
            .checked_div(self.lines_counted)
            .map_or(32, |b| b.max(1));
        size / bpl
    }
}

/// Names deep-mass will visit per look before declaring the floor (`≥`).
const MASS_NAME_CAP: u64 = 500_000;

/// Everything the walk consults beyond the tree itself.
pub struct LookCtx<'a> {
    pub map: &'a crate::furniture::Map,
    pub view: &'a crate::furniture::View,
    pub kinds: &'a crate::kind::Map,
    /// Count non-blank instead of physical lines.
    pub non_blank: bool,
    /// Stay on the starting filesystem (default; `--no-one-fs` clears).
    pub one_fs: bool,
    pub reads: ReadMeter,
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
        kinds: &'a crate::kind::Map,
        non_blank: bool,
        one_fs: bool,
        read_budget: u64,
    ) -> Self {
        LookCtx {
            map,
            view,
            kinds,
            non_blank,
            one_fs,
            reads: ReadMeter::new(read_budget),
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

fn count_file(path: &Path, size: u64, ctx: &mut LookCtx) -> Count {
    count_file_at(path, size, ctx, false)
}

/// `visible` files (lines of the look itself) are read even past the
/// budget — the degraded forms belong to the unseen deep walk, not to the
/// glance's own lines. The read is still charged.
fn count_file_at(path: &Path, size: u64, ctx: &mut LookCtx, visible: bool) -> Count {
    use crate::kind::Kind;
    let name = path
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default();
    let kind = ctx.kinds.kind(&name);
    if kind == Kind::Binary {
        return Count::Binary;
    }
    if !visible && !ctx.reads.can_read(size) {
        return match kind {
            Kind::Text => Count::Est(ctx.reads.estimate(size)),
            // Unknown and unread: claims nothing, not even an estimate.
            _ => Count::Binary,
        };
    }
    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(_) => {
            return match kind {
                Kind::Text => Count::Denied,
                _ => Count::Binary,
            }
        }
    };
    if kind == Kind::Unknown && crate::kind::looks_binary(&bytes[..bytes.len().min(1024)]) {
        return Count::Binary;
    }
    let lines = if ctx.non_blank {
        crate::kind::non_blank_lines(&bytes)
    } else {
        crate::kind::physical_lines(&bytes)
    };
    ctx.reads.charge(bytes.len() as u64, lines);
    Count::Lines(lines)
}

/// `depth` is how many generations below the root to print. `0` = no limit.
pub fn gather(
    path: &Path,
    depth: u32,
    walk: &mut WalkBudget,
    ctx: &mut LookCtx,
) -> io::Result<Node> {
    let remain = if depth == 0 { None } else { Some(depth) };
    gather_at(path, remain, walk, ctx)
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

fn census_entries(entries: &[Entry], bounded: bool) -> Census {
    let dirs = entries.iter().filter(|e| e.is_dir).count();
    let dir_name = (dirs == 1)
        .then(|| entries.iter().find(|e| e.is_dir).map(|e| e.name.clone()))
        .flatten();
    let files: Vec<&Entry> = entries.iter().filter(|e| !e.is_dir).collect();
    let single = (dirs == 0 && files.len() == 1).then(|| files[0].name.clone());
    Census {
        dirs,
        dir_name,
        dir_files: None,
        buckets: bucketize(files.iter().map(|e| suffix_bucket(&e.name))),
        single,
        bounded,
    }
}

fn census_nodes(nodes: &[Node]) -> Census {
    let dirs = nodes.iter().filter(|n| n.is_dir).count();
    let dir_name = (dirs == 1)
        .then(|| nodes.iter().find(|n| n.is_dir).map(|n| n.name.clone()))
        .flatten();
    let mut dir_files: Option<Mass> = None;
    for n in nodes.iter().filter(|n| n.is_dir) {
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
    let single = (dirs == 0 && files.len() == 1).then(|| files[0].name.clone());
    Census {
        dirs,
        dir_name,
        dir_files,
        buckets: bucketize(files.iter().map(|n| suffix_bucket(&n.name))),
        single,
        bounded: false,
    }
}

/// Two censuses of disjoint name-sets become one. `bounded` is sticky.
fn merge_census(a: Census, b: Census) -> Census {
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
    let (_reading, keep) = crate::furniture::read_names(
        ctx.map,
        ctx.view,
        entries.iter().map(|e| (e.name.as_str(), e.is_dir)),
    );
    for (e, listed) in entries.iter().zip(keep) {
        if !listed {
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
            match count_file(&p, meta.len(), ctx) {
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
                return Ok(Node {
                    name,
                    is_dir: false,
                    mtime,
                    mode,
                    uid,
                    gid,
                    link: target,
                    link_broken: true,
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
        let size = meta.is_file().then_some(meta.len());
        let lines = if meta.is_file() {
            match count_file_at(path, meta.len(), ctx, true) {
                Count::Lines(n) => Some(n),
                _ => None,
            }
        } else {
            None
        };
        return Ok(Node {
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
            ..Node::default()
        });
    }
    let key = (meta.dev(), meta.ino());
    if ctx.root_dev.is_none() {
        ctx.root_dev = Some(meta.dev());
    } else if ctx.one_fs && ctx.root_dev != Some(meta.dev()) {
        // A filesystem boundary, not an empty directory, stopped here.
        return Ok(Node {
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
        });
    }
    if ctx.dir_stack.contains(&key) {
        return Ok(Node {
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
        });
    }
    let mass_dup = !ctx.seen.insert(key);
    ctx.dir_stack.push(key);
    let node = gather_dir(path, name, remain, walk, ctx, mtime, link, mass_dup);
    ctx.dir_stack.pop();
    node.map(|mut n| {
        n.mode = mode;
        n.uid = uid;
        n.gid = gid;
        n
    })
}

/// Identity facts every stat already paid for (quiet-columns' substrate).
fn ids_of(meta: &fs::Metadata) -> (Option<u32>, Option<u32>, Option<u32>) {
    (
        Some(meta.mode() & 0o7777),
        Some(meta.uid()),
        Some(meta.gid()),
    )
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
    // Furniture folds into state on this line before any census or child
    // list (src/def-furniture.md).
    let (reading, keep) = crate::furniture::read_names(
        ctx.map,
        ctx.view,
        entries.iter().map(|e| (e.name.as_str(), e.is_dir)),
    );
    walk.furniture_hidden += reading.hidden as u64;
    let mut facets = Vec::new();
    let has = |n: &str| entries.iter().any(|e| e.name == n);
    if reading.kinds.iter().any(|k| k == "git")
        && has(".git")
        && let Some(f) = crate::git::facet(path)
    {
        facets.push(f);
    }
    if reading.kinds.iter().any(|k| k == "github")
        && has(".github")
        && let Some(f) = crate::github::facet(path)
    {
        facets.push(f);
    }
    let kinds = reading.kinds;
    let entries: Vec<Entry> = entries
        .into_iter()
        .zip(keep)
        .filter(|(_, listed)| *listed)
        .map(|(e, _)| e)
        .collect();
    if remain == Some(0) {
        // Depth cutoff: census of the children, deep mass of the subtree.
        let mass = deep_mass_here(path, ctx, iter_err);
        let mut census = census_entries(&entries, iter_err);
        if census.dirs > 0 {
            let direct_files = census.buckets.iter().map(|(_, n)| *n as u64).sum::<u64>();
            census.dir_files = Some(Mass {
                files: mass.files.saturating_sub(direct_files),
                lines: 0,
                est: mass.est,
                bounded: mass.bounded,
            });
        }
        return Ok(Node {
            name,
            is_dir: true,
            leftover: if census.is_empty() { None } else { Some(census) },
            iter_err,
            kinds,
            facets,
            mtime,
            link,
            mass: Some(mass),
            mass_dup,
            ..Node::default()
        });
    }
    let child_remain = remain.map(|n| n.saturating_sub(1));
    let mut children: Vec<Node> = Vec::new();
    let mut cut = false;
    let mut early: Option<Census> = None;
    for (i, ent) in entries.iter().enumerate() {
        if walk.exhausted() {
            walk.tripped = true;
            cut = true;
            early = Some(census_entries(&entries[i..], false));
            break;
        }
        walk.charge();
        let child_path = path.join(&ent.name);
        let kid = match gather_at(&child_path, child_remain, walk, ctx) {
            Ok(kid) => kid,
            Err(_) => Node {
                name: ent.name.clone(),
                is_dir: ent.is_dir,
                denied: true,
                ..Node::default()
            },
        };
        children.push(kid);
    }
    // Subtree mass, bottom-up: expanded children carry theirs already.
    let mut mass = Mass::default();
    for c in &children {
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
                        && ctx.kinds.kind(&c.name) == crate::kind::Kind::Text
                    {
                        mass.lines += ctx.reads.estimate(s);
                        mass.est = true;
                    }
                }
            }
        }
    }
    mass.bounded |= iter_err;
    if let Some(rest) = early {
        // The walk bound cut this level: the un-statted remainder is a
        // typed census; its subtrees were never walked, so mass floors.
        mass.bounded = true;
        if children.is_empty() {
            return Ok(Node {
                name,
                is_dir: true,
                leftover: Some(rest),
                cut,
                iter_err,
                kinds,
                facets,
                mtime,
                link,
                mass: Some(mass),
                mass_dup,
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
            facets,
            mtime,
            link,
            mass: Some(mass),
            mass_dup,
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
        facets,
        mtime,
        link,
        mass: Some(mass),
        mass_dup,
        ..Node::default()
    })
}

/// Deep mass at a cutoff. The cutoff dir itself is already on the stack
/// and in `seen`, so only its innards are walked here.
fn deep_mass_here(path: &Path, ctx: &mut LookCtx, iter_err: bool) -> Mass {
    let mut m = deep_mass(path, ctx);
    m.bounded |= iter_err;
    m
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
/// important-files.md adds the middle tier): dirs > important > plain.
fn weight(n: &Node) -> u32 {
    if n.is_dir {
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
    let remain = budget.saturating_sub(1);
    if remain == 0 {
        explain.push(format!(
            "{}: budget 1 — census on this line, not a child [+]",
            node.name
        ));
        fold_children_into_dir_census(node);
        return;
    }
    if remain < n {
        let show = remain.saturating_sub(1);
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
        let need2: Vec<bool> = node.children.iter().map(|c| c.children.len() >= 2).collect();
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
