//! Ignored-dir bytes (design/ignored-bytes.md): an unopened body still
//! says how big it is — on the `⊘` line in the `bytes` column, and on a
//! hidden furniture has-word — never expanded, never in mass, never on
//! the `--walk` budget. Real binary, real repos, isolated XDG and HOME.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static SEQ: AtomicU64 = AtomicU64::new(0);

fn fresh(tag: &str) -> (PathBuf, PathBuf) {
    let n = SEQ.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!(
        "aspectus-ib-{tag}-{}-{}-{}",
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

fn run(dir: &Path, xdg: &Path, env: &[(&str, &str)], args: &[&str]) -> (i32, String, String) {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_aspectus"));
    cmd.args(args)
        .current_dir(dir)
        .env("XDG_CONFIG_HOME", xdg)
        .env("HOME", xdg)
        .env("ASPECTUS_COLUMNS_HEAT", "off")
        .env_remove("ASPECTUS_LINES")
        .env_remove("ASPECTUS_DEPTH")
        .env_remove("ASPECTUS_COLUMNS_SIZE")
        .env_remove("ASPECTUS_FORMAT_SIZE");
    for (k, v) in env {
        cmd.env(k, v);
    }
    let out = cmd.output().unwrap();
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

fn git_init(dir: &Path) {
    assert!(
        Command::new("git")
            .args(["init", "-q"])
            .current_dir(dir)
            .status()
            .unwrap()
            .success()
    );
}

fn line_of<'a>(o: &'a str, name: &str) -> &'a str {
    o.lines()
        .find(|l| l.contains(name))
        .unwrap_or_else(|| panic!("no line for {name}: {o}"))
}

/// A repo with an ignored `logs/` holding 3,050,000 bytes (one of them
/// hardlinked twice) and 30 small files, plus a tracked `a.md`.
fn ignored_fixture(tag: &str) -> (PathBuf, PathBuf) {
    let (dir, xdg) = fresh(tag);
    git_init(&dir);
    fs::write(dir.join(".gitignore"), "logs/\n").unwrap();
    fs::create_dir_all(dir.join("logs/sub")).unwrap();
    fs::write(dir.join("logs/a.bin"), vec![0u8; 3_000_000]).unwrap();
    fs::hard_link(dir.join("logs/a.bin"), dir.join("logs/sub/same.bin")).unwrap();
    fs::write(dir.join("logs/sub/b.bin"), vec![0u8; 50_000]).unwrap();
    for i in 0..30 {
        fs::write(dir.join(format!("logs/f{i}.log")), "").unwrap();
    }
    fs::write(dir.join("a.md"), "x\n").unwrap();
    (dir, xdg)
}

/// Subfeatures 1 and 2: the `⊘` line carries its body's bytes in the
/// `bytes` column (which appears for it under the quiet default),
/// hardlinks once; nothing inside prints.
#[test]
fn ignored_dir_says_how_big() {
    let (dir, xdg) = ignored_fixture("line");
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "2"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("bytes"), "the bytes heading appears: {o}");
    let logs = line_of(&o, "logs/");
    assert!(logs.contains('\u{2298}'), "still ⊘: {logs}");
    // 3,050,000 B = 2.9 MiB; with the hardlink counted twice it would be 5.8.
    assert!(logs.contains("2.9M"), "bytes on the ⊘ line: {o}");
    assert!(!o.contains("f0.log") && !o.contains("same.bin"), "{o}");
}

/// Subfeature 3: `format.size = bytes` is the raw integer.
#[test]
fn raw_bytes_format() {
    let (dir, xdg) = ignored_fixture("raw");
    let (c, o, e) = run(&dir, &xdg, &[("ASPECTUS_FORMAT_SIZE", "bytes")], &["--depth", "2"]);
    assert_eq!(c, 0, "{e}");
    assert!(line_of(&o, "logs/").contains("3050000"), "{o}");
}

/// Subfeature 4: `columns.size = off` is off — the caller's explicit ask
/// wins, as `lines` off silences mass (a call, design/ignored-bytes.md).
#[test]
fn size_off_is_off() {
    let (dir, xdg) = ignored_fixture("off");
    let (c, o, e) = run(&dir, &xdg, &[("ASPECTUS_COLUMNS_SIZE", "off")], &["--depth", "2"]);
    assert_eq!(c, 0, "{e}");
    assert!(!o.contains("2.9M") && !o.contains("bytes"), "{o}");
}

/// Subfeature 5: weighing costs the `--walk` budget nothing — 30+ names
/// inside the ignored dir under `--walk 3` trip no bound.
#[test]
fn weighing_is_off_the_walk_budget() {
    let (dir, xdg) = ignored_fixture("walk");
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "2", "--walk", "3"]);
    assert_eq!(c, 0, "{e}");
    assert!(!o.contains("[walk bound]"), "{o}");
    assert!(line_of(&o, "logs/").contains("2.9M"), "{o}");
}

/// Subfeature 6: JSON carries `ignored_body` on the ⊘ node; the parent's
/// figures never include it.
#[test]
fn json_ignored_body() {
    let (dir, xdg) = ignored_fixture("json");
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "2", "--format", "json"]);
    assert_eq!(c, 0, "{e}");
    // 33 entries are files (the hardlink is its own entry); its bytes
    // weigh once: 3,000,000 + 50,000.
    assert!(
        o.contains("\"ignored_body\":{\"files\":33,\"bytes\":3050000}"),
        "{o}"
    );
}

/// Subfeature 7: hidden furniture carries bytes on its has-word when
/// they could answer a disk question (≥ 1 MiB); a small body keeps its
/// file count only.
#[test]
fn hidden_furniture_has_word_bytes() {
    let (dir, xdg) = fresh("has");
    fs::write(dir.join("Cargo.toml"), "[package]\nname = \"x\"\n").unwrap();
    fs::create_dir_all(dir.join("target/debug")).unwrap();
    fs::write(dir.join("target/debug/big"), vec![0u8; 2 * 1024 * 1024]).unwrap();
    fs::write(dir.join("target/debug/small"), "x").unwrap();
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("build \u{2248}2f \u{2248}2.0MB"), "{o}");
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1", "--format", "json"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("\"kind\":\"build\",\"files\":2,\"bytes\":2097153"), "{o}");

    fs::remove_file(dir.join("target/debug/big")).unwrap();
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("build \u{2248}1f,") || o.contains("build \u{2248}1f]"), "{o}");
    assert!(!o.contains("1B"), "a tiny body's bytes stay quiet: {o}");
}

/// Help teaches it.
#[test]
fn help_teaches_ignored_bytes() {
    let out = Command::new(env!("CARGO_BIN_EXE_aspectus"))
        .arg("--help")
        .output()
        .unwrap();
    let h = String::from_utf8_lossy(&out.stdout);
    assert!(h.contains("how big"), "{h}");
}
