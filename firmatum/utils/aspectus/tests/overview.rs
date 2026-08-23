//! Overview invariants: absolute root + time of the look + config drift.
//! Real binary. Isolated XDG so the machine's user-home cannot add a
//! drift line the tests did not ask for.

use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static SEQ: AtomicU64 = AtomicU64::new(0);

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_aspectus"))
}

fn fixture() -> (PathBuf, PathBuf) {
    let n = SEQ.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!(
        "aspectus-ov-{}-{}-{}",
        std::process::id(),
        n,
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(dir.join("a")).unwrap();
    File::create(dir.join("f"))
        .unwrap()
        .write_all(b"f")
        .unwrap();
    let xdg = PathBuf::from(format!("{}-xdg", dir.display()));
    fs::create_dir_all(xdg.join("aspectus")).unwrap();
    (dir, xdg)
}

fn run(dir: &Path, xdg: &Path, args: &[&str]) -> std::process::Output {
    bin()
        .args(args)
        .current_dir(dir)
        .env("XDG_CONFIG_HOME", xdg)
        .env_remove("ASPECTUS_LINES")
        .env_remove("ASPECTUS_DEPTH")
        .env_remove("ASPECTUS_WALK")
        .env_remove("ASPECTUS_COLUMNS_MTIME")
        .env_remove("ASPECTUS_COLUMNS_HEAT")
        .env_remove("ASPECTUS_FORMAT")
        .output()
        .unwrap()
}

#[test]
fn header_is_absolute_path_and_utc_stamp() {
    let (dir, xdg) = fixture();
    let out = run(&dir, &xdg, &[]);
    assert_eq!(out.status.code(), Some(0));
    assert!(
        String::from_utf8_lossy(&out.stderr)
            .lines()
            .all(|l| l.is_empty() || l.starts_with("*(This is a critical")),
        "{:?}",
        String::from_utf8_lossy(&out.stderr)
    ); // stderr: only the feedback footer (teaching) since 2026-08-22
    let stdout = String::from_utf8_lossy(&out.stdout);
    // Isolated defaults: stamp, then the root — no config-drift line.
    let mut lines = stdout.lines();
    let stamp = lines.next().expect("stamp line");
    let root = lines.next().expect("root line");
    assert!(
        stamp.len() == 20 && stamp.ends_with('Z') && stamp.contains('T'),
        "ISO-8601 UTC stamp on its own first line: {stamp:?}"
    );
    let abs = std::path::absolute(&dir).unwrap();
    assert!(
        root.starts_with(&format!("{}", abs.display()))
            || root.contains(&abs.to_string_lossy().to_string()),
        "absolute root on its own line: {root}"
    );
    assert!(!root.starts_with("./"), "{root}");
}

/// Simple-header decision (2026-08-14): when the root has facts to say
/// they get their own header line between stamp and path — and the path
/// line stays bare, directly above its children.
#[test]
fn root_facts_get_their_own_line_path_stays_bare() {
    let (dir, xdg) = fixture();
    let out = bin()
        .args([] as [&str; 0])
        .current_dir(&dir)
        .env("XDG_CONFIG_HOME", &xdg)
        .env("ASPECTUS_COLUMNS_MTIME", "on") // root mtime becomes a fact
        .env_remove("ASPECTUS_LINES")
        .env_remove("ASPECTUS_DEPTH")
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&out.stdout);
    let mut lines = stdout.lines();
    let _stamp = lines.next().unwrap();
    // columns.mtime = on differs from default quiet — a drift line.
    let maybe_drift = lines.next().unwrap();
    let (facts, path) = if maybe_drift.contains("columns.mtime") {
        (lines.next().unwrap(), lines.next().unwrap())
    } else {
        (maybe_drift, lines.next().unwrap())
    };
    // mtime's default form is relative now ("0m ago", 2026-08-14); the
    // claim under test is only that the facts line is its own line and
    // the path line stays bare.
    assert!(
        facts.contains("ago") && !facts.contains('/'),
        "root facts on their own line: {facts:?}"
    );
    // (cwd-resolved roots may gain the /private prefix on macOS — compare
    // by suffix.)
    let name = dir.file_name().unwrap().to_string_lossy().into_owned();
    assert!(
        path.starts_with('/') && path.ends_with(&format!("{name}/")) && !path.contains("  "),
        "the path and nothing else: {path:?}"
    );
}

#[test]
fn named_path_also_absolute() {
    let (dir, xdg) = fixture();
    let parent = dir.parent().unwrap();
    let name = dir.file_name().unwrap().to_string_lossy().into_owned();
    let out = run(parent, &xdg, &[&name]);
    assert_eq!(out.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&out.stdout);
    let root = stdout
        .lines()
        .find(|l| l.starts_with('/'))
        .expect("root path line");
    let abs = std::path::absolute(&dir).unwrap();
    assert!(root.contains(&*abs.to_string_lossy()), "{root}");
}

/// Config-drift header (design/overview-invariants.md §Config drift):
/// after the stamp, before the facts/path, every effective setting that
/// differs from the built-in defaults. Absent when nothing differs.
#[test]
fn config_drift_line_after_stamp_when_flags_differ() {
    let (dir, xdg) = fixture();
    let out = run(&dir, &xdg, &["--depth", "3", "--lines", "200"]);
    assert_eq!(out.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&out.stdout);
    let mut lines = stdout.lines();
    let stamp = lines.next().unwrap();
    assert!(stamp.ends_with('Z'), "{stamp}");
    let drift = lines.next().unwrap();
    assert!(
        drift.contains("--depth 3 (flag)") && drift.contains("--lines 200 (flag)"),
        "flag form, both values: {drift:?}"
    );
    assert!(drift.contains(" · "), "items joined by middot: {drift:?}");
    let path = stdout
        .lines()
        .find(|l| l.starts_with('/'))
        .expect("path after drift");
    assert!(path.starts_with('/'), "{path}");
}

#[test]
fn config_drift_absent_at_defaults() {
    let (dir, xdg) = fixture();
    let out = run(&dir, &xdg, &[]);
    assert_eq!(out.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&out.stdout);
    let second = stdout.lines().nth(1).unwrap();
    assert!(
        second.starts_with('/'),
        "no drift line when nothing differs: {second:?}\n{stdout}"
    );
}

#[test]
fn config_drift_user_home_and_json() {
    let (dir, xdg) = fixture();
    fs::write(xdg.join("aspectus/aspectus.toml"), "depth = 3\n").unwrap();
    let out = run(&dir, &xdg, &[]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let drift = stdout.lines().nth(1).unwrap();
    assert_eq!(drift, "depth = 3 (user-home)");

    let out = run(&dir, &xdg, &["--format", "json"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("\"config_drift\":[")
            && stdout.contains("\"key\":\"depth\"")
            && stdout.contains("\"value\":\"3\"")
            && stdout.contains("\"source\":\"user-home\""),
        "JSON names the same drift: {stdout}"
    );
    // Serialization format is the output channel, not the eyes.
    assert!(
        !stdout.contains("\"key\":\"format\""),
        "format text/json is not drift: {stdout}"
    );
}

#[test]
fn config_drift_omitted_from_json_when_nothing_differs() {
    let (dir, xdg) = fixture();
    let out = run(&dir, &xdg, &["--format", "json"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        !stdout.contains("config_drift"),
        "absent, never faked: {stdout}"
    );
}

/// File-as-PATH: the facts line uses the count cell with its unit glyph
/// (no heading follows, so `𝓁` stays) and the heat cluster labelled `heat`.
#[test]
fn file_root_facts_use_count_cell() {
    let (dir, xdg) = fixture();
    let file = dir.join("f");
    File::create(&file)
        .unwrap()
        .write_all(b"a\nb\nc\n")
        .unwrap();
    let out = run(&dir, &xdg, &[file.to_str().unwrap()]);
    assert_eq!(out.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&out.stdout);
    let facts = stdout.lines().nth(1).expect("facts line of a file look");
    assert!(
        facts.contains('\u{1D4C1}') && facts.contains('.'),
        "count cell with 𝓁 (no heading on a file look): {facts:?}\n{stdout}"
    );
}
