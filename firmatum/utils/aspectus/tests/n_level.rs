//! Depth is generations below the root. Real binary. Isolated XDG.

use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static SEQ: AtomicU64 = AtomicU64::new(0);

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_aspectus"))
}

fn fixture() -> (PathBuf, PathBuf) {
    let n = SEQ.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!(
        "aspectus-nl-{}-{}-{}",
        std::process::id(),
        n,
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(dir.join("a/b")).unwrap();
    File::create(dir.join("a/inside.txt"))
        .unwrap()
        .write_all(b"g")
        .unwrap();
    File::create(dir.join("a/b/deep.txt"))
        .unwrap()
        .write_all(b"gg")
        .unwrap();
    File::create(dir.join("f")).unwrap().write_all(b"f").unwrap();
    let xdg = std::env::temp_dir().join(format!(
        "aspectus-nl-xdg-{}-{}",
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
        .output()
        .unwrap();
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

#[test]
fn depth_1_is_children_only() {
    let (dir, xdg) = fixture();
    let (c, o, e) = run(&dir, &xdg, &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("a/"), "{o}");
    assert!(o.contains('f'), "{o}");
    assert!(!o.contains("inside.txt"), "depth 1 has no grandchildren: {o}");
    assert!(!o.contains("deep.txt"), "{o}");
}

#[test]
fn depth_2_includes_grandchildren() {
    let (dir, xdg) = fixture();
    let (c, o, e) = run(&dir, &xdg, &["--depth", "2"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("inside.txt"), "{o}");
    assert!(o.contains("b/"), "{o}");
    assert!(!o.contains("deep.txt"), "depth 2 stops at grandchildren: {o}");
}

#[test]
fn default_without_user_config_is_depth_2() {
    let (dir, xdg) = fixture();
    let (c, o, e) = run(&dir, &xdg, &[]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("inside.txt"), "{o}");
    assert!(!o.contains("deep.txt"), "{o}");
}

#[test]
fn user_home_depth_3_shows_great_grandchildren() {
    let (dir, xdg) = fixture();
    fs::write(xdg.join("aspectus/aspectus.toml"), "depth = 3\n").unwrap();
    let (c, o, e) = run(&dir, &xdg, &[]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("deep.txt"), "config depth 3: {o}");
}
