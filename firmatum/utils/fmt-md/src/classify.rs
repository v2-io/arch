//! The break classifier, in the binary: 12-feature extraction (exact parity
//! with model/features.py — the model was trained on those computations, so
//! serving must match them, including their blank-line block conventions),
//! forest evaluation, and the isotonic calibration lookup, all from the
//! committed model/model.json embedded at compile time.

use serde::Deserialize;

#[derive(Deserialize)]
struct TreeJson {
    feat: Vec<i32>,
    thr: Vec<f64>,
    left: Vec<i32>,
    right: Vec<i32>,
    value: Vec<[f64; 2]>,
}

#[derive(Deserialize)]
struct CalibJson {
    breakpoints_x: Vec<f64>,
    breakpoints_y: Vec<f64>,
}

#[derive(Deserialize)]
struct ModelJson {
    features: Vec<String>,
    trees: Vec<TreeJson>,
    calibration: CalibJson,
}

pub struct Classifier {
    model: ModelJson,
}

/// Feature order the exporter used; assert against model.json at load.
const FEATURES: [&str; 14] = [
    "b_first_cls", "b_start2", "blk_label_frac", "a_self_styled",
    "a_last_cls", "a_bold_cov", "blk_edge_var", "blk_int_labels",
    "a_end2", "a_width", "rejoin_w", "blk_frac_sent", "d_frac6090", "blk_n",
];

impl Classifier {
    pub fn load() -> Result<Self, String> {
        let model: ModelJson = serde_json::from_str(include_str!("../model/model.json"))
            .map_err(|e| format!("model.json: {e}"))?;
        if model.features != FEATURES {
            return Err(format!(
                "model.json feature order {:?} != compiled order {:?}",
                model.features, FEATURES
            ));
        }
        Ok(Self { model })
    }

    /// Calibrated P(phb) for the break after `lines[i]`, judged within the
    /// paragraph spanning source lines [pstart, pend) (0-based).
    pub fn p_phb(&self, lines: &[&str], pstart: usize, i: usize, pend: usize, fstats: &FileStats) -> f64 {
        let f = features(lines, pstart, i, pend, fstats);
        self.eval(&f)
    }

    /// Debug surface (--explain): calibrated p plus named feature values.
    pub fn explain(&self, lines: &[&str], pstart: usize, i: usize, pend: usize, fstats: &FileStats) -> (f64, Vec<(&'static str, f64)>) {
        let f = features(lines, pstart, i, pend, fstats);
        (self.eval(&f), FEATURES.iter().copied().zip(f).collect())
    }

    fn eval(&self, f: &[f64; 14]) -> f64 {
        let mut acc = 0.0;
        for t in &self.model.trees {
            let mut n = 0usize;
            loop {
                if t.left[n] < 0 {
                    let v = &t.value[n];
                    acc += v[1] / (v[0] + v[1]).max(1.0);
                    break;
                }
                let fi = t.feat[n] as usize;
                n = if f[fi] <= t.thr[n] { t.left[n] as usize } else { t.right[n] as usize };
            }
        }
        let raw = acc / self.model.trees.len() as f64;
        isotonic(&self.model.calibration, raw)
    }
}

fn isotonic(c: &CalibJson, x: f64) -> f64 {
    let bx = &c.breakpoints_x;
    let by = &c.breakpoints_y;
    if bx.is_empty() {
        return x;
    }
    if x <= bx[0] {
        return by[0];
    }
    if x >= bx[bx.len() - 1] {
        return by[by.len() - 1];
    }
    let j = bx.partition_point(|&v| v < x);
    let (x0, x1, y0, y1) = (bx[j - 1], bx[j], by[j - 1], by[j]);
    if (x1 - x0).abs() < 1e-12 { y1 } else { y0 + (y1 - y0) * (x - x0) / (x1 - x0) }
}

// ---- feature computation: parity port of model/features.py ---------------

const ABBREV: [&str; 9] = ["cf.", "e.g.", "i.e.", "pp.", "vs.", "etc.", "al.", "no.", "fig."];
const FSTRUCT: [&str; 6] = ["#", "|", ">", "```", "---", "==="];

fn width(s: &str) -> f64 {
    s.trim_end().chars().count() as f64
}

fn is_structural(line: &str) -> bool {
    let s = line.trim();
    if s.is_empty() {
        return true;
    }
    if FSTRUCT.iter().any(|p| s.starts_with(p)) {
        return true;
    }
    // LIST_RE: ^\s*([-*+]|\d+[.)])\s
    let t = line.trim_start();
    let mut ch = t.chars();
    match ch.next() {
        Some('-') | Some('*') | Some('+') => ch.next().is_some_and(|c| c.is_whitespace()),
        Some(d) if d.is_ascii_digit() => {
            let rest: String = t.chars().skip_while(|c| c.is_ascii_digit()).collect();
            let mut r = rest.chars();
            matches!(r.next(), Some('.') | Some(')')) && r.next().is_some_and(|c| c.is_whitespace())
        }
        _ => false,
    }
}

fn sent_end(s: &str) -> bool {
    let s = s.trim_end();
    let Some(last) = s.chars().last() else { return false };
    if !".?!:;".contains(last) {
        return false;
    }
    let lw = s.split_whitespace().last().unwrap_or("").to_lowercase();
    !ABBREV.iter().any(|a| lw.ends_with(a))
}

fn char_class(c: char) -> f64 {
    if c.is_alphabetic() {
        return if c.is_lowercase() { 1.0 } else { 2.0 };
    }
    if c.is_ascii_digit() {
        return 3.0;
    }
    match c {
        '.' | '?' | '!' => 4.0,
        ',' => 5.0,
        ';' => 6.0,
        ':' => 7.0,
        '—' | '-' => 8.0,
        ')' | ']' => 9.0,
        '"' | '\'' => 10.0,
        '*' => 11.0,
        '`' => 12.0,
        '(' | '[' => 13.0,
        '$' => 14.0,
        _ => 0.0,
    }
}

fn end2_class(s: &str) -> f64 {
    let s = s.trim_end();
    for (pat, code) in [(":**", 1.0), ("**:", 2.0), (".**", 3.0), ("**", 4.0)] {
        if s.ends_with(pat) {
            return code;
        }
    }
    let Some(last) = s.chars().last() else { return 0.0 };
    match last {
        ':' => 5.0,
        ';' => 6.0,
        ',' => 7.0,
        '.' | '?' | '!' => 8.0,
        ')' | ']' => 9.0,
        '"' => 10.0,
        '—' | '-' => 11.0,
        '`' => 12.0,
        '$' => 13.0,
        c if c.is_alphanumeric() => 14.0,
        _ => 0.0,
    }
}

fn start2_class(s: &str) -> f64 {
    let s = s.trim_start();
    if s.starts_with("**") {
        return 1.0;
    }
    let mut ch = s.chars();
    let Some(c0) = ch.next() else { return 0.0 };
    let c1 = ch.next();
    if c0.is_uppercase() {
        return match c1 {
            Some(c) if c.is_lowercase() => 2.0,
            Some(c) if c.is_uppercase() => 3.0,
            _ => 4.0,
        };
    }
    if c0.is_lowercase() {
        return 5.0;
    }
    if c0.is_ascii_digit() {
        return 6.0;
    }
    match c0 {
        '`' => 7.0,
        '*' => 8.0,
        '[' => 9.0,
        '(' => 10.0,
        '"' => 11.0,
        '$' => 12.0,
        '_' => 13.0,
        _ => 14.0,
    }
}

/// LABEL_RE parity: ^\s*\*\*[A-Z][^*\n]{0,50}?:?\*\*
fn is_label_line(line: &str) -> bool {
    let t = line.trim_start();
    let Some(rest) = t.strip_prefix("**") else { return false };
    let mut ch = rest.chars();
    if !ch.next().is_some_and(|c| c.is_ascii_uppercase()) {
        return false;
    }
    // closing ** within the next 51 non-* chars
    let mut count = 0;
    let bytes: Vec<char> = rest.chars().collect();
    for w in 1..bytes.len() {
        if bytes[w] == '*' {
            return bytes.get(w + 1) == Some(&'*');
        }
        count += 1;
        if count > 51 {
            return false;
        }
    }
    false
}

pub struct FileStats {
    pub frac6090: f64,
}

pub fn file_stats(lines: &[&str]) -> FileStats {
    let widths: Vec<f64> = lines
        .iter()
        .filter(|l| !l.trim().is_empty() && !is_structural(l))
        .map(|l| width(l))
        .collect();
    if widths.len() < 3 {
        // small-file fallback (port requirement discovered 2026-07-22)
        return FileStats { frac6090: 0.2 };
    }
    let n = widths.len() as f64;
    let f = widths.iter().filter(|&&w| (60.0..=90.0).contains(&w)).count() as f64;
    FileStats { frac6090: f / n }
}

/// Styling self-containment (Joseph, 2026-07-22): a line that opens and
/// closes its own inline styling is likelier an independent unit.
fn self_styled(s: &str) -> (f64, f64) {
    let bolds = s.matches("**").count();
    let ticks = s.chars().filter(|&c| c == '`').count();
    let dollars = s.chars().filter(|&c| c == '$').count();
    let balanced = bolds % 2 == 0 && ticks % 2 == 0 && dollars % 2 == 0;
    let selfs = (s.starts_with("**") || s.starts_with('*') || s.starts_with('`') || s.starts_with('_'))
        && bolds >= 2
        && balanced;
    let mut cov = 0.0;
    if bolds >= 2 && !s.is_empty() {
        let (mut inside, mut n_in, mut total) = (false, 0usize, 0usize);
        let chars: Vec<char> = s.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            if chars[i] == '*' && chars.get(i + 1) == Some(&'*') {
                inside = !inside;
                i += 2;
                total += 2;
                continue;
            }
            if inside {
                n_in += 1;
            }
            total += 1;
            i += 1;
        }
        cov = n_in as f64 / total.max(1) as f64;
    }
    (if selfs { 1.0 } else { 0.0 }, cov)
}

fn features(lines: &[&str], pstart: usize, i: usize, pend: usize, fs: &FileStats) -> [f64; 14] {
    let a = lines[i].trim_end(); // marker-blind
    let b = lines[i + 1];
    let bs = b.trim();
    let b_first: &str = bs.split_whitespace().next().unwrap_or("");
    let blk: Vec<&str> = (pstart..pend).map(|k| lines[k]).collect();
    let widths: Vec<f64> = blk.iter().map(|l| width(l)).collect();
    let edge: &[f64] = if widths.len() > 1 { &widths[..widths.len() - 1] } else { &widths };
    let edge_var = if edge.len() > 1 {
        let m = edge.iter().sum::<f64>() / edge.len() as f64;
        edge.iter().map(|w| (w - m) * (w - m)).sum::<f64>() / edge.len() as f64
    } else {
        0.0
    };
    let labels_n = blk.iter().filter(|l| is_label_line(l)).count() as f64;
    let frac_sent = blk.iter().filter(|l| sent_end(l)).count() as f64 / blk.len() as f64;
    let (a_self, a_cov) = self_styled(a.trim_start());
    [
        bs.chars().next().map_or(0.0, char_class), // b_first_cls
        start2_class(bs),                          // b_start2
        labels_n / blk.len() as f64,               // blk_label_frac
        a_self,                                    // a_self_styled
        a.chars().last().map_or(0.0, char_class),  // a_last_cls
        a_cov,                                     // a_bold_cov
        edge_var,                                  // blk_edge_var
        labels_n,                                  // blk_int_labels
        end2_class(a),                             // a_end2
        width(a),                                  // a_width
        width(a) + 1.0 + b_first.chars().count() as f64, // rejoin_w
        frac_sent,                                 // blk_frac_sent
        fs.frac6090,                               // d_frac6090
        blk.len() as f64,                          // blk_n
    ]
}
