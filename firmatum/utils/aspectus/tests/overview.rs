//! Overview invariants: absolute root + time of the look. Real binary.

use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static SEQ: AtomicU64 = AtomicU64::new(0);

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_aspectus"))
}

fn fixture() -> PathBuf {
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
    File::create(dir.join("f")).unwrap().write_all(b"f").unwrap();
    dir
}

#[test]
fn header_is_absolute_path_and_utc_stamp() {
    let dir = fixture();
    let out = bin()
        .current_dir(&dir)
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(0));
    assert!(out.stderr.is_empty(), "{:?}", String::from_utf8_lossy(&out.stderr));
    let stdout = String::from_utf8_lossy(&out.stdout);
    let first = stdout.lines().next().expect("header");
    let abs = std::path::absolute(&dir).unwrap();
    assert!(
        first.starts_with(&format!("{}", abs.display()))
            || first.contains(&abs.to_string_lossy().to_string()),
        "absolute root on header: {first}"
    );
    assert!(!first.starts_with("./"), "{first}");
    let stamp = first.rsplit_once("  ").map(|(_, s)| s).expect("stamp");
    assert!(
        stamp.len() == 20 && stamp.ends_with('Z') && stamp.contains('T'),
        "ISO-8601 UTC stamp: {stamp:?}"
    );
}

#[test]
fn named_path_also_absolute() {
    let dir = fixture();
    let parent = dir.parent().unwrap();
    let name = dir.file_name().unwrap().to_string_lossy().into_owned();
    let out = bin()
        .current_dir(parent)
        .arg(&name)
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&out.stdout);
    let first = stdout.lines().next().unwrap();
    let abs = std::path::absolute(&dir).unwrap();
    assert!(first.contains(&*abs.to_string_lossy()), "{first}");
}
