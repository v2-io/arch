//! Config show: drive the real `aspectus` binary. Isolated XDG, never HOME=scratch.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static SEQ: AtomicU64 = AtomicU64::new(0);

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_aspectus"))
}

fn fresh_xdg() -> PathBuf {
    let n = SEQ.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!(
        "aspectus-xdg-{}-{}-{}",
        std::process::id(),
        n,
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(dir.join("aspectus")).unwrap();
    dir
}

fn run_with_xdg(xdg: &Path, args: &[&str]) -> (i32, String, String) {
    let out = bin()
        .args(args)
        .env("XDG_CONFIG_HOME", xdg)
        .env_remove("ASPECTUS_LINES")
        .output()
        .unwrap_or_else(|e| panic!("launch aspectus {args:?}: {e}"));
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

fn run_with_xdg_env(xdg: &Path, envp: &[(&str, &str)], args: &[&str]) -> (i32, String, String) {
    let mut cmd = bin();
    cmd.args(args)
        .env("XDG_CONFIG_HOME", xdg)
        .env_remove("ASPECTUS_LINES");
    for (k, v) in envp {
        cmd.env(k, v);
    }
    let out = cmd
        .output()
        .unwrap_or_else(|e| panic!("launch aspectus {args:?}: {e}"));
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

#[test]
fn config_defaults_stdout_exit_0() {
    let xdg = fresh_xdg();
    let (c, o, e) = run_with_xdg(&xdg, &["config"]);
    assert_eq!(c, 0, "stderr={e}");
    assert!(e.lines().all(|l| l.is_empty() || l.starts_with("*(This is a critical")), "config show stderr must be empty: {e:?}");  // stderr: only the feedback footer (teaching) since 2026-08-22
    assert!(o.contains("defaults"), "{o}");
    assert!(o.contains("user-home"), "{o}");
    assert!(o.contains("global"), "{o}");
    assert!(o.contains("agent-type"), "{o}");
    assert!(o.contains("env"), "{o}");
    assert!(o.contains("flags"), "{o}");
    assert!(o.contains("won:"), "{o}");
    assert!(o.contains("lines = 80"), "{o}");
    assert!(o.contains("(defaults)"), "{o}");
}

#[test]
fn config_show_twice_identical() {
    let xdg = fresh_xdg();
    let (_, a, _) = run_with_xdg(&xdg, &["config"]);
    let (_, b, _) = run_with_xdg(&xdg, &["config"]);
    assert_eq!(a, b);
}

#[test]
fn decoy_at_locus_does_not_win() {
    let xdg = fresh_xdg();
    let locus = std::env::temp_dir().join(format!(
        "aspectus-locus-{}-{}",
        std::process::id(),
        SEQ.fetch_add(1, Ordering::SeqCst)
    ));
    fs::create_dir_all(&locus).unwrap();
    fs::write(locus.join("aspectus.toml"), "lines = 999\n").unwrap();
    fs::write(locus.join(".aspectus.toml"), "lines = 998\n").unwrap();

    let out = bin()
        .args(["config"])
        .current_dir(&locus)
        .env("XDG_CONFIG_HOME", &xdg)
        .env_remove("ASPECTUS_LINES")
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert_eq!(out.status.code(), Some(0));
    assert!(
        stdout.contains("lines = 80"),
        "decoy must not win: {stdout}"
    );
    // "lines = 999", not bare "999": the xdg scratch path carries a nanosecond
    // timestamp that can itself contain "999" (seen live, 2026-08-14).
    assert!(!stdout.contains("lines = 999"), "{stdout}");
    assert!(!stdout.contains("lines = 998"), "{stdout}");
    assert!(!stdout.contains(locus.join("aspectus.toml").to_str().unwrap()), "{stdout}");
}

#[test]
fn config_flag_substitutes_user_home() {
    let xdg = fresh_xdg();
    let real = xdg.join("aspectus/aspectus.toml");
    fs::write(&real, "lines = 11\n").unwrap();
    let alt = std::env::temp_dir().join(format!(
        "aspectus-alt-{}-{}.toml",
        std::process::id(),
        SEQ.fetch_add(1, Ordering::SeqCst)
    ));
    fs::write(&alt, "lines = 22\n").unwrap();

    let (c, o, e) = run_with_xdg(&xdg, &["config"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("lines = 11"), "user-home should win: {o}");
    assert!(o.contains("(user-home)"), "{o}");

    let alt_s = alt.to_str().unwrap();
    let (c, o, e) = run_with_xdg(&xdg, &["config", "--config", alt_s]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("lines = 22"), "--config must win: {o}");
    assert!(o.contains(alt_s), "show must name the --config file: {o}");
    assert!(!o.contains("lines = 11"), "{o}");
}

#[test]
fn env_beats_user_home() {
    let xdg = fresh_xdg();
    fs::write(xdg.join("aspectus/aspectus.toml"), "lines = 11\n").unwrap();

    let (c, o, e) = run_with_xdg_env(&xdg, &[("ASPECTUS_LINES", "33")], &["config"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("lines = 33"), "env should win: {o}");
    assert!(o.contains("(env)"), "{o}");

}

#[test]
fn caller_flag_names_agent_type_layer() {
    let xdg = fresh_xdg();
    let (c, without, e) = run_with_xdg(&xdg, &["config"]);
    assert_eq!(c, 0, "{e}");
    assert!(without.contains("--caller not set"), "{without}");

    fs::write(xdg.join("aspectus/caller-demo.toml"), "lines = 55\n").unwrap();
    let (c, with, e) = run_with_xdg(&xdg, &["config", "--caller", "demo"]);
    assert_eq!(c, 0, "{e}");
    assert!(with.contains("caller-demo.toml"), "{with}");
    assert!(with.contains("lines = 55"), "{with}");
    assert!(with.contains("(agent-type)"), "{with}");
    assert_ne!(without, with);
}

#[test]
fn missing_layers_are_not_errors() {
    let xdg = fresh_xdg();
    let (c, o, e) = run_with_xdg(&xdg, &["config"]);
    assert_eq!(c, 0, "{e}");
    assert!(e.lines().all(|l| l.is_empty() || l.starts_with("*(This is a critical")), "{e:?}");  // stderr: only the feedback footer (teaching) since 2026-08-22
    assert!(o.contains("absent"), "{o}");
}

#[test]
fn config_names_the_embedded_layer() {
    let xdg = fresh_xdg();
    let (c, o, e) = run_with_xdg(&xdg, &["config"]);
    assert_eq!(c, 0, "{e}");
    assert!(
        o.contains("built-in (defaults.toml, embedded)"),
        "defaults layer names the file: {o}"
    );
    assert!(o.contains("layout:"), "{o}");
    assert!(o.contains("far-left"), "{o}");
    assert!(o.contains("(unbuilt position)"), "far-left is unbuilt: {o}");
    assert!(o.contains("furniture:"), "{o}");
    assert!(o.contains(".git = git"), "{o}");
    assert!(o.contains("(default)"), "{o}");
    assert!(o.contains("kinds:"), "{o}");
    assert!(o.contains("md = text/markdown"), "{o}");
    assert!(o.contains("important:"), "{o}");
    assert!(o.contains("README*"), "{o}");
}

#[test]
fn config_defaults_prints_the_embedded_file() {
    let xdg = fresh_xdg();
    let (c, o, e) = run_with_xdg(&xdg, &["config", "defaults"]);
    assert_eq!(c, 0, "stderr={e}");
    assert_eq!(
        o,
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/defaults.toml"
        )),
        "stdout is the embedded file verbatim"
    );
    assert!(
        e.is_empty(),
        "config defaults is stdout only, no footer: {e:?}"
    );
}

#[test]
fn old_columns_key_warns_naming_layout() {
    let xdg = fresh_xdg();
    fs::write(xdg.join("aspectus/aspectus.toml"), "columns.size = on\n").unwrap();
    let (c, o, e) = run_with_xdg(&xdg, &["config"]);
    assert_eq!(c, 0, "{e}");
    assert!(e.contains("[layout]"), "stderr names [layout]: {e}");
    assert!(e.contains("columns.size"), "{e}");
    assert!(o.contains("columns.size = on"), "old key still wins: {o}");
}

#[test]
fn furniture_table_overlay_matches_bang() {
    let xdg = fresh_xdg();
    fs::write(
        xdg.join("aspectus/aspectus.toml"),
        "[furniture]\n\".mystery\" = \"lab\"\n\"target/\" = \"!\"\n",
    )
    .unwrap();
    let (c, o, e) = run_with_xdg(&xdg, &["config"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains(".mystery = lab"), "added row: {o}");
    assert!(o.contains("(user-home)"), "{o}");
    assert!(
        !o.lines().any(|l| l.contains("target/") && l.contains("build")),
        "dropped shipped row: {o}"
    );
}

#[test]
fn round_trip_defaults_file_is_identity() {
    let xdg = fresh_xdg();
    let (c, dumped, e) = run_with_xdg(&xdg, &["config", "defaults"]);
    assert_eq!(c, 0, "{e}");
    let copy = xdg.join("copy.toml");
    fs::write(&copy, &dumped).unwrap();

    let locus = std::env::temp_dir().join(format!(
        "aspectus-rt-{}-{}",
        std::process::id(),
        SEQ.fetch_add(1, Ordering::SeqCst)
    ));
    fs::create_dir_all(&locus).unwrap();
    fs::write(locus.join("a.md"), "hi\n").unwrap();

    let strip = |stdout: &str| {
        stdout
            .lines()
            .skip(1)
            .collect::<Vec<_>>()
            .join("\n")
    };
    let none = bin()
        .arg(&locus)
        .env("XDG_CONFIG_HOME", &xdg)
        .env_remove("ASPECTUS_LINES")
        .output()
        .unwrap();
    let with = bin()
        .arg(&locus)
        .arg("--config")
        .arg(&copy)
        .env("XDG_CONFIG_HOME", &xdg)
        .env_remove("ASPECTUS_LINES")
        .output()
        .unwrap();
    assert_eq!(none.status.code(), Some(0), "{}", String::from_utf8_lossy(&none.stderr));
    assert_eq!(with.status.code(), Some(0), "{}", String::from_utf8_lossy(&with.stderr));
    assert_eq!(
        strip(&String::from_utf8_lossy(&none.stdout)),
        strip(&String::from_utf8_lossy(&with.stdout)),
        "copy of defaults.toml as --config must not move the look"
    );
}
