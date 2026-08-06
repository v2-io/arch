//! Corpus property tests + ground-truth scoring.
//!
//! Invariants (every fixture): idempotence; render-equality (comrak HTML,
//! whitespace-collapsed outside <pre>). Ground truth: format(before) vs the
//! human-approved after from asf git history — reported, with exact-match
//! asserted only for the de1082d gold pairs as they are verified to pass.

use std::fs;
use std::path::{Path, PathBuf};

fn fixture_dir(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures")
        .join(rel)
}

fn md_files(dir: &Path) -> Vec<PathBuf> {
    let mut v = Vec::new();
    if let Ok(rd) = fs::read_dir(dir) {
        for e in rd.flatten() {
            let p = e.path();
            if p.is_dir() {
                v.extend(md_files(&p));
            } else if p.extension().is_some_and(|x| x == "md") {
                v.push(p);
            }
        }
    }
    v.sort();
    v
}

#[test]
fn invariants_hold_on_all_fixtures() {
    let mut checked = 0;
    for dir in ["corpus", "asf-history/before", "asf-history/after"] {
        for path in md_files(&fixture_dir(dir)) {
            let input = fs::read_to_string(&path).unwrap();
            let once = md_press::format(&input);
            let twice = md_press::format(&once);
            assert_eq!(once, twice, "not idempotent: {}", path.display());
            assert_eq!(
                md_press::render_fingerprint(&input),
                md_press::render_fingerprint(&once),
                "render changed: {}",
                path.display()
            );
            checked += 1;
        }
    }
    assert!(
        checked >= 20,
        "fixture corpus missing? only {checked} files"
    );
}

#[test]
fn ground_truth_score_report() {
    // Not a pass/fail gate yet: prints per-pair closeness so progress is
    // measurable run over run. Promote pairs to hard asserts as they pass.
    let before_dir = fixture_dir("asf-history/before");
    let after_dir = fixture_dir("asf-history/after");
    let mut exact = 0;
    let mut total = 0;
    for b in md_files(&before_dir) {
        let name = b.file_name().unwrap();
        let a = after_dir.join(name);
        if !a.exists() {
            continue;
        }
        total += 1;
        let got = md_press::format(&fs::read_to_string(&b).unwrap());
        let want = fs::read_to_string(&a).unwrap();
        if got == want {
            exact += 1;
        } else {
            let diff_lines = got
                .lines()
                .zip(want.lines())
                .filter(|(g, w)| g != w)
                .count()
                + got.lines().count().abs_diff(want.lines().count());
            eprintln!("pair {:?}: {} differing lines", name, diff_lines);
        }
    }
    eprintln!("ground truth: {exact}/{total} exact matches");
    assert!(total >= 16, "history pairs missing");
}

#[test]
fn random_wrap_recovery() {
    // Joseph's named property: take clean (already-canonical) prose, insert
    // newlines at word boundaries inside paragraphs, verify recovery.
    // Uses the approved 'after' files as the clean corpus and a fixed-seed
    // LCG so the test is deterministic.
    let mut seed: u64 = 0x5DEECE66D;
    let mut rand = move || {
        seed = seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (seed >> 33) as usize
    };
    let mut tested = 0;
    for path in md_files(&fixture_dir("asf-history/after")) {
        let clean = fs::read_to_string(&path).unwrap();
        // canonical baseline (some afters carry incumbent residuals):
        let canon = md_press::format(&clean);
        // wrap: split long prose lines at a word boundary near a random column
        let wrapped: String = canon
            .lines()
            .map(|l| {
                let is_prose = !l
                    .trim_start()
                    .starts_with(['#', '|', '>', '-', '*', '`', '$'])
                    && !l.starts_with("    ")
                    && l.len() > 90;
                if is_prose {
                    let mut out = String::new();
                    let mut cur = l;
                    while cur.len() > 90 {
                        let col = 60 + rand() % 25;
                        let mut end = col.min(cur.len());
                        while !cur.is_char_boundary(end) {
                            end -= 1;
                        }
                        match cur[..end].rfind(' ') {
                            // avoid split points whose continuation md-press
                            // deliberately preserves (definitions, math-led
                            // lines) — recovery is non-unique there by design
                            Some(sp)
                                if sp > 0
                                    && !cur[sp + 1..].trim_start().starts_with(['[', '$']) =>
                            {
                                out.push_str(&cur[..sp]);
                                out.push('\n');
                                cur = &cur[sp + 1..];
                            }
                            _ => break,
                        }
                    }
                    out.push_str(cur);
                    out
                } else {
                    l.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
            + if canon.ends_with('\n') { "\n" } else { "" };
        let recovered = md_press::format(&wrapped);
        assert_eq!(
            recovered,
            canon,
            "random-wrap recovery failed for {}",
            path.display()
        );
        tested += 1;
    }
    assert!(tested >= 16);
}
