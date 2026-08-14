//! Denied: an unreadable dir says so — never rendered as empty or plain.
//! Real binary. Isolated XDG. Skips (with a note) when running as root,
//! since root reads everything.

use std::fs::{self, File};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static SEQ: AtomicU64 = AtomicU64::new(0);

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_aspectus"))
}

fn running_as_root() -> bool {
    let out = Command::new("id").arg("-u").output().unwrap();
    String::from_utf8_lossy(&out.stdout).trim() == "0"
}

/// Root holds `open/` (one file), `secrets/` (one file, then chmod 000),
/// and `top.txt`.
fn fixture() -> (PathBuf, PathBuf) {
    let n = SEQ.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!(
        "aspectus-dn-{}-{}-{}",
        std::process::id(),
        n,
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(dir.join("open")).unwrap();
    File::create(dir.join("open/a.txt")).unwrap();
    fs::create_dir_all(dir.join("secrets")).unwrap();
    File::create(dir.join("secrets/hidden.txt")).unwrap();
    fs::set_permissions(dir.join("secrets"), fs::Permissions::from_mode(0o000)).unwrap();
    File::create(dir.join("top.txt")).unwrap();
    let xdg = std::env::temp_dir().join(format!(
        "aspectus-dn-xdg-{}-{}",
        std::process::id(),
        n
    ));
    fs::create_dir_all(xdg.join("aspectus")).unwrap();
    (dir, xdg)
}

fn unlock(dir: &Path) {
    let _ = fs::set_permissions(dir.join("secrets"), fs::Permissions::from_mode(0o755));
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
fn denied_dir_says_so_when_expanded() {
    if running_as_root() {
        eprintln!("skipped: running as root");
        return;
    }
    let (dir, xdg) = fixture();
    let (c, o, e) = run(&dir, &xdg, &["--depth", "2"]);
    unlock(&dir);
    assert_eq!(c, 0, "one denied child does not fail the look: {e}");
    // Non-name material sits at the look's tab stop (design/columns.md).
    let line = o.lines().find(|l| l.contains("secrets/")).expect(&o);
    assert!(line.contains("[denied]"), "{o}");
    assert!(!o.contains("hidden.txt"), "{o}");
    // The readable neighbors still render normally.
    assert!(o.contains("a.txt"), "{o}");
}

#[test]
fn denied_dir_at_depth_cutoff_is_not_an_empty_census() {
    if running_as_root() {
        eprintln!("skipped: running as root");
        return;
    }
    let (dir, xdg) = fixture();
    let (c, o, e) = run(&dir, &xdg, &["--depth", "1"]);
    unlock(&dir);
    assert_eq!(c, 0, "{e}");
    let line = o.lines().find(|l| l.contains("secrets/")).expect(&o);
    assert!(line.contains("[denied]"), "{o}");
    // It must not print a census claiming knowledge it does not have.
    assert_eq!(line.matches('[').count(), 1, "census on a denied dir: {o}");
    // open/ still censuses.
    // Census, reworked form: a census that would conceal exactly one
    // cheap name shows the name (design/dir-census.md).
    assert!(o.contains("[a.txt]"), "{o}");
}

#[test]
fn denied_root_says_so_not_empty() {
    if running_as_root() {
        eprintln!("skipped: running as root");
        return;
    }
    let (dir, xdg) = fixture();
    let secrets = dir.join("secrets");
    let (c, o, e) = run(&dir, &xdg, &[secrets.to_str().unwrap()]);
    unlock(&dir);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("[denied]"), "root itself denied must say so: {o}");
}

#[test]
fn help_names_denied() {
    let (dir, xdg) = fixture();
    let (c, o, _e) = run(&dir, &xdg, &["help"]);
    unlock(&dir);
    assert_eq!(c, 0);
    assert!(o.contains("[denied]"), "help teaches the confession: {o}");
}
