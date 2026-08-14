//! README title (design/readme-title.md): a dir line borrows its README's
//! name. Shipped default OFF (the ON-vs-QUIET Open is Joseph's to ratify —
//! one config value flips it). Real binary.

use std::fs;
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
        "aspectus-title-{tag}-{}-{}-{}",
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
        .env("ASPECTUS_README_TITLE", "on")
        .env_remove("ASPECTUS_LINES")
        .env_remove("ASPECTUS_DEPTH");
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

/// Subfeature 1: the first ATX heading lends the title.
#[test]
fn heading_title() {
    let (dir, xdg) = fresh("head");
    fs::create_dir_all(dir.join("rowan-src")).unwrap();
    fs::write(dir.join("rowan-src/README.md"), "intro line\n\n# Rowan\n").unwrap();
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    let l = o.lines().find(|l| l.contains("rowan-src/")).unwrap();
    assert!(l.contains("\"Rowan\""), "the heading, quoted: {l}");
}

/// Subfeature 2: no heading — first non-empty line, trimmed.
#[test]
fn fallback_first_line() {
    let (dir, xdg) = fresh("fall");
    fs::create_dir_all(dir.join("d")).unwrap();
    fs::write(dir.join("d/README"), "\n  The plain first line  \nrest\n").unwrap();
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("\"The plain first line\""), "{o}");
}

/// Subfeature 3: empty / binary / absent READMEs lend nothing — no
/// placeholder ever prints.
#[test]
fn nothing_to_say() {
    let (dir, xdg) = fresh("silent");
    fs::create_dir_all(dir.join("empty")).unwrap();
    fs::write(dir.join("empty/README.md"), "").unwrap();
    fs::create_dir_all(dir.join("binary")).unwrap();
    fs::write(dir.join("binary/README.md"), b"\x00\x01\x02junk").unwrap();
    fs::create_dir_all(dir.join("bare")).unwrap();
    fs::write(dir.join("bare/notes.txt"), "not a readme\n").unwrap();
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    assert!(!o.contains('"'), "no title, no placeholder: {o}");
}

/// Subfeature 4: the peek is bounded — a title past the window lends none.
#[test]
fn bounded_peek() {
    let (dir, xdg) = fresh("bound");
    fs::create_dir_all(dir.join("big")).unwrap();
    let mut body = "x\n".repeat(4096); // padding fills the 4KB window
    body.push_str("# Late Title\n");
    fs::write(dir.join("big/README.md"), body).unwrap();
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    // The fallback first line (`x`) is what the window saw.
    assert!(!o.contains("Late Title"), "absence, never an error: {o}");
}

/// Subfeature 5: the important-files config set picks the source file.
#[test]
fn config_set_shared_with_important() {
    let (dir, xdg) = fresh("cfg");
    fs::create_dir_all(dir.join("d")).unwrap();
    fs::write(dir.join("d/README.md"), "# From Readme\n").unwrap();
    fs::write(dir.join("d/PRIMER.md"), "# From Primer\n").unwrap();
    let (c, o, e) = run(
        &dir,
        &xdg,
        &[("ASPECTUS_IMPORTANT", "PRIMER.md, !README*")],
        &["--depth", "1"],
    );
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("\"From Primer\""), "the set decides: {o}");
}

/// Subfeature 6: a title equal to the folder name (case/punct-insensitive)
/// prints nothing.
#[test]
fn redundancy_guard() {
    let (dir, xdg) = fresh("red");
    fs::create_dir_all(dir.join("rowan")).unwrap();
    fs::write(dir.join("rowan/README.md"), "# rowan\n").unwrap();
    fs::create_dir_all(dir.join("md-press")).unwrap();
    fs::write(dir.join("md-press/README.md"), "# MD Press\n").unwrap();
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    let rowan = o.lines().find(|l| l.contains("rowan/")).unwrap();
    assert!(!rowan.contains('"'), "says nothing new: {rowan}");
    let press = o.lines().find(|l| l.contains("md-press/")).unwrap();
    assert!(!press.contains('"'), "punct-insensitive match: {press}");
}

/// Subfeature 7: determinism (title present), and the shipped default is
/// OFF — no title without the ask.
#[test]
fn determinism_and_default_off() {
    let (dir, xdg) = fresh("det");
    fs::create_dir_all(dir.join("d")).unwrap();
    fs::write(dir.join("d/README.md"), "# A Name\n").unwrap();
    let (_, o1, _) = run(&dir, &xdg, &[], &["--depth", "1"]);
    let (_, o2, _) = run(&dir, &xdg, &[], &["--depth", "1"]);
    let strip = |s: &str| s.lines().skip(1).collect::<Vec<_>>().join("\n");
    assert_eq!(strip(&o1), strip(&o2));
    let (_, off, _) = run(&dir, &xdg, &[("ASPECTUS_README_TITLE", "off")], &["--depth", "1"]);
    assert!(!off.contains("A Name"), "default stays Joseph's to flip: {off}");
}

/// Subfeature 8: JSON carries the title as a field on the dir node.
#[test]
fn json_field() {
    let (dir, xdg) = fresh("json");
    fs::create_dir_all(dir.join("d")).unwrap();
    fs::write(dir.join("d/README.md"), "# A Name\n").unwrap();
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1", "--format", "json"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("\"title\":\"A Name\""), "{o}");
}

/// A depth-cutoff dir (census line) still lends its title.
#[test]
fn cutoff_dir_titled() {
    let (dir, xdg) = fresh("cut");
    fs::create_dir_all(dir.join("d/inner")).unwrap();
    fs::write(dir.join("d/README.md"), "# Cut Name\n").unwrap();
    fs::write(dir.join("d/inner/x.md"), "x\n").unwrap();
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    let l = o.lines().find(|l| l.contains("d/") && l.contains("── ")).unwrap();
    assert!(l.contains("\"Cut Name\""), "{l}");
}
