//! Heat (design/heat.md): git-heat's commit-decay model on the visible
//! set, default ON in-repo, absent outside git. Real binary, isolated XDG,
//! repo fixtures built with the real git.

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
        "aspectus-ht-{tag}-{}-{}-{}",
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

static TICK: AtomicU64 = AtomicU64::new(1_600_000_000);

/// Each commit gets its own second — same-second commits would tie the
/// last-touch register the recency test exercises.
fn git(dir: &Path, args: &[&str]) {
    let t = TICK.fetch_add(60, Ordering::SeqCst);
    let stamp = format!("{t} +0000");
    let ok = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .env("GIT_AUTHOR_DATE", &stamp)
        .env("GIT_COMMITTER_DATE", &stamp)
        .env("GIT_AUTHOR_NAME", "t")
        .env("GIT_AUTHOR_EMAIL", "t@t")
        .env("GIT_COMMITTER_NAME", "t")
        .env("GIT_COMMITTER_EMAIL", "t@t")
        .output()
        .unwrap()
        .status
        .success();
    assert!(ok, "git {args:?} failed in {dir:?}");
}

fn run(dir: &Path, xdg: &Path, args: &[&str]) -> (i32, String, String) {
    let out = bin()
        .args(args)
        .current_dir(dir)
        .env("XDG_CONFIG_HOME", xdg)
        .env_remove("ASPECTUS_SORT")
        .env_remove("ASPECTUS_COLUMNS_HEAT")
        .output()
        .unwrap();
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

/// A repo with history: hot.md touched in the last two commits, cold.md
/// only in the initial commit (excluded from heat by the model).
fn repo_fixture() -> (PathBuf, PathBuf) {
    let (dir, xdg) = fresh("repo");
    git(&dir, &["init", "-q"]);
    fs::write(dir.join("cold.md"), "cold\n").unwrap();
    fs::write(dir.join("hot.md"), "v1\n").unwrap();
    git(&dir, &["add", "."]);
    git(&dir, &["commit", "-qm", "initial"]);
    fs::write(dir.join("hot.md"), "v2\n").unwrap();
    git(&dir, &["add", "."]);
    git(&dir, &["commit", "-qm", "touch hot"]);
    fs::write(dir.join("hot.md"), "v3\n").unwrap();
    git(&dir, &["add", "."]);
    git(&dir, &["commit", "-qm", "touch hot again"]);
    (dir, xdg)
}

fn line_of<'a>(o: &'a str, name: &str) -> &'a str {
    o.lines().find(|l| l.contains(name)).unwrap_or_else(|| panic!("{name} not in {o}"))
}

#[test]
fn heat_cluster_in_repo() {
    let (dir, xdg) = repo_fixture();
    let (c, o, e) = run(&dir, &xdg, &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    let hot = line_of(&o, "hot.md");
    // score · age as one cluster; touched-every-commit at half-life 7
    // scores near the model's ceiling shape.
    assert!(hot.contains("· 0"), "score paired with an age: {hot:?}");
    assert!(hot.contains("ago"), "human-relative age: {hot:?}");
    let cold = line_of(&o, "cold.md");
    assert!(!cold.contains("ago"), "initial-commit-only file claims no heat: {cold:?}");
}

#[test]
fn no_heat_outside_git() {
    let (dir, xdg) = fresh("plain");
    fs::write(dir.join("a.md"), "1\n").unwrap();
    let (c, o, e) = run(&dir, &xdg, &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    assert!(!o.contains("ago"), "no aliveness cluster outside git: {o}");
}

#[test]
fn noise_basenames_claim_nothing() {
    let (dir, xdg) = fresh("noise");
    git(&dir, &["init", "-q"]);
    fs::write(dir.join("Cargo.toml"), "[package]\n").unwrap();
    fs::write(dir.join("real.rs"), "fn a() {}\n").unwrap();
    git(&dir, &["add", "."]);
    git(&dir, &["commit", "-qm", "initial"]);
    for i in 0..3 {
        fs::write(dir.join("Cargo.toml"), format!("[package] # {i}\n")).unwrap();
        fs::write(dir.join("real.rs"), format!("fn a() {{}} // {i}\n")).unwrap();
        git(&dir, &["add", "."]);
        git(&dir, &["commit", "-qm", "churn"]);
    }
    let (c, o, e) = run(&dir, &xdg, &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    assert!(!line_of(&o, "Cargo.toml").contains("ago"), "noise basename, heat 0 → silent: {o}");
    assert!(line_of(&o, "real.rs").contains("ago"), "{o}");
}

#[test]
fn dir_heat_is_max_of_leaves() {
    let (dir, xdg) = fresh("dirmax");
    git(&dir, &["init", "-q"]);
    fs::create_dir_all(dir.join("sub")).unwrap();
    fs::write(dir.join("sub/deep.md"), "v1\n").unwrap();
    git(&dir, &["add", "."]);
    git(&dir, &["commit", "-qm", "initial"]);
    fs::write(dir.join("sub/deep.md"), "v2\n").unwrap();
    git(&dir, &["add", "."]);
    git(&dir, &["commit", "-qm", "touch"]);
    let (c, o, e) = run(&dir, &xdg, &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    assert!(line_of(&o, "sub/").contains("ago"), "dir carries leaf heat: {o}");
}

#[test]
fn sort_by_heat_is_built() {
    let (dir, xdg) = repo_fixture();
    let (c, _, e) = run(&dir, &xdg, &["--sort", "heat", "--depth", "1"]);
    assert_eq!(c, 0, "heat is a built sort key now: {e}");
}

#[test]
fn config_can_turn_heat_off() {
    let (dir, xdg) = repo_fixture();
    fs::write(xdg.join("aspectus/aspectus.toml"), "columns.heat = \"off\"\n").unwrap();
    let (c, o, e) = run(&dir, &xdg, &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    assert!(!o.contains("ago"), "off removes the cluster: {o}");
}

#[test]
fn git_recency_source_orders_by_last_touch() {
    let (dir, xdg) = repo_fixture();
    // mtime says cold.md is old; touch it newer than hot.md on disk, then
    // ask recency to follow git — hot.md stays on top.
    let future = std::time::SystemTime::now();
    let _ = future; // mtimes: rewrite cold.md so its mtime is newest
    fs::write(dir.join("cold.md"), "cold but freshly written\n").unwrap();
    fs::write(xdg.join("aspectus/aspectus.toml"), "recency-source = \"git\"\n").unwrap();
    let (c, o, e) = run(&dir, &xdg, &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    let hot_at = o.find("hot.md").expect(&o);
    let cold_at = o.find("cold.md").expect(&o);
    assert!(hot_at < cold_at, "git last-touch outranks a fresh mtime: {o}");
}
