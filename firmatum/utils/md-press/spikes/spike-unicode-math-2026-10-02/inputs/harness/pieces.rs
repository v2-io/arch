// Emit (as JSON lines) every piece md-press's math stage would send to the
// model for the files named on stdin, using a recording fake model.
use md_press::{MathSite, math};
use std::{cell::RefCell, io::BufRead, rc::Rc};
fn main() {
    let seen = Rc::new(RefCell::new(Vec::<String>::new()));
    let log = seen.clone();
    let model = math::Model::fake(move |t: &str| { log.borrow_mut().push(t.to_string()); t.to_string() });
    for p in std::io::stdin().lock().lines() {
        let p = p.unwrap();
        let Ok(input) = std::fs::read_to_string(&p) else { continue };
        let r = md_press::format_plain(&input);
        let mut in_display = false;
        for (i, line) in r.output.lines().enumerate() {
            let site = r.math_sites.get(i).cloned().unwrap_or(MathSite::None);
            let t = line.trim_start_matches([' ', '\t', '>']).trim_end();
            let d = t.matches("$$").count();
            let inside = in_display;
            if site != MathSite::None { if in_display { in_display = d % 2 == 0 } else if t.starts_with("$$") && d % 2 == 1 { in_display = true } } else { in_display = false }
            if inside { continue }
            match site {
                MathSite::Whole => { math::promote_site(&model, line); }
                MathSite::Cells(rs) => for &(a, b) in &rs { if a <= b && b <= line.len() { math::promote_site(&model, line[a..b].trim()); } },
                MathSite::None => {}
            }
            for piece in seen.borrow_mut().drain(..) {
                println!("{}", serde_json::json!({"file": p, "line": i + 1, "text": piece}));
            }
        }
    }
}
