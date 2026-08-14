//! Help & Version: drive the real `aspectus` binary.

use std::process::Command;

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_aspectus"))
}

fn run(args: &[&str]) -> (i32, String, String) {
    let out = bin()
        .args(args)
        .output()
        .unwrap_or_else(|e| panic!("launch aspectus {args:?}: {e}"));
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

fn assert_help(stdout: &str, stderr: &str, code: i32) {
    assert_eq!(code, 0, "stderr={stderr}");
    assert!(stderr.is_empty(), "help stderr must be empty, got {stderr:?}");
    assert!(stdout.contains("aspectus"), "{stdout}");
    assert!(stdout.contains("aspecta"), "{stdout}");
    assert!(stdout.contains("faculty of looking at a locus"), "{stdout}");
    assert!(stdout.contains("location of action"), "{stdout}");
    assert!(stdout.contains("seen-things"), "{stdout}");
    assert!(stdout.contains("how the place looks right now"), "{stdout}");
    assert!(stdout.contains("usage:"), "{stdout}");
    assert!(
        stdout.contains(&format!("aspectus {}", env!("CARGO_PKG_VERSION"))),
        "help must name this build's version: {stdout}"
    );
    assert!(stdout.contains("help"), "{stdout}");
    assert!(stdout.contains("-h"), "{stdout}");
    assert!(stdout.contains("--help"), "{stdout}");
    assert!(stdout.contains("version"), "{stdout}");
    assert!(stdout.contains("-v"), "{stdout}");
    assert!(stdout.contains("--version"), "{stdout}");
    assert!(stdout.contains("--"), "{stdout}");
    assert!(stdout.contains("Examples:"), "{stdout}");
}

#[test]
fn help_verb_stdout_exit_0() {
    let (c, o, e) = run(&["help"]);
    assert_help(&o, &e, c);
}

#[test]
fn help_short_flag() {
    let (c, o, e) = run(&["-h"]);
    assert_help(&o, &e, c);
}

#[test]
fn help_long_flag() {
    let (c, o, e) = run(&["--help"]);
    assert_help(&o, &e, c);
}

#[test]
fn three_help_spellings_identical() {
    let (_, a, _) = run(&["help"]);
    let (_, b, _) = run(&["-h"]);
    let (_, c, _) = run(&["--help"]);
    assert_eq!(a, b);
    assert_eq!(b, c);
}

fn assert_version(stdout: &str, stderr: &str, code: i32) {
    assert_eq!(code, 0, "stderr={stderr}");
    assert!(stderr.is_empty(), "version stderr must be empty, got {stderr:?}");
    let line = stdout.trim_end_matches('\n');
    assert!(!line.contains('\n'), "one line, got {stdout:?}");
    let rest = line
        .strip_prefix("aspectus ")
        .unwrap_or_else(|| panic!("expected 'aspectus …', got {stdout:?}"));
    assert!(
        rest.chars().next().is_some_and(|c| c.is_ascii_digit()),
        "semver after name, got {stdout:?}"
    );
    assert!(!rest.contains("rustc"), "{stdout:?}");
}

#[test]
fn version_verb() {
    let (c, o, e) = run(&["version"]);
    assert_version(&o, &e, c);
}

#[test]
fn version_short_flag_is_version_not_verbose() {
    let (c, o, e) = run(&["-v"]);
    assert_version(&o, &e, c);
}

#[test]
fn version_long_flag() {
    let (c, o, e) = run(&["--version"]);
    assert_version(&o, &e, c);
}

#[test]
fn three_version_spellings_identical() {
    let (_, a, _) = run(&["version"]);
    let (_, b, _) = run(&["-v"]);
    let (_, c, _) = run(&["--version"]);
    assert_eq!(a, b);
    assert_eq!(b, c);
}

#[test]
fn unknown_option_stderr_exit_2() {
    let (c, o, e) = run(&["--nope"]);
    assert_eq!(c, 2);
    assert!(o.is_empty(), "stdout must be empty, got {o:?}");
    assert!(e.contains("unknown option"), "{e}");
    assert!(e.contains("--nope"), "{e}");
    assert!(e.contains("aspectus help"), "{e}");
    assert!(!e.contains("faculty of looking"), "must not reprint help: {e}");
}

#[test]
fn unknown_verb_distinct_from_option() {
    let (c, o, e) = run(&["frob"]);
    assert_eq!(c, 2);
    assert!(o.is_empty(), "stdout must be empty, got {o:?}");
    assert!(e.contains("unknown verb"), "{e}");
    assert!(e.contains("frob"), "{e}");
    assert!(e.contains("aspectus help"), "{e}");
    assert!(!e.contains("unknown option"), "{e}");
}

#[test]
fn end_of_flags_help_is_not_help() {
    let (c, o, e) = run(&["--", "--help"]);
    assert_eq!(c, 2, "stdout={o:?} stderr={e:?}");
    assert!(o.is_empty(), "{o:?}");
    assert!(
        e.contains("unknown verb") || e.contains("not found") || e.contains("No such"),
        "expected unknown verb or not-found, got {e:?}"
    );
    assert!(!e.contains("faculty of looking"), "{e}");
}

#[test]
fn help_lists_every_accepted_flag() {
    let (_, page, _) = run(&["help"]);
    for needle in [
        "--help",
        "--version",
        "--config",
        "--caller",
        "-h",
        "-v",
        "config",
    ] {
        assert!(page.contains(needle), "help missing {needle}");
    }
    for gone in [
        "--lines",
        "--visit",
        "--show-all",
        "--inspect",
        "--no-one-fs",
        "--color",
        "--color=auto",
    ] {
        let (c, _, e) = run(&[gone]);
        assert_eq!(c, 2, "{gone} should be unknown, got {e}");
        assert!(e.contains("unknown option"), "{gone}: {e}");
    }
}
