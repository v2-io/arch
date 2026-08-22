//! Line counts (design/linecount.md): per-file counts on non-binary files,
//! suffix-map-driven, uncached-first. Real binary, isolated XDG.

use std::fs::{self, File};
use std::io::Write;
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
        "aspectus-lc-{tag}-{}-{}-{}",
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
    c.args(args).current_dir(dir).env("XDG_CONFIG_HOME", xdg);
    for k in [
        "ASPECTUS_LINES",
        "ASPECTUS_DEPTH",
        "ASPECTUS_SORT",
        "ASPECTUS_COLUMNS_SIZE",
        "ASPECTUS_COLUMNS_MTIME",
        "ASPECTUS_COLUMNS_LINE_COUNT",
        "ASPECTUS_COLUMNS_HEAT",
        "ASPECTUS_FORMAT_LINE_COUNT",
        "ASPECTUS_KINDS",
        "ASPECTUS_READS",
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

fn user_home(xdg: &Path, body: &str) {
    fs::write(xdg.join("aspectus/aspectus.toml"), body).unwrap();
}

/// Fresh files would surprise the quiet mtime law (recent vs now); these
/// fixtures are about line counts, not surprise — so age everything.
fn backdate(dir: &Path) {
    for ent in fs::read_dir(dir).unwrap().flatten() {
        if let Ok(f) = File::open(ent.path()) {
            let _ = f.set_modified(
                std::time::UNIX_EPOCH + std::time::Duration::from_secs(1_700_000_000),
            );
        }
    }
}

fn line_of<'a>(o: &'a str, name: &str) -> &'a str {
    o.lines()
        .find(|l| l.contains(name))
        .unwrap_or_else(|| panic!("{name} not in {o}"))
}

/// Subfeatures 1, 3, 4: counts on text, empty is zero, unterminated final
/// line counts as one.
#[test]
fn counts_on_text_files() {
    let (dir, xdg) = fresh("basic");
    fs::write(dir.join("twelve.md"), "x\n".repeat(12)).unwrap();
    fs::write(dir.join("empty.txt"), "").unwrap();
    fs::write(dir.join("unterm.md"), "a\nb").unwrap();
    backdate(&dir);
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    assert!(line_of(&o, "twelve.md").contains("12"), "{o}");
    let empty = line_of(&o, "empty.txt");
    // 2026-08-22 count-cell slice: empty text is `0.` (the `.` always).
    assert!(empty.contains("0."), "empty text is a real 0: {o}");
    assert!(
        line_of(&o, "unterm.md").contains('2'),
        "a\\nb counts 2: {o}"
    );
}

/// Subfeature 2: binary omits — not 0, no placeholder.
#[test]
fn binary_omits_never_zero() {
    let (dir, xdg) = fresh("bin");
    File::create(dir.join("img.png"))
        .unwrap()
        .write_all(&[0x89, 0x50, 0x4e, 0x47, 0, 1, 2])
        .unwrap();
    backdate(&dir);
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    let line = line_of(&o, "img.png");
    assert_eq!(
        line.trim_end(),
        line.trim_end().trim_end_matches('0'),
        "no 0 on binary: {o}"
    );
    assert!(
        !line.contains("img.png "),
        "nothing after the name: {line:?}"
    );
}

/// Unknown suffix: the null-byte sniff decides (design leaning).
#[test]
fn unknown_suffix_sniffs() {
    let (dir, xdg) = fresh("sniff");
    fs::write(dir.join("notes.zzz"), "one\ntwo\n").unwrap();
    File::create(dir.join("blob.zzy"))
        .unwrap()
        .write_all(&[1, 0, 2, 0, 3])
        .unwrap();
    backdate(&dir);
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    assert!(line_of(&o, "notes.zzz").contains('2'), "text by sniff: {o}");
    let blob = line_of(&o, "blob.zzy");
    assert!(
        blob.trim_end().ends_with("blob.zzy"),
        "binary by sniff, no count: {o}"
    );
}

/// Subfeature 5: config suffix-map override, both directions.
#[test]
fn suffix_map_is_config_driven() {
    let (dir, xdg) = fresh("map");
    fs::write(dir.join("a.md"), "1\n2\n3\n").unwrap();
    user_home(&xdg, "kinds = \"md:binary\"\n");
    backdate(&dir);
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    assert!(
        line_of(&o, "a.md").trim_end().ends_with("a.md"),
        "marked binary, count gone: {o}"
    );
    user_home(&xdg, "kinds = \"md:text\"\n");
    let (_, o2, _) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert!(line_of(&o2, "a.md").contains('3'), "text restores it: {o2}");
}

/// Subfeature 6: non-blank format.
#[test]
fn non_blank_excludes_whitespace_lines() {
    let (dir, xdg) = fresh("nb");
    fs::write(dir.join("gaps.md"), "a\n\n  \nb\n").unwrap();
    user_home(&xdg, "format.line-count = \"non-blank\"\n");
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    assert!(line_of(&o, "gaps.md").contains('2'), "{o}");
}

/// Subfeature 7: no own flag; OFF via config removes the column.
#[test]
fn compose_only_no_flag() {
    let (dir, xdg) = fresh("flag");
    fs::write(dir.join("a.md"), "1\n").unwrap();
    let (c, _, e) = run(&dir, &xdg, &[], &["--line-count"]);
    assert_eq!(c, 2);
    assert!(
        e.contains("columns.line-count"),
        "refusal names the ask: {e}"
    );
    user_home(&xdg, "columns.line-count = \"off\"\n");
    backdate(&dir);
    let (c2, o, _) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert_eq!(c2, 0);
    assert!(
        line_of(&o, "a.md").trim_end().ends_with("a.md"),
        "off removes it: {o}"
    );
}

/// Subfeature 8: unreadable file — no count, never a guess.
#[test]
fn unreadable_file_claims_nothing() {
    use std::os::unix::fs::PermissionsExt;
    if is_root() {
        eprintln!("skipped: running as root");
        return;
    }
    let (dir, xdg) = fresh("deny");
    let p = dir.join("locked.md");
    fs::write(&p, "1\n2\n").unwrap();
    backdate(&dir);
    fs::set_permissions(&p, fs::Permissions::from_mode(0o000)).unwrap();
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1"]);
    fs::set_permissions(&p, fs::Permissions::from_mode(0o644)).unwrap();
    assert_eq!(c, 0, "{e}");
    assert!(
        line_of(&o, "locked.md").trim_end().ends_with("locked.md"),
        "{o}"
    );
}

fn is_root() -> bool {
    std::process::Command::new("id")
        .arg("-u")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "0")
        .unwrap_or(false)
}

/// Subfeature 9: uncached correctness — two runs identical (stamp aside).
#[test]
fn deterministic_across_runs() {
    let (dir, xdg) = fresh("det");
    fs::write(dir.join("a.md"), "1\n2\n").unwrap();
    fs::write(dir.join("b.rs"), "fn main() {}\n").unwrap();
    let (_, o1, _) = run(&dir, &xdg, &[], &["--depth", "1"]);
    let (_, o2, _) = run(&dir, &xdg, &[], &["--depth", "1"]);
    let tail = |s: &str| s.lines().skip(1).collect::<Vec<_>>().join("\n");
    assert_eq!(tail(&o1), tail(&o2));
}
