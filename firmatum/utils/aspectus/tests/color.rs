//! Color: the flag paints. Drive the real binary.

use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static SEQ: AtomicU64 = AtomicU64::new(0);

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_aspectus"))
}

fn fixture() -> PathBuf {
    let n = SEQ.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!(
        "aspectus-color-{}-{}-{}",
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
    dir
}

fn run_in(dir: &Path, args: &[&str]) -> (i32, Vec<u8>, String) {
    let out = bin()
        .args(args)
        .current_dir(dir)
        .output()
        .unwrap_or_else(|e| panic!("launch: {e}"));
    (
        out.status.code().unwrap_or(-1),
        out.stdout,
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

fn has_csi(b: &[u8]) -> bool {
    b.windows(2).any(|w| w == [0x1b, b'['])
}

#[test]
fn always_colors_directories() {
    let dir = fixture();
    let (c, o, e) = run_in(&dir, &["--color=always"]);
    assert_eq!(c, 0, "{e}");
    assert!(
        e.lines()
            .all(|l| l.is_empty() || l.starts_with("*(This is a critical")),
        "{e:?}"
    ); // stderr: only the feedback footer (teaching) since 2026-08-22
    assert!(has_csi(&o), "always must emit CSI: {o:?}");
    let text = String::from_utf8_lossy(&o);
    assert!(text.contains("a/"), "{text}");
    assert!(
        text.contains("\u{1b}[01;34m"),
        "dirs use bold blue: {text:?}"
    );
}

#[test]
fn never_has_no_csi() {
    let dir = fixture();
    let (c, o, e) = run_in(&dir, &["--color=never"]);
    assert_eq!(c, 0, "{e}");
    assert!(!has_csi(&o), "never must not emit CSI");
}

#[test]
fn auto_in_a_pipe_has_no_csi() {
    let dir = fixture();
    let (c, o, e) = run_in(&dir, &["--color=auto"]);
    assert_eq!(c, 0, "{e}");
    assert!(!has_csi(&o), "piped auto must not emit CSI");
}

#[test]
fn default_auto_in_a_pipe_has_no_csi() {
    let dir = fixture();
    let (c, o, e) = run_in(&dir, &[]);
    assert_eq!(c, 0, "{e}");
    assert!(!has_csi(&o), "piped default is auto");
}

#[test]
fn bad_color_value_is_usage() {
    let (c, o, e) = run_in(Path::new("/"), &["--color=purple"]);
    assert_eq!(c, 2);
    assert!(o.is_empty(), "{o:?}");
    assert!(
        e.contains("auto") || e.contains("never") || e.contains("usage"),
        "{e}"
    );
    assert!(
        !e.contains("faculty of looking"),
        "must not reprint help: {e}"
    );
}
