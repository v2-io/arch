//! Globify (design/globify.md): real sequences collapse; every guard
//! leans against false-positive collapse. Real binary.

use std::fs::{self, File};
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
        "aspectus-glob-{tag}-{}-{}-{}",
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

fn run(dir: &Path, xdg: &Path, envs: &[(&str, &str)], args: &[&str]) -> (i32, String, String) {
    let mut c = bin();
    c.args(args)
        .current_dir(dir)
        .env("XDG_CONFIG_HOME", xdg)
        .env_remove("ASPECTUS_LINES")
        .env_remove("ASPECTUS_DEPTH")
        .env_remove("ASPECTUS_GLOBIFY");
    for (k, v) in envs {
        c.env(k, v);
    }
    let out = c.output().unwrap();
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

/// Subfeature 1: a real series renders as one line, pattern + count.
#[test]
fn collapse() {
    let (dir, xdg) = fresh("collapse");
    for i in 1..=47 {
        File::create(dir.join(format!("output-{i:03}.bak"))).unwrap();
    }
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("output-[001-047].bak"), "the pattern: {o}");
    assert!(o.contains("(47 files)"), "the exact count: {o}");
    assert!(
        !o.contains("output-001.bak"),
        "members compact, not listed: {o}"
    );
}

/// Subfeature 2: below the threshold, members list individually.
#[test]
fn below_threshold_lists() {
    let (dir, xdg) = fresh("below");
    for i in 1..=4 {
        File::create(dir.join(format!("n{i}.txt"))).unwrap();
    }
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("n1.txt") && o.contains("n4.txt"), "{o}");
    assert!(!o.contains('['), "no collapse below min: {o}");
}

/// Subfeature 3: the guards — chapters (below threshold), mixed widths,
/// two varying fields — all stay individual.
#[test]
fn not_a_series_stays_individual() {
    let (dir, xdg) = fresh("guards");
    // Mixed digit widths (1..=12 unpadded: widths 1 and 2 both present,
    // ≥5 members each side would be needed; the 1-wide cohort is 9 and
    // *does* legitimately collapse — so use a width-split of 4/8).
    for i in 1..=4 {
        File::create(dir.join(format!("w-{i}.dat"))).unwrap();
    }
    for i in 10..=17 {
        File::create(dir.join(format!("w-{i}.dat"))).unwrap();
    }
    // Two varying fields.
    for (a, b) in [(1, 2), (2, 3), (3, 4), (4, 5), (5, 6), (6, 7)] {
        File::create(dir.join(format!("x{a}-y{b}.log"))).unwrap();
    }
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1", "--lines", "0"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("w-1.dat"), "narrow cohort below min lists: {o}");
    assert!(
        o.contains("w-[10-17].dat"),
        "the wide cohort is a series: {o}"
    );
    assert!(
        !o.contains("w-[1-17].dat") && !o.contains("w-[01-17].dat"),
        "mixed widths never fuse: {o}"
    );
    // x{a}-y{b}: both runs vary, so no (prefix, suffix) pair repeats — no
    // group forms and every name lists.
    assert!(o.contains("x1-y2.log") && o.contains("x6-y7.log"), "{o}");
    assert!(!o.contains("x[1-6]"), "two varying fields never fuse: {o}");
}

/// Subfeature 4: gaps are told — range is min–max, count is true.
#[test]
fn gaps_are_told() {
    let (dir, xdg) = fresh("gaps");
    for i in 1..=47 {
        if [13, 22, 30].contains(&i) {
            continue;
        }
        File::create(dir.join(format!("output-{i:03}.bak"))).unwrap();
    }
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    assert!(
        o.contains("output-[001-047].bak") && o.contains("(44 files)"),
        "span 001–047, count 44 — count ≠ span is the gap signal: {o}"
    );
}

/// Subfeature 5: the group costs one line; a fold's census counts the
/// members, not the listee.
#[test]
fn budget_arithmetic() {
    let (dir, xdg) = fresh("budget");
    fs::create_dir_all(dir.join("sub")).unwrap();
    for i in 1..=30 {
        File::create(dir.join(format!("sub/s{i:02}.dat"))).unwrap();
    }
    fs::write(dir.join("a.md"), "x\n").unwrap();
    // With room, the group is one listee costing one line.
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "2", "--lines", "7"]);
    assert_eq!(c, 0, "{e}");
    assert!(
        o.contains("s[01-30].dat") && o.contains("(30 files)"),
        "{o}"
    );
    // One line tighter: sub/ folds to its census — the collapsed group
    // folds back as 30 files, never one.
    let (c2, o2, e2) = run(&dir, &xdg, &[], &["--depth", "2", "--lines", "6"]);
    assert_eq!(c2, 0, "{e2}");
    let sub = o2.lines().find(|l| l.contains("sub/")).unwrap();
    assert!(
        sub.contains("dat×30"),
        "membership survives the fold: {sub}"
    );
}

/// Subfeature 6: an important file inside the pattern stays listed by
/// name; the rest still collapse.
#[test]
fn important_exempt() {
    let (dir, xdg) = fresh("imp");
    for i in 1..=6 {
        File::create(dir.join(format!("README-{i:03}.md"))).unwrap();
    }
    // The shipped README* row would make every member important; the
    // fixture narrows the set to exactly one member.
    let (c, o, e) = run(
        &dir,
        &xdg,
        &[("ASPECTUS_IMPORTANT", "README-001.md, !README*")],
        &["--depth", "1"],
    );
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("README-001.md"), "important stays by name: {o}");
    assert!(
        o.contains("README-[002-006].md") && o.contains("(5 files)"),
        "the rest still clear the threshold: {o}"
    );
}

/// Subfeature 7: `globify = off` and `--show-all` restore the names.
#[test]
fn off_switch_restores() {
    let (dir, xdg) = fresh("off");
    for i in 1..=8 {
        File::create(dir.join(format!("f{i}.tmp"))).unwrap();
    }
    let (c, o, e) = run(
        &dir,
        &xdg,
        &[("ASPECTUS_GLOBIFY", "off")],
        &["--depth", "1", "--lines", "0"],
    );
    assert_eq!(c, 0, "{e}");
    assert!(
        o.contains("f1.tmp") && o.contains("f8.tmp"),
        "config off: {o}"
    );
    let (c2, o2, _) = run(
        &dir,
        &xdg,
        &[],
        &["--depth", "1", "--lines", "0", "--show-all"],
    );
    assert_eq!(c2, 0);
    assert!(o2.contains("f1.tmp"), "--show-all restores: {o2}");
}

/// Subfeature 8: determinism — identical grouping and rendering.
#[test]
fn determinism() {
    let (dir, xdg) = fresh("det");
    for i in 1..=9 {
        File::create(dir.join(format!("a{i}.x"))).unwrap();
        File::create(dir.join(format!("b{i}.x"))).unwrap();
    }
    let (_, o1, _) = run(&dir, &xdg, &[], &["--depth", "1"]);
    let (_, o2, _) = run(&dir, &xdg, &[], &["--depth", "1"]);
    let strip = |s: &str| s.lines().skip(1).collect::<Vec<_>>().join("\n");
    assert_eq!(strip(&o1), strip(&o2), "modulo the header stamp");
}

/// Subfeature 9: JSON carries the group structured — pattern, min, max,
/// width, count, kind.
#[test]
fn json_structured() {
    let (dir, xdg) = fresh("json");
    for i in 1..=6 {
        fs::create_dir_all(dir.join(format!("run-{i:02}"))).unwrap();
    }
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1", "--format", "json"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("\"name\":\"run-[01-06]\""), "{o}");
    assert!(
        o.contains("\"glob\":{\"min\":1,\"max\":6,\"width\":2,\"count\":6,\"kind\":\"dir\"}"),
        "members recoverable in principle: {o}"
    );
}

/// Collapsed dirs are never expanded — no exemplar children.
#[test]
fn collapsed_dirs_not_expanded() {
    let (dir, xdg) = fresh("dirs");
    for i in 1..=6 {
        fs::create_dir_all(dir.join(format!("run-{i:02}"))).unwrap();
        fs::write(dir.join(format!("run-{i:02}/inner.md")), "x\n").unwrap();
    }
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "3", "--lines", "0"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("run-[01-06]") && o.contains("(6 dirs)"), "{o}");
    assert!(!o.contains("inner.md"), "no exemplar expansion: {o}");
}
