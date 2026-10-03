// For each file on stdin (one path per line): unwrap (plain), take the math
// sites, run the detector on each site text, and print TSV:
// path \t line \t excluded \t trigger-chars \t text
use std::io::BufRead;
use md_press::{MathSite, math};
fn triggers(s: &str) -> String {
    // re-derive which chars fired, outside code/$ (approximation of masked())
    let mut out = String::new();
    let (mut code, mut m) = (false, false);
    for c in s.chars() {
        match c { '`' if !m => code = !code, '$' if !code => m = !m,
          _ if code || m => {},
          _ => if "‖→↦≤≥≠≈≡·×∈∉∞±∂∇∑∏√⊂⊃⊆⊇∪∩∅¬𝒯𝒪𝒜𝒞𝓔𝓜αβγδεηθικλμνπρξστυφχψωζΓΘΛΞΣΦΨΩΠΔ".contains(c) && !out.contains(c) { out.push(c) } }
    }
    out
}
fn main() {
    let mut ex = md_press::exclude::Excluder::new();
    for p in std::io::stdin().lock().lines() {
        let p = p.unwrap();
        let Ok(input) = std::fs::read_to_string(&p) else { continue };
        let excluded = ex.excluded(std::path::Path::new(&p)).is_some();
        let r = md_press::format_plain(&input);
        for (i, line) in r.output.lines().enumerate() {
            let pieces: Vec<String> = match r.math_sites.get(i) {
                Some(MathSite::Whole) => vec![line.to_string()],
                Some(MathSite::Cells(rs)) => rs.iter().filter(|&&(a,b)| a<=b && b<=line.len()).map(|&(a,b)| line[a..b].trim().to_string()).collect(),
                _ => vec![],
            };
            for t in pieces {
                if math::needs_math_pass(&t) {
                    let short: String = t.chars().take(220).collect();
                    println!("{}\t{}\t{}\t{}\t{}", p, i+1, if excluded {"X"} else {"-"}, triggers(&t), short.replace('\t'," "));
                }
            }
        }
    }
}
