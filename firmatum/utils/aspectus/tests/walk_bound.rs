//! Walk bound: stop statting after N names and say so with `≥`.
//! Real binary. Isolated XDG.

use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static SEQ: AtomicU64 = AtomicU64::new(0);

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_aspectus"))
}

/// Root holds `big/` with 20 files; plus `top.txt`.
fn fixture() -> (PathBuf, PathBuf) {
    let n = SEQ.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!(
        "aspectus-wb-{}-{}-{}",
        std::process::id(),
        n,
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(dir.join("big")).unwrap();
    for i in 0..20 {
        File::create(dir.join(format!("big/file-{i:02}.txt"))).unwrap();
    }
    File::create(dir.join("top.txt")).unwrap();
    let xdg = std::env::temp_dir().join(format!(
        "aspectus-wb-xdg-{}-{}",
        std::process::id(),
        n
    ));
    fs::create_dir_all(xdg.join("aspectus")).unwrap();
    (dir, xdg)
}

fn run(dir: &Path, xdg: &Path, args: &[&str]) -> (i32, String, String) {
    let out = bin()
        .args(args)
        .current_dir(dir)
        .env("XDG_CONFIG_HOME", xdg)
        .env_remove("ASPECTUS_DEPTH")
        .env_remove("ASPECTUS_WALK")
        .env_remove("ASPECTUS_LINES")
        .output()
        .unwrap();
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

#[test]
fn unbounded_walk_is_exact() {
    let (dir, xdg) = fixture();
    let (c, o, e) = run(&dir, &xdg, &["--walk", "0", "--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("[txt×20]"), "exact census, no ≥: {o}");
    assert!(!o.contains('≥'), "{o}");
    assert!(!o.contains("[walk bound]"), "{o}");
}

#[test]
fn default_bound_does_not_trip_on_a_small_tree() {
    let (dir, xdg) = fixture();
    let (c, o, e) = run(&dir, &xdg, &[]);
    assert_eq!(c, 0, "{e}");
    assert!(!o.contains('≥'), "{o}");
    assert!(!o.contains("[walk bound]"), "{o}");
    assert!(e.lines().all(|l| l.is_empty() || l.starts_with("*(This is a critical")), "success is quiet: {e}");  // stderr: only the feedback footer (teaching) since 2026-08-22
}

#[test]
fn bound_keeps_level_membership_exact() {
    let (dir, xdg) = fixture();
    // Budget 1 stat: `big/` (dirs first) is statted and gets its full exact
    // census at the cutoff; `top.txt` falls to an exact remainder.
    let (c, o, e) = run(&dir, &xdg, &["--walk", "1", "--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("[txt×20]"), "census stays exact: {o}");
    assert!(o.contains("[+ "), "unspent sibling remainder keeps the + form: {o}");
    assert!(o.contains("[walk bound]"), "the cut is said: {o}");
    assert!(!o.contains('≥'), "nothing is unknown, so no floor-mark: {o}");
}

#[test]
fn bound_cutting_a_listing_marks_the_dir() {
    let (dir, xdg) = fixture();
    // Budget 1 stat: `big` is expanded, but none of its children can be
    // statted — they stay fully counted, and the dir line confesses.
    let (c, o, e) = run(&dir, &xdg, &["--walk", "1", "--depth", "2"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("[txt×20]"), "membership still total: {o}");
    assert!(o.contains("[walk bound]"), "a cut listing must confess: {o}");
    assert!(!o.contains("file-00.txt"), "no children were expanded: {o}");
}

#[test]
fn expansion_order_is_sorted_not_readdir_roulette() {
    let (dir, xdg) = fixture();
    // One stat: `big/` (first in dirs-first sort) must be the one
    // expanded — deterministically.
    let (c, o, e) = run(&dir, &xdg, &["--walk", "1", "--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("big/"), "first-in-sort-order is expanded: {o}");
    let (c2, o2, _) = run(&dir, &xdg, &["--walk", "1", "--depth", "1"]);
    assert_eq!(c2, 0);
    assert_eq!(o.lines().skip(1).collect::<Vec<_>>(),
               o2.lines().skip(1).collect::<Vec<_>>(),
               "two cut looks agree below the header");
}

#[test]
fn walk_rides_the_caller_stack() {
    let (dir, xdg) = fixture();
    fs::write(xdg.join("aspectus/aspectus.toml"), "walk = 1\n").unwrap();
    let (c, o, e) = run(&dir, &xdg, &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("[walk bound]"), "user-home walk=1 must bound: {o}");
    // Flags outrank the file.
    let (c2, o2, e2) = run(&dir, &xdg, &["--depth", "1", "--walk", "0"]);
    assert_eq!(c2, 0, "{e2}");
    assert!(!o2.contains("[walk bound]"), "flag 0 lifts the bound: {o2}");
}

#[test]
fn explain_budget_names_the_walk_stop() {
    let (dir, xdg) = fixture();
    let (c, _o, e) = run(&dir, &xdg, &["--walk", "1", "--explain-budget"]);
    assert_eq!(c, 0);
    assert!(e.contains("walk"), "stderr explains the walk stop: {e}");
}

#[test]
fn walk_needs_a_number() {
    let (dir, xdg) = fixture();
    let (c, _o, e) = run(&dir, &xdg, &["--walk", "many"]);
    assert_eq!(c, 2);
    assert!(e.contains("--walk"), "{e}");
}

#[test]
fn help_teaches_walk() {
    let (dir, xdg) = fixture();
    let (c, o, _e) = run(&dir, &xdg, &["help"]);
    assert_eq!(c, 0);
    assert!(o.contains("--walk"), "{o}");
    assert!(o.contains('≥'), "help names the ≥ confession: {o}");
}
