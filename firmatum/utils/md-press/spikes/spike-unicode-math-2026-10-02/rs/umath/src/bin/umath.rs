//! JSON lines in (`{"id": …, "text": "…"}`), JSON lines out (`{"id": …, "out": "…"}`),
//! in input order.
//!
//!   umath [--v3|--v4|--v5|--v6]  (default: v6) [--spans] [--changed-only] [--threads N] < in.jsonl > out.jsonl
//!
//! --spans         also emit "spans": [[start, end, latex|null, conf], …] (char offsets)
//! --changed-only  emit only records whose text changed or that have spans
//! --threads N     worker threads (default: available parallelism)
//! On a reference-reproduced error the record carries "error" instead of "out".

use serde_json::{Value, json};
use std::io::{BufRead, Write};

fn main() {
    let mut ver = umath::Ver::V6;
    let (mut spans, mut changed_only) = (false, false);
    let mut threads = std::thread::available_parallelism().map_or(1, |n| n.get());
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--v3" => ver = umath::Ver::V3,
            "--v4" => ver = umath::Ver::V4,
            "--v5" => ver = umath::Ver::V5,
            "--v6" => ver = umath::Ver::V6,
            "--spans" => spans = true,
            "--changed-only" => changed_only = true,
            "--threads" => threads = args.next().and_then(|x| x.parse().ok()).expect("--threads N"),
            _ => {
                eprintln!("usage: umath [--v3|--v4|--v5|--v6] [--spans] [--changed-only] [--threads N] < in.jsonl");
                std::process::exit(2);
            }
        }
    }
    let lines: Vec<String> = std::io::stdin().lock().lines().map(|l| l.expect("stdin")).collect();
    let work = |line: &String| -> Option<String> {
        if line.trim().is_empty() {
            return None;
        }
        let r: Value = serde_json::from_str(line).expect("json line");
        let id = r.get("id").cloned().unwrap_or(Value::Null);
        let text = r["text"].as_str().expect("\"text\" string");
        let res = umath::convert_ver(text, ver);
        let rec = match res {
            Ok((out, sp)) => {
                if changed_only && out == text && sp.is_empty() {
                    return None;
                }
                let mut o = json!({"id": id, "out": out});
                if spans {
                    o["spans"] = sp.iter().map(|s| json!([s.start, s.end, s.latex, s.conf])).collect();
                }
                o
            }
            Err(e) => json!({"id": id, "error": e.to_string()}),
        };
        Some(rec.to_string())
    };
    let threads = threads.max(1);
    let chunk = lines.len().div_ceil(threads * 8).max(1);
    let chunks: Vec<&[String]> = lines.chunks(chunk).collect();
    let results: Vec<Vec<String>> = if threads == 1 {
        chunks.iter().map(|c| c.iter().filter_map(work).collect()).collect()
    } else {
        let next = std::sync::atomic::AtomicUsize::new(0);
        let slots: Vec<std::sync::Mutex<Vec<String>>> = chunks.iter().map(|_| Default::default()).collect();
        std::thread::scope(|sc| {
            for _ in 0..threads {
                sc.spawn(|| loop {
                    let k = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if k >= chunks.len() {
                        break;
                    }
                    *slots[k].lock().unwrap() = chunks[k].iter().filter_map(work).collect();
                });
            }
        });
        slots.into_iter().map(|m| m.into_inner().unwrap()).collect()
    };
    let mut w = std::io::BufWriter::new(std::io::stdout().lock());
    for r in results.iter().flatten() {
        writeln!(w, "{r}").unwrap();
    }
}
