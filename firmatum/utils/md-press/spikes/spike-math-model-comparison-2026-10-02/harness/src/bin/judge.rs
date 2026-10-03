// Read JSON lines {text, proposal} on stdin; run md-press's real gates on
// each proposal (via the fake-model seam); print {outcome, reason, result}.
use md_press::math::{self, MathOutcome};
use std::io::BufRead;
fn main() {
    for l in std::io::stdin().lock().lines() {
        let v: serde_json::Value = serde_json::from_str(&l.unwrap()).unwrap();
        let text = v["text"].as_str().unwrap().to_string();
        let prop = v["proposal"].as_str().unwrap_or("").to_string();
        let m = math::Model::fake(move |_t: &str| prop.clone());
        let (o, why, res) = match math::promote_text(&m, &text) {
            MathOutcome::Unchanged => ("unchanged", String::new(), text.clone()),
            MathOutcome::Converted(c) => ("converted", String::new(), c),
            MathOutcome::Flagged(w, d) => ("refused", w.split(':').next().unwrap_or("").to_string(), d.unwrap_or(text.clone())),
            MathOutcome::Unavailable(_) => ("unavailable", String::new(), text.clone()),
        };
        let mut out = v.clone();
        out["outcome"] = o.into(); out["reason"] = why.into(); out["result"] = res.into();
        println!("{out}");
    }
}
