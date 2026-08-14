//! Important files (design/important-files.md subfeatures): pure survival
//! weight — no column, no glyph, no order change. Real binary, isolated XDG.

use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, UNIX_EPOCH};

static SEQ: AtomicU64 = AtomicU64::new(0);

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_aspectus"))
}

fn fresh(tag: &str) -> (PathBuf, PathBuf) {
    let n = SEQ.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!(
        "aspectus-imp-{tag}-{}-{}-{}",
        std::process::id(),
        n,
        std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&dir).unwrap();
    let xdg = PathBuf::from(format!("{}-xdg", dir.display()));
    fs::create_dir_all(xdg.join("aspectus")).unwrap();
    (dir, xdg)
}

fn touch(dir: &Path, name: &str, epoch_secs: u64) {
    let p = dir.join(name);
    File::create(&p).unwrap().write_all(b"x\n").unwrap();
    File::open(&p)
        .unwrap()
        .set_modified(UNIX_EPOCH + Duration::from_secs(epoch_secs))
        .unwrap();
}

fn run(dir: &Path, xdg: &Path, envs: &[(&str, &str)], args: &[&str]) -> (i32, String, String) {
    let mut c = bin();
    c.args(args).current_dir(dir).env("XDG_CONFIG_HOME", xdg);
    for k in ["ASPECTUS_LINES", "ASPECTUS_DEPTH", "ASPECTUS_SORT", "ASPECTUS_IMPORTANT"] {
        c.env_remove(k);
    }
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

/// 30 files, README the *oldest*, budget for a few: README is listed even
/// when 25 siblings out-recency it; the census absorbs the rest.
fn squeeze_fixture() -> (PathBuf, PathBuf) {
    let (dir, xdg) = fresh("sq");
    touch(&dir, "README.md", 1_700_000_000); // oldest
    for i in 0..29 {
        touch(&dir, &format!("f{i:02}.md"), 1_700_001_000 + i * 10);
    }
    (dir, xdg)
}

/// Subfeature 1: survives the squeeze.
#[test]
fn survives_the_squeeze() {
    let (dir, xdg) = squeeze_fixture();
    // stamp + root + 4 children + leaf census = 7
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1", "--lines", "7"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("README.md"), "oldest but important survives: {o}");
    assert!(o.contains("f28.md"), "newest plain files fill the rest: {o}");
    assert!(!o.contains("f00.md"), "{o}");
    assert!(o.contains("[+ "), "leftover census: {o}");
}

/// Subfeature 2: with budget to spare README sits wherever recency puts it
/// — dead last here. No early listing, no glyph.
#[test]
fn order_untouched_when_loose() {
    let (dir, xdg) = squeeze_fixture();
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1", "--lines", "0"]);
    assert_eq!(c, 0, "{e}");
    let names: Vec<&str> = o
        .lines()
        .skip(2)
        .filter_map(|l| l.rsplit("── ").next())
        .map(|s| s.split_whitespace().next().unwrap_or(""))
        .collect();
    assert_eq!(names.last(), Some(&"README.md"), "recency order untouched: {o}");
}

/// Subfeature 3: config set — adding survives, `!README*` demotes.
#[test]
fn config_set_extends_and_drops() {
    let (dir, xdg) = fresh("cfg");
    touch(&dir, "DESIGN.md", 1_700_000_000); // oldest
    for i in 0..10 {
        touch(&dir, &format!("f{i}.md"), 1_700_001_000 + i * 10);
    }
    fs::write(xdg.join("aspectus/aspectus.toml"), "important = \"DESIGN.md\"\n").unwrap();
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1", "--lines", "6"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("DESIGN.md"), "config-important survives: {o}");

    let (dir2, xdg2) = squeeze_fixture();
    fs::write(xdg2.join("aspectus/aspectus.toml"), "important = \"!README*\"\n").unwrap();
    let (c, o, e) = run(&dir2, &xdg2, &[], &["--depth", "1", "--lines", "7"]);
    assert_eq!(c, 0, "{e}");
    assert!(!o.contains("README.md"), "demoted README folds by recency: {o}");
}

/// Subfeature 4: no ancestor rescue — an unexpanded dir holding only a
/// README renders as census/mass like any other.
#[test]
fn no_ancestor_rescue() {
    let (dir, xdg) = fresh("anc");
    let sub = dir.join("docs");
    fs::create_dir_all(&sub).unwrap();
    touch(&sub, "README.md", 1_700_000_000);
    for i in 0..10 {
        touch(&dir, &format!("f{i}.md"), 1_700_001_000 + i * 10);
    }
    File::open(&sub)
        .unwrap()
        .set_modified(UNIX_EPOCH + Duration::from_secs(1_700_000_500))
        .unwrap();
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1", "--lines", "0"]);
    assert_eq!(c, 0, "{e}");
    let docs = o.lines().find(|l| l.contains("docs/")).unwrap();
    assert!(docs.contains("[README.md]"), "just part of the census: {o}");
    assert!(!o.contains("├── README.md") && !o.contains("└── README.md"),
        "not pulled up as a line: {o}");
}

/// Subfeature 5: five importants, three file slots — key-within-tier picks
/// the newest three; the leftover falls into the typed census.
#[test]
fn too_many_importants_compete() {
    let (dir, xdg) = fresh("many");
    fs::write(
        xdg.join("aspectus/aspectus.toml"),
        "important = \"IMP?.md\"\n",
    )
    .unwrap();
    for i in 0..5u64 {
        touch(&dir, &format!("IMP{i}.md"), 1_700_000_000 + i * 100);
    }
    for i in 0..10u64 {
        touch(&dir, &format!("f{i}.md"), 1_700_002_000 + i * 10);
    }
    // stamp + root + 3 children + census = 6
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1", "--lines", "6"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("IMP4.md") && o.contains("IMP3.md") && o.contains("IMP2.md"),
        "newest importants win the tier: {o}");
    assert!(!o.contains("IMP0.md"), "leftover important censused: {o}");
    assert!(o.contains("[+ "), "{o}");
}

/// Subfeature 6: determinism, tight or loose.
#[test]
fn deterministic_both_budgets() {
    let (dir, xdg) = squeeze_fixture();
    for lines in ["7", "0"] {
        let (_, o1, _) = run(&dir, &xdg, &[], &["--depth", "1", "--lines", lines]);
        let (_, o2, _) = run(&dir, &xdg, &[], &["--depth", "1", "--lines", lines]);
        assert_eq!(
            o1.lines().skip(1).collect::<Vec<_>>(),
            o2.lines().skip(1).collect::<Vec<_>>()
        );
    }
}
