//! Planning: every decision the two stages would make on one file, each
//! with a code that flips it, and the text with the chosen flips applied.
//! The binary reports a plan or writes it; this module is the part tests can
//! drive (with `math::Model::fake` for the math stage).

use crate::BreakAction;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::path::PathBuf;

// ---------------------------------------------------------------- codes

pub const B32: &[u8; 32] = b"0123456789abcdefghjkmnpqrstvwxyz";
pub const CODE_LEN: usize = 5;

/// A code from a hash. FNV leaves its low bits nearly blind to the last bytes
/// of short, similar inputs (codes for neighbouring lines shared suffixes),
/// so fold and finish with splitmix64's mixer before taking characters.
pub fn b32(h: u128) -> String {
    let mix = |mut z: u64| {
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    };
    let a = mix((h as u64) ^ ((h >> 64) as u64).rotate_left(17));
    let b = mix(a ^ (h >> 64) as u64);
    let v = ((a as u128) << 64) | b as u128;
    (0..25).map(|k| B32[((v >> (k * 5)) & 31) as usize] as char).collect()
}

// ---------------------------------------------------------------- plans

#[derive(Clone)]
pub enum Detail {
    /// the confident joins of one paragraph, toggled together
    Para { first: usize, last: usize, joins: Vec<(usize, String, String)>, flipped: bool },
    Break { item: crate::BreakItem },
    Math { before: String, removed: String, added: String, after: String, applied: bool },
    Blank { after: bool, delim: String, applied: bool },
}

#[derive(Clone)]
pub struct Item {
    pub code: String,
    pub full: String,
    pub line: usize, // 1-based input line
    pub sure: bool,
    pub detail: Detail,
}

impl Item {
    pub fn flipped(&self) -> bool {
        match &self.detail {
            Detail::Para { flipped, .. } => *flipped,
            Detail::Break { item } => item.action != item.default,
            Detail::Math { applied, .. } | Detail::Blank { applied, .. } => !applied,
        }
    }
}

/// Run-wide tallies behind the one-line summaries.
#[derive(Default)]
pub struct Tally {
    pub math_refused: Vec<String>, // "path: line N[ cell M]: reason"
    pub math_files: BTreeSet<String>,
    pub math_skipped: bool,
    pub excluded: Vec<String>,
    pub foreign: Vec<String>,
}

pub struct Plan {
    pub ctx: String,
    pub path: Option<PathBuf>,
    pub input: String,
    pub output: String,
    pub items: Vec<Item>,
    /// given codes this plan used
    pub matched: Vec<String>,
}

pub fn matches_code(full: &str, given: &[String]) -> Vec<String> {
    given.iter().filter(|g| g.len() >= CODE_LEN && full.starts_with(g.as_str())).cloned().collect()
}

/// Shorten each code to CODE_LEN characters, or longer where two of this
/// file's codes would collide.
pub fn assign_codes(items: &mut [Item]) {
    let mut seen: HashMap<String, usize> = HashMap::new();
    for it in items.iter() {
        *seen.entry(it.full[..CODE_LEN].to_string()).or_default() += 1;
    }
    for it in items.iter_mut() {
        let n = if seen[&it.full[..CODE_LEN]] > 1 { 9 } else { CODE_LEN };
        it.code = it.full[..n].to_string();
    }
}

/// Plan one file's changes, with the given codes flipped.
pub fn plan_file(
    ctx: &str,
    key: &str,
    input: String,
    run_id: &str,
    clf: Option<&crate::classify::Classifier>,
    model: Option<&crate::math::Model>,
    given: &[String],
    explain: bool,
    tally: &mut Tally,
) -> Result<Plan, String> {
    let seed = format!("{:032x}", crate::fnv128(&[run_id.as_bytes(), key.as_bytes(), input.as_bytes()]));
    let code_for = |kind: &str, ident: &str| b32(crate::fnv128(&[seed.as_bytes(), kind.as_bytes(), ident.as_bytes()]));
    let mut matched: Vec<String> = Vec::new();

    // the default plan names the break decisions and their codes
    let explain_first = explain && given.is_empty();
    let r0 = crate::format_with(&input, clf, explain_first, &HashSet::new());
    let mut flips: HashSet<usize> = HashSet::new();
    let mut paras: Vec<(usize, Vec<usize>)> = Vec::new(); // para_start -> confident join lines
    for b in &r0.breaks {
        if b.sure() && b.default == BreakAction::Join {
            match paras.last_mut() {
                Some((s, v)) if *s == b.para_start => v.push(b.line),
                _ => paras.push((b.para_start, vec![b.line])),
            }
        } else {
            let m = matches_code(&code_for("break", &b.line.to_string()), given);
            if !m.is_empty() {
                flips.insert(b.line);
                matched.extend(m);
            }
        }
    }
    let mut para_flipped: HashSet<usize> = HashSet::new();
    for (start, lines) in &paras {
        let m = matches_code(&code_for("para", &start.to_string()), given);
        if !m.is_empty() {
            flips.extend(lines.iter().copied());
            para_flipped.insert(*start);
            matched.extend(m);
        }
    }
    // what each confident paragraph join looks like, from the default plan
    let para_details: Vec<(usize, Vec<(usize, String, String)>)> = paras
        .iter()
        .map(|(start, lines)| {
            let joins = r0
                .breaks
                .iter()
                .filter(|b| lines.contains(&b.line))
                .map(|b| (b.line, b.line_a.clone(), b.line_b.clone()))
                .collect();
            (*start, joins)
        })
        .collect();
    let r = if given.is_empty() { r0 } else { crate::format_with(&input, clf, explain, &flips) };

    // Built-in safety gate on the unwrap stage: render-equality before any
    // write — compared against the gate variant, because the two-space marks
    // deliberately change rendering (their narrower guarantee: only add
    // "  " at an existing break, or join at a soft break). The math pass
    // runs after with its own gates.
    if r.gate_output != input && crate::render_fingerprint(&input) != crate::render_fingerprint(&r.gate_output) {
        return Err(format!(
            "{ctx}: SKIPPED — result would change rendered document (bug or unsupported construct; please report)"
        ));
    }

    let mut items: Vec<Item> = Vec::new();
    // confident paragraph joins, one item each
    for (start, joins) in para_details {
        let first = joins[0].0;
        let last = joins[joins.len() - 1].0 + 1;
        items.push(Item {
            code: String::new(),
            full: code_for("para", &start.to_string()),
            line: first,
            sure: true,
            detail: Detail::Para { first, last, joins, flipped: para_flipped.contains(&start) },
        });
    }
    // every other break decision
    for b in &r.breaks {
        if b.sure() && b.default == BreakAction::Join {
            continue;
        }
        items.push(Item {
            code: String::new(),
            full: code_for("break", &b.line.to_string()),
            line: b.line,
            sure: b.sure(),
            detail: Detail::Break { item: b.clone() },
        });
    }

    // the math pass, its pieces declinable by code
    let mut output = r.output.clone();
    if let Some(m) = model {
        let mut occ: HashMap<String, usize> = HashMap::new();
        let mut decided: Vec<(usize, String, String, String, bool)> = Vec::new();
        let promoted = math_pass(&r.output, &r.math_sites, m, ctx, tally, &mut |line, orig, out| {
            let k = format!("{orig}\0{out}");
            let n = occ.entry(k.clone()).or_default();
            *n += 1;
            let full = code_for("math", &format!("{k}\0{n}"));
            let m = matches_code(&full, given);
            let applied = m.is_empty();
            matched.extend(m);
            decided.push((r.origins.get(line).copied().unwrap_or(line + 1), full, orig.to_string(), out.to_string(), applied));
            applied
        });
        for (line, full, orig, out, applied) in decided {
            let (before, removed, added, after) = minimal_change(&orig, &out);
            items.push(Item {
                code: String::new(),
                full,
                line,
                sure: true,
                detail: Detail::Math { before, removed, added, after, applied },
            });
        }
        let mut docc: HashMap<String, usize> = HashMap::new();
        let mut blank_codes: HashMap<(usize, bool), String> = HashMap::new();
        // list the insertions, decide them by code, then redo with declines
        let (text, blanks) = crate::math::fix_display_math_blanks_decided(&promoted, Some(&r.math_sites), &|_| false);
        let mut declined: HashSet<(usize, bool)> = HashSet::new();
        for b in &blanks {
            let k = format!("{}\0{}", b.after, b.delim);
            let n = docc.entry(k.clone()).or_default();
            *n += 1;
            let full = code_for("blank", &format!("{k}\0{n}"));
            let m = matches_code(&full, given);
            if !m.is_empty() {
                declined.insert((b.line, b.after));
                matched.extend(m);
            }
            blank_codes.insert((b.line, b.after), full);
        }
        output = if declined.is_empty() {
            text
        } else {
            crate::math::fix_display_math_blanks_decided(&promoted, Some(&r.math_sites), &|b| {
                declined.contains(&(b.line, b.after))
            })
            .0
        };
        for b in blanks {
            items.push(Item {
                code: String::new(),
                full: blank_codes[&(b.line, b.after)].clone(),
                line: r.origins.get(b.line).copied().unwrap_or(b.line + 1),
                sure: true,
                detail: Detail::Blank {
                    after: b.after,
                    delim: b.delim,
                    applied: !declined.contains(&(b.line, b.after)),
                },
            });
        }
    }
    items.sort_by_key(|i| (!i.sure, i.line));
    assign_codes(&mut items);
    Ok(Plan { ctx: ctx.to_string(), path: None, input, output, items, matched })
}

/// The shortest region where `a` and `b` differ, with up to 30 characters of
/// unchanged context on each side.
pub fn minimal_change(a: &str, b: &str) -> (String, String, String, String) {
    let ac: Vec<char> = a.chars().collect();
    let bc: Vec<char> = b.chars().collect();
    let pre = ac.iter().zip(&bc).take_while(|(x, y)| x == y).count();
    let max_suf = ac.len().min(bc.len()) - pre;
    let suf = ac.iter().rev().zip(bc.iter().rev()).take(max_suf).take_while(|(x, y)| x == y).count();
    let s = |v: &[char]| v.iter().collect::<String>();
    let ctx_l = pre.saturating_sub(30);
    let before = format!("{}{}", if ctx_l > 0 { "…" } else { "" }, s(&ac[ctx_l..pre]));
    let after_end = (ac.len() - suf + 30).min(ac.len());
    let after = format!("{}{}", s(&ac[ac.len() - suf..after_end]), if after_end < ac.len() { "…" } else { "" });
    (before.trim_start().to_string(), s(&ac[pre..ac.len() - suf]), s(&bc[pre..bc.len() - suf]), after.trim_end().to_string())
}

/// Apply the math pass (deterministic \(…\) normalization + LLM promotion)
/// to already-unwrapped text, promoting only at the prose sites the parse
/// identified — whole paragraph/heading lines, or table cells individually.
/// Code, frontmatter, and HTML have no sites and are structurally out of
/// reach, and so are the interiors of multi-line $$ display blocks. Each
/// changed piece is offered to `decide` (output line, original, result),
/// which returns whether to keep the change.
pub fn math_pass(
    input: &str,
    sites: &[crate::MathSite],
    model: &crate::math::Model,
    ctx: &str,
    tally: &mut Tally,
    decide: &mut dyn FnMut(usize, &str, &str) -> bool,
) -> String {
    use crate::MathSite;
    use crate::math::promote_site;
    let mut out_lines: Vec<String> = Vec::new();
    let mut in_display = false;
    let mut assemble = |i: usize, r: &crate::math::SiteResult| -> String {
        r.pieces
            .iter()
            .map(|(orig, res)| if orig != res && !decide(i, orig, res) { orig.as_str() } else { res.as_str() })
            .collect()
    };
    // carry each line's own ending (LF or CRLF, or none on a final line)
    // through untouched: `str::lines` drops `\r` and the final empty line
    for (i, raw) in input.split_inclusive('\n').enumerate() {
        let (line, eol) = match raw.strip_suffix("\r\n") {
            Some(l) => (l, "\r\n"),
            None => raw.strip_suffix('\n').map_or((raw, ""), |l| (l, "\n")),
        };
        let mut refused = |cell: Option<usize>, why: &str| {
            let at = match cell {
                Some(n) => format!("line {} cell {}", i + 1, n),
                None => format!("line {}", i + 1),
            };
            tally.math_refused.push(format!("{ctx}: {at}: {why}"));
            tally.math_files.insert(ctx.to_string());
        };
        let site = sites.get(i).cloned().unwrap_or(MathSite::None);
        // track multi-line $$ blocks: their interior lines are paragraph
        // text to the parser but LaTeX to the reader
        let t = line.trim_start_matches([' ', '\t', '>']).trim_end();
        let delims = t.matches("$$").count();
        let inside = in_display;
        if site != MathSite::None {
            if in_display {
                in_display = delims % 2 == 0;
            } else if t.starts_with("$$") && delims % 2 == 1 {
                in_display = true;
            }
        } else {
            in_display = false;
        }
        let new_line = match site {
            MathSite::None => line.to_string(),
            _ if inside => line.to_string(),
            MathSite::Whole => {
                let r = promote_site(model, line);
                tally.math_skipped |= r.skipped;
                for f in &r.flags {
                    refused(None, f);
                }
                assemble(i, &r)
            }
            MathSite::Cells(ranges) => {
                let mut s = String::with_capacity(line.len());
                let mut cursor = 0;
                for (n, &(a, b)) in ranges.iter().enumerate() {
                    if a < cursor || b > line.len() || a > b {
                        continue; // defensive: malformed range, leave bytes as-is
                    }
                    s.push_str(&line[cursor..a]);
                    let cell = &line[a..b];
                    // promote the trimmed interior; the cell's own padding is
                    // table formatting and must survive untouched
                    if cell.trim().is_empty() {
                        // an empty cell is all padding; lead and trail would
                        // both be the whole cell and double it every run
                        s.push_str(cell);
                        cursor = b;
                        continue;
                    }
                    let lead = &cell[..cell.len() - cell.trim_start().len()];
                    let trail = &cell[cell.trim_end().len()..];
                    let r = promote_site(model, cell.trim());
                    tally.math_skipped |= r.skipped;
                    for f in &r.flags {
                        refused(Some(n + 1), f);
                    }
                    // a proposal may never mint a new cell boundary
                    if r.text.matches('|').count() != cell.trim().matches('|').count() {
                        refused(Some(n + 1), "proposal changed table delimiters");
                        s.push_str(cell);
                    } else {
                        s.push_str(&format!("{lead}{}{trail}", assemble(i, &r)));
                    }
                    cursor = b;
                }
                s.push_str(&line[cursor..]);
                s
            }
        };
        out_lines.push(new_line + eol);
    }
    out_lines.concat()
}

