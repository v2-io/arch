//! GitHub furniture: .github/ is parent-line state, not a child. No network.

use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static SEQ: AtomicU64 = AtomicU64::new(0);

fn tmp(tag: &str) -> PathBuf {
    let n = SEQ.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!(
        "aspectus-gh-{tag}-{}-{}",
        std::process::id(),
        n
    ));
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn run(dir: &Path, args: &[&str]) -> (i32, String, String) {
    let xdg = tmp("xdg");
    fs::create_dir_all(xdg.join("aspectus")).unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_aspectus"))
        .args(args)
        .current_dir(dir)
        .env("XDG_CONFIG_HOME", &xdg)
        .env_remove("ASPECTUS_FURNITURE")
        .output()
        .unwrap();
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

#[test]
fn workflows_count_on_the_parent() {
    let dir = tmp("wf");
    fs::create_dir_all(dir.join(".github/workflows")).unwrap();
    File::create(dir.join(".github/workflows/ci.yml"))
        .unwrap()
        .write_all(b"on: push")
        .unwrap();
    File::create(dir.join(".github/workflows/release.yaml"))
        .unwrap()
        .write_all(b"on: tag")
        .unwrap();
    File::create(dir.join(".github/workflows/README.md"))
        .unwrap()
        .write_all(b"x")
        .unwrap();
    let (c, o, e) = run(&dir, &[]);
    assert_eq!(c, 0, "{e}");
    assert!(!o.contains(".github"), "{o}");
    // Root is the second line: the stamp has its own line above it.
    let root = o.lines().nth(1).unwrap();
    assert!(root.contains("[github: 2 workflows]"), "{o}");
    assert!(root.contains("github"), "{o}");
}

#[test]
fn kind_alone_when_no_workflows() {
    let dir = tmp("bare");
    fs::create_dir_all(dir.join(".github")).unwrap();
    File::create(dir.join(".github/FUNDING.yml"))
        .unwrap()
        .write_all(b"x")
        .unwrap();
    let (_, o, _) = run(&dir, &[]);
    assert!(!o.contains(".github/"), "{o}");
    assert!(!o.contains("[github:"), "no workflow claim to make: {o}");
    assert!(o.contains("[has: github]"), "{o}");
}

#[test]
fn inspect_github_lists_it() {
    let dir = tmp("insp");
    fs::create_dir_all(dir.join(".github/workflows")).unwrap();
    File::create(dir.join(".github/workflows/ci.yml"))
        .unwrap()
        .write_all(b"x")
        .unwrap();
    let (_, o, _) = run(&dir, &["--inspect", "github"]);
    assert!(o.contains(".github/"), "{o}");
    assert!(o.contains("workflows/"), "walk enters it: {o}");
}
