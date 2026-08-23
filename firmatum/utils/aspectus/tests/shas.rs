//! The sha facts (lattice initial-sha / latest-sha row; design/heat.md —
//! same obtain as heat's log pass): compose-only via config, `H~N` /
//! short / full spellings, omitted outside git — never guessed.

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
        "aspectus-sha-{tag}-{}-{}-{}",
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

fn git(dir: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .args(args)
        .current_dir(dir)
        .env("GIT_AUTHOR_NAME", "t")
        .env("GIT_AUTHOR_EMAIL", "t@t")
        .env("GIT_COMMITTER_NAME", "t")
        .env("GIT_COMMITTER_EMAIL", "t@t")
        .output()
        .unwrap();
    assert!(out.status.success(), "git {args:?}");
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

/// Three commits: a.md born in the first, touched in the third; b.md born
/// in the second, never touched again.
fn repo(tag: &str) -> (PathBuf, PathBuf, String, String) {
    let (dir, xdg) = fresh(tag);
    git(&dir, &["init", "-q"]);
    fs::write(dir.join("a.md"), "1\n").unwrap();
    git(&dir, &["add", "."]);
    git(&dir, &["commit", "-qm", "one"]);
    let first = git(&dir, &["rev-parse", "HEAD"]);
    fs::write(dir.join("b.md"), "1\n").unwrap();
    git(&dir, &["add", "."]);
    git(&dir, &["commit", "-qm", "two"]);
    fs::write(dir.join("a.md"), "2\n").unwrap();
    git(&dir, &["add", "."]);
    git(&dir, &["commit", "-qm", "three"]);
    let head = git(&dir, &["rev-parse", "HEAD"]);
    (dir, xdg, first, head)
}

/// Compose-only: no own flag — the refusal names the config path.
#[test]
fn no_own_flag() {
    let (dir, xdg) = fresh("flag");
    let (c, _, e) = run(&dir, &xdg, &[], &["--latest-sha"]);
    assert_eq!(c, 2);
    assert!(
        e.contains("columns.latest-sha"),
        "refusal names the ask: {e}"
    );
}

/// Short spelling (the default), asked through config.
#[test]
fn short_columns() {
    let (dir, xdg, first, head) = repo("short");
    let (c, o, e) = run(
        &dir,
        &xdg,
        &[
            ("ASPECTUS_COLUMNS_INITIAL_SHA", "on"),
            ("ASPECTUS_COLUMNS_LATEST_SHA", "on"),
        ],
        &["--depth", "1", "--sort", "name"],
    );
    assert_eq!(c, 0, "{e}");
    assert!(
        o.contains("initial-sha") && o.contains("latest-sha"),
        "headed: {o}"
    );
    let a = o.lines().find(|l| l.contains("a.md")).unwrap();
    assert!(a.contains(&first[..7]), "born in the first commit: {a}");
    assert!(a.contains(&head[..7]), "touched at HEAD: {a}");
}

/// `h~n` spelling: commits behind HEAD; HEAD itself renders `H`.
#[test]
fn hn_spelling() {
    let (dir, xdg, _, _) = repo("hn");
    let (c, o, e) = run(
        &dir,
        &xdg,
        &[
            ("ASPECTUS_COLUMNS_INITIAL_SHA", "on"),
            ("ASPECTUS_COLUMNS_LATEST_SHA", "on"),
            ("ASPECTUS_FORMAT_INITIAL_SHA", "h~n"),
            ("ASPECTUS_FORMAT_LATEST_SHA", "h~n"),
        ],
        &["--depth", "1", "--sort", "name"],
    );
    assert_eq!(c, 0, "{e}");
    let a = o.lines().find(|l| l.contains("a.md")).unwrap();
    // a.md: introduced 2 behind, last touched at HEAD.
    assert!(a.contains("H~2"), "intro two behind: {a}");
    let b = o.lines().find(|l| l.contains("b.md")).unwrap();
    assert!(
        b.contains("H~1"),
        "b.md born and last touched one behind: {b}"
    );
    let cells: Vec<&str> = a.split_whitespace().collect();
    assert!(cells.contains(&"H"), "HEAD spells H: {a}");
}

/// Outside git, nothing is claimed — the columns stay silent, never `0`
/// or a guess.
#[test]
fn outside_git_omits() {
    let (dir, xdg) = fresh("nogit");
    fs::write(dir.join("a.md"), "1\n").unwrap();
    let (c, o, e) = run(
        &dir,
        &xdg,
        &[
            ("ASPECTUS_COLUMNS_INITIAL_SHA", "on"),
            ("ASPECTUS_COLUMNS_LATEST_SHA", "on"),
        ],
        &["--depth", "1"],
    );
    assert_eq!(c, 0, "{e}");
    // No value anywhere → the columns don't exist → no headings for them.
    assert!(
        !o.lines()
            .any(|l| l.contains("initial-sha") && !l.contains("columns.initial-sha")),
        "no heading over silence: {o}"
    );
}

/// An uncommitted file in a repo has no history: absent, never guessed.
#[test]
fn uncommitted_file_claims_nothing() {
    let (dir, xdg, _, head) = repo("newfile");
    fs::write(dir.join("fresh.md"), "new\n").unwrap();
    let (c, o, e) = run(
        &dir,
        &xdg,
        &[("ASPECTUS_COLUMNS_LATEST_SHA", "on")],
        &["--depth", "1", "--sort", "name"],
    );
    assert_eq!(c, 0, "{e}");
    let f = o.lines().find(|l| l.contains("fresh.md")).unwrap();
    assert!(!f.contains(&head[..7]), "no borrowed sha: {f}");
}

/// JSON carries the full shas and the behind-counts.
#[test]
fn json_full_shas() {
    let (dir, xdg, first, head) = repo("json");
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1", "--format", "json"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains(&format!("\"initial_sha\":\"{first}\"")), "{o}");
    assert!(o.contains(&format!("\"latest_sha\":\"{head}\"")), "{o}");
    assert!(o.contains("\"latest_behind\":0"), "{o}");
}

/// The sort keys stay honestly unbuilt — refused by name.
#[test]
fn sha_sort_still_refused() {
    let (dir, xdg, _, _) = repo("sort");
    let (c, _, e) = run(&dir, &xdg, &[], &["--sort", "latest-sha"]);
    assert_eq!(c, 2);
    assert!(e.contains("not built yet"), "{e}");
}
