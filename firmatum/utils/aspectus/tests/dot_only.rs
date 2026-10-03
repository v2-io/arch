//! Dot-only directory (design/dot-only.md): a dir whose every readdir name
//! starts with `.` says `[dot-only]` — distinct from `[empty]`, beside
//! whatever its line already says (`[has: …]`, a census). Real binary,
//! isolated XDG and HOME.

use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static SEQ: AtomicU64 = AtomicU64::new(0);

fn fresh(tag: &str) -> (PathBuf, PathBuf) {
    let n = SEQ.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!(
        "aspectus-dot-{tag}-{}-{}-{}",
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
    let out = Command::new(env!("CARGO_BIN_EXE_aspectus"))
        .args(args)
        .current_dir(dir)
        .env("XDG_CONFIG_HOME", xdg)
        .env("HOME", xdg)
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

fn line_of<'a>(o: &'a str, name: &str) -> &'a str {
    o.lines()
        .find(|l| l.contains(name))
        .unwrap_or_else(|| panic!("no line for {name}: {o}"))
}

/// The rule, every child kind: an omitted file (.DS_Store), a listed
/// dotfile, a hidden furniture dir (with its has-spot kept beside the
/// mark — Joseph: "No need to hide what we already know might be
/// relevant"), a dot symlink. Mixed and empty dirs do not claim it.
#[test]
fn every_name_dotted_says_so() {
    let (dir, xdg) = fresh("rule");
    fs::create_dir_all(dir.join("ds")).unwrap();
    File::create(dir.join("ds/.DS_Store")).unwrap();
    fs::create_dir_all(dir.join("keep")).unwrap();
    File::create(dir.join("keep/.gitkeep")).unwrap();
    fs::create_dir_all(dir.join("arch/.archive")).unwrap();
    File::create(dir.join("arch/.archive/old.md")).unwrap();
    fs::create_dir_all(dir.join("lnk")).unwrap();
    std::os::unix::fs::symlink("../keep", dir.join("lnk/.back")).unwrap();
    fs::create_dir_all(dir.join("mixed")).unwrap();
    File::create(dir.join("mixed/.env")).unwrap();
    File::create(dir.join("mixed/a.md")).unwrap();
    fs::create_dir_all(dir.join("hollow")).unwrap();
    let (c, o, e) = run(&dir, &xdg, &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    for name in ["ds/", "keep/", "lnk/"] {
        assert!(line_of(&o, name).ends_with("[dot-only]"), "{name}: {o}");
    }
    let arch = line_of(&o, "arch/");
    assert!(arch.contains("[has: archive") && arch.ends_with("[dot-only]"), "{o}");
    assert!(!line_of(&o, "mixed/").contains("[dot-only]"), "{o}");
    let hollow = line_of(&o, "hollow/");
    assert!(hollow.ends_with("[empty]") && !hollow.contains("[dot-only]"), "{o}");
}

/// Expanded levels and the root carry it too; JSON says `dot_only`.
#[test]
fn root_expanded_and_json() {
    let (dir, xdg) = fresh("root");
    File::create(dir.join(".gitkeep")).unwrap();
    let (c, o, e) = run(&dir, &xdg, &[]);
    assert_eq!(c, 0, "{e}");
    assert!(o.lines().any(|l| l == "[dot-only]"), "root facts line: {o}");
    let (c, o, e) = run(&dir, &xdg, &["--format", "json"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("\"dot_only\":true"), "{o}");
    assert!(o.contains("\"truncated\":false"), "{o}");
}

/// Help teaches it.
#[test]
fn help_teaches_dot_only() {
    let out = Command::new(env!("CARGO_BIN_EXE_aspectus"))
        .arg("--help")
        .output()
        .unwrap();
    assert!(String::from_utf8_lossy(&out.stdout).contains("[dot-only]"));
}
