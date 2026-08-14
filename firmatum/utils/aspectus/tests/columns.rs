//! Columns: selection through the caller stack, formats, refusals,
//! alignment, the inventory surface. Real binary. Isolated XDG.

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
        "aspectus-col-{tag}-{}-{}-{}",
        std::process::id(),
        n,
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&dir).unwrap();
    // Outside the looked-at tree, or it would join the look.
    let xdg = PathBuf::from(format!("{}-xdg", dir.display()));
    fs::create_dir_all(xdg.join("aspectus")).unwrap();
    (dir, xdg)
}

fn fixture() -> (PathBuf, PathBuf) {
    let (dir, xdg) = fresh("fx");
    File::create(dir.join("a.md"))
        .unwrap()
        .write_all(&[b'x'; 5])
        .unwrap();
    File::create(dir.join("big.md"))
        .unwrap()
        .write_all(&vec![b'x'; 2048])
        .unwrap();
    fs::create_dir_all(dir.join("sub")).unwrap();
    // Old mtimes: fresh files would surprise the quiet mtime law (recent
    // vs now) — this fixture is about selection, not surprise.
    for n in ["a.md", "big.md", "sub"] {
        let f = File::open(dir.join(n)).unwrap();
        f.set_modified(std::time::UNIX_EPOCH + std::time::Duration::from_secs(1_700_000_000))
            .unwrap();
    }
    (dir, xdg)
}

fn run(dir: &Path, xdg: &Path, envs: &[(&str, &str)], args: &[&str]) -> (i32, String, String) {
    let mut c = bin();
    c.args(args)
        .current_dir(dir)
        .env("XDG_CONFIG_HOME", xdg)
        .env_remove("ASPECTUS_LINES")
        .env_remove("ASPECTUS_DEPTH")
        .env_remove("ASPECTUS_SORT")
        .env_remove("ASPECTUS_COLUMNS_SIZE")
        .env_remove("ASPECTUS_COLUMNS_MTIME")
        .env_remove("ASPECTUS_FORMAT_SIZE")
        .env_remove("ASPECTUS_FORMAT_MTIME");
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

/// Subfeature 1: with no config, quiet facts render nothing.
#[test]
fn defaults_show_no_size_or_mtime() {
    let (dir, xdg) = fixture();
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    let a_line = o.lines().find(|l| l.contains("a.md")).unwrap();
    assert!(!a_line.contains("5B"), "size is quiet by default: {a_line}");
    // The stamp is the only ISO time in the look.
    assert_eq!(o.matches('Z').count(), 1, "mtime is quiet by default: {o}");
}

/// Subfeature 2: config turns a fact on; a higher layer (env) wins over it.
#[test]
fn config_turns_size_on_env_overrides() {
    let (dir, xdg) = fixture();
    user_home(&xdg, "columns.size = on\n");
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("5B") && o.contains("2.0K"), "human sizes: {o}");
    let (c, o, _) = run(
        &dir,
        &xdg,
        &[("ASPECTUS_COLUMNS_SIZE", "off")],
        &["--depth", "1"],
    );
    assert_eq!(c, 0);
    assert!(!o.contains("5B"), "env layer outranks user-home: {o}");
}

/// Subfeature 4: format override through the same stack.
#[test]
fn format_override_bytes() {
    let (dir, xdg) = fixture();
    user_home(&xdg, "columns.size = on\nformat.size = bytes\n");
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("2048"), "bytes format: {o}");
    assert!(!o.contains("2.0K"), "{o}");
}

#[test]
fn mtime_epoch_format() {
    let (dir, xdg) = fixture();
    user_home(&xdg, "columns.mtime = on\nformat.mtime = epoch\n");
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    // Epoch seconds on the a.md line, no second ISO stamp in the look.
    assert_eq!(o.matches('Z').count(), 1, "{o}");
    let a_line = o.lines().find(|l| l.contains("a.md")).unwrap();
    assert!(
        a_line.split_whitespace().any(|w| w.len() >= 10 && w.chars().all(|c| c.is_ascii_digit())),
        "epoch seconds: {a_line}"
    );
}

#[test]
fn unknown_format_refused_naming_options() {
    let (dir, xdg) = fixture();
    user_home(&xdg, "columns.size = on\nformat.size = log\n");
    let (c, _, e) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert_eq!(c, 2);
    assert!(e.contains("human") && e.contains("bytes"), "menu: {e}");
}

/// Subfeature 3: a fact gets no flag of its own; the refusal names the
/// config path. Unbuilt facts refused honestly too.
#[test]
fn fact_flags_refused_with_config_path() {
    let (dir, xdg) = fixture();
    let (c, _, e) = run(&dir, &xdg, &[], &["--size"]);
    assert_eq!(c, 2);
    assert!(e.contains("columns.size"), "refusal names the ask: {e}");
    let (c, _, e) = run(&dir, &xdg, &[], &["--owner"]);
    assert_eq!(c, 2);
    assert!(e.contains("columns.owner"), "built since Wave D; ask named: {e}");
    let (c, _, e) = run(&dir, &xdg, &[], &["--linkcount"]);
    assert_eq!(c, 2);
    assert!(
        e.contains("columns.linkcount") && e.contains("not built"),
        "unbuilt named: {e}"
    );
}

/// Subfeature 5: a wide line is one line — columns never change the count.
#[test]
fn columns_do_not_change_line_count() {
    let (dir, xdg) = fixture();
    let (_, plain, _) = run(&dir, &xdg, &[], &["--depth", "1", "--lines", "20"]);
    user_home(&xdg, "columns.size = on\ncolumns.mtime = on\n");
    let (_, wide, _) = run(&dir, &xdg, &[], &["--depth", "1", "--lines", "20"]);
    // The *tree* keeps its shape; the header may gain the root-facts line
    // when a turned-on fact gives the root something to say (simple-header
    // decision, 2026-08-14).
    let tree_lines = |s: &str| {
        s.lines()
            .skip_while(|l| !l.starts_with('├') && !l.starts_with('└'))
            .count()
    };
    assert_eq!(
        tree_lines(&plain),
        tree_lines(&wide),
        "same tree shape, wider lines:\n{plain}\n----\n{wide}"
    );
}

/// Subfeature 6: byte-identical under different terminal-width claims.
#[test]
fn determinism_ignores_terminal_width() {
    let (dir, xdg) = fixture();
    user_home(&xdg, "columns.size = on\ncolumns.mtime = on\n");
    let (_, o1, _) = run(&dir, &xdg, &[("COLUMNS", "40")], &["--depth", "1"]);
    let (_, o2, _) = run(&dir, &xdg, &[("COLUMNS", "200")], &["--depth", "1"]);
    assert_eq!(
        o1.lines().skip(1).collect::<Vec<_>>(),
        o2.lines().skip(1).collect::<Vec<_>>(),
        "width never shapes the look"
    );
}

/// Column values align: both sizes end at the same column (right edge).
#[test]
fn size_column_right_aligned() {
    let (dir, xdg) = fixture();
    user_home(&xdg, "columns.size = on\n");
    let (_, o, _) = run(&dir, &xdg, &[], &["--depth", "1"]);
    let end = |name: &str| {
        let l = o.lines().find(|l| l.contains(name)).unwrap();
        let v = if name == "a.md" { "5B" } else { "2.0K" };
        l.find(v).unwrap() + v.len()
    };
    assert_eq!(end("a.md"), end("big.md"), "right edges align: {o}");
}

/// Subfeature 7: the inventory surface lists every lattice fact the
/// machinery knows, with state and ask.
#[test]
fn config_shows_fact_inventory() {
    let (dir, xdg) = fixture();
    let (c, o, e) = run(&dir, &xdg, &[], &["config"]);
    assert_eq!(c, 0, "{e}");
    for key in [
        "name", "child-count", "mtime", "size", "filetype", "kind",
        "symlink-target", "denied", "walk-marks", "heat", "owner",
    ] {
        assert!(o.contains(key), "inventory lists {key}: {o}");
    }
    assert!(o.contains("columns.size"), "the ask is taught: {o}");
    assert!(o.contains("unbuilt"), "unbuilt facts named honestly: {o}");
    // State reflects the stack.
    user_home(&xdg, "columns.size = on\n");
    let (_, o, _) = run(&dir, &xdg, &[], &["config"]);
    let size_row = o.lines().find(|l| l.trim_start().starts_with("size")).unwrap();
    assert!(size_row.contains(" on"), "state is current, not default: {size_row}");
}

/// The column-headings line (design/columns.md §Column headings): dimmed
/// (plain when uncolored), right-aligned over the fact columns, above the
/// first child; absent when no fact column renders; budget-true.
#[test]
fn headings_line_names_the_columns() {
    let (dir, xdg) = fixture();
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    let lines: Vec<&str> = o.lines().collect();
    let h = lines.iter().position(|l| l.trim_start().starts_with("lines")).unwrap();
    assert!(lines[h + 1].starts_with('├') || lines[h + 1].starts_with('└'),
        "headings sit directly above the children: {o}");
    // Right edge of the heading aligns with the column's values (right-
    // aligned like them): a.md is one unterminated line -> value "1".
    // Char positions, not byte offsets — the tree glyphs are multibyte.
    let chars_to = |l: &str, byte: usize| l[..byte].chars().count();
    let head_end = chars_to(lines[h], lines[h].find("lines").unwrap()) + "lines".len();
    let a_line = lines.iter().find(|l| l.contains("a.md")).unwrap();
    let val_end = chars_to(a_line, a_line.rfind('1').unwrap()) + 1;
    assert_eq!(head_end, val_end, "heading right-aligns over its column: {o}");
}

/// No fact columns ⇒ no headings line (an all-off look stays bare).
#[test]
fn headings_absent_without_columns() {
    let (dir, xdg) = fixture();
    user_home(&xdg, "columns.line-count = off\ncolumns.heat = off\n");
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    assert!(!o.contains("lines"), "no headings without columns: {o}");
}

/// The headings line is charged to --lines: total output never exceeds it.
#[test]
fn headings_charge_the_budget() {
    let (dir, xdg) = fixture();
    for n in 4..8 {
        let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1", "--lines", &n.to_string()]);
        assert_eq!(c, 0, "{e}");
        assert!(o.lines().count() <= n, "--lines {n} holds: {o}");
    }
}

/// A file argument has no headings line below, so its header facts carry
/// their column words (`N lines`) instead of bare numbers.
#[test]
fn file_root_facts_are_labeled() {
    let (dir, xdg) = fixture();
    let (c, o, e) = run(&dir, &xdg, &[], &["a.md"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("1 lines"), "labeled line count on the facts line: {o}");
}

/// mtime's default text form is relative — one register with heat's age —
/// and a quiet mtime stays silent where the heat cluster already says it.
#[test]
fn mtime_defaults_to_relative_age() {
    let (dir, xdg) = fixture();
    user_home(&xdg, "columns.mtime = on\n");
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    assert_eq!(o.matches('Z').count(), 1, "stamp is the only ISO time: {o}");
    let a_line = o.lines().find(|l| l.contains("a.md")).unwrap();
    assert!(a_line.contains("ago"), "relative age spelled: {a_line}");
}

/// Symlink target is decoration on the name (lattice INFO ON; shorthand
/// decoration class); broken says so.
#[test]
fn symlink_target_decorates_the_name() {
    let (dir, xdg) = fresh("ln");
    File::create(dir.join("real.md")).unwrap();
    std::os::unix::fs::symlink("real.md", dir.join("here")).unwrap();
    std::os::unix::fs::symlink("gone.md", dir.join("dangling")).unwrap();
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("here -> real.md"), "{o}");
    assert!(o.contains("dangling -> gone.md [broken]"), "{o}");
}
