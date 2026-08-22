//! Hardening pass 2026-08-14 (audit/hallway-2026-08-14.md): the defects
//! the unprimed testers found, pinned through the real binary.

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
        "aspectus-hard-{tag}-{}-{}-{}",
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
        .env_remove("ASPECTUS_DEPTH")
        .env_remove("ASPECTUS_READS");
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

/// A hidden furniture dir keeps presence and magnitude on the has-spot
/// (three independent testimonies: "don't go in" ≠ "doesn't exist").
#[test]
fn hidden_dir_mass_rides_the_has_spot() {
    let (dir, xdg) = fresh("hide");
    fs::create_dir_all(dir.join(".archive/deep")).unwrap();
    for i in 0..7 {
        File::create(dir.join(format!(".archive/deep/f{i}.md"))).unwrap();
    }
    File::create(dir.join(".archive/top.md")).unwrap();
    File::create(dir.join("visible.md")).unwrap();
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    assert!(
        o.contains("archive ≈8f"),
        "hidden mass folds into the has-facet: {o}"
    );
    assert!(!o.contains(".archive/"), "still not a child: {o}");
}

/// `.gitignore` alone no longer stamps `git` — presence-of-ignore-file is
/// not this-is-a-repo (both hallway testers; one bad row poisoned the
/// facet). At a real repo the family word folds away as subsumed.
#[test]
fn gitignore_does_not_claim_git() {
    let (dir, xdg) = fresh("gitig");
    fs::create_dir_all(dir.join(".pytest_cache")).unwrap();
    fs::write(dir.join(".pytest_cache/.gitignore"), "*\n").unwrap();
    fs::write(dir.join("code.py"), "x = 1\n").unwrap();
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "2", "--show-all"]);
    assert_eq!(c, 0, "{e}");
    let cache_line = o.lines().find(|l| l.contains(".pytest_cache")).unwrap();
    assert!(
        !cache_line.contains("git]") && !cache_line.contains("git,"),
        "no git claim from an ignore file: {cache_line}"
    );
    assert!(
        cache_line.contains("gitignore"),
        "its own word instead: {cache_line}"
    );
}

/// An unknown --inspect kind is refused by name with the menu (it used to
/// exit 0 silently).
#[test]
fn inspect_unknown_kind_refused() {
    let (dir, xdg) = fresh("insp");
    let (c, _, e) = run(&dir, &xdg, &[], &["--inspect", "no-such-kind"]);
    assert_eq!(c, 2);
    assert!(
        e.contains("no-such-kind") && e.contains("git") && e.contains("build"),
        "refusal names the kind and the menu: {e}"
    );
}

/// An explicitly named --config file that does not exist is a refusal,
/// not a silent absent layer.
#[test]
fn missing_explicit_config_refused() {
    let (dir, xdg) = fresh("cfg");
    let (c, _, e) = run(&dir, &xdg, &[], &["--config", "/nonexistent/x.toml"]);
    assert_eq!(c, 2);
    assert!(e.contains("not found") && e.contains("x.toml"), "{e}");
}

/// The version line identifies the actual build: a build stamp is always
/// present (the +sha froze across rebuilds and hid a stale install).
#[test]
fn version_carries_build_stamp() {
    let (dir, xdg) = fresh("ver");
    let (c, o, _) = run(&dir, &xdg, &[], &["version"]);
    assert_eq!(c, 0);
    assert!(o.contains("(built ") && o.contains('Z'), "build stamp: {o}");
}

/// Past the read budget, line totals are *estimates* and say so with `~`
/// (not `≈`, which now means exact-but-grouped) — and a big visible file's
/// per-file count is honestly absent instead of slurping gigabytes.
#[test]
fn estimates_marked_distinctly_and_big_visible_files_degrade() {
    let (dir, xdg) = fresh("est");
    fs::create_dir_all(dir.join("sub")).unwrap();
    let line = "0123456789012345678901234567890123456789\n";
    fs::write(dir.join("sub/body.md"), line.repeat(20_000)).unwrap(); // ~820KB
    fs::write(dir.join("big.md"), line.repeat(20_000)).unwrap();
    // Budget of 1 byte: nothing deep can be read; visible big.md is past
    // both the budget and the visible floor.
    let (c, o, e) = run(&dir, &xdg, &[("ASPECTUS_READS", "1")], &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    let sub = o.lines().find(|l| l.contains("sub/")).unwrap();
    // 2026-08-22 count-cell slice: `~` in the m slot; "lines" is the heading.
    assert!(sub.contains('~'), "estimate marked ~: {sub}");
    assert!(
        !sub.contains("≈"),
        "≈ is reserved for exact counts now: {sub}"
    );
    let big = o.lines().find(|l| l.contains("big.md")).unwrap();
    assert!(
        !big.split_whitespace()
            .any(|w| w.chars().all(|c| c.is_ascii_digit())),
        "per-file count absent past the budget: {big}"
    );
    // With budget, the same totals are exact and wear ≈.
    let (_, o2, _) = run(&dir, &xdg, &[], &["--depth", "1"]);
    let sub2 = o2.lines().find(|l| l.contains("sub/")).unwrap();
    // 2026-08-22 count-cell slice: 20,000 lines → ≈ 20.0K in the lines column.
    assert!(
        sub2.contains('\u{2248}') && sub2.contains("20.0K"),
        "exact count, grouped: {sub2}"
    );
    assert!(
        !sub2.contains("≈20k lines"),
        "old mass tail retired: {sub2}"
    );
}

/// JSON heat is rounded to at most four decimals — no 17-digit spurious
/// precision for a decayed sum.
#[test]
fn json_heat_rounded() {
    let (dir, xdg) = fresh("jheat");
    let git = |args: &[&str]| {
        assert!(
            Command::new("git")
                .args(args)
                .current_dir(&dir)
                .env("GIT_AUTHOR_NAME", "t")
                .env("GIT_AUTHOR_EMAIL", "t@t")
                .env("GIT_COMMITTER_NAME", "t")
                .env("GIT_COMMITTER_EMAIL", "t@t")
                .output()
                .unwrap()
                .status
                .success()
        );
    };
    git(&["init", "-q"]);
    fs::write(dir.join("a.md"), "1\n").unwrap();
    git(&["add", "."]);
    git(&["commit", "-qm", "one"]);
    fs::write(dir.join("a.md"), "2\n").unwrap();
    git(&["add", "."]);
    git(&["commit", "-qm", "two"]);
    let (c, o, e) = run(&dir, &xdg, &[], &["--format", "json", "--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    if let Some(i) = o.find("\"heat\":") {
        let num: String = o[i + 7..]
            .chars()
            .take_while(|c| c.is_ascii_digit() || *c == '.')
            .collect();
        let decimals = num.split('.').nth(1).map(str::len).unwrap_or(0);
        assert!(decimals <= 4, "heat rounded: {num}");
    } else {
        panic!("expected a heat fact in a two-commit repo: {o}");
    }
}

/// A gitlink `.git` under --inspect names its gitdir instead of
/// pretending at children.
#[test]
fn gitlink_names_its_gitdir() {
    let (dir, xdg) = fresh("gitlink");
    fs::write(dir.join(".git"), "gitdir: ../elsewhere/modules/x\n").unwrap();
    fs::write(dir.join("a.md"), "1\n").unwrap();
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1", "--inspect", "git"]);
    assert_eq!(c, 0, "{e}");
    let g = o.lines().find(|l| l.contains(".git")).unwrap();
    assert!(
        g.contains("gitdir: ../elsewhere/modules/x"),
        "the gitlink says where it points: {g}"
    );
}

/// Close-audit tranche (2026-08-14): a hidden-furniture count that hit its
/// cap is a `≥` floor, and JSON `truncated` must see it — the code had
/// missed what impl/json.md already promised.
#[test]
fn truncated_sees_hidden_furniture_floor() {
    let (dir, xdg) = fresh("trunchid");
    // A huge hidden furniture dir: the 20k-name hidden-count cap trips,
    // has_counts comes back bounded, truncated must say so.
    fs::create_dir_all(dir.join(".archive")).unwrap();
    for i in 0..25_000 {
        File::create(dir.join(format!(".archive/f{i}"))).unwrap();
    }
    fs::write(dir.join("a.md"), "x\n").unwrap();
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1", "--format", "json"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("\"bounded\":true"), "the floor is data: {o}");
    assert!(
        o.contains("\"truncated\":true"),
        "and truncated sees it: {o}"
    );
}

/// The steward's feedback footer (verbatim ask, 2026-08-14) rides every
/// look on **stderr** (moved off stdout 2026-08-22 — stdout is data):
/// text look, JSON (no `feedback` field in the document), `config` too;
/// `version` keeps its one-line contract.
#[test]
fn feedback_footer_rides_every_output() {
    let (dir, xdg) = fresh("footer");
    fs::write(dir.join("a.md"), "x\n").unwrap();
    let probe = "unproven tool";
    let (_, o, e) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert!(!o.contains(probe), "stdout is the look only: {o}");
    assert!(e.contains(probe), "footer on stderr: {e}");
    let (_, j, e) = run(&dir, &xdg, &[], &["--depth", "1", "--format", "json"]);
    assert!(
        !j.contains("\"feedback\""),
        "no feedback field in the document: {j}"
    );
    assert!(e.contains(probe), "footer on stderr in json mode: {e}");
    let (_, cfg, e) = run(&dir, &xdg, &[], &["config"]);
    assert!(
        !cfg.contains(probe) && e.contains(probe),
        "config: {cfg} / {e}"
    );
    let (_, v, _) = run(&dir, &xdg, &[], &["version"]);
    assert_eq!(v.lines().count(), 1, "version stays one line: {v}");
    let (_, h, _) = run(&dir, &xdg, &[], &["help"]);
    assert!(h.contains("inbox.md"), "help mentions it: {h}");
}
