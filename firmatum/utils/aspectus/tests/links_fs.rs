//! Links and one filesystem (design/links-and-fs.md): targets shown,
//! symlinked dirs followed and recursed, cycles marked not hung, one-fs
//! default. Real binary, isolated XDG.

use std::fs;
use std::os::unix::fs::symlink;
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
        "aspectus-ln-{tag}-{}-{}-{}",
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
        .env_remove("ASPECTUS_SORT")
        .output()
        .unwrap();
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

/// Subfeature 1: targets render as recorded, relative and absolute.
#[test]
fn target_shown_verbatim() {
    let (dir, xdg) = fresh("tgt");
    fs::write(dir.join("real.md"), "x\n").unwrap();
    symlink("real.md", dir.join("rel-link")).unwrap();
    symlink(dir.join("real.md"), dir.join("abs-link")).unwrap();
    let (c, o, e) = run(&dir, &xdg, &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("rel-link -> real.md"), "{o}");
    assert!(o.contains(&format!("abs-link -> {}", dir.join("real.md").display())), "{o}");
}

/// Subfeature 2: a broken link says so, exit still 0.
#[test]
fn broken_link_confesses() {
    let (dir, xdg) = fresh("broken");
    symlink("nowhere", dir.join("dangling")).unwrap();
    let (c, o, e) = run(&dir, &xdg, &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("dangling -> nowhere [broken]"), "{o}");
}

/// Subfeature 3: a symlinked dir's children print, spending depth from the
/// link's position.
#[test]
fn symlinked_dir_recurses() {
    let (dir, xdg) = fresh("dir");
    fs::create_dir_all(dir.join("real")).unwrap();
    fs::write(dir.join("real/inside.md"), "1\n2\n").unwrap();
    symlink("real", dir.join("door")).unwrap();
    let (c, o, e) = run(&dir, &xdg, &["--depth", "2"]);
    assert_eq!(c, 0, "{e}");
    let door_idx = o.find("door/ -> real").expect(&o);
    let after = &o[door_idx..];
    assert!(after.contains("inside.md"), "children under the link line: {o}");
}

/// Subfeature 4: a symlinked file carries the target's facts.
#[test]
fn symlinked_file_has_target_facts() {
    let (dir, xdg) = fresh("facts");
    fs::write(dir.join("real.md"), "1\n2\n3\n").unwrap();
    symlink("real.md", dir.join("alias")).unwrap();
    let (c, o, e) = run(&dir, &xdg, &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    let line = o.lines().find(|l| l.contains("alias")).expect(&o);
    assert!(line.contains('3'), "target's line count on the link: {o}");
}

/// Subfeature 5: a link to an ancestor completes in bounded time with a
/// cycle mark.
#[test]
fn cycle_marks_and_terminates() {
    let (dir, xdg) = fresh("cycle");
    fs::create_dir_all(dir.join("a")).unwrap();
    symlink("..", dir.join("a/loop")).unwrap();
    let (c, o, e) = run(&dir, &xdg, &["--depth", "0"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("loop/ -> .."), "{o}");
    assert!(o.contains("[cycle]"), "{o}");
}

/// Subfeature 6: two links to one dir — deterministic, both expanded in the
/// look, mass counted once.
#[test]
fn diamond_is_deterministic() {
    let (dir, xdg) = fresh("diamond");
    fs::create_dir_all(dir.join("real")).unwrap();
    fs::write(dir.join("real/x.md"), "1\n").unwrap();
    symlink("real", dir.join("l1")).unwrap();
    symlink("real", dir.join("l2")).unwrap();
    let (_, o1, _) = run(&dir, &xdg, &["--depth", "2"]);
    let (_, o2, _) = run(&dir, &xdg, &["--depth", "2"]);
    let tail = |s: &str| s.lines().skip(1).collect::<Vec<_>>().join("\n");
    assert_eq!(tail(&o1), tail(&o2));
    assert_eq!(o1.matches("x.md").count(), 3, "real + both links expand: {o1}");
}

/// Subfeature 9: an unexpanded symlinked dir still gets a dir census.
#[test]
fn symlinked_dir_censuses_at_cutoff() {
    let (dir, xdg) = fresh("census");
    fs::create_dir_all(dir.join("real")).unwrap();
    fs::write(dir.join("real/a.md"), "1\n").unwrap();
    fs::write(dir.join("real/b.md"), "1\n").unwrap();
    symlink("real", dir.join("door")).unwrap();
    let (c, o, e) = run(&dir, &xdg, &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    let line = o.lines().find(|l| l.contains("door/")).expect(&o);
    assert!(line.contains("[md×2]"), "census like a real dir: {o}");
}

/// Subfeature 8 (half): --no-one-fs is accepted; the one-fs default needs
/// a mount fixture, so the flag path is what a portable test can hold.
#[test]
fn no_one_fs_flag_accepted() {
    let (dir, xdg) = fresh("nofs");
    fs::write(dir.join("a.md"), "1\n").unwrap();
    let (c, _, e) = run(&dir, &xdg, &["--no-one-fs", "--depth", "1"]);
    assert_eq!(c, 0, "{e}");
}
