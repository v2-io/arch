//! Mass (design/mass.md) and the reworked census (design/dir-census.md):
//! deep aggregates on unexpanded subtrees, furniture excluded, `≥` under
//! bounds. Real binary, isolated XDG.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static SEQ: AtomicU64 = AtomicU64::new(0);

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_aspectus"))
}

fn fresh(tag: &str) -> (PathBuf, PathBuf) {
    let n = SEQ.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!(
        "aspectus-ms-{tag}-{}-{}-{}",
        std::process::id(),
        n,
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&dir).unwrap();
    let xdg = PathBuf::from(format!("{}-xdg", dir.display()));
    fs::create_dir_all(xdg.join("aspectus")).unwrap();
    (dir, xdg)
}

fn run(dir: &Path, xdg: &Path, args: &[&str]) -> (i32, String, String) {
    let out = bin()
        .args(args)
        .current_dir(dir)
        .env("XDG_CONFIG_HOME", xdg)
        .env_remove("ASPECTUS_LINES")
        .env_remove("ASPECTUS_DEPTH")
        .env_remove("ASPECTUS_WALK")
        .env_remove("ASPECTUS_READS")
        .output()
        .unwrap();
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

/// Three levels of files below a depth-1 cutoff: the dir bucket carries the
/// deep file-count, the line total follows the census.
fn deep_fixture() -> (PathBuf, PathBuf) {
    let (dir, xdg) = fresh("deep");
    fs::create_dir_all(dir.join("top/mid/leaf")).unwrap();
    fs::write(dir.join("top/one.md"), "1\n2\n").unwrap();
    fs::write(dir.join("top/mid/two.md"), "1\n2\n3\n").unwrap();
    fs::write(dir.join("top/mid/leaf/three.md"), "1\n").unwrap();
    (dir, xdg)
}

#[test]
fn unexpanded_dir_carries_deep_weight() {
    let (dir, xdg) = deep_fixture();
    let (c, o, e) = run(&dir, &xdg, &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    let line = o.lines().find(|l| l.contains("top/")).expect(&o);
    // One dir among the children → name form; its deep files = 2 (below
    // mid/), direct one.md in the suffix bucket; subtree lines = 6.
    assert!(line.contains("mid/ ≈2f"), "container with deep files: {o}");
    assert!(line.contains("md×1"), "direct file bucket: {o}");
    assert!(line.contains("≈6 lines"), "subtree text lines: {o}");
}

#[test]
fn furniture_does_not_count() {
    let (dir, xdg) = fresh("furn");
    fs::create_dir_all(dir.join("crate/src")).unwrap();
    fs::create_dir_all(dir.join("crate/target/debug")).unwrap();
    fs::write(dir.join("crate/src/main.rs"), "fn main() {}\n").unwrap();
    for i in 0..30 {
        fs::write(dir.join(format!("crate/target/debug/junk{i}.d")), "x\n").unwrap();
    }
    let (c, o, e) = run(&dir, &xdg, &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    let line = o.lines().find(|l| l.contains("crate/")).expect(&o);
    assert!(line.contains("≈1f"), "target/'s debris is not the crate's mass: {o}");
    assert!(!line.contains("31"), "{o}");
}

#[test]
fn walk_bound_makes_mass_a_floor() {
    let (dir, xdg) = fresh("floor");
    fs::create_dir_all(dir.join("big/a")).unwrap();
    fs::create_dir_all(dir.join("big/b")).unwrap();
    for i in 0..10 {
        fs::write(dir.join(format!("big/a/f{i}.md")), "1\n").unwrap();
        fs::write(dir.join(format!("big/b/f{i}.md")), "1\n").unwrap();
    }
    // Two stats at depth 2: `big/` expands, `a/` is statted and censuses,
    // `b/` is never statted — the cut is said, and any deep number over
    // the cut subtree is a floor, never an exact claim.
    let (c, o, e) = run(&dir, &xdg, &["--depth", "2", "--walk", "2"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("[walk bound]"), "{o}");
    assert!(!o.contains("≈20f"), "cut mass must not claim exactness: {o}");
}

/// A dir whose files are all binary shows the census without a lines claim.
#[test]
fn binary_only_subtree_claims_no_lines() {
    let (dir, xdg) = fresh("binonly");
    fs::create_dir_all(dir.join("pics")).unwrap();
    fs::write(dir.join("pics/a.png"), [0u8, 1, 2]).unwrap();
    let (c, o, e) = run(&dir, &xdg, &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    let line = o.lines().find(|l| l.contains("pics/")).expect(&o);
    assert!(!line.contains("lines"), "no lines fact on binary mass: {o}");
    assert!(line.contains("[a.png]"), "single name shows: {o}");
}

/// Past the read budget, deep line totals are estimated — still `≈`,
/// never silence and never a stall.
#[test]
fn read_budget_estimates_marked() {
    let (dir, xdg) = fresh("budget");
    fs::create_dir_all(dir.join("deep")).unwrap();
    for i in 0..5 {
        fs::write(dir.join(format!("deep/f{i}.md")), "word\n".repeat(100)).unwrap();
    }
    fs::write(
        xdg.join("aspectus/aspectus.toml"),
        "reads = \"1\"\n", // effectively nothing may be read deep
    )
    .unwrap();
    let (c, o, e) = run(&dir, &xdg, &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    let line = o.lines().find(|l| l.contains("deep/")).expect(&o);
    assert!(line.contains("≈") && line.contains("lines"), "estimated, marked: {o}");
}

#[test]
fn deterministic_double_run() {
    let (dir, xdg) = deep_fixture();
    let (_, o1, _) = run(&dir, &xdg, &["--depth", "1"]);
    let (_, o2, _) = run(&dir, &xdg, &["--depth", "1"]);
    let tail = |s: &str| s.lines().skip(1).collect::<Vec<_>>().join("\n");
    assert_eq!(tail(&o1), tail(&o2));
}
