// For each JSON line {gid, text} on stdin, emit {gid, parts: [[text, is_separator], ...]}
// using md-press's own split_at_prose_separators + split_sentences (the pieces
// today's pipeline would send to a model). Concatenating the parts gives text.
use md_press::math;
use std::io::BufRead;
fn main() {
    for l in std::io::stdin().lock().lines() {
        let v: serde_json::Value = serde_json::from_str(&l.unwrap()).unwrap();
        let text = v["text"].as_str().unwrap();
        let mut parts: Vec<(String, bool)> = Vec::new();
        for (p, sep) in math::split_at_prose_separators(text) {
            if sep { parts.push((p.to_string(), true)); } else {
                for (q, s2) in math::split_sentences(p) { parts.push((q.to_string(), s2)); }
            }
        }
        println!("{}", serde_json::json!({"gid": v["gid"], "parts": parts}));
    }
}
