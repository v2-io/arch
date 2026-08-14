//! Line budget + leaf census. Real binary. Isolated XDG.

use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static SEQ: AtomicU64 = AtomicU64::new(0);

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_aspectus"))
}

fn fixture_many_files() -> (PathBuf, PathBuf) {
    let n = SEQ.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!(
        "aspectus-bal-{}-{}-{}",
        std::process::id(),
        n,
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&dir).unwrap();
    for i in 0..8 {
        File::create(dir.join(format!("n{i}.txt")))
            .unwrap()
            .write_all(b"x")
            .unwrap();
    }
    fs::create_dir_all(dir.join("sub")).unwrap();
    File::create(dir.join("sub/a.rs")).unwrap().write_all(b"a").unwrap();
    let xdg = std::env::temp_dir().join(format!("aspectus-bal-xdg-{}-{}", std::process::id(), n));
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
        .output()
        .unwrap();
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

#[test]
fn tight_lines_has_plus_census_not_ellipsis() {
    let (dir, xdg) = fixture_many_files();
    let (c, o, e) = run(&dir, &xdg, &["--depth", "1", "--lines", "5"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("[+"), "leaf leftover uses + : {o}");
    assert!(o.contains(".txt"), "{o}");
    assert!(!o.contains("unlisted"), "{o}");
    assert!(!o.contains("…"), "{o}");
}

#[test]
fn first_child_does_not_eat_the_budget() {
    let (dir, xdg) = fixture_many_files();
    let (c, o, e) = run(&dir, &xdg, &["--depth", "1", "--lines", "6"]);
    assert_eq!(c, 0, "{e}");
    let listed_txt = o.matches(".txt").count();
    assert!(
        listed_txt >= 2,
        "siblings share; more than one file name should appear: {o}"
    );
}

#[test]
fn explain_budget_goes_to_stderr() {
    let (dir, xdg) = fixture_many_files();
    let (c, o, e) = run(
        &dir,
        &xdg,
        &["--depth", "1", "--lines", "5", "--explain-budget"],
    );
    assert_eq!(c, 0);
    assert!(!e.is_empty(), "explain on stderr");
    assert!(e.contains("budget") || e.contains("listed") || e.contains("share"), "{e}");
    assert!(!o.is_empty());
}

#[test]
fn dir_with_only_its_own_line_censuses_on_the_name() {
    let n = SEQ.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!(
        "aspectus-fold-{}-{}-{}",
        std::process::id(),
        n,
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(dir.join("ux/autocolors")).unwrap();
    File::create(dir.join("ux/autocolors/a.md"))
        .unwrap()
        .write_all(b"a")
        .unwrap();
    File::create(dir.join("ux/autocolors/b.md"))
        .unwrap()
        .write_all(b"b")
        .unwrap();
    let xdg = std::env::temp_dir().join(format!("aspectus-fold-xdg-{}-{}", std::process::id(), n));
    fs::create_dir_all(xdg.join("aspectus")).unwrap();
    let (c, o, e) = run(&dir, &xdg, &["--depth", "3", "--lines", "3"]);
    assert_eq!(c, 0, "{e}");
    assert!(
        o.contains("autocolors/") && o.contains('['),
        "census on the dir line: {o}"
    );
    assert!(
        !o.contains("[+"),
        "must not spend a child line on the same census: {o}"
    );
}

#[test]
fn unlimited_lines_zero_lists_all() {
    let (dir, xdg) = fixture_many_files();
    let (c, o, e) = run(&dir, &xdg, &["--depth", "1", "--lines", "0"]);
    assert_eq!(c, 0, "{e}");
    assert!(!o.contains("[+"), "no omit when unlimited: {o}");
    for i in 0..8 {
        assert!(o.contains(&format!("n{i}.txt")), "missing n{i}.txt in {o}");
    }
}
