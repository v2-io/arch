//! Bounded walk: never enter absorb; ≥ on visit stop; cycle guard; -x.

use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use aspectus::{same_filesystem, walk, WalkOptions};

fn fresh_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "aspectus-{}-{}-{}",
        tag,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn write_file(p: &Path, body: &str) {
    if let Some(parent) = p.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    let mut f = File::create(p).unwrap();
    f.write_all(body.as_bytes()).unwrap();
}

fn absorb_tree() -> PathBuf {
    let root = fresh_dir("absorb");
    write_file(&root.join(".git/objects/pack/canary"), "SECRET");
    write_file(&root.join(".git/hooks/applypatch-msg.sample"), "hook");
    write_file(&root.join(".git/COMMIT_EDITMSG"), "msg");
    write_file(&root.join("target/debug/foo"), "bin");
    write_file(&root.join("src/main.rs"), "fn main() {}");
    write_file(&root.join("Cargo.toml"), "[package]\nname=\"x\"\n");
    write_file(&root.join(".archive/old.md"), "old");
    write_file(&root.join(".orient/note"), "n");
    write_file(&root.join(".DS_Store"), "junk");
    write_file(&root.join(".mystery"), "hidden");
    root
}

#[test]
fn walk_does_not_open_git_or_target() {
    let root = absorb_tree();
    let result = walk(&root, WalkOptions::default()).unwrap();
    let opened = &result.stats.dirs_opened;
    assert!(
        opened.iter().all(|p| {
            let rel = p.strip_prefix(&root).unwrap_or(p);
            let s = rel.to_string_lossy();
            !s.starts_with(".git") && !s.starts_with("target")
        }),
        "opened absorb trees: {opened:?}"
    );
    let names: Vec<&str> = result.aspecta.node.children.iter().map(|c| c.name.as_str()).collect();
    assert!(!names.contains(&".git"), "children={names:?}");
    assert!(!names.contains(&"target"), "children={names:?}");
    assert!(names.contains(&"src"), "children={names:?}");
    assert!(names.contains(&".archive"), "children={names:?}");
    assert!(names.contains(&".orient"), "children={names:?}");
    assert!(names.contains(&".mystery"), "children={names:?}");
    assert!(!names.contains(&".DS_Store"), "children={names:?}");
    assert!(result.aspecta.node.annotations.kinds.iter().any(|k| k == "git"));
}

#[test]
fn visit_budget_stop_is_truncated_with_omitted_names() {
    let root = fresh_dir("visit");
    for i in 0..20 {
        write_file(&root.join(format!("f{i}.txt")), "x");
    }
    let mut opts = WalkOptions::default();
    opts.visit_budget = 3;
    let result = walk(&root, opts).unwrap();
    assert!(result.aspecta.node.annotations.truncated);
    assert!(
        !result.aspecta.node.annotations.omitted.is_empty(),
        "truncated walk must name what it did not take"
    );
}

#[test]
fn symlink_dir_cycle_does_not_loop() {
    let root = fresh_dir("cycle");
    fs::create_dir(root.join("a")).unwrap();
    fs::create_dir(root.join("b")).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        symlink(root.join("b"), root.join("a/to_b")).unwrap();
        symlink(root.join("a"), root.join("b/to_a")).unwrap();
        let result = walk(&root, WalkOptions::default()).unwrap();
        // Finite: we got a tree back.
        assert_eq!(result.aspecta.node.name, root.file_name().unwrap().to_string_lossy());
        let text = aspectus::render_text(&result.aspecta.node, 40);
        assert!(text.contains("a/"), "{text}");
    }
}

#[test]
fn same_filesystem_predicate_is_the_walks() {
    let root = fresh_dir("fs");
    write_file(&root.join("a"), "a");
    let ma = fs::metadata(&root).unwrap();
    let mb = fs::metadata(root.join("a")).unwrap();
    assert!(same_filesystem(&ma, &mb));
    #[cfg(unix)]
    {
        if let Ok(dev) = fs::metadata("/dev/null") {
            // Different device is possible; the walk calls this same function.
            let _ = same_filesystem(&ma, &dev);
        }
    }
}

#[test]
fn walk_render_names_src_not_target_as_child() {
    let root = absorb_tree();
    let result = walk(&root, WalkOptions::default()).unwrap();
    let text = aspectus::render_text(&result.aspecta.node, 80);
    assert!(text.contains("src/"), "{text}");
    assert!(
        !text.lines().any(|l| l.contains("target/") && !l.contains('[')),
        "target/ must not be a listed child:\n{text}"
    );
}

#[cfg(unix)]
#[test]
fn one_filesystem_does_not_descend_other_device() {
    let root = fresh_dir("xdev");
    write_file(&root.join("keep.txt"), "k");
    let here = fs::metadata(&root).unwrap();
    let there = match fs::metadata("/dev") {
        Ok(m) => m,
        Err(_) => return,
    };
    if same_filesystem(&here, &there) {
        return;
    }
    std::os::unix::fs::symlink("/dev", root.join("other_fs")).unwrap();
    let result = walk(&root, WalkOptions::default()).unwrap();
    assert!(
        result.stats.dirs_opened.iter().all(|p| !p.starts_with("/dev")),
        "opened /dev: {:?}",
        result.stats.dirs_opened
    );
}

#[test]
fn raw_is_the_only_way_into_git() {
    let root = absorb_tree();
    let def = walk(&root, WalkOptions::default()).unwrap();
    assert!(def.aspecta.node.children.iter().all(|c| c.name != ".git"));

    let mut raw = WalkOptions::default();
    raw.inspect = Some("*".into());
    let opened = walk(&root, raw).unwrap();
    assert!(
        opened.aspecta.node.children.iter().any(|c| c.name == ".git"),
        " --show-all must list .git as a child"
    );
    assert!(
        opened.stats.dirs_opened.iter().any(|p| p.ends_with(".git")),
        " --show-all must list .git"
    );
}
