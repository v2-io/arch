//! Two-level look: real `aspectus` binary on a fixture.

use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static SEQ: AtomicU64 = AtomicU64::new(0);

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_aspectus"))
}

fn fixture() -> PathBuf {
    let n = SEQ.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!(
        "aspectus-2l-{}-{}-{}",
        std::process::id(),
        n,
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(dir.join("a")).unwrap();
    File::create(dir.join("a").join("inside.txt"))
        .unwrap()
        .write_all(b"secret-grandchild")
        .unwrap();
    File::create(dir.join(".hidden"))
        .unwrap()
        .write_all(b"h")
        .unwrap();
    File::create(dir.join("f")).unwrap().write_all(b"f").unwrap();
    fs::create_dir_all(dir.join(".git")).unwrap();
    dir
}

fn run_in(dir: &Path, args: &[&str]) -> (i32, String, String) {
    let out = bin()
        .args(args)
        .current_dir(dir)
        .output()
        .unwrap_or_else(|e| panic!("launch: {e}"));
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

#[test]
fn default_cwd_two_levels() {
    let dir = fixture();
    let (c, o, e) = run_in(&dir, &[]);
    assert_eq!(c, 0, "{e}");
    assert!(e.is_empty(), "success is quiet: {e:?}");
    let base = dir.file_name().unwrap().to_string_lossy();
    assert!(o.contains(&format!("{base}/")), "root name: {o}");
    assert!(o.contains("a/"), "{o}");
    assert!(o.contains(".hidden"), "{o}");
    assert!(o.contains('f'), "{o}");
    assert!(o.contains(".git/"), "hidden dir listed: {o}");
    assert!(!o.contains("inside.txt"), "no grandchild: {o}");
    assert!(!o.contains("secret-grandchild"), "{o}");
    assert!(!o.lines().any(|l| l.contains("./") && l.contains("──")), "{o}");
}

#[test]
fn twice_identical() {
    let dir = fixture();
    let (_, a, _) = run_in(&dir, &[]);
    let (_, b, _) = run_in(&dir, &[]);
    assert_eq!(a, b);
}

#[test]
fn named_path_from_parent() {
    let dir = fixture();
    let parent = dir.parent().unwrap();
    let name = dir.file_name().unwrap().to_string_lossy().into_owned();
    let (c, o, e) = run_in(parent, &[&name]);
    assert_eq!(c, 0, "{e}");
    assert!(e.is_empty(), "{e:?}");
    assert!(o.contains(&format!("{name}/")), "{o}");
    assert!(o.contains("a/"), "{o}");
    assert!(!o.contains("inside.txt"), "{o}");
}

#[test]
fn dirs_marked_files_not() {
    let dir = fixture();
    let (_, o, _) = run_in(&dir, &[]);
    assert!(o.contains("a/"), "{o}");
    // file `f` must appear without a trailing slash on its own line
    let file_line = o.lines().find(|l| l.contains('f') && !l.contains("fixture"));
    assert!(file_line.is_some(), "{o}");
    let line = file_line.unwrap();
    assert!(!line.trim().ends_with("f/"), "{line}");
}

#[test]
fn missing_path_not_found() {
    let (c, o, e) = run_in(Path::new("/"), &["/no/such/aspectus-locus"]);
    assert_eq!(c, 2);
    assert!(o.is_empty(), "{o:?}");
    assert!(e.contains("not found"), "{e}");
    assert!(!e.contains("aspectus help"), "not-found is not a help menu: {e}");
}

#[test]
fn help_examples_include_glance() {
    let (_, o, _) = run_in(Path::new("/"), &["help"]);
    assert!(
        o.lines().any(|l| l.trim() == "aspectus"),
        "example `aspectus`: {o}"
    );
    assert!(
        o.contains("aspectus PATH"),
        "example `aspectus PATH`: {o}"
    );
}
