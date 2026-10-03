//! Empty directory (design/empty-dir.md): a dir whose readdir yielded no
//! names says `[empty]` on its own line — expanded, at a cutoff, as the
//! root, through a symlink — and nothing that merely *looks* empty to the
//! look (ignored, hide-only, ignored-files-only, omit-only) claims it.
//! Real binary, isolated XDG and HOME.

use std::fs::{self, File};
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
        "aspectus-empty-{tag}-{}-{}-{}",
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

fn git(dir: &Path, args: &[&str]) {
    assert!(
        Command::new("git")
            .args(args)
            .current_dir(dir)
            .output()
            .unwrap()
            .status
            .success(),
        "git {args:?}"
    );
}

fn line_of<'a>(o: &'a str, name: &str) -> &'a str {
    o.lines()
        .find(|l| l.contains(name))
        .unwrap_or_else(|| panic!("no line for {name}: {o}"))
}

/// Subfeatures 1 and 5: an expanded level; a symlink to an empty dir.
#[test]
fn empty_child_says_so() {
    let (dir, xdg) = fresh("child");
    fs::create_dir_all(dir.join("hollow")).unwrap();
    fs::create_dir_all(dir.join("full")).unwrap();
    File::create(dir.join("full/a.txt")).unwrap();
    std::os::unix::fs::symlink("hollow", dir.join("via")).unwrap();
    let (c, o, e) = run(&dir, &xdg, &["--depth", "2"]);
    assert_eq!(c, 0, "{e}");
    assert!(line_of(&o, "hollow/").ends_with("[empty]"), "{o}");
    assert!(!line_of(&o, "full/").contains("[empty]"), "{o}");
    // 2026-10-03 name-stop slice: beside names this short the target
    // spills to a `╰` sub-row under `via`; the mark stays on the node row.
    let via = line_of(&o, "── via");
    assert!(via.ends_with("[empty]"), "{o}");
    assert!(o.contains("\u{2570} -> hollow"), "{o}");
}

/// Subfeature 2: at the depth cutoff an empty dir used to print bare (its
/// census is empty, so nothing rendered).
#[test]
fn empty_at_cutoff_is_not_bare() {
    let (dir, xdg) = fresh("cutoff");
    fs::create_dir_all(dir.join("a/b")).unwrap();
    let (c, o, e) = run(&dir, &xdg, &["--depth", "2"]);
    assert_eq!(c, 0, "{e}");
    assert!(line_of(&o, "b/").ends_with("[empty]"), "{o}");
    assert!(!line_of(&o, "a/").contains("[empty]"), "{o}");
}

/// Subfeature 3: the root's facts line carries the mark above the path.
#[test]
fn empty_root_says_so() {
    let (dir, xdg) = fresh("root");
    let (c, o, e) = run(&dir, &xdg, &[]);
    assert_eq!(c, 0, "{e}");
    let lines: Vec<&str> = o.lines().collect();
    let path_at = lines
        .iter()
        .position(|l| l.ends_with('/') && l.starts_with('/'))
        .expect(&o);
    assert_eq!(lines[path_at - 1], "[empty]", "{o}");
}

/// Subfeature 4: nothing that merely looks empty to the look claims it —
/// an ignored dir (never read), a dir holding only hidden furniture, one
/// holding only ignored files, one holding only omitted names.
#[test]
fn looks_empty_is_not_empty() {
    let (dir, xdg) = fresh("notempty");
    git(&dir, &["init", "-q"]);
    fs::write(dir.join(".gitignore"), "skipped/\n*.log\n").unwrap();
    fs::create_dir_all(dir.join("skipped")).unwrap();
    fs::create_dir_all(dir.join("hidden-only/.archive")).unwrap();
    File::create(dir.join("hidden-only/.archive/x")).unwrap();
    fs::create_dir_all(dir.join("logs-only")).unwrap();
    File::create(dir.join("logs-only/a.log")).unwrap();
    fs::create_dir_all(dir.join("ds-only")).unwrap();
    File::create(dir.join("ds-only/.DS_Store")).unwrap();
    fs::create_dir_all(dir.join("truly")).unwrap();
    let (c, o, e) = run(&dir, &xdg, &["--depth", "2"]);
    assert_eq!(c, 0, "{e}");
    for name in ["skipped/", "hidden-only/", "logs-only/", "ds-only/"] {
        assert!(!line_of(&o, name).contains("[empty]"), "{name}: {o}");
    }
    assert!(line_of(&o, "truly/").ends_with("[empty]"), "{o}");
}

/// Subfeature 6: JSON carries `empty: true`; it is not truncation.
#[test]
fn json_empty_field() {
    let (dir, xdg) = fresh("json");
    fs::create_dir_all(dir.join("hollow")).unwrap();
    let (c, o, e) = run(&dir, &xdg, &["--format", "json"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("\"name\":\"hollow\""), "{o}");
    let hollow = &o[o.find("\"name\":\"hollow\"").unwrap()..];
    // `hollow` is the last child: its object ends at the children's `}]`.
    let obj = &hollow[..hollow.find("}]").unwrap()];
    assert!(obj.contains("\"empty\":true"), "{o}");
    assert!(o.contains("\"truncated\":false"), "{o}");
}

/// Subfeature 7: help teaches the mark.
#[test]
fn help_teaches_empty() {
    let out = bin().arg("--help").output().unwrap();
    let h = String::from_utf8_lossy(&out.stdout);
    assert!(h.contains("[empty]"), "{h}");
}
