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
    // 2026-08-22 count-cell slice: human size is no longer `5B`.
    assert!(
        !a_line.contains("5B") && !a_line.contains("5."),
        "size is quiet by default: {a_line}"
    );
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
    // 2026-08-22 count-cell slice: scale at ≥10,000, so 5 B and 2048 B are
    // exact (`5.`, `2·048.`) not `5B` / `2.0K`. Unit blanked under `bytes`.
    assert!(
        o.contains("5.") && o.contains("2·048."),
        "count-cell sizes: {o}"
    );
    assert!(
        !o.contains("5B") && !o.contains("2.0K"),
        "old human_size retired: {o}"
    );
    let (c, o, _) = run(
        &dir,
        &xdg,
        &[("ASPECTUS_COLUMNS_SIZE", "off")],
        &["--depth", "1"],
    );
    assert_eq!(c, 0);
    assert!(
        !o.contains("5.") && !o.contains("2·048."),
        "env layer outranks user-home: {o}"
    );
}

/// Subfeature 4: format override through the same stack.
#[test]
fn format_override_bytes() {
    let (dir, xdg) = fixture();
    user_home(&xdg, "columns.size = on\nformat.size = bytes\n");
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("2048"), "bytes format: {o}");
    // 2026-08-22 count-cell slice: format.size = bytes stays the raw integer.
    assert!(!o.contains("2.0K") && !o.contains("2·048."), "{o}");
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
        a_line
            .split_whitespace()
            .any(|w| w.len() >= 10 && w.chars().all(|c| c.is_ascii_digit())),
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
    assert!(
        e.contains("columns.owner"),
        "built since Wave D; ask named: {e}"
    );
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
    // 2026-08-22 count-cell slice: both exact, unit blanked; the `.` is
    // the right edge after trim. Char positions — `·` is two UTF-8 bytes.
    let end = |name: &str| {
        let l = o.lines().find(|l| l.contains(name)).unwrap();
        l.char_indices()
            .rev()
            .find(|(_, c)| *c == '.')
            .map(|(i, _)| l[..i].chars().count() + 1)
            .unwrap()
    };
    assert_eq!(end("a.md"), end("big.md"), "right edges align: {o}");
}

/// Subfeature 7: the inventory surface lists every lattice-2 fact the
/// machinery knows, with its position, office, state, and ask
/// (design/lattice-2.md; rewritten with the grid prefactor 2026-08-22 —
/// the slugs are the lattice's, so `size` is `bytes` and `child-count` is
/// gone, being the census's totals rather than a fact of its own).
#[test]
fn config_shows_fact_inventory() {
    let (dir, xdg) = fixture();
    let (c, o, e) = run(&dir, &xdg, &[], &["config"]);
    assert_eq!(c, 0, "{e}");
    for key in [
        "filename",
        "leaf-census",
        "dir-census",
        "lines",
        "bytes",
        "mtime",
        "filetype",
        "filekind-word",
        "symlink-target",
        "denied",
        "walk-bound",
        "heat",
        "owner",
        "has",
        "facet",
        "git-status",
    ] {
        assert!(o.contains(key), "inventory lists {key}: {o}");
    }
    // The position vocabulary is lattice-2's, and it says where the fact
    // paints *today* — the old table's `place` words were inverted.
    for word in [
        "name-location",
        "after-name",
        "near-right",
        "far-right",
        "far-left",
        "level-location",
    ] {
        assert!(
            o.contains(word),
            "inventory speaks lattice-2 positions ({word}): {o}"
        );
    }
    // Offices, and the fields that make a row actionable or honest.
    for word in ["office", "census", "mark", "weight", "derived-from"] {
        assert!(o.contains(word), "inventory carries {word}: {o}");
    }
    assert!(o.contains("columns.size"), "the ask is taught: {o}");
    assert!(o.contains("unbuilt"), "unbuilt facts named honestly: {o}");
    // State reflects the stack.
    user_home(&xdg, "columns.size = on\n");
    let (_, o, _) = run(&dir, &xdg, &[], &["config"]);
    let bytes_row = o
        .lines()
        .find(|l| l.contains(" bytes ") && l.contains("columns.size"))
        .unwrap();
    assert!(
        bytes_row.contains(" on"),
        "state is current, not default: {bytes_row}"
    );
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
    let h = lines
        .iter()
        .position(|l| l.trim_start().starts_with("lines"))
        .unwrap();
    assert!(
        lines[h + 1].starts_with('├') || lines[h + 1].starts_with('└'),
        "headings sit directly above the children: {o}"
    );
    // 2026-08-22 heading-to-dot: the word ends at cell 9 (the `.`), not
    // cell 12. Char positions, not bytes — the tree glyphs are multibyte.
    let chars_to = |l: &str, byte: usize| l[..byte].chars().count();
    let head_end = chars_to(lines[h], lines[h].find("lines").unwrap()) + "lines".len();
    let a_line = lines.iter().find(|l| l.contains("a.md")).unwrap();
    let dot_end = chars_to(a_line, a_line.rfind('.').unwrap()) + 1;
    assert_eq!(head_end, dot_end, "heading ends above the `.`: {o}");
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
        let (c, o, e) = run(
            &dir,
            &xdg,
            &[],
            &["--depth", "1", "--lines", &n.to_string()],
        );
        assert_eq!(c, 0, "{e}");
        // (Footer on stderr since 2026-08-22 — stdout is exactly the look.)
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
    // 2026-08-22 count-cell slice: file-root has no headings line, so the
    // unit slot speaks (`𝓁`) instead of the `"N lines"` wrap.
    assert!(
        o.contains("1.") && o.contains('\u{1D4C1}'),
        "count cell with unit on the facts line: {o}"
    );
    assert!(!o.contains("1 lines"), "old wrap retired: {o}");
}

/// mtime's default text form is SIGNA — one register with the age column —
/// and `format.mtime = relative` restores `13.6d ago`. JSON stays iso-8601
/// (the stamp is the only `Z` in the text look).
#[test]
fn mtime_defaults_to_signa() {
    let (dir, xdg) = fixture();
    user_home(&xdg, "columns.mtime = on\n");
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    assert_eq!(o.matches('Z').count(), 1, "stamp is the only ISO time: {o}");
    let a_line = o.lines().find(|l| l.contains("a.md")).unwrap();
    assert!(
        a_line.contains('⬤') || a_line.contains('◉') || a_line.contains('◎'),
        "SIGNA age spelled: {a_line}"
    );
    assert!(
        !a_line.contains("ago"),
        "relative is not the default: {a_line}"
    );
    user_home(&xdg, "columns.mtime = on\nformat.mtime = relative\n");
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    let a_line = o.lines().find(|l| l.contains("a.md")).unwrap();
    assert!(a_line.contains("ago"), "relative still works: {a_line}");
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

/// Heading tokens sit exactly over their columns (steward repro,
/// 2026-08-14: `bin/`'s heat score sat well left of the `heat · age`
/// heading). The cluster aligns as two sub-columns — score under `heat`,
/// age under `age` — with the `·` at one char position on every row.
#[test]
fn heading_sits_over_the_cluster() {
    let (dir, xdg) = fresh("headpos");
    // A git repo so heat exists; two files with different age widths so
    // the cluster's halves would wander without sub-alignment.
    let git = |args: &[&str]| {
        assert!(
            std::process::Command::new("git")
                .args(args)
                .current_dir(&dir)
                .env("GIT_AUTHOR_NAME", "t")
                .env("GIT_AUTHOR_EMAIL", "t@t")
                .env("GIT_COMMITTER_NAME", "t")
                .env("GIT_COMMITTER_EMAIL", "t@t")
                .output()
                .unwrap()
                .status
                .success()
        );
    };
    git(&["init", "-q"]);
    fs::write(dir.join("a.md"), "1\n").unwrap();
    fs::write(dir.join("b.md"), "1\n").unwrap();
    git(&["add", "."]);
    git(&["commit", "-qm", "one"]);
    fs::write(dir.join("a.md"), "2\n").unwrap();
    git(&["add", "."]);
    git(&["commit", "-qm", "two"]);
    // Different mtime ages → different age-string widths.
    let f = File::open(dir.join("b.md")).unwrap();
    f.set_modified(std::time::UNIX_EPOCH + std::time::Duration::from_secs(1_700_000_000))
        .unwrap();
    user_home(&xdg, "format.heat = score\nformat.mtime = relative\n");
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    let pos = |l: &str| {
        l.chars()
            .collect::<Vec<_>>()
            .windows(3)
            .position(|w| w == [' ', '·', ' '])
    };
    // Sub-aligned, the heading may read `heat ·   age` — find it by parts.
    let head = o
        .lines()
        .find(|l| l.contains("heat") && l.contains("age") && !l.contains("── "))
        .expect("heading");
    let hp = pos(head).expect("heading has the ·");
    for l in o.lines().filter(|l| l.contains("── ") && l.contains(" · ")) {
        assert_eq!(pos(l), Some(hp), "the · aligns under the heading: {l}");
    }
}

/// A QUIET column in which no cell speaks claims no heading — a heading
/// only rides a column present in this look (confirmed against the
/// 2026-08-14 vivarium report: there the size cell *did* speak once).
#[test]
fn quiet_column_without_speakers_has_no_heading() {
    let (dir, xdg) = fresh("noheads");
    // Same-size same-kind files: nothing surprises, size stays silent.
    for n in ["a.md", "b.md", "c.md"] {
        fs::write(dir.join(n), "same\n").unwrap();
        let f = File::open(dir.join(n)).unwrap();
        f.set_modified(std::time::UNIX_EPOCH + std::time::Duration::from_secs(1_700_000_000))
            .unwrap();
    }
    let (c, o, e) = run(&dir, &xdg, &[], &["--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    assert!(!o.contains("size"), "no heading over silence: {o}");
    assert!(
        o.contains("lines"),
        "the speaking column keeps its word: {o}"
    );
}
