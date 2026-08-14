//! Labels: a kind on a line is a claim, not a guess. Real binary.

use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static SEQ: AtomicU64 = AtomicU64::new(0);

fn fixture() -> (PathBuf, PathBuf) {
    let n = SEQ.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!(
        "aspectus-lab-{}-{}",
        std::process::id(),
        n
    ));
    let xdg = dir.join("xdg");
    fs::create_dir_all(dir.join("tree")).unwrap();
    fs::create_dir_all(xdg.join("aspectus")).unwrap();
    (dir.join("tree"), xdg)
}

fn run(dir: &Path, xdg: &Path, args: &[&str]) -> (i32, String, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_aspectus"))
        .args(args)
        .current_dir(dir)
        .env("XDG_CONFIG_HOME", xdg)
        .env_remove("ASPECTUS_FURNITURE")
        .output()
        .unwrap();
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

fn touch(p: &Path) {
    File::create(p).unwrap().write_all(b"x").unwrap();
}

#[test]
fn mark_rows_stay_children_and_claim_a_kind() {
    let (dir, xdg) = fixture();
    touch(&dir.join("Cargo.toml"));
    touch(&dir.join("main.rs"));
    let (c, o, e) = run(&dir, &xdg, &[]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("Cargo.toml"), "mark rows are still children: {o}");
    assert!(o.contains("[kind: rust]"), "the claim on the parent: {o}");
}

#[test]
fn kinds_claim_only_what_known_names_say() {
    let (dir, xdg) = fixture();
    touch(&dir.join("pyproject.toml"));
    touch(&dir.join("CLAUDE.md"));
    let (_, o, _) = run(&dir, &xdg, &[]);
    assert!(o.contains("[kind: agents, python]"), "{o}");
    assert!(!o.contains("rust"), "no guess: {o}");
}

#[test]
fn a_dir_with_no_known_names_makes_no_claim() {
    let (dir, xdg) = fixture();
    touch(&dir.join("notes.txt"));
    let (_, o, _) = run(&dir, &xdg, &[]);
    assert!(!o.contains("[kind"), "empty set prints nothing: {o}");
}

#[test]
fn cutoff_dirs_still_claim_their_kinds() {
    let (dir, xdg) = fixture();
    fs::create_dir_all(dir.join("proj")).unwrap();
    touch(&dir.join("proj/Cargo.toml"));
    fs::create_dir_all(dir.join("proj/target")).unwrap();
    touch(&dir.join("proj/target/junk.o"));
    // Depth 1: proj/ is at the cutoff — its census must exclude target/,
    // and the line still says what the place is.
    let (_, o, _) = run(&dir, &xdg, &["--depth", "1"]);
    let proj = o.lines().find(|l| l.contains("proj/")).unwrap();
    assert!(proj.contains("[kind: build, rust]"), "{o}");
    assert!(proj.contains("[Cargo.toml]"), "census counts children of the look only: {o}");
}
