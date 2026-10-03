// Every math site (whole prose line, or table cell) md-press's parse finds in
// the files named on stdin, as JSON lines: {file, line, cell, pre, body, suf,
// trig, display}. `body` is the site with markdown structure split off (what a
// converter may edit); `trig` is md-press's own needs_math_pass on the body;
// `display` marks lines inside a multi-line $$ block (never edited).
// Files under a .md-pressignore and .udon files are skipped, as md-press does.
use md_press::{MathSite, math};
use std::io::BufRead;

fn main() {
    let mut ex = md_press::exclude::Excluder::new();
    for p in std::io::stdin().lock().lines() {
        let p = p.unwrap();
        let path = std::path::Path::new(&p);
        if md_press::exclude::foreign_language(path).is_some() || ex.excluded(path).is_some() {
            continue;
        }
        let Ok(input) = std::fs::read_to_string(&p) else { continue };
        let r = md_press::format_plain(&input);
        let mut in_display = false;
        for (i, line) in r.output.lines().enumerate() {
            let site = r.math_sites.get(i).cloned().unwrap_or(MathSite::None);
            let t = line.trim_start_matches([' ', '\t', '>']).trim_end();
            let d = t.matches("$$").count();
            let inside = in_display;
            if site != MathSite::None {
                if in_display { in_display = d % 2 == 0 } else if t.starts_with("$$") && d % 2 == 1 { in_display = true }
            } else { in_display = false }
            let emit = |cell: Option<usize>, text: &str| {
                let (pre, body, suf) = math::split_structure(text);
                println!("{}", serde_json::json!({"file": p, "line": i + 1, "cell": cell, "pre": pre, "body": body,
                    "suf": suf, "trig": math::needs_math_pass(body), "display": inside}));
            };
            match site {
                MathSite::Whole => emit(None, line),
                MathSite::Cells(rs) => for (n, &(a, b)) in rs.iter().enumerate() {
                    if a <= b && b <= line.len() { emit(Some(n + 1), line[a..b].trim()) }
                },
                MathSite::None => {}
            }
        }
    }
}
