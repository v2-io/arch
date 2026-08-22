//! Ignored bodies (design/gitignore-bodies.md): ignored contents out of
//! the look and out of mass; presence still shows; agrees with
//! `git check-ignore`; tracked beats ignored. Real binary, real repos.

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
        "aspectus-ig-{tag}-{}-{}-{}",
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
        // A caller's global git excludes must not leak into the fixture.
        .env("HOME", xdg)
        // Serial fixture names; globify is its own row, pinned off here.
        .env("ASPECTUS_GLOBIFY", "off")
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
            .env("GIT_AUTHOR_NAME", "t")
            .env("GIT_AUTHOR_EMAIL", "t@t")
            .env("GIT_COMMITTER_NAME", "t")
            .env("GIT_COMMITTER_EMAIL", "t@t")
            .output()
            .unwrap()
            .status
            .success(),
        "git {args:?}"
    );
}

fn repo(tag: &str) -> (PathBuf, PathBuf) {
    let (dir, xdg) = fresh(tag);
    git(&dir, &["init", "-q"]);
    (dir, xdg)
}

/// Subfeature 1: an ignored dir's line shows (with the glyph); its
/// children do not print; no census of its innards.
#[test]
fn ignored_dir_presence_without_innards() {
    let (dir, xdg) = repo("dirpres");
    fs::write(dir.join(".gitignore"), "logs/\n").unwrap();
    fs::create_dir_all(dir.join("logs")).unwrap();
    for i in 0..30 {
        File::create(dir.join(format!("logs/f{i}.log"))).unwrap();
    }
    fs::write(dir.join("a.md"), "x\n").unwrap();
    let (c, o, e) = run(&dir, &xdg, &["--depth", "2"]);
    assert_eq!(c, 0, "{e}");
    let line = o
        .lines()
        .find(|l| l.contains("logs/"))
        .expect("presence shows");
    assert!(line.contains("⊘"), "the ignored glyph: {line}");
    assert!(!line.contains("log×"), "no census of the innards: {line}");
    assert!(!o.contains("f0.log"), "children do not print: {o}");
}

/// Subfeature 2: ignored files appear only in the typed remainder.
#[test]
fn ignored_files_are_a_typed_remainder() {
    let (dir, xdg) = repo("filerem");
    fs::write(dir.join(".gitignore"), "*.log\n").unwrap();
    for i in 0..3 {
        File::create(dir.join(format!("x{i}.log"))).unwrap();
    }
    fs::write(dir.join("a.md"), "x\n").unwrap();
    let (c, o, e) = run(&dir, &xdg, &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    assert!(!o.contains("x0.log"), "not listed: {o}");
    assert!(o.contains("ignored×3"), "exact count in the remainder: {o}");
}

/// Subfeature 3: mass over a subtree excludes ignored bodies — identical
/// to the same tree with the ignored parts deleted.
#[test]
fn mass_excludes_ignored_bodies() {
    let compare = |with_ignored: bool| -> String {
        let (dir, xdg) = repo(if with_ignored { "massA" } else { "massB" });
        fs::write(dir.join(".gitignore"), "junk/\n*.tmp\n").unwrap();
        fs::create_dir_all(dir.join("sub/deep")).unwrap();
        fs::write(dir.join("sub/deep/a.md"), "1\n2\n3\n").unwrap();
        fs::write(dir.join("sub/b.md"), "1\n").unwrap();
        if with_ignored {
            fs::create_dir_all(dir.join("sub/junk")).unwrap();
            for i in 0..20 {
                fs::write(dir.join(format!("sub/junk/f{i}.md")), "zzz\n".repeat(50)).unwrap();
            }
            fs::write(dir.join("sub/c.tmp"), "ignored\n".repeat(10)).unwrap();
        }
        let (c, o, e) = run(&dir, &xdg, &["--depth", "1"]);
        assert_eq!(c, 0, "{e}");
        o.lines()
            .find(|l| l.contains("sub/"))
            .expect("sub listed")
            .split_once("sub/")
            .unwrap()
            .1
            .trim()
            .to_string()
    };
    let a = compare(true);
    let b = compare(false);
    // The *aggregates* are identical to the tree with the ignored parts
    // deleted; the census still tells presence (dir count, ignored×1) —
    // that is the design's other half, not a leak into the numbers.
    for look in [&a, &b] {
        // 2026-08-22 count-cell slice: 4 is exact/ungrouped, so `4.` in the
        // lines column; the word "lines" is the heading.
        assert!(look.contains("4."), "un-ignored lines only: {look}");
        assert!(!look.contains("≈4 lines"), "old mass tail retired: {look}");
        assert!(look.contains("≈1f"), "junk/'s 20 files stay out: {look}");
    }
    assert!(a.contains("ignored×1"), "the cut is typed: {a}");
}

/// Subfeature 4: tracked beats ignored — `git add -f` keeps a file listed.
#[test]
fn tracked_beats_ignored() {
    let (dir, xdg) = repo("tracked");
    fs::write(dir.join(".gitignore"), "*.bak\n").unwrap();
    fs::write(dir.join("keep.bak"), "kept\n").unwrap();
    fs::write(dir.join("drop.bak"), "dropped\n").unwrap();
    git(&dir, &["add", "-f", "keep.bak"]);
    let (c, o, e) = run(&dir, &xdg, &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("keep.bak"), "tracked lists normally: {o}");
    assert!(!o.contains("drop.bak"), "untracked-ignored does not: {o}");
    assert!(o.contains("ignored×1"), "and is told: {o}");
}

/// Subfeature 5: nested `.gitignore` + negation behave as
/// `git check-ignore` says — cross-checked against the real git.
#[test]
fn nested_and_negation_agree_with_git() {
    let (dir, xdg) = repo("nested");
    fs::write(dir.join(".gitignore"), "*.log\n").unwrap();
    fs::create_dir_all(dir.join("sub")).unwrap();
    fs::write(dir.join("sub/.gitignore"), "!keep.log\ndata/\n").unwrap();
    fs::write(dir.join("sub/keep.log"), "kept\n").unwrap();
    fs::write(dir.join("sub/drop.log"), "dropped\n").unwrap();
    fs::create_dir_all(dir.join("sub/data")).unwrap();
    fs::write(dir.join("sub/data/x.md"), "x\n").unwrap();
    let (c, o, e) = run(&dir, &xdg, &["--depth", "2"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("keep.log"), "deeper negation re-includes: {o}");
    assert!(!o.contains("drop.log"), "outer pattern holds: {o}");
    let data = o.lines().find(|l| l.contains("data/")).expect("presence");
    assert!(data.contains("⊘"), "nested dir-only pattern: {data}");
    // The contract, checked against git itself.
    for (path, ignored) in [
        ("sub/keep.log", false),
        ("sub/drop.log", true),
        ("sub/data", true),
    ] {
        let st = Command::new("git")
            .args(["check-ignore", "-q", path])
            .current_dir(&dir)
            .status()
            .unwrap();
        assert_eq!(st.success(), ignored, "git agrees about {path}");
    }
}

/// Subfeature 6: a nested repo's ignores come from its own files; the
/// outer repo's patterns do not reach inside.
#[test]
fn nested_repo_uses_its_own_rules() {
    let (dir, xdg) = repo("subrepo");
    fs::write(dir.join(".gitignore"), "*.md\n").unwrap();
    let inner = dir.join("inner");
    fs::create_dir_all(&inner).unwrap();
    git(&inner, &["init", "-q"]);
    fs::write(inner.join(".gitignore"), "*.log\n").unwrap();
    fs::write(inner.join("doc.md"), "outer pattern must not reach\n").unwrap();
    fs::write(inner.join("x.log"), "inner pattern must\n").unwrap();
    let (c, o, e) = run(&dir, &xdg, &["--depth", "2"]);
    assert_eq!(c, 0, "{e}");
    assert!(
        o.contains("doc.md"),
        "outer *.md does not cross the repo: {o}"
    );
    assert!(!o.contains("x.log"), "inner rule applies: {o}");
}

/// Subfeature 7: outside a work tree a `.gitignore` changes nothing.
#[test]
fn non_repo_gitignore_is_just_a_file() {
    let (dir, xdg) = fresh("plain");
    fs::write(dir.join(".gitignore"), "*.log\n").unwrap();
    fs::write(dir.join("x.log"), "listed\n").unwrap();
    let (c, o, e) = run(&dir, &xdg, &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("x.log"), "no repo, no ignoring: {o}");
    assert!(!o.contains("⊘") && !o.contains("ignored×"), "{o}");
}

/// Subfeature 8: `--show-all` restores ignored contents; the marks remain.
#[test]
fn show_all_restores_with_marks() {
    let (dir, xdg) = repo("showall");
    fs::write(dir.join(".gitignore"), "logs/\n*.tmp\n").unwrap();
    fs::create_dir_all(dir.join("logs")).unwrap();
    fs::write(dir.join("logs/a.log"), "x\n").unwrap();
    fs::write(dir.join("b.tmp"), "y\n").unwrap();
    let (c, o, e) = run(&dir, &xdg, &["--depth", "2", "--show-all"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("a.log"), "contents list: {o}");
    assert!(o.contains("b.tmp"), "files list: {o}");
    let logs = o.lines().find(|l| l.contains("logs/")).unwrap();
    assert!(logs.contains("⊘"), "mark remains: {logs}");
    let tmp = o.lines().find(|l| l.contains("b.tmp")).unwrap();
    assert!(tmp.contains("⊘"), "file mark too: {tmp}");
}

/// Subfeature 9: the walk does not recurse into ignored dirs — a huge
/// ignored dir does not drain the walk budget.
#[test]
fn ignored_dirs_cost_no_walk_budget() {
    let (dir, xdg) = repo("walkcost");
    fs::write(dir.join(".gitignore"), "huge/\n").unwrap();
    fs::create_dir_all(dir.join("huge")).unwrap();
    for i in 0..200 {
        File::create(dir.join(format!("huge/f{i}.dat"))).unwrap();
    }
    for i in 0..8 {
        fs::write(dir.join(format!("real{i}.md")), "x\n").unwrap();
    }
    // Budget 50 names: the 200 inside huge/ would trip it; skipping means
    // every real file still lists and nothing says [walk bound].
    let (c, o, e) = run(
        &dir,
        &xdg,
        &["--depth", "2", "--walk", "50", "--lines", "0"],
    );
    assert_eq!(c, 0, "{e}");
    for i in 0..8 {
        assert!(o.contains(&format!("real{i}.md")), "real{i}.md: {o}");
    }
    assert!(!o.contains("[walk bound]"), "budget not drained: {o}");
}

/// Subfeature 10: JSON carries the same claims as fields.
#[test]
fn json_fields() {
    let (dir, xdg) = repo("json");
    fs::write(dir.join(".gitignore"), "logs/\n*.tmp\n").unwrap();
    fs::create_dir_all(dir.join("logs")).unwrap();
    fs::write(dir.join("a.tmp"), "x\n").unwrap();
    fs::write(dir.join("b.md"), "x\n").unwrap();
    let (c, o, e) = run(&dir, &xdg, &["--depth", "1", "--format", "json"]);
    assert_eq!(c, 0, "{e}");
    assert!(
        o.contains("\"gitignored\":true"),
        "dir status is a field: {o}"
    );
    assert!(o.contains("\"ignored_files\":1"), "remainder is data: {o}");
    assert!(!o.contains("a.tmp"), "ignored file carries no node: {o}");
}

/// The ignored count also reaches a depth-cutoff census.
#[test]
fn cutoff_census_counts_ignored() {
    let (dir, xdg) = repo("cutoff");
    fs::write(dir.join(".gitignore"), "*.tmp\n").unwrap();
    fs::create_dir_all(dir.join("sub")).unwrap();
    fs::write(dir.join("sub/a.md"), "x\n").unwrap();
    fs::write(dir.join("sub/b.tmp"), "x\n").unwrap();
    fs::write(dir.join("sub/c.tmp"), "x\n").unwrap();
    let (c, o, e) = run(&dir, &xdg, &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    let sub = o.lines().find(|l| l.contains("sub/")).unwrap();
    assert!(sub.contains("ignored×2"), "the census tells the cut: {sub}");
    assert!(!sub.contains("tmp×"), "no bucket for ignored bodies: {sub}");
}
