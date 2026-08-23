//! Git furniture: .git is not a child; the parent line says what git is
//! here. Real binary; real `git` for the fixtures (skips if absent).

use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static SEQ: AtomicU64 = AtomicU64::new(0);

fn have_git() -> bool {
    Command::new("git")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn tmp(tag: &str) -> PathBuf {
    let n = SEQ.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!("aspectus-git-{tag}-{}-{}", std::process::id(), n));
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn git(dir: &Path, args: &[&str]) {
    let ok = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
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

fn repo(tag: &str) -> PathBuf {
    let dir = tmp(tag);
    git(&dir, &["init", "-q", "-b", "main"]);
    File::create(dir.join("a.txt"))
        .unwrap()
        .write_all(b"a")
        .unwrap();
    git(&dir, &["add", "."]);
    git(&dir, &["commit", "-q", "-m", "one"]);
    git(
        &dir,
        &[
            "remote",
            "add",
            "origin",
            "git@github.com:v2-io/example.git",
        ],
    );
    dir
}

fn run(dir: &Path, args: &[&str]) -> (i32, String, String) {
    let xdg = tmp("xdg");
    fs::create_dir_all(xdg.join("aspectus")).unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_aspectus"))
        .args(args)
        .current_dir(dir)
        .env("XDG_CONFIG_HOME", &xdg)
        .env_remove("ASPECTUS_FURNITURE")
        .output()
        .unwrap();
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

#[test]
fn work_tree_facts_on_the_parent_line() {
    if !have_git() {
        return;
    }
    let dir = repo("wt");
    let (c, o, e) = run(&dir, &[]);
    assert_eq!(c, 0, "{e}");
    assert!(!o.contains(".git/"), ".git is furniture: {o}");
    // Root is the second line: the stamp has its own line above it.
    let root = o.lines().nth(1).unwrap();
    assert!(root.contains("[git: "), "{o}");
    assert!(root.contains("remote<github.com/v2-io/example>"), "{o}");
    assert!(root.contains("br<main>"), "{o}");
    assert!(root.contains('@'), "short HEAD: {o}");
    assert!(!root.contains("dirty"), "clean prints no dirt: {o}");
    assert!(root.contains("[has: git]"), "{o}");
}

#[test]
fn porcelain_dirt_only_when_dirty() {
    if !have_git() {
        return;
    }
    let dir = repo("dirty");
    File::create(dir.join("b.txt"))
        .unwrap()
        .write_all(b"b")
        .unwrap();
    fs::write(dir.join("a.txt"), "changed").unwrap();
    let (_, o, _) = run(&dir, &[]);
    assert!(o.contains("dirty<2>"), "{o}");
}

#[test]
fn nested_repo_carries_its_own_facts() {
    if !have_git() {
        return;
    }
    let outer = tmp("outer");
    let inner = outer.join("inner");
    fs::create_dir_all(&inner).unwrap();
    git(&inner, &["init", "-q", "-b", "work"]);
    File::create(inner.join("x.txt"))
        .unwrap()
        .write_all(b"x")
        .unwrap();
    git(&inner, &["add", "."]);
    git(&inner, &["commit", "-q", "-m", "one"]);
    let (_, o, _) = run(&outer, &[]);
    let line = o.lines().find(|l| l.contains("inner/")).unwrap();
    assert!(line.contains("br<work>"), "{o}");
    assert!(!o.contains(".git/"), "{o}");
}

#[test]
fn submodule_gitlink_is_the_same_furniture() {
    if !have_git() {
        return;
    }
    // Emulate the submodule layout: a `.git` *file* pointing at a gitdir
    // under the superproject's modules dir (what `git submodule` writes).
    let sup = repo("sup");
    let sub = sup.join("sub");
    fs::create_dir_all(&sub).unwrap();
    git(&sub, &["init", "-q", "-b", "subbr"]);
    File::create(sub.join("s.txt"))
        .unwrap()
        .write_all(b"s")
        .unwrap();
    git(&sub, &["add", "."]);
    git(&sub, &["commit", "-q", "-m", "one"]);
    let modules = sup.join(".git/modules");
    fs::create_dir_all(&modules).unwrap();
    fs::rename(sub.join(".git"), modules.join("sub")).unwrap();
    fs::write(sub.join(".git"), "gitdir: ../.git/modules/sub\n").unwrap();
    fs::write(modules.join("sub/core-noop"), "").unwrap();
    let (_, o, _) = run(&sup, &[]);
    let line = o.lines().find(|l| l.contains("sub/")).unwrap();
    assert!(
        line.contains("br<subbr>"),
        "gitlink read like a work tree: {o}"
    );
    assert!(!o.contains(".git/"), "{o}");
}

#[test]
fn inspect_git_lists_dot_git() {
    if !have_git() {
        return;
    }
    let dir = repo("inspect");
    let (_, o, _) = run(&dir, &["--inspect", "git", "--depth", "1"]);
    assert!(o.contains(".git/"), "{o}");
}

/// Far-left git-status: worktree letter on files, ⁇ on untracked files
/// *and* untracked directories (`?? dir/`), blank on tracked-and-clean
/// dirs, positional blank on clean rows. Stamp / root-facts / root-path
/// do not pay the cell; headings and tree rows do.
#[test]
fn far_left_git_status_letters() {
    if !have_git() {
        return;
    }
    let dir = repo("letters");
    fs::create_dir(dir.join("kept")).unwrap();
    fs::write(dir.join("kept/y.txt"), "y").unwrap();
    git(&dir, &["add", "."]);
    git(&dir, &["commit", "-q", "-m", "two"]);
    fs::write(dir.join("a.txt"), "changed").unwrap();
    File::create(dir.join("new.txt"))
        .unwrap()
        .write_all(b"n")
        .unwrap();
    fs::create_dir(dir.join("subdir")).unwrap();
    fs::write(dir.join("subdir/x.txt"), "x").unwrap();
    let (_, o, _) = run(&dir, &["--depth", "2", "--sort", "name"]);
    let file = |name: &str| {
        o.lines()
            .find(|l| l.contains(name) && !l.contains("dirty"))
            .unwrap_or_else(|| panic!("missing {name} in {o}"))
            .to_string()
    };
    let a = file("a.txt");
    assert!(
        a.split_once("a.txt").unwrap().0.contains('M'),
        "modified is M, far-left: {a}"
    );
    let n = file("new.txt");
    assert!(
        n.split_once("new.txt").unwrap().0.contains('\u{2047}'),
        "untracked is ⁇: {n}"
    );
    let sub = o.lines().find(|l| l.contains("subdir/")).expect(&o);
    let before = sub.split_once("subdir/").unwrap().0;
    assert!(
        before.contains('\u{2047}'),
        "untracked dir (?? dir/) is ⁇: {sub}"
    );
    let kept = o.lines().find(|l| l.contains("kept/")).expect(&o);
    let kept_before = kept.split_once("kept/").unwrap().0;
    assert!(
        !kept_before.contains('\u{2047}') && !kept_before.contains('M'),
        "tracked-and-clean dir stays blank: {kept}"
    );
    let stamp = o.lines().next().expect(&o);
    assert!(
        stamp.ends_with('Z') && !stamp.starts_with(' '),
        "stamp does not pay far-left: {stamp:?}"
    );
    let path = o
        .lines()
        .find(|l| l.starts_with('/') && l.ends_with('/'))
        .expect(&o);
    assert!(
        path.starts_with('/') && !path.starts_with(" /"),
        "root path does not pay far-left: {path:?}"
    );
    let facts = o.lines().nth(1).expect(&o);
    assert!(
        !facts.starts_with(' '),
        "root-facts does not pay far-left: {facts:?}"
    );
    let tree = o
        .lines()
        .find(|l| l.contains("├── ") || l.contains("└── "))
        .expect(&o);
    assert!(
        tree.starts_with(' ')
            || tree.starts_with('M')
            || tree.starts_with('\u{2047}')
            || tree.starts_with('\u{2298}'),
        "far-left cell is positional, width 1: {tree}"
    );
    let headings = o
        .lines()
        .find(|l| l.trim_start().starts_with("lines") && !l.contains('/'))
        .expect(&o);
    assert!(
        headings.starts_with(' '),
        "headings line keeps the far-left cell: {headings:?}"
    );
}

/// A look at a non-repo that *contains* a repo still pays the far-left
/// cell on tree rows (look-wide presence). A look with no repo at all
/// does not — width unchanged.
#[test]
fn far_left_absent_outside_any_repo() {
    if !have_git() {
        return;
    }
    let outer = tmp("plain");
    File::create(outer.join("plain.txt"))
        .unwrap()
        .write_all(b"p")
        .unwrap();
    let (_, o, _) = run(&outer, &["--depth", "1"]);
    let line = o.lines().find(|l| l.contains("plain.txt")).expect(&o);
    assert!(line.contains("├── ") || line.contains("└── "), "{line}");
    let tree_at = line.find("── ").expect(line);
    // No far-left cell: the branch glyph sits at column 0.
    assert!(
        line.starts_with('├') || line.starts_with('└'),
        "no repo → no far-left block: {line} (tree at {tree_at})"
    );
}

/// `[layout] far-left` without git-status turns the cell off even in a repo.
#[test]
fn layout_list_turns_git_status_off() {
    if !have_git() {
        return;
    }
    let dir = repo("off");
    fs::write(dir.join("a.txt"), "changed").unwrap();
    let xdg = tmp("xdg-off");
    fs::create_dir_all(xdg.join("aspectus")).unwrap();
    fs::write(xdg.join("aspectus/aspectus.toml"), "layout.far-left = []\n").unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_aspectus"))
        .args(["--depth", "1"])
        .current_dir(&dir)
        .env("XDG_CONFIG_HOME", &xdg)
        .output()
        .unwrap();
    let o = String::from_utf8_lossy(&out.stdout);
    let line = o.lines().find(|l| l.contains("a.txt")).expect(&o);
    assert!(
        !line.split_once("a.txt").unwrap().0.contains('M'),
        "empty far-left list paints no letter: {line}"
    );
    assert!(
        line.starts_with('├') || line.starts_with('└'),
        "no far-left block when git-status is not listed: {line}"
    );
}
