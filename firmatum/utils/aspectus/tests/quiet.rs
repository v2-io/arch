//! Quiet columns, cold baseline (design/quiet-columns.md subfeatures).
//! Real binary, isolated XDG. Mtimes are backdated so only the surprise
//! under test speaks; the root-owned law is covered at the decision layer
//! (src/quiet.rs unit tests) since a real root fixture needs privilege.

use std::fs::{self, File};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
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
        "aspectus-quiet-{tag}-{}-{}-{}",
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

fn write_file(dir: &Path, name: &str, bytes: usize) {
    let mut f = File::create(dir.join(name)).unwrap();
    f.write_all(&vec![b'x'; bytes]).unwrap();
}

fn backdate_all(dir: &Path) {
    for ent in fs::read_dir(dir).unwrap().flatten() {
        if let Ok(f) = File::open(ent.path()) {
            let _ = f.set_modified(UNIX_EPOCH + Duration::from_secs(1_700_000_000));
        }
    }
    if let Ok(f) = File::open(dir) {
        let _ = f.set_modified(UNIX_EPOCH + Duration::from_secs(1_700_000_000));
    }
}

fn run(dir: &Path, xdg: &Path, envs: &[(&str, &str)], args: &[&str]) -> (i32, String, String) {
    let mut c = bin();
    c.args(args)
        .current_dir(dir)
        .env("XDG_CONFIG_HOME", xdg)
        // Numbered-series fixtures probe sibling-norm statistics; globify
        // (2026-08-14) would fuse the cohort — pinned off, its own row.
        .env("ASPECTUS_GLOBIFY", "off");
    for k in [
        "ASPECTUS_LINES",
        "ASPECTUS_DEPTH",
        "ASPECTUS_SORT",
        "ASPECTUS_COLUMNS_SIZE",
        "ASPECTUS_COLUMNS_MTIME",
        "ASPECTUS_COLUMNS_PERMISSIONS",
        "ASPECTUS_COLUMNS_OWNER",
        "ASPECTUS_COLUMNS_FILEKIND",
        "ASPECTUS_QUIET_SENSITIVITY",
        "ASPECTUS_QUIET_SENSITIVITY_SIZE",
        "ASPECTUS_QUIET_SENSITIVITY_MTIME",
        "ASPECTUS_FORMAT",
    ] {
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

fn line_of<'a>(o: &'a str, name: &str) -> &'a str {
    // Skip the stamp and root header — a temp path can contain any name.
    o.lines()
        .skip(2)
        .find(|l| l.contains(name))
        .unwrap_or_else(|| panic!("{name} not in {o}"))
}

/// Subfeature 1: all-usual level — no permissions/owner/size/kind glyphs
/// anywhere, no mtime (old files).
#[test]
fn usual_is_silent() {
    let (dir, xdg) = fresh("usual");
    for n in ["a.md", "b.md", "c.md"] {
        write_file(&dir, n, 10);
    }
    backdate_all(&dir);
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    assert!(!o.contains("644"), "usual mode silent: {o}");
    assert!(!o.contains("binary") && !o.contains("text"), "{o}");
    assert_eq!(o.matches('Z').count(), 1, "only the stamp is a time: {o}");
    assert!(!o.contains("10B"), "usual size silent: {o}");
}

/// Subfeature 2: one 600 among 644s shows its mode; the 644s stay silent.
#[test]
fn odd_mode_speaks() {
    let (dir, xdg) = fresh("mode");
    for n in ["a.md", "b.md", "c.md", "secret.md"] {
        write_file(&dir, n, 10);
    }
    fs::set_permissions(dir.join("secret.md"), fs::Permissions::from_mode(0o600)).unwrap();
    backdate_all(&dir);
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    assert!(line_of(&o, "secret.md").contains("600"), "{o}");
    assert!(!line_of(&o, "a.md").contains("644"), "{o}");
}

/// Subfeature 3: a bin-like level of all-755 files is silent — the
/// majority normalizes what convention alone would flag.
#[test]
fn majority_normalizes_exec() {
    let (dir, xdg) = fresh("bin");
    for n in ["run1", "run2", "run3"] {
        write_file(&dir, n, 10);
        fs::set_permissions(dir.join(n), fs::Permissions::from_mode(0o755)).unwrap();
    }
    backdate_all(&dir);
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    assert!(
        !o.contains("755"),
        "sibling-usual bin dir stays silent: {o}"
    );
}

/// Subfeature 4: a setuid file speaks even in an all-setuid level.
#[test]
fn special_bits_always_speak() {
    let (dir, xdg) = fresh("suid");
    for n in ["s1", "s2"] {
        write_file(&dir, n, 10);
        fs::set_permissions(dir.join(n), fs::Permissions::from_mode(0o4755)).unwrap();
    }
    backdate_all(&dir);
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    assert!(line_of(&o, "s1").contains("4755"), "{o}");
    assert!(line_of(&o, "s2").contains("4755"), "{o}");
}

/// Subfeature 6: the whale's size prints; the shoal stays silent.
#[test]
fn size_whale_speaks() {
    let (dir, xdg) = fresh("whale");
    for i in 0..10 {
        write_file(&dir, &format!("f{i}.md"), 2048);
    }
    write_file(&dir, "whale.md", 80 << 20);
    backdate_all(&dir);
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    // 2026-08-22 count-cell slice: 80 MiB → ≈ 80.0M (always one fraction
    // digit when scaled); 2048 B is exact `2·048.`, so the old `2.0K`
    // silence check still holds.
    assert!(line_of(&o, "whale.md").contains("80.0M"), "{o}");
    assert!(!line_of(&o, "f1.md").contains("2.0K"), "{o}");
}

/// Subfeature 7: a tiny level under the floor is silent even at 10×.
#[test]
fn tiny_level_floor() {
    let (dir, xdg) = fresh("tiny");
    write_file(&dir, "small.md", 100);
    write_file(&dir, "bigger.md", 1000);
    backdate_all(&dir);
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    // Tree lines only — the temp-dir path in the header carries digits.
    let tree: String = o
        .lines()
        .skip_while(|l| !l.starts_with('/'))
        .skip(1)
        .collect();
    assert!(
        !tree.contains("1000") && !tree.contains("1.0K") && !tree.contains("100B"),
        "{o}"
    );
}

/// Subfeature 8: the binary among twenty .md carries the kind word.
#[test]
fn kind_intruder_speaks() {
    let (dir, xdg) = fresh("kind");
    for i in 0..20 {
        write_file(&dir, &format!("n{i}.md"), 10);
    }
    write_file(&dir, "blob.png", 10);
    backdate_all(&dir);
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1", "--lines", "0"]);
    assert_eq!(c, 0, "{e}");
    assert!(line_of(&o, "blob.png").contains("binary"), "{o}");
    assert!(
        !line_of(&o, "n3.md").contains("text"),
        "the plurality is silent: {o}"
    );
}

/// Recent mtime speaks (cold recency window), old stays silent.
#[test]
fn recent_mtime_speaks() {
    let (dir, xdg) = fresh("recent");
    write_file(&dir, "old.md", 10);
    write_file(&dir, "hot.md", 10);
    File::open(dir.join("old.md"))
        .unwrap()
        .set_modified(UNIX_EPOCH + Duration::from_secs(1_700_000_000))
        .unwrap();
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    // mtime speaks in its relative default form now (2026-08-14).
    assert!(line_of(&o, "hot.md").contains("ago"), "recent speaks: {o}");
    assert!(!line_of(&o, "old.md").contains("ago"), "old silent: {o}");
}

/// Subfeature 9: a node listed at --lines 4 and --lines 0 carries the same
/// quiet facts — norms come from the full level, not the survivors.
#[test]
fn budget_independent() {
    let (dir, xdg) = fresh("budget");
    for i in 0..30 {
        write_file(&dir, &format!("f{i:02}.md"), 2048);
    }
    write_file(&dir, "whale.md", 80 << 20);
    backdate_all(&dir);
    // whale is newest by a hair so it survives the squeeze deterministically.
    File::open(dir.join("whale.md"))
        .unwrap()
        .set_modified(UNIX_EPOCH + Duration::from_secs(1_700_000_100))
        .unwrap();
    let (_, o_all, _) = run(&dir, &xdg, &[], &["--depth", "1", "--lines", "0"]);
    let (_, o_tight, _) = run(&dir, &xdg, &[], &["--depth", "1", "--lines", "5"]);
    // Compare facts, not padding: column tab-stops are a function of each
    // look's own content, so the two looks may align differently.
    let squeeze = |s: &str| s.split_whitespace().collect::<Vec<_>>().join(" ");
    assert_eq!(
        squeeze(line_of(&o_all, "whale.md")),
        squeeze(line_of(&o_tight, "whale.md")),
        "identical quiet facts across budgets"
    );
    // 2026-08-22 count-cell slice: 80 MiB is `80.0M`, not `80M`.
    assert!(line_of(&o_tight, "whale.md").contains("80.0M"));
}

/// Subfeature 10: the dial silences a 10× outlier; a per-fact override
/// re-voices it.
#[test]
fn sensitivity_dial_and_override() {
    let (dir, xdg) = fresh("dial");
    for i in 0..10 {
        write_file(&dir, &format!("f{i}.md"), 1 << 20);
    }
    write_file(&dir, "big.md", 10 << 20); // 10×: log10 dev = 1.0
    backdate_all(&dir);
    let (_, o, _) = run(&dir, &xdg, &[], &["--depth", "1"]);
    // 2026-08-22 count-cell slice: 10 MiB is `10.0M`, not `10M`.
    assert!(
        line_of(&o, "big.md").contains("10.0M"),
        "default 1.0 voices 10x: {o}"
    );
    fs::write(
        xdg.join("aspectus/aspectus.toml"),
        "quiet.sensitivity = \"2.0\"\n",
    )
    .unwrap();
    let (_, o, _) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert!(
        !line_of(&o, "big.md").contains("10.0M"),
        "2.0 silences 10x: {o}"
    );
    fs::write(
        xdg.join("aspectus/aspectus.toml"),
        "quiet.sensitivity = \"2.0\"\nquiet.sensitivity.size = \"1.0\"\n",
    )
    .unwrap();
    let (_, o, _) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert!(
        line_of(&o, "big.md").contains("10.0M"),
        "per-fact override re-voices: {o}"
    );
}

/// Subfeature 11: same tree, same config — byte-identical below the stamp
/// (fixture mtimes pinned old so the recency window is stable).
#[test]
fn determinism() {
    let (dir, xdg) = fresh("det");
    for i in 0..5 {
        write_file(&dir, &format!("f{i}.md"), 2048);
    }
    write_file(&dir, "big.bin", 600 << 10);
    backdate_all(&dir);
    let (_, o1, _) = run(&dir, &xdg, &[], &["--depth", "1"]);
    let (_, o2, _) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert_eq!(
        o1.lines().skip(1).collect::<Vec<_>>(),
        o2.lines().skip(1).collect::<Vec<_>>()
    );
}

/// Subfeature 12: JSON carries the facts regardless of quiet — quiet is a
/// text-rendering law, not a data cut.
#[test]
fn json_unaffected_by_quiet() {
    let (dir, xdg) = fresh("json");
    for n in ["a.md", "b.md", "c.md"] {
        write_file(&dir, n, 10);
    }
    backdate_all(&dir);
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1", "--format", "json"]);
    assert_eq!(c, 0, "{e}");
    // Text is silent on all of these; the data carries them anyway.
    assert!(o.contains("\"mode\":\"644\""), "{o}");
    assert!(o.contains("\"uid\":"), "{o}");
    assert!(o.contains("\"mtime\":\"2023-11-14"), "{o}");
}
