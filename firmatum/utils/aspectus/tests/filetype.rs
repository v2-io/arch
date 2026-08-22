//! Filetype ladder (design/filetype.md): magic, shebang, suffix as
//! tie-breaker, JSON `type`, census grain. Real binary, isolated XDG.

use std::fs::{self, File};
use std::io::Write;
use std::os::unix::fs::{FileTypeExt, PermissionsExt};
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
        "aspectus-ft-{tag}-{}-{}-{}",
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
        "ASPECTUS_FORMAT",
        "ASPECTUS_FORMAT_CENSUS",
        "ASPECTUS_COLUMNS_HEAT",
        "ASPECTUS_KINDS",
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

fn backdate(dir: &Path) {
    for ent in fs::read_dir(dir).unwrap().flatten() {
        let p = ent.path();
        // Opening a fifo blocks until a writer appears — don't.
        if let Ok(ft) = ent.file_type()
            && (ft.is_fifo() || ft.is_socket() || ft.is_char_device() || ft.is_block_device())
        {
            continue;
        }
        if let Ok(f) = File::open(&p) {
            let _ = f.set_modified(
                std::time::UNIX_EPOCH + std::time::Duration::from_secs(1_700_000_000),
            );
        }
    }
}

/// Magic beats a lying suffix: a `.md` that starts PNG is an image, no count.
#[test]
fn magic_overrides_suffix_for_linecount() {
    let (dir, xdg) = fresh("magic");
    let mut f = File::create(dir.join("lie.md")).unwrap();
    f.write_all(b"\x89PNG\r\n\x1a\n").unwrap();
    f.write_all(&[0u8; 32]).unwrap();
    backdate(&dir);
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    let line = o
        .lines()
        .find(|l| l.contains("lie.md"))
        .unwrap_or_else(|| panic!("{o}"));
    assert!(
        line.trim_end().ends_with("lie.md") || line.contains("image"),
        "png-bytes .md omits the line count: {line:?}\n{o}"
    );
    assert!(!line.contains("1.") && !line.contains("2."), "{line:?}");
}

/// Shebang on an extensionless file is exe/script and still counts lines.
#[test]
fn shebang_extensionless_counts() {
    let (dir, xdg) = fresh("shebang");
    let p = dir.join("runme");
    fs::write(&p, "#!/usr/bin/env python3\nprint(1)\nprint(2)\n").unwrap();
    fs::set_permissions(&p, fs::Permissions::from_mode(0o755)).unwrap();
    backdate(&dir);
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1", "--format", "json"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("\"major\":\"exe\""), "exe/script: {o}");
    assert!(o.contains("\"minor\":\"script\""), "{o}");
    assert!(o.contains("\"trait\":\"python\""), "{o}");
    assert!(o.contains("\"lines\":3"), "shebang script counts: {o}");
}

/// JSON carries type on every node, dirs included.
#[test]
fn json_type_on_every_node() {
    let (dir, xdg) = fresh("json");
    fs::write(dir.join("a.md"), "hi\n").unwrap();
    fs::create_dir(dir.join("sub")).unwrap();
    backdate(&dir);
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1", "--format", "json"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("\"type\":{\"major\":\"dir\"}"), "dir type: {o}");
    assert!(
        o.contains("\"type\":{\"major\":\"text\",\"minor\":\"markdown\"}"),
        "md type: {o}"
    );
}

/// `format.census = major` buckets by major; default suffix is unchanged.
#[test]
fn census_grain_major() {
    let (dir, xdg) = fresh("grain");
    fs::create_dir(dir.join("box")).unwrap();
    fs::write(dir.join("box/a.md"), "x\n").unwrap();
    fs::write(dir.join("box/b.md"), "x\n").unwrap();
    fs::write(dir.join("box/c.rs"), "x\n").unwrap();
    backdate(&dir);
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("md×2"), "default grain is suffix: {o}");
    assert!(o.contains("rs×1"), "{o}");

    fs::write(
        xdg.join("aspectus/aspectus.toml"),
        "format.census = \"major\"\n",
    )
    .unwrap();
    let (_, o2, _) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert!(o2.contains("text×3"), "major grain collapses md+rs: {o2}");
}

/// Specials classify (JSON) and do not grow an `ls -F` suffix by default.
#[test]
fn fifo_classifies_without_glyph() {
    let (dir, xdg) = fresh("fifo");
    let p = dir.join("pipe");
    let st = Command::new("mkfifo").arg(&p).status().unwrap();
    if !st.success() {
        eprintln!("skipped: mkfifo failed");
        return;
    }
    backdate(&dir);
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1", "--format", "json"]);
    assert_eq!(c, 0, "{e}");
    assert!(
        o.contains("\"major\":\"special\"") && o.contains("\"minor\":\"fifo\""),
        "fifo type: {o}"
    );
    let (c2, text, _) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert_eq!(c2, 0);
    let line = text
        .lines()
        .find(|l| l.contains("pipe"))
        .unwrap_or_else(|| panic!("{text}"));
    assert!(
        !line.contains("pipe|"),
        "no ls -F glyph by default: {line:?}"
    );
}
