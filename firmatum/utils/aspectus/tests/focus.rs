//! Focus set from several positional paths (design/focus.md §Multiple
//! paths): one look of the common ancestor, depth from each selected path,
//! unselected siblings folded to typed remainders — never cut. Real
//! binary, isolated XDG.

use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::UNIX_EPOCH;

static SEQ: AtomicU64 = AtomicU64::new(0);

fn fresh(tag: &str) -> (PathBuf, PathBuf) {
    let n = SEQ.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!(
        "aspectus-focus-{tag}-{}-{}-{}",
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

fn write(p: &Path, body: &str) {
    fs::create_dir_all(p.parent().unwrap()).unwrap();
    File::create(p).unwrap().write_all(body.as_bytes()).unwrap();
}

/// The bare root path line: stamp, maybe a facts line, then the path.
fn root_line(o: &str) -> String {
    o.lines()
        .find(|l| l.starts_with('/'))
        .unwrap_or_default()
        .trim_end()
        .to_string()
}

fn run(dir: &Path, xdg: &Path, args: &[&str]) -> (i32, String, String) {
    let mut c = Command::new(env!("CARGO_BIN_EXE_aspectus"));
    c.args(args).current_dir(dir).env("XDG_CONFIG_HOME", xdg);
    for k in [
        "ASPECTUS_LINES",
        "ASPECTUS_DEPTH",
        "ASPECTUS_SORT",
        "ASPECTUS_IMPORTANT",
        "ASPECTUS_GLOBIFY",
        "ASPECTUS_COLUMNS_HEAT",
    ] {
        c.env_remove(k);
    }
    let out = c.output().unwrap();
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

/// Joseph's ask in miniature: four sibling volumes under one parent, each
/// four generations deep, with unselected siblings beside them.
fn volumes() -> (PathBuf, PathBuf) {
    let (dir, xdg) = fresh("vol");
    let asf = dir.join("asf");
    for v in ["01-aat", "02-tst", "03-llm", "04-eli"] {
        write(&asf.join(v).join("src/g1/g2/g3/deep.md"), "deep\n");
        write(&asf.join(v).join("OUTLINE.md"), "outline\n");
    }
    for other in ["audits", "doc", "msc"] {
        write(&asf.join(other).join("note.md"), "x\n");
        write(&asf.join(other).join("sub/more.md"), "x\n");
    }
    write(&asf.join("TOP.md"), "top\n");
    (dir, xdg)
}

/// The whole ask: several paths make one look of their common ancestor,
/// and `--depth` is spent under each selected path, not from the root.
#[test]
fn several_paths_are_one_look_with_depth_from_each() {
    let (dir, xdg) = volumes();
    let (code, o, _) = run(
        &dir,
        &xdg,
        &[
            "--lines", "0", "--depth", "3",
            "asf/01-aat", "asf/02-tst", "asf/03-llm", "asf/04-eli",
        ],
    );
    assert_eq!(code, 0, "{o}");
    // Root is the common ancestor, on its own bare line.
    assert!(root_line(&o).ends_with("/asf/"), "root is the ancestor: {o}");
    for v in ["01-aat/", "02-tst/", "03-llm/", "04-eli/"] {
        assert!(o.contains(v), "selected {v} listed: {o}");
    }
    // Three generations under each selected path — src/g1/g2 shows, and
    // g2's contents are the cutoff census, not a line.
    assert!(o.contains("── g2/"), "depth counts from the selected path: {o}");
    assert!(!o.contains("── g3/"), "depth 3 stops under each selected: {o}");
}

/// Depth from the *root* would have shown one generation less; this is the
/// difference the decision is about (design: counting from the root makes
/// the ask unexpressible).
#[test]
fn the_connective_chain_spends_no_depth() {
    let (dir, xdg) = fresh("chain");
    write(&dir.join("a/b/one/x/y/z.md"), "z\n");
    write(&dir.join("a/b/two/x/y/z.md"), "z\n");
    write(&dir.join("a/other.md"), "o\n");
    let (code, o, _) = run(&dir, &xdg, &["--lines", "0", "--depth", "2", "a/b/one", "a/b/two"]);
    assert_eq!(code, 0, "{o}");
    // Root a/b, chain spends nothing, two generations under each selected.
    assert!(root_line(&o).ends_with("/a/b/"), "root is the ancestor: {o}");
    assert!(o.contains("── x/"), "{o}");
    assert!(o.contains("── y/"), "{o}");
    assert!(!o.contains("── z.md"), "depth 2 stops below y/: {o}");
}

/// Unselected siblings are compressed, never cut: one typed remainder line
/// per connective level, carrying the deep file count.
#[test]
fn unselected_siblings_fold_to_one_typed_remainder() {
    let (dir, xdg) = volumes();
    let (_, o, _) = run(&dir, &xdg, &["--lines", "0", "--depth", "1", "asf/01-aat", "asf/02-tst"]);
    let rem: Vec<&str> = o.lines().filter(|l| l.contains("[+ ")).collect();
    assert_eq!(rem.len(), 1, "exactly one remainder line: {o}");
    let rem = rem[0];
    // Two unselected volumes + audits/ doc/ msc/ — every one of them
    // present in the remainder, typed, not one of them dropped.
    assert!(rem.contains("dir×5"), "the five unselected dirs are typed: {rem}");
    assert!(rem.contains("f"), "with their deep file count: {rem}");
    assert!(rem.contains("md×1"), "and TOP.md by suffix: {rem}");
    // Level membership is honest: nothing at the ancestor's level vanished.
    assert!(!o.contains("audits/"), "folded, not listed: {o}");
}

/// The caller's explicit ask outranks every standing default: under a
/// budget too tight for four, the selected are what survives — and what
/// could not fit is confessed, not silently cut.
#[test]
fn selected_paths_take_the_top_survival_tier() {
    let (dir, xdg) = volumes();
    let (_, o, _) = run(&dir, &xdg, &["--lines", "7", "--depth", "1", "asf/01-aat", "asf/04-eli"]);
    assert!(o.contains("01-aat/") && o.contains("04-eli/"), "{o}");
    let (_, tight, err) = run(&dir, &xdg, &["--lines", "5", "--depth", "1", "asf/01-aat", "asf/04-eli"]);
    assert!(
        err.contains("could not give every focus path a line"),
        "a leftover match is typed, never silent: {err}"
    );
    assert!(tight.lines().count() <= 5, "budget honored: {tight}");
}

/// `--lines` is a promise: a folded remainder costs a line of the budget
/// like any other line.
#[test]
fn the_line_budget_still_binds() {
    let (dir, xdg) = volumes();
    for n in ["8", "12", "20"] {
        let (_, o, _) = run(&dir, &xdg, &["--lines", n, "--depth", "4", "asf/01-aat", "asf/02-tst"]);
        assert!(
            o.lines().count() <= n.parse::<usize>().unwrap(),
            "--lines {n} honored: {o}"
        );
    }
}

/// A path that is not there is confessed and dropped; the rest of the ask
/// is still a serviceable look, exit 0.
#[test]
fn a_missing_focus_path_is_confessed_not_fatal() {
    let (dir, xdg) = volumes();
    let (code, o, err) = run(
        &dir,
        &xdg,
        &["--lines", "0", "--depth", "1", "asf/01-aat", "asf/nope", "asf/02-tst"],
    );
    assert_eq!(code, 0, "the look succeeded: {err}");
    assert!(err.contains("not found"), "confessed: {err}");
    assert!(err.contains("nope"), "by name: {err}");
    assert!(o.contains("01-aat/") && o.contains("02-tst/"), "{o}");
}

/// Every named path missing: there is no place to look, so it refuses like
/// a single bad path does.
#[test]
fn all_paths_missing_refuses() {
    let (dir, xdg) = volumes();
    let (code, _, err) = run(&dir, &xdg, &["asf/nope", "asf/gone"]);
    assert_eq!(code, 2, "{err}");
    assert!(err.contains("not found"), "{err}");
}

/// A path inside another named path is dropped with a word about why —
/// depth counts from the outer one, so the inner ask is already served.
#[test]
fn a_nested_selection_collapses_to_the_outer_one() {
    let (dir, xdg) = volumes();
    let (code, o, err) = run(
        &dir,
        &xdg,
        &["--lines", "0", "--depth", "2", "asf/01-aat", "asf/01-aat/src"],
    );
    assert_eq!(code, 0, "{err}");
    assert!(err.contains("already inside another"), "{err}");
    // One survivor: today's ordinary look, at that path.
    assert!(root_line(&o).ends_with("/asf/01-aat/"), "{o}");
}

/// One path is today's behavior exactly — the arity is the whole rule.
#[test]
fn one_path_is_unchanged() {
    let (dir, xdg) = volumes();
    let (_, one, _) = run(&dir, &xdg, &["--lines", "0", "--depth", "2", "asf/01-aat"]);
    let (_, twice, _) = run(&dir, &xdg, &["--lines", "0", "--depth", "2", "asf/01-aat", "asf/01-aat"]);
    let strip = |s: &str| s.lines().skip(1).collect::<Vec<_>>().join("\n");
    assert_eq!(strip(&one), strip(&twice), "a repeated path is one path");
}

/// Same tree, same ask: byte-identical (the determinism claim).
#[test]
fn determinism() {
    let (dir, xdg) = volumes();
    let args = ["--lines", "30", "--depth", "3", "asf/01-aat", "asf/02-tst", "asf/03-llm"];
    let (_, a, _) = run(&dir, &xdg, &args);
    let (_, b, _) = run(&dir, &xdg, &args);
    assert_eq!(
        a.lines().skip(1).collect::<Vec<_>>(),
        b.lines().skip(1).collect::<Vec<_>>()
    );
}

/// JSON: `root` is the ancestor, `matched` marks the selected nodes, and
/// the folded siblings ride in `omitted` — the schema's shape is unchanged.
#[test]
fn json_names_the_root_and_marks_the_matches() {
    let (dir, xdg) = volumes();
    let (code, o, _) = run(
        &dir,
        &xdg,
        &["--format", "json", "--lines", "0", "--depth", "1", "asf/01-aat", "asf/02-tst"],
    );
    assert_eq!(code, 0, "{o}");
    assert!(o.contains("/asf\","), "root is the ancestor: {o}");
    assert_eq!(o.matches("\"matched\":true").count(), 2, "{o}");
    assert!(o.contains("\"omitted\""), "the folded siblings are data too: {o}");
}

/// The help page teaches the arity rule (help is the law channel).
#[test]
fn help_teaches_several_paths() {
    let (dir, xdg) = volumes();
    let (_, o, _) = run(&dir, &xdg, &["help"]);
    assert!(o.contains("usage: aspectus [PATH ...]"), "{o}");
    assert!(o.contains("Several paths are one look"), "{o}");
    assert!(o.contains("01-aat-core"), "the brace-expansion example: {o}");
}
