//! Grid snapshots — the prefactor's safety net (impl/grid-cleanup.md step 1),
//! re-blessed for the count-cell slice (step 3, 2026-08-22): `lines`/`bytes`
//! are 12-cell fields, dir mass lives in `lines`, the census tail is gone.
//! Re-blessed 2026-08-22: count-cell headings end at the `.` (cell 9).
//! Re-blessed 2026-08-23: config-drift header — this file pins heat off
//! via env, so `columns.heat = off (env)` appears; invocations that set
//! `--depth`/`--lines` away from default add those flags too.
//!
//! These fixtures exercise the whole row grammar at once —
//! furniture, facets, dir census, leaf census, symlinks (one broken), a
//! walk-bound mark, quiet columns, gitignored entries, the `[has: …]` block
//! — and compare against goldens byte-for-byte.
//!
//! Heat is off in every snapshot on purpose: `score · age` is a function of
//! wall-clock time, so a golden containing it would rot. Heat's own paint
//! (including the two-sub-column cluster alignment) is pinned by
//! `tests/heat.rs` and `tests/columns.rs::heading_sits_over_the_cluster`.
//! The look's first line is the stamp (also wall-clock) and is dropped.
//!
//! Refresh a golden deliberately with `ASPECTUS_SNAPSHOT_BLESS=1 cargo test
//! --test grid_snapshot` and read the diff before committing it.

use std::fs::{self, File};
use std::io::Write;
use std::os::unix::fs as unixfs;
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
        "aspectus-snap-{tag}-{}-{}-{}",
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

/// Old, fixed mtimes everywhere: a fresh file surprises the quiet mtime law
/// (recent vs now), which would make the look time-dependent.
const OLD: u64 = 1_600_000_000;

fn age(p: &Path) {
    let f = File::open(p).unwrap();
    f.set_modified(std::time::UNIX_EPOCH + std::time::Duration::from_secs(OLD))
        .unwrap();
}

fn write(p: &Path, body: &str) {
    if let Some(parent) = p.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    File::create(p).unwrap().write_all(body.as_bytes()).unwrap();
}

/// Age every path in the tree, deepest first, so a child's write does not
/// re-touch the parent afterwards.
fn age_all(root: &Path) {
    let mut stack = vec![root.to_path_buf()];
    let mut all = Vec::new();
    while let Some(d) = stack.pop() {
        for e in fs::read_dir(&d).unwrap() {
            let p = e.unwrap().path();
            if p.is_dir() && !p.is_symlink() {
                stack.push(p.clone());
            }
            all.push(p);
        }
    }
    all.sort_by_key(|p| std::cmp::Reverse(p.components().count()));
    for p in &all {
        if p.is_symlink() {
            continue;
        }
        age(p);
    }
    age(root);
}

fn run(dir: &Path, xdg: &Path, args: &[&str]) -> String {
    let out = bin()
        .args(args)
        .current_dir(dir)
        .env("XDG_CONFIG_HOME", xdg)
        .env("ASPECTUS_COLUMNS_HEAT", "off")
        .env_remove("ASPECTUS_LINES")
        .env_remove("ASPECTUS_DEPTH")
        .env_remove("ASPECTUS_SORT")
        .env_remove("ASPECTUS_COLUMNS_SIZE")
        .env_remove("ASPECTUS_COLUMNS_MTIME")
        .env_remove("ASPECTUS_FORMAT_SIZE")
        .env_remove("ASPECTUS_FORMAT_MTIME")
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "aspectus {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let s = String::from_utf8_lossy(&out.stdout).into_owned();
    // Line 1 is the wall-clock stamp; the tree below it is the subject.
    // The absolute root path (a temp dir) is volatile too.
    let dir_s = dir.canonicalize().unwrap().display().to_string();
    s.lines()
        .skip(1)
        .map(|l| l.replace(&dir_s, "<ROOT>"))
        .collect::<Vec<_>>()
        .join("\n")
        + "\n"
}

fn golden(name: &str, got: &str) {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/snapshots")
        .join(format!("{name}.txt"));
    if std::env::var_os("ASPECTUS_SNAPSHOT_BLESS").is_some() {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, got).unwrap();
        return;
    }
    let want = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("missing golden {}: {e} (bless to create)", path.display()));
    if want != got {
        panic!(
            "snapshot {name} moved.\n--- want ---\n{want}\n--- got ---\n{got}\n\
             (if the change is intended: ASPECTUS_SNAPSHOT_BLESS=1 cargo test --test grid_snapshot)"
        );
    }
}

/// A tree that puts one of nearly every near-right part on the page: a
/// facet-less furniture claim (`[has: …]`), a dir census under the depth
/// cut, symlinks (one broken), a binary file (the quiet kind word), files
/// with line counts, and odd permissions.
fn kitchen(dir: &Path) {
    write(&dir.join("README.md"), "# Fixture\n\nprose\n");
    write(&dir.join("Cargo.toml"), "[package]\nname = \"fx\"\n");
    write(&dir.join("src/main.rs"), "fn main() {}\n");
    write(&dir.join("src/lib.rs"), "pub fn a() {}\npub fn b() {}\n");
    // A cut level: deep/ is below the depth the look asks for, so it
    // renders as a dir census with a mass tail.
    for i in 0..3 {
        write(&dir.join(format!("deep/one/two/f{i}.md")), "a\nb\nc\n");
    }
    write(&dir.join("deep/one/two/notes.txt"), "x\n");
    write(&dir.join("deep/one/other/z.md"), "z\n");
    // A binary among the text (the filekind word speaks).
    File::create(dir.join("src/blob.bin"))
        .unwrap()
        .write_all(&[0u8, 1, 2, 3, 0, 7])
        .unwrap();
    // Odd permissions for its level (the quiet perms column speaks).
    let odd = dir.join("run.sh");
    write(&odd, "#!/bin/sh\necho hi\n");
    fs::set_permissions(
        &odd,
        <fs::Permissions as std::os::unix::fs::PermissionsExt>::from_mode(0o700),
    )
    .unwrap();
    age_all(dir);
    // Symlinks last: they carry the target's mtime, and creating them does
    // not disturb the aged tree.
    unixfs::symlink("README.md", dir.join("readme-link")).unwrap();
    unixfs::symlink("nowhere.md", dir.join("dangling")).unwrap();
    // A broken link is lstat'd, so it carries its *own* mtime — freshly
    // made, that mtime is "now" and the quiet mtime column would speak a
    // different value on every run. `touch -h` ages the link itself.
    let ok = Command::new("touch")
        .args(["-h", "-t", "202009130000"])
        .arg(dir.join("dangling"))
        .env("TZ", "UTC")
        .status()
        .unwrap()
        .success();
    assert!(ok, "touch -h (aging the broken link) failed");
    // Making the links re-touched the root; age it once more so the root's
    // own facts line is stable too.
    age(dir);
}

#[test]
fn kitchen_tree_grid() {
    let (dir, xdg) = fresh("kitchen");
    kitchen(&dir);
    golden("kitchen", &run(&dir, &xdg, &["--depth", "2"]));
}

/// The same tree under a tight line budget: the leaf census (`[+ …]`)
/// stands where a name would, and the allocator's remainder shows.
#[test]
fn leaf_census_grid() {
    let (dir, xdg) = fresh("leaf");
    kitchen(&dir);
    golden(
        "leaf-census",
        &run(&dir, &xdg, &["--depth", "3", "--lines", "8"]),
    );
}

/// Depth 1: every dir is a census line, and the mass tail speaks.
#[test]
fn census_grid() {
    let (dir, xdg) = fresh("census");
    kitchen(&dir);
    golden("census", &run(&dir, &xdg, &["--depth", "1"]));
}

/// Columns asked on: the far-right block with several columns, headings
/// over them, right edges aligned.
#[test]
fn columns_on_grid() {
    let (dir, xdg) = fresh("cols");
    kitchen(&dir);
    fs::write(
        xdg.join("aspectus/aspectus.toml"),
        "columns.size = on\ncolumns.permissions = on\nformat.mtime = epoch\ncolumns.mtime = on\n",
    )
    .unwrap();
    golden("columns-on", &run(&dir, &xdg, &["--depth", "2"]));
}

fn git(dir: &Path, args: &[&str]) {
    let stamp = "1600000000 +0000";
    let ok = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .env("GIT_AUTHOR_DATE", stamp)
        .env("GIT_COMMITTER_DATE", stamp)
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

/// A real repo: the `[git: …]` facet on the parent line, the far-left
/// git-status cell (clean → blank; `⊘` lives here, not in the marks
/// column), the ignored remainder, and `[has: git, rust]`.
/// Re-blessed 2026-08-22: git-status far-left slice.
/// Re-blessed 2026-08-22: untracked-dir ⁇; stamp/facts/path skip far-left.
#[test]
fn git_repo_grid() {
    let (dir, xdg) = fresh("git");
    write(&dir.join("README.md"), "# Repo\n");
    write(&dir.join("Cargo.toml"), "[package]\n");
    write(&dir.join("src/main.rs"), "fn main() {}\n");
    write(&dir.join(".gitignore"), "target/\nsecret.txt\n");
    write(&dir.join("secret.txt"), "shh\n");
    write(&dir.join("target/debug/out.bin"), "junk\n");
    git(&dir, &["init", "-q", "-b", "main"]);
    git(&dir, &["add", "-A"]);
    git(&dir, &["commit", "-qm", "one"]);
    age_all(&dir);
    golden("git-repo", &run(&dir, &xdg, &["--depth", "2"]));
}
