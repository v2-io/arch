//! JSON (design/json.md subfeatures): the same look serialized. Real
//! binary, isolated XDG. Validity is checked with a small structural
//! parser here — no dependencies, same as the binary's emitter.

use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, UNIX_EPOCH};

static SEQ: AtomicU64 = AtomicU64::new(0);

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_aspectus"))
}

fn fresh(tag: &str) -> (PathBuf, PathBuf) {
    let n = SEQ.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!(
        "aspectus-json-{tag}-{}-{}-{}",
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

fn touch(dir: &Path, name: &str, epoch_secs: u64) {
    let p = dir.join(name);
    File::create(&p).unwrap().write_all(b"one\ntwo\n").unwrap();
    File::open(&p)
        .unwrap()
        .set_modified(UNIX_EPOCH + Duration::from_secs(epoch_secs))
        .unwrap();
}

fn run(dir: &Path, xdg: &Path, envs: &[(&str, &str)], args: &[&str]) -> (i32, String, String) {
    let mut c = bin();
    c.args(args).current_dir(dir).env("XDG_CONFIG_HOME", xdg);
    for k in [
        "ASPECTUS_LINES",
        "ASPECTUS_DEPTH",
        "ASPECTUS_SORT",
        "ASPECTUS_WALK",
        "ASPECTUS_FORMAT",
        "ASPECTUS_FORMAT_SIZE",
        "ASPECTUS_FORMAT_MTIME",
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

/// Minimal structural JSON check: consumes exactly one value, whitespace-
/// tolerant, escapes honored. Returns rest on success.
fn skip_value(s: &str) -> Result<&str, String> {
    let s = s.trim_start();
    let mut cs = s.char_indices();
    match cs.next().map(|(_, c)| c) {
        Some('{') => {
            let mut rest = s[1..].trim_start();
            if let Some(r) = rest.strip_prefix('}') {
                return Ok(r);
            }
            loop {
                rest = skip_value(rest)?; // key
                rest = rest
                    .trim_start()
                    .strip_prefix(':')
                    .ok_or_else(|| format!("expected : at {rest:.30}"))?;
                rest = skip_value(rest)?;
                rest = rest.trim_start();
                if let Some(r) = rest.strip_prefix(',') {
                    rest = r;
                } else {
                    return rest
                        .strip_prefix('}')
                        .ok_or_else(|| format!("expected }} at {rest:.30}"));
                }
            }
        }
        Some('[') => {
            let mut rest = s[1..].trim_start();
            if let Some(r) = rest.strip_prefix(']') {
                return Ok(r);
            }
            loop {
                rest = skip_value(rest)?;
                rest = rest.trim_start();
                if let Some(r) = rest.strip_prefix(',') {
                    rest = r;
                } else {
                    return rest
                        .strip_prefix(']')
                        .ok_or_else(|| format!("expected ] at {rest:.30}"));
                }
            }
        }
        Some('"') => {
            let mut esc = false;
            for (i, c) in s[1..].char_indices() {
                match (esc, c) {
                    (true, _) => esc = false,
                    (false, '\\') => esc = true,
                    (false, '"') => return Ok(&s[1 + i + 1..]),
                    _ => {}
                }
            }
            Err("unterminated string".into())
        }
        Some(c) if c == '-' || c.is_ascii_digit() => {
            let end = s
                .find(|c: char| !(c.is_ascii_digit() || "+-.eE".contains(c)))
                .unwrap_or(s.len());
            Ok(&s[end..])
        }
        _ => ["true", "false", "null"]
            .iter()
            .find_map(|w| s.strip_prefix(w))
            .ok_or_else(|| format!("bad value at {s:.30}")),
    }
}

fn assert_valid_json(o: &str) {
    let rest = skip_value(o).unwrap_or_else(|e| panic!("{e}\nin: {o}"));
    assert!(rest.trim().is_empty(), "one document only: {rest:.60}");
}

fn fixture() -> (PathBuf, PathBuf) {
    let (dir, xdg) = fresh("fx");
    touch(&dir, "a.md", 1_700_000_100);
    touch(&dir, "b.md", 1_700_000_200);
    let sub = dir.join("sub");
    fs::create_dir_all(&sub).unwrap();
    touch(&sub, "deep.md", 1_700_000_300);
    File::open(&sub)
        .unwrap()
        .set_modified(UNIX_EPOCH + Duration::from_secs(1_700_000_300))
        .unwrap();
    (dir, xdg)
}

/// Subfeature 2: single valid document on stdout, stderr empty, exit 0.
#[test]
fn valid_transport() {
    let (dir, xdg) = fixture();
    let (c, o, e) = run(&dir, &xdg, &[], &["--format", "json"]);
    assert_eq!(c, 0, "{e}");
    assert!(
        e.lines()
            .all(|l| l.is_empty() || l.starts_with("*(This is a critical")),
        "stderr empty on success: {e}"
    ); // stderr: only the feedback footer (teaching) since 2026-08-22
    assert_valid_json(&o);
    assert!(o.starts_with("{\"aspectus\":"), "{o:.80}");
    assert!(o.contains("\"schema\":1"), "versioned from birth: {o:.120}");
}

/// Subfeature 1: node set and census numbers equal the text look's.
#[test]
fn same_look_as_text() {
    let (dir, xdg) = fresh("same");
    for i in 0..12 {
        touch(&dir, &format!("f{i:02}.md"), 1_700_000_000 + i * 10);
    }
    let args = ["--depth", "1", "--lines", "8"];
    // Numbered-series fixture; globify (2026-08-14) would collapse it to
    // one listee in both renderings — parity of the budget fold is this
    // test's subject, so it is pinned off.
    let g = [("ASPECTUS_GLOBIFY", "off")];
    let (_, text, _) = run(&dir, &xdg, &g, &args);
    let mut jargs = vec!["--format", "json"];
    jargs.extend_from_slice(&args);
    let (_, j, _) = run(&dir, &xdg, &g, &jargs);
    assert_valid_json(&j);
    // The same four survivors, in the same order; the same leftover count.
    let listed: Vec<String> = text
        .lines()
        .filter(|l| l.contains("── ")) // children only, not header/headings
        .filter(|l| !l.contains("[+"))
        .filter_map(|l| l.rsplit("── ").next())
        .map(|s| s.split_whitespace().next().unwrap().to_string())
        .collect();
    for name in &listed {
        assert!(
            j.contains(&format!("\"name\":\"{name}\"")),
            "{name} in json"
        );
    }
    let json_names = j.matches("\"name\":\"f").count();
    assert_eq!(json_names, listed.len(), "no extra nodes for machines: {j}");
    // md×9, not ×8: the text look's column-headings line (2026-08-14)
    // costs one child slot, and JSON carries the *same* look — same
    // budget, same survivors — rather than growing wider for machines.
    assert!(text.contains("[+ md×9]"), "{text}");
    assert!(
        j.contains("\"omitted\":{\"total\":9"),
        "leaf census as an object: {j}"
    );
}

/// Subfeature 3: sizes stay integer bytes and times canonical whatever the
/// text formats say.
#[test]
fn bytes_and_canonical_times() {
    let (dir, xdg) = fixture();
    fs::write(
        xdg.join("aspectus/aspectus.toml"),
        "format.size = \"human\"\nformat.mtime = \"epoch\"\ncolumns.size = \"on\"\n",
    )
    .unwrap();
    let (c, o, e) = run(&dir, &xdg, &[], &["--format", "json", "--depth", "1"]);
    assert_eq!(c, 0, "{e}");
    assert!(o.contains("\"size\":8"), "bytes, not 8B: {o}");
    assert!(
        o.contains("\"mtime\":\"2023-11-14T"),
        "iso-8601 despite epoch config: {o}"
    );
}

/// Subfeature 4: denied and walk-bound are fields, censuses objects.
#[test]
fn marks_as_data() {
    use std::os::unix::fs::PermissionsExt;
    let (dir, xdg) = fresh("marks");
    let locked = dir.join("locked");
    fs::create_dir_all(&locked).unwrap();
    fs::set_permissions(&locked, fs::Permissions::from_mode(0o000)).unwrap();
    touch(&dir, "a.md", 1_700_000_000);
    let (c, o, _) = run(&dir, &xdg, &[], &["--format", "json", "--depth", "2"]);
    fs::set_permissions(&locked, fs::Permissions::from_mode(0o755)).unwrap();
    assert_eq!(c, 0);
    assert_valid_json(&o);
    assert!(o.contains("\"denied\":true"), "{o}");
    assert!(o.contains("\"truncated\":true"), "{o}");

    let (dir2, xdg2) = fresh("wb");
    for i in 0..30 {
        touch(&dir2, &format!("f{i:02}.md"), 1_700_000_000);
    }
    let (c, o, _) = run(
        &dir2,
        &xdg2,
        &[],
        &["--format", "json", "--walk", "5", "--depth", "1"],
    );
    assert_eq!(c, 0);
    assert!(o.contains("\"walk_bound\":true"), "{o}");
    assert!(
        o.contains("\"census\":{\"total\":") || o.contains("\"omitted\":{\"total\":"),
        "{o}"
    );
}

/// Subfeature 5: a complete look clears the top-level flag.
#[test]
fn truncated_reflects_the_look() {
    let (dir, xdg) = fixture();
    let (_, o, _) = run(
        &dir,
        &xdg,
        &[],
        &["--format", "json", "--depth", "0", "--lines", "0"],
    );
    assert!(o.contains("\"truncated\":false"), "complete look: {o}");
}

/// Subfeature 6: byte-identical across runs and terminal conditions
/// (modulo the honest time field).
#[test]
fn deterministic_json() {
    let (dir, xdg) = fixture();
    let strip_time = |s: &str| s.replace(|c: char| c.is_ascii_digit(), "#");
    let (_, o1, _) = run(&dir, &xdg, &[("COLUMNS", "40")], &["--format", "json"]);
    let (_, o2, _) = run(&dir, &xdg, &[("COLUMNS", "200")], &["--format", "json"]);
    // Only the look-time may differ; digits elsewhere identical too, but
    // masking all digits keeps the comparison simple and strict enough.
    assert_eq!(strip_time(&o1), strip_time(&o2));
    let t1 = o1.split("\"time\":").nth(1).unwrap();
    let t2 = o2.split("\"time\":").nth(1).unwrap();
    let after = |s: &str| s.split_once(',').unwrap().1.to_string();
    assert_eq!(after(t1), after(t2), "identical after the stamp");
}

/// Subfeature 7: refusals are machine-shaped in machine mode, exit 2.
#[test]
fn structured_refusal() {
    let (dir, xdg) = fixture();
    let (c, o, e) = run(&dir, &xdg, &[], &["--format", "json", "--no-such"]);
    assert_eq!(c, 2);
    assert!(o.is_empty(), "stdout stays data-or-nothing: {o}");
    assert_valid_json(&e);
    assert!(e.contains("\"class\":\"unknown option\""), "{e}");
    assert!(e.contains("\"next\":[\"aspectus help\"]"), "{e}");
}

/// Subfeature 8: help teaches --format; a bad value is refused with the
/// menu.
#[test]
fn help_and_refusal() {
    let (dir, xdg) = fixture();
    let (c, o, _) = run(&dir, &xdg, &[], &["help"]);
    assert_eq!(c, 0);
    assert!(o.contains("--format text|json"), "{o}");
    assert!(o.contains("aspectus --format json PATH"), "example: {o}");
    let (c, _, e) = run(&dir, &xdg, &[], &["--format", "yaml"]);
    assert_eq!(c, 2);
    assert!(e.contains("text or json"), "{e}");
}
