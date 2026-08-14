//! Furniture: well-known names are parent-line state, not children.
//! Real binary. Isolated XDG so the user's own map cannot leak in.

use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static SEQ: AtomicU64 = AtomicU64::new(0);

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_aspectus"))
}

fn tmpdir(tag: &str) -> PathBuf {
    let n = SEQ.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!(
        "aspectus-furn-{tag}-{}-{}-{}",
        std::process::id(),
        n,
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn touch(p: &Path) {
    File::create(p).unwrap().write_all(b"x").unwrap();
}

/// A dir with build debris, agent machinery, noise, and an unknown hidden name.
fn fixture() -> (PathBuf, PathBuf) {
    let dir = tmpdir("fix");
    fs::create_dir_all(dir.join("target/debug")).unwrap();
    touch(&dir.join("target/debug/junk.o"));
    fs::create_dir_all(dir.join("__pycache__")).unwrap();
    touch(&dir.join("__pycache__/m.pyc"));
    fs::create_dir_all(dir.join(".claude")).unwrap();
    touch(&dir.join(".claude/settings.json"));
    touch(&dir.join(".DS_Store"));
    fs::create_dir_all(dir.join(".mystery")).unwrap();
    touch(&dir.join("src.rs"));
    let xdg = tmpdir("xdg");
    fs::create_dir_all(xdg.join("aspectus")).unwrap();
    (dir, xdg)
}

fn run(dir: &Path, xdg: &Path, args: &[&str]) -> (i32, String, String) {
    let out = bin()
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

#[test]
fn well_known_names_fold_to_parent_state() {
    let (dir, xdg) = fixture();
    let (c, o, e) = run(&dir, &xdg, &[]);
    assert_eq!(c, 0, "{e}");
    assert!(!o.contains("target/"), "build debris listed: {o}");
    assert!(!o.contains("__pycache__"), "{o}");
    assert!(!o.contains(".claude"), "{o}");
    assert!(o.contains("[kind: agents, build, python]"), "kind spot: {o}");
    assert!(o.contains(".mystery/"), "unknown hidden name stays a child: {o}");
    assert!(o.contains("src.rs"), "{o}");
}

#[test]
fn omit_is_not_mentioned_at_all() {
    let (dir, xdg) = fixture();
    let (_, o, _) = run(&dir, &xdg, &[]);
    assert!(!o.contains(".DS_Store"), "{o}");
    assert!(!o.contains("other"), ".DS_Store must not leak into a census: {o}");
}

#[test]
fn hidden_names_do_not_join_censuses() {
    let (dir, xdg) = fixture();
    // Depth 1: .mystery/ is unexpanded but target/ etc. must not be counted
    // anywhere — 2 dirs would be a silent lie; the kind spot carries them.
    let (_, o, _) = run(&dir, &xdg, &["--depth", "1"]);
    let root = o.lines().next().unwrap();
    assert!(!root.contains("3 dir"), "hidden dirs joined the picture: {o}");
    assert!(o.contains("[kind: agents, build, python]"), "{o}");
}

#[test]
fn show_all_lists_furniture_as_ordinary_children() {
    let (dir, xdg) = fixture();
    let (c, o, e) = run(&dir, &xdg, &["--show-all"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("target/"), "{o}");
    assert!(o.contains("debug/"), "walk enters shown furniture: {o}");
    assert!(o.contains("__pycache__/"), "{o}");
    assert!(o.contains(".claude/"), "{o}");
    assert!(o.contains(".DS_Store"), "{o}");
    // The claims stay true even when the names are shown.
    assert!(o.contains("[kind:"), "{o}");
}

#[test]
fn inspect_opens_one_kind_only() {
    let (dir, xdg) = fixture();
    let (c, o, e) = run(&dir, &xdg, &["--inspect", "build"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("target/"), "{o}");
    assert!(o.contains("__pycache__/"), "{o}");
    assert!(!o.contains(".claude"), "agents kind not asked for: {o}");
    assert!(!o.contains(".DS_Store"), "omit has no kind to inspect: {o}");
}

#[test]
fn config_extends_and_removes_map_rows() {
    let (dir, xdg) = fixture();
    fs::write(
        xdg.join("aspectus/aspectus.toml"),
        "furniture = \".mystery/:lab, !target/\"\n",
    )
    .unwrap();
    let (c, o, e) = run(&dir, &xdg, &[]);
    assert_eq!(c, 0, "{e}");
    assert!(!o.contains(".mystery"), "config row hides it: {o}");
    assert!(o.contains("lab"), "config kind claimed: {o}");
    assert!(o.contains("target/"), "!target/ un-furnitures it: {o}");
}

#[test]
fn explain_budget_confesses_furniture_on_stderr() {
    let (dir, xdg) = fixture();
    let (_, o, e) = run(&dir, &xdg, &["--explain-budget"]);
    assert!(e.contains("furniture"), "{e}");
    assert!(!o.contains("furniture:"), "stdout is the look only: {o}");
}

#[test]
fn non_name_material_lands_at_one_tab_stop() {
    // design/columns.md (decided 2026-08-14): computed pseudo-tab-stops,
    // a pure function of the look's content, never terminal width.
    let dir = tmpdir("align");
    fs::create_dir_all(dir.join("short/inner")).unwrap();
    touch(&dir.join("short/inner/a.txt"));
    fs::create_dir_all(dir.join("much-longer-name/inner")).unwrap();
    touch(&dir.join("much-longer-name/inner/b.txt"));
    let xdg = tmpdir("align-xdg");
    fs::create_dir_all(xdg.join("aspectus")).unwrap();
    let (_, o, _) = run(&dir, &xdg, &["--depth", "1"]);
    let cols: Vec<usize> = o
        .lines()
        .filter(|l| l.contains("[1: 1 dir]"))
        .map(|l| l.chars().position(|c| c == '[').unwrap())
        .collect();
    assert_eq!(cols.len(), 2, "{o}");
    assert_eq!(cols[0], cols[1], "censuses not at one stop: {o}");
}

#[test]
fn empty_kind_set_prints_nothing() {
    let dir = tmpdir("plain");
    touch(&dir.join("only.txt"));
    let xdg = tmpdir("plain-xdg");
    fs::create_dir_all(xdg.join("aspectus")).unwrap();
    let (_, o, _) = run(&dir, &xdg, &[]);
    assert!(!o.contains("[kind"), "no claims, no spot: {o}");
}
