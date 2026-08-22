//! Sort: recency default, groupings, keys, refusals, survival, totality.
//! Real binary. Isolated XDG. Mtimes are set explicitly so the fixtures
//! do not depend on creation timing.

use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, UNIX_EPOCH};

static SEQ: AtomicU64 = AtomicU64::new(0);

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_aspectus"))
}

fn fresh(tag: &str) -> (PathBuf, PathBuf) {
    let n = SEQ.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!(
        "aspectus-sort-{tag}-{}-{}-{}",
        std::process::id(),
        n,
        std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&dir).unwrap();
    // Outside the looked-at tree, or it would join the look.
    let xdg = PathBuf::from(format!("{}-xdg", dir.display()));
    fs::create_dir_all(xdg.join("aspectus")).unwrap();
    (dir, xdg)
}

fn set_mtime(p: &Path, epoch_secs: u64) {
    let f = File::options().write(true).open(p).unwrap();
    f.set_modified(UNIX_EPOCH + Duration::from_secs(epoch_secs))
        .unwrap();
}

fn touch(dir: &Path, name: &str, epoch_secs: u64) {
    let p = dir.join(name);
    File::create(&p).unwrap().write_all(b"x").unwrap();
    set_mtime(&p, epoch_secs);
}

fn run(dir: &Path, xdg: &Path, args: &[&str]) -> (i32, String, String) {
    let out = bin()
        .args(args)
        .current_dir(dir)
        .env("XDG_CONFIG_HOME", xdg)
        .env_remove("ASPECTUS_LINES")
        .env_remove("ASPECTUS_DEPTH")
        .env_remove("ASPECTUS_SORT")
        .env_remove("ASPECTUS_COLUMNS_SIZE")
        .env_remove("ASPECTUS_COLUMNS_MTIME")
        .output()
        .unwrap();
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

fn names_in_order(o: &str) -> Vec<String> {
    o.lines()
        // Header: stamp, the root's facts line when it has any, the bare
        // root path (starts with /), and the column-headings line
        // (2026-08-14) — child lines are the branch-glyph ones.
        .skip_while(|l| !l.starts_with('/'))
        .skip(1)
        .filter(|l| l.contains("── "))
        .filter(|l| !l.contains("[+"))
        .filter_map(|l| {
            l.rsplit("── ")
                .next()
                .map(|s| s.split_whitespace().next().unwrap_or("").to_string())
        })
        .collect()
}

/// old < mid < new, plus a dir. Recency default: dir first (dirs-first),
/// then newest file first.
fn fixture() -> (PathBuf, PathBuf) {
    let (dir, xdg) = fresh("fx");
    touch(&dir, "mid.md", 1_700_000_500);
    touch(&dir, "old.md", 1_700_000_000);
    touch(&dir, "new.md", 1_700_000_900);
    fs::create_dir_all(dir.join("zdir")).unwrap();
    // A fresh dir would surprise the quiet mtime law; this fixture is
    // about order, not surprise.
    File::open(dir.join("zdir"))
        .unwrap()
        .set_modified(UNIX_EPOCH + Duration::from_secs(1_700_001_000))
        .unwrap();
    (dir, xdg)
}

#[test]
fn default_is_dirs_first_then_newest() {
    let (dir, xdg) = fixture();
    let (c, o, e) = run(&dir, &xdg, &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    assert_eq!(
        names_in_order(&o),
        ["zdir/", "new.md", "mid.md", "old.md"],
        "{o}"
    );
}

#[test]
fn sort_name_restores_alphabetical() {
    let (dir, xdg) = fixture();
    let (c, o, e) = run(&dir, &xdg, &["--depth", "1", "--sort", "name"]);
    assert_eq!(c, 0, "{e}");
    assert_eq!(
        names_in_order(&o),
        ["zdir/", "mid.md", "new.md", "old.md"],
        "{o}"
    );
}

#[test]
fn reversed_key_is_oldest_first() {
    let (dir, xdg) = fixture();
    let (c, o, e) = run(&dir, &xdg, &["--depth", "1", "--sort", "-recency"]);
    assert_eq!(c, 0, "{e}");
    assert_eq!(
        names_in_order(&o),
        ["zdir/", "old.md", "mid.md", "new.md"],
        "{o}"
    );
}

#[test]
fn equal_mtimes_tiebreak_by_name() {
    let (dir, xdg) = fresh("tie");
    for n in ["b.md", "a.md", "c.md"] {
        touch(&dir, n, 1_700_000_000);
    }
    let (c, o, e) = run(&dir, &xdg, &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    assert_eq!(names_in_order(&o), ["a.md", "b.md", "c.md"], "{o}");
}

#[test]
fn dotfiles_first_groups_within_dirs_first() {
    let (dir, xdg) = fresh("dots");
    touch(&dir, ".x", 1_700_000_100);
    touch(&dir, "f", 1_700_000_900);
    fs::create_dir_all(dir.join("a")).unwrap();
    fs::create_dir_all(dir.join(".b")).unwrap();
    let (c, o, e) = run(&dir, &xdg, &["--depth", "1", "--dotfiles-first", "--sort", "name"]);
    assert_eq!(c, 0, "{e}");
    assert_eq!(names_in_order(&o), [".b/", "a/", ".x", "f"], "{o}");
}

#[test]
fn config_sets_order_flag_wins() {
    let (dir, xdg) = fixture();
    fs::write(xdg.join("aspectus/aspectus.toml"), "sort = name\n").unwrap();
    let (c, o, e) = run(&dir, &xdg, &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    assert_eq!(names_in_order(&o)[1], "mid.md", "user-home wins: {o}");
    let (c, o, e) = run(&dir, &xdg, &["--depth", "1", "--sort", "recency"]);
    assert_eq!(c, 0, "{e}");
    assert_eq!(names_in_order(&o)[1], "new.md", "flag wins over config: {o}");
}

#[test]
fn unbuilt_key_refused_with_menu() {
    let (dir, xdg) = fixture();
    // heat landed; created is still an unbuilt lattice sort fact.
    let (c, _, e) = run(&dir, &xdg, &["--sort", "created"]);
    assert_eq!(c, 2);
    assert!(e.contains("not built"), "class named: {e}");
    assert!(
        e.contains("recency") && e.contains("name") && e.contains("size"),
        "menu of built keys: {e}"
    );
}

#[test]
fn unknown_key_refused() {
    let (dir, xdg) = fixture();
    let (c, _, e) = run(&dir, &xdg, &["--sort", "flavor"]);
    assert_eq!(c, 2);
    assert!(e.contains("not a sortable fact"), "{e}");
}

/// Survival follows the key within weights: under a tight budget the
/// newest files are the ones listed, not the alphabetical head.
#[test]
fn tight_budget_keeps_the_newest() {
    let (dir, xdg) = fresh("surv");
    for i in 0..8 {
        // a..h created so that later alphabet = newer.
        let name = format!("{}.md", (b'a' + i) as char);
        touch(&dir, &name, 1_700_000_000 + u64::from(i) * 100);
    }
    // stamp + headings + root + 3 files + [+5 census] = 7 lines
    // (the column-headings line, 2026-08-14, costs one when columns render).
    let (c, o, e) = run(&dir, &xdg, &["--depth", "1", "--lines", "7"]);
    assert_eq!(c, 0, "{e}");
    assert!(
        o.contains("h.md") && o.contains("g.md") && o.contains("f.md"),
        "newest survive: {o}"
    );
    assert!(!o.contains("a.md"), "oldest folds into the census: {o}");
    assert!(o.contains("[+ md×5]"), "{o}");
}

/// An explicitly asked key puts its evidence on the line; the recency
/// default does not (position already carries the signal).
#[test]
fn explicit_sort_key_implies_its_column() {
    let (dir, xdg) = fixture();
    let (_, o, _) = run(&dir, &xdg, &["--depth", "1"]);
    assert_eq!(o.matches('Z').count(), 1, "default look stays quiet: {o}");
    let (_, o, _) = run(&dir, &xdg, &["--depth", "1", "--sort", "size"]);
    // 2026-08-22 count-cell slice: 1 byte is `1.` under the `bytes` heading,
    // not `1B`. The heading is the evidence the column was implied on.
    assert!(o.contains("bytes"), "size evidence on the line: {o}");
    let (_, o, _) = run(&dir, &xdg, &["--depth", "1", "--sort", "recency"]);
    // The implied mtime column speaks in the relative default (2026-08-14).
    assert!(o.matches("ago").count() > 1, "mtime evidence when asked: {o}");
}

/// Total order: byte-identical across runs regardless of creation order.
#[test]
fn total_order_survives_shuffled_creation() {
    let mk = |names: &[&str]| {
        let (dir, xdg) = fresh("shuf");
        for n in names {
            touch(&dir, n, 1_700_000_000);
        }
        let (_, o, _) = run(&dir, &xdg, &["--depth", "1"]);
        names_in_order(&o)
    };
    assert_eq!(
        mk(&["c.md", "a.md", "b.md"]),
        mk(&["b.md", "c.md", "a.md"]),
        "creation order is invisible"
    );
}

#[test]
fn double_run_is_byte_identical_below_stamp() {
    let (dir, xdg) = fixture();
    let (_, o1, _) = run(&dir, &xdg, &["--depth", "1"]);
    let (_, o2, _) = run(&dir, &xdg, &["--depth", "1"]);
    assert_eq!(
        o1.lines().skip(1).collect::<Vec<_>>(),
        o2.lines().skip(1).collect::<Vec<_>>()
    );
}
