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
    assert!(e.is_empty(), "config show stderr must be empty: {e:?}");
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
    assert!(!stdout.contains("999"), "{stdout}");
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
    assert!(e.is_empty(), "{e:?}");
    assert!(o.contains("absent"), "{o}");
}
