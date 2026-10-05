//! The MCP tools: `identify`, `range`, `search`, `blocks`.
//!
//! Each handler takes the call's `arguments` object and returns either a JSON
//! value (rendered to text for the model) or a message for an `isError` result.
//! Every handler is a pure function of its arguments and the embedded UCD.

use crate::ucd::{self, NameTarget, NoteKind, Ucd, MAX_CP, UCD_VERSION};
use serde_json::{json, Map, Value};

pub const MAX_TEXT_CHARS: usize = 4000;
pub const MAX_ITEMS: usize = 512;
pub const MAX_RANGE: u32 = 512;
pub const MAX_SEARCH_LIMIT: usize = 500;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum Detail {
    Brief,
    Standard,
    Full,
}

impl Detail {
    fn parse(v: Option<&Value>, default: Detail) -> Result<Detail, String> {
        match v.and_then(Value::as_str) {
            None => Ok(default),
            Some("brief") => Ok(Detail::Brief),
            Some("standard") => Ok(Detail::Standard),
            Some("full") => Ok(Detail::Full),
            Some(o) => Err(format!("detail must be \"brief\", \"standard\" or \"full\", not {o:?}")),
        }
    }
    fn max_chars(self) -> usize {
        match self {
            Detail::Brief => 512,
            Detail::Standard => 128,
            Detail::Full => 32,
        }
    }
}

// ------------------------------------------------------------------ schemas

pub fn tool_list() -> Value {
    let v = UCD_VERSION;
    json!([
        {
            "name": "identify",
            "title": "Identify characters",
            "description": format!(
"Look up Unicode characters (Unicode {v}). Give glyphs as `text` (every code point in it is identified, in order of first appearance; named emoji/character sequences in it are reported too), and/or code points as `codepoints` (\"U+2460\", \"0x2460\", \"\\\\u2460\", \"&#x2460;\", bare hex like \"2460\", or a single glyph), and/or exact character `names` (\"CIRCLED DIGIT ONE\"; case/spacing/underscores ignored; aliases, named sequences and emoji names like \"flag: Japan\" accepted). \
detail=\"brief\": code point, glyph, name, category, block. \"standard\" (default) adds the code-chart heading the character sits under, script, Unicode version of first assignment, numeric value, decomposition, case mappings, aliases, Unicode's own notes and cross-references to related characters, emoji properties, and for Han ideographs the Unihan definition, readings and radical/strokes. \"full\" adds bidi class, combining class, East Asian width, UTF-8/UTF-16 encodings, decimal value, variation sequences and more Unihan readings."),
            "inputSchema": {
                "type": "object",
                "properties": {
                    "text": {"type": "string", "description": "Glyphs to identify (any string; each distinct code point is reported)."},
                    "codepoints": {"type": "array", "items": {"type": "string"}, "description": "Code points such as \"U+33F3\", \"0x1F0A1\", \"2460\" (bare strings of 2+ hex digits are hex), or single glyphs."},
                    "names": {"type": "array", "items": {"type": "string"}, "description": "Exact Unicode character names or aliases, e.g. \"WHITE CIRCLE\", \"CJK UNIFIED IDEOGRAPH-4E00\"."},
                    "detail": {"type": "string", "enum": ["brief", "standard", "full"], "description": "Amount of information per character (default \"standard\")."}
                }
            },
            "annotations": {"readOnlyHint": true, "destructiveHint": false, "idempotentHint": true, "openWorldHint": false}
        },
        {
            "name": "range",
            "title": "List a code point range",
            "description": format!(
"List consecutive code points (Unicode {v}), i.e. a character's neighbours in the code charts. Either `around` a glyph or code point with `before` and/or `after` counts, or from `start` to `end` inclusive. At most {MAX_RANGE} code points. Runs of unassigned, private-use, surrogate or noncharacter code points are collapsed into one row. detail defaults to \"brief\"."),
            "inputSchema": {
                "type": "object",
                "properties": {
                    "around": {"type": "string", "description": "Centre: a single glyph or a code point (\"U+2474\")."},
                    "before": {"type": "integer", "description": "How many code points below `around` to include (default 0)."},
                    "after": {"type": "integer", "description": "How many code points above `around` to include (default 0)."},
                    "start": {"type": "string", "description": "First code point (glyph or \"U+3358\")."},
                    "end": {"type": "string", "description": "Last code point, inclusive."},
                    "detail": {"type": "string", "enum": ["brief", "standard", "full"], "description": "Amount of information per character (default \"brief\")."}
                }
            },
            "annotations": {"readOnlyHint": true, "destructiveHint": false, "idempotentHint": true, "openWorldHint": false}
        },
        {
            "name": "search",
            "title": "Search character names",
            "description": format!(
"Find characters whose Unicode {v} name, formal alias, or informal alias contains the words of `query` (all words must match; case-insensitive). mode \"prefix\" (default): each query word matches the start of a word in the name (\"circ dig\" finds CIRCLED DIGIT ONE); \"word\": whole words only; \"substring\": anywhere. Optional filters: `block` (block name or part of it), `script` (e.g. \"Latin\"), `category` (general category, e.g. \"So\", \"Nd\", or \"S\" for all symbols). With han_definitions=true the query is also matched against Unihan English definitions of Han ideographs. Also matches named and emoji sequences, and reports blocks whose names match (use `range` or `blocks` to list them). Results are in code point order (exact name match first), up to `limit` (default 50, max {MAX_SEARCH_LIMIT}); the total match count is always reported."),
            "inputSchema": {
                "type": "object",
                "properties": {
                    "query": {"type": "string", "description": "Words to find in character names, e.g. \"circled number\", \"tai xuan jing\"."},
                    "mode": {"type": "string", "enum": ["prefix", "word", "substring"]},
                    "block": {"type": "string"},
                    "script": {"type": "string"},
                    "category": {"type": "string"},
                    "han_definitions": {"type": "boolean"},
                    "limit": {"type": "integer"}
                },
                "required": ["query"]
            },
            "annotations": {"readOnlyHint": true, "destructiveHint": false, "idempotentHint": true, "openWorldHint": false}
        },
        {
            "name": "blocks",
            "title": "Unicode blocks and chart headings",
            "description": format!(
"Unicode {v} blocks. With no `query`: every block with its range. With `query` = a block name (or part of one), a glyph, or a code point: the matching block(s); when exactly one block matches, also its code-chart headings (the sub-groups the Unicode code charts divide the block into, each with its code point range and Unicode's notes) and how many of its code points are assigned."),
            "inputSchema": {
                "type": "object",
                "properties": {
                    "query": {"type": "string", "description": "Block name or part of one (\"enclosed\"), or a glyph, or a code point."}
                }
            },
            "annotations": {"readOnlyHint": true, "destructiveHint": false, "idempotentHint": true, "openWorldHint": false}
        }
    ])
}

pub fn call(u: &Ucd, name: &str, args: &Value) -> Option<Result<Value, String>> {
    let empty = Map::new();
    let a = args.as_object().unwrap_or(&empty);
    Some(match name {
        "identify" => identify(u, a),
        "range" => range(u, a),
        "search" => search(u, a),
        "blocks" => blocks(u, a),
        _ => return None,
    })
}

// ------------------------------------------------------------------ parsing

/// Parse one code point token. Accepts U+XXXX, 0xXXXX, \uXXXX, \u{XXXX},
/// \UXXXXXXXX, &#xXXXX;, &#NNNN;, bare hex of 2..=6 digits, or one glyph.
pub fn parse_cp(tok: &str) -> Result<u32, String> {
    let t = tok.trim();
    let mut chars = t.chars();
    if let (Some(c), None) = (chars.next(), chars.next()) {
        return Ok(c as u32);
    }
    if t.is_empty() {
        // a lone whitespace glyph was trimmed away; take it literally
        if let Some(c) = tok.chars().next() {
            if tok.chars().count() == 1 {
                return Ok(c as u32);
            }
        }
        return Err("empty code point".into());
    }
    let lower = t.to_ascii_lowercase();
    let (digits, radix) = if let Some(r) = lower.strip_prefix("u+") {
        (r.to_string(), 16)
    } else if let Some(r) = lower.strip_prefix("0x") {
        (r.to_string(), 16)
    } else if let Some(r) = lower.strip_prefix("\\u{").and_then(|r| r.strip_suffix('}')) {
        (r.to_string(), 16)
    } else if let Some(r) = lower.strip_prefix("\\u") {
        (r.to_string(), 16)
    } else if let Some(r) = lower.strip_prefix("&#x").and_then(|r| r.strip_suffix(';')) {
        (r.to_string(), 16)
    } else if let Some(r) = lower.strip_prefix("&#").and_then(|r| r.strip_suffix(';')) {
        (r.to_string(), 10)
    } else if (2..=6).contains(&t.len()) && t.bytes().all(|b| b.is_ascii_hexdigit()) {
        (lower.clone(), 16)
    } else {
        return Err(format!(
            "{tok:?} is not a code point (use \"U+XXXX\", bare hex of 2+ digits, or a single glyph; put strings of glyphs in `text`)"
        ));
    };
    let n = u32::from_str_radix(&digits, radix).map_err(|_| format!("{tok:?}: bad digits"))?;
    if digits.is_empty() || digits.len() > 8 || n > MAX_CP {
        return Err(format!("{tok:?} is outside the code space U+0000..U+10FFFF"));
    }
    Ok(n)
}

fn parse_cp_value(v: &Value) -> Result<u32, String> {
    match v {
        Value::String(s) => parse_cp(s),
        Value::Number(n) => match n.as_u64() {
            Some(x) if x <= MAX_CP as u64 => Ok(x as u32),
            _ => Err(format!("{n} is not a code point (0..=1114111)")),
        },
        o => Err(format!("{o} is not a code point")),
    }
}

fn get_str<'a>(a: &'a Map<String, Value>, k: &str) -> Result<Option<&'a str>, String> {
    match a.get(k) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(s)) => Ok(Some(s)),
        Some(o) => Err(format!("`{k}` must be a string, got {o}")),
    }
}

fn get_uint(a: &Map<String, Value>, k: &str) -> Result<Option<u64>, String> {
    match a.get(k) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::Number(n)) => n
            .as_u64()
            .or_else(|| n.as_f64().filter(|f| *f >= 0.0 && f.fract() == 0.0).map(|f| f as u64))
            .map(Some)
            .ok_or_else(|| format!("`{k}` must be a non-negative integer")),
        Some(Value::String(s)) => s.trim().parse::<u64>().map(Some).map_err(|_| format!("`{k}` must be a non-negative integer")),
        Some(o) => Err(format!("`{k}` must be a non-negative integer, got {o}")),
    }
}

fn get_list<'a>(a: &'a Map<String, Value>, k: &str) -> Result<Vec<&'a Value>, String> {
    match a.get(k) {
        None | Some(Value::Null) => Ok(vec![]),
        Some(Value::Array(v)) => Ok(v.iter().collect()),
        // Be forgiving: a single string where a list was expected.
        Some(v @ Value::String(_)) | Some(v @ Value::Number(_)) => Ok(vec![v]),
        Some(o) => Err(format!("`{k}` must be a list, got {o}")),
    }
}

// ------------------------------------------------------------------ rendering

pub fn cp_str(cp: u32) -> String {
    format!("U+{cp:04X}")
}

fn glyph(cp: u32) -> Option<String> {
    char::from_u32(cp).map(String::from)
}

fn cps_glyphs(cps: &[u32]) -> String {
    cps.iter().filter_map(|&c| char::from_u32(c)).collect()
}

fn parse_cp_list(s: &str) -> Vec<u32> {
    s.split_whitespace()
        .filter_map(|t| u32::from_str_radix(t.trim_start_matches("U+"), 16).ok())
        .collect()
}

/// "U+25EF ◯ LARGE CIRCLE"
fn one_line(u: &Ucd, cp: u32) -> String {
    match glyph(cp).filter(|_| printable(u, cp)) {
        Some(g) => format!("{} {} {}", cp_str(cp), g, u.display_name(cp)),
        None => format!("{} {}", cp_str(cp), u.display_name(cp)),
    }
}

/// Whether showing the raw glyph inline is useful (not a control/format/space/unassigned).
fn printable(u: &Ucd, cp: u32) -> bool {
    !matches!(u.general_category(cp), "Cc" | "Cf" | "Cs" | "Cn" | "Zl" | "Zp")
}

fn cross_ref(u: &Ucd, text: &str) -> String {
    // "(white circle - 25CB)" or "25CB"
    let inner = text.trim().trim_start_matches('(').trim_end_matches(')');
    let hexpart = inner.rsplit_once(" - ").map(|(_, h)| h).unwrap_or(inner).trim();
    if (4..=6).contains(&hexpart.len()) && hexpart.bytes().all(|b| b.is_ascii_hexdigit()) {
        if let Ok(cp) = u32::from_str_radix(hexpart, 16) {
            return one_line(u, cp);
        }
    }
    text.to_string()
}

pub fn char_json(u: &Ucd, cp: u32, detail: Detail) -> Value {
    let mut m = Map::new();
    let gc = u.general_category(cp);
    m.insert("cp".into(), json!(cp_str(cp)));
    if let Some(g) = glyph(cp) {
        m.insert("char".into(), json!(g));
    }
    match u.name(cp) {
        Some(n) => {
            m.insert("name".into(), json!(n));
        }
        None => {
            m.insert("name".into(), Value::Null);
            m.insert("label".into(), json!(u.label(cp)));
        }
    }
    let gc_long = u.long_value("gc", gc);
    if detail == Detail::Brief {
        m.insert("category".into(), json!(gc));
    } else {
        m.insert("category".into(), json!(format!("{gc} ({gc_long})")));
    }
    if let Some(b) = u.block(cp) {
        m.insert("block".into(), json!(b.name));
    } else {
        m.insert("block".into(), json!("No_Block"));
    }
    if gc == "Cn" {
        m.insert("status".into(), json!(if ucd::is_noncharacter(cp) { "noncharacter (permanently reserved)" } else { "unassigned (reserved)" }));
    }
    if detail == Detail::Brief {
        if let Some((_, v)) = u.numeric(cp) {
            m.insert("numeric".into(), json!(v));
        }
        return Value::Object(m);
    }

    if let Some(al) = u.aliases.get(&cp) {
        let v: Vec<String> = al.iter().map(|(a, t)| format!("{a} ({t})")).collect();
        m.insert("aliases".into(), json!(v));
    }
    if let Some(sh) = u.subheader(cp) {
        m.insert("chart_heading".into(), json!(sh.title));
    }
    m.insert("script".into(), json!(u.script(cp)));
    if let Some(sx) = u.script_extensions(cp) {
        m.insert("script_extensions".into(), json!(sx));
    }
    m.insert("age".into(), json!(u.age(cp)));
    if let Some((t, v)) = u.numeric(cp) {
        m.insert("numeric".into(), json!({"type": t, "value": v}));
    }

    let ud = u.ud(cp);
    if let Some(d) = ud.map(|d| d.decomp).filter(|d| !d.is_empty()) {
        let (kind, rest) = match d.strip_prefix('<') {
            Some(r) => {
                let (tag, rest) = r.split_once('>').unwrap_or((r, ""));
                (tag.to_string(), rest)
            }
            None => ("canonical".to_string(), d),
        };
        let cps = parse_cp_list(rest);
        m.insert(
            "decomposition".into(),
            json!({"type": kind, "to": cps_glyphs(&cps), "codepoints": cps.iter().map(|&c| one_line(u, c)).collect::<Vec<_>>()}),
        );
    } else if let Some(cps) = Ucd::hangul_decomposition(cp) {
        m.insert(
            "decomposition".into(),
            json!({"type": "canonical (algorithmic Hangul)", "to": cps_glyphs(&cps), "codepoints": cps.iter().map(|&c| one_line(u, c)).collect::<Vec<_>>()}),
        );
    }
    if let Some(d) = ud {
        let mut case = Map::new();
        for (k, v) in [("upper", d.upper), ("lower", d.lower), ("title", d.title)] {
            if !v.is_empty() && !(k == "title" && v == d.upper) {
                let c = parse_cp_list(v);
                case.insert(k.into(), json!(format!("{} {}", cps_glyphs(&c), c.iter().map(|&x| cp_str(x)).collect::<Vec<_>>().join(" "))));
            }
        }
        if !case.is_empty() {
            m.insert("case".into(), Value::Object(case));
        }
    }

    let notes = u.notes(cp);
    let pick = |want: fn(&NoteKind) -> bool| -> Vec<&str> { notes.iter().filter(|(k, _)| want(k)).map(|(_, t)| *t).collect() };
    let informal = pick(|k| matches!(k, NoteKind::Alias));
    if !informal.is_empty() {
        m.insert("informal_aliases".into(), json!(informal));
    }
    let comments: Vec<&str> = notes.iter().filter(|(k, _)| matches!(k, NoteKind::Comment | NoteKind::Notice)).map(|(_, t)| *t).collect();
    if !comments.is_empty() {
        m.insert("notes".into(), json!(comments));
    }
    let xrefs: Vec<String> = pick(|k| matches!(k, NoteKind::CrossRef)).iter().map(|t| cross_ref(u, t)).collect();
    if !xrefs.is_empty() {
        m.insert("see_also".into(), json!(xrefs));
    }
    let emoji = u.emoji_props(cp);
    if !emoji.is_empty() {
        m.insert("emoji".into(), json!(emoji));
    }

    let han = u.unihan(cp);
    if !han.is_empty() {
        let mut h = Map::new();
        let standard_keys = [
            "kDefinition", "kMandarin", "kTotalStrokes", "kRSUnicode", "kSimplifiedVariant", "kTraditionalVariant",
            "kPrimaryNumeric", "kAccountingNumeric", "kOtherNumeric",
        ];
        for (k, v) in han {
            if detail == Detail::Standard && !standard_keys.contains(k) {
                continue;
            }
            let key = k.trim_start_matches('k');
            let val: Value = if k.ends_with("Variant") {
                json!(v.split_whitespace().map(|t| cross_ref(u, t.split('<').next().unwrap_or(t).trim_start_matches("U+"))).collect::<Vec<_>>())
            } else {
                json!(*v)
            };
            h.insert(key.to_string(), val);
            if *k == "kRSUnicode" {
                if let Some(r) = radical(u, v) {
                    h.insert("radical".into(), json!(r));
                }
            }
        }
        m.insert("han".into(), Value::Object(h));
    }

    if detail == Detail::Full {
        let variations = pick(|k| matches!(k, NoteKind::Variation));
        if !variations.is_empty() {
            m.insert("variation_sequences".into(), json!(variations));
        }
        if let Some(d) = ud {
            m.insert("bidi_class".into(), json!(format!("{} ({})", d.bidi, u.long_value("bc", d.bidi))));
            m.insert("bidi_mirrored".into(), json!(d.mirrored));
            m.insert("combining_class".into(), json!(format!("{} ({})", d.ccc, u.long_value("ccc", d.ccc))));
        }
        let ea = u.east_asian_width(cp);
        m.insert("east_asian_width".into(), json!(format!("{ea} ({})", u.long_value("ea", ea))));
        m.insert("decimal".into(), json!(cp));
        if let Some(c) = char::from_u32(cp) {
            let mut b = [0u8; 4];
            let s = c.encode_utf8(&mut b);
            m.insert("utf8".into(), json!(s.bytes().map(|x| format!("{x:02X}")).collect::<Vec<_>>().join(" ")));
            let mut w = [0u16; 2];
            m.insert("utf16".into(), json!(c.encode_utf16(&mut w).iter().map(|x| format!("{x:04X}")).collect::<Vec<_>>().join(" ")));
        }
    }
    Value::Object(m)
}

/// kRSUnicode "85.6" → "⽔ KANGXI RADICAL WATER (#85) + 6 strokes"
fn radical(u: &Ucd, rs: &str) -> Option<String> {
    let first = rs.split_whitespace().next()?;
    let (r, s) = first.split_once('.')?;
    let simplified = r.ends_with('\'') || r.ends_with('"');
    let n: u32 = r.trim_end_matches(['\'', '"']).parse().ok()?;
    if !(1..=214).contains(&n) {
        return None;
    }
    let rc = 0x2F00 + n - 1;
    Some(format!(
        "{} {} (#{n}{}) + {s} strokes",
        glyph(rc).unwrap_or_default(),
        u.display_name(rc),
        if simplified { ", simplified form" } else { "" }
    ))
}

fn seq_json(u: &Ucd, i: usize) -> Value {
    let s = &u.sequences[i];
    json!({
        "sequence": cps_glyphs(&s.cps),
        "codepoints": s.cps.iter().map(|&c| cp_str(c)).collect::<Vec<_>>().join(" "),
        "name": s.name,
        "kind": s.kind,
    })
}

// ------------------------------------------------------------------ identify

fn identify(u: &Ucd, a: &Map<String, Value>) -> Result<Value, String> {
    let detail = Detail::parse(a.get("detail"), Detail::Standard)?;
    let text = get_str(a, "text")?;
    let cps_in = get_list(a, "codepoints")?;
    let names_in = get_list(a, "names")?;
    if text.is_none() && cps_in.is_empty() && names_in.is_empty() {
        return Err("give at least one of `text`, `codepoints`, `names`".into());
    }
    let mut order: Vec<u32> = Vec::new();
    let mut seen = std::collections::HashSet::new();
    let mut out = Map::new();
    out.insert("unicode_version".into(), json!(UCD_VERSION));
    let mut unresolved: Vec<Value> = Vec::new();
    let mut seqs: Vec<Value> = Vec::new();

    if let Some(t) = text {
        let all: Vec<u32> = t.chars().map(|c| c as u32).collect();
        if all.len() > MAX_TEXT_CHARS {
            return Err(format!("`text` has {} code points; the limit is {MAX_TEXT_CHARS}", all.len()));
        }
        if all.len() <= 64 {
            out.insert("text_codepoints".into(), json!(all.iter().map(|&c| cp_str(c)).collect::<Vec<_>>().join(" ")));
        } else {
            out.insert("text_length".into(), json!(all.len()));
        }
        let mut i = 0;
        let mut seen_seq = std::collections::HashSet::new();
        while i < all.len() {
            if let Some(s) = u.sequence_at(&all, i) {
                if seen_seq.insert(s) {
                    seqs.push(seq_json(u, s));
                }
                i += u.sequences[s].cps.len();
            } else {
                i += 1;
            }
        }
        for c in all {
            if seen.insert(c) {
                order.push(c);
            }
        }
    }
    for v in cps_in {
        match parse_cp_value(v) {
            Ok(c) => {
                if seen.insert(c) {
                    order.push(c);
                }
            }
            Err(e) => unresolved.push(json!({"input": v, "error": e})),
        }
    }
    for v in names_in {
        let Some(n) = v.as_str() else {
            unresolved.push(json!({"input": v, "error": "names must be strings"}));
            continue;
        };
        match u.lookup_name(n) {
            Some(NameTarget::Cp(c)) => {
                if seen.insert(c) {
                    order.push(c);
                }
            }
            Some(NameTarget::Seq(s)) => seqs.push(seq_json(u, s)),
            None => unresolved.push(json!({"input": n, "error": "no character, alias or named sequence has this name (try the search tool)"})),
        }
    }
    if order.len() > MAX_ITEMS {
        return Err(format!("{} distinct code points requested; the limit is {MAX_ITEMS}", order.len()));
    }
    let cap = detail.max_chars();
    let shown = order.len().min(cap);
    let rows: Vec<Value> = order[..shown].iter().map(|&c| char_json(u, c, detail)).collect();
    out.insert("characters".into(), Value::Array(rows));
    if shown < order.len() {
        out.insert(
            "truncated".into(),
            json!(format!("{} of {} code points shown; at detail={} at most {cap} are returned per call (use detail=\"brief\" or split the request)", shown, order.len(), detail_name(detail))),
        );
    }
    if !seqs.is_empty() {
        out.insert("sequences".into(), Value::Array(seqs));
    }
    if !unresolved.is_empty() {
        out.insert("unresolved".into(), Value::Array(unresolved));
    }
    Ok(Value::Object(out))
}

fn detail_name(d: Detail) -> &'static str {
    match d {
        Detail::Brief => "brief",
        Detail::Standard => "standard",
        Detail::Full => "full",
    }
}

// ------------------------------------------------------------------ range

fn range(u: &Ucd, a: &Map<String, Value>) -> Result<Value, String> {
    let detail = Detail::parse(a.get("detail"), Detail::Brief)?;
    let (lo, hi) = if let Some(c) = get_str(a, "around")? {
        let c = parse_cp(c)?;
        let before = get_uint(a, "before")?.unwrap_or(0).min(MAX_CP as u64) as u32;
        let after = get_uint(a, "after")?.unwrap_or(0).min(MAX_CP as u64) as u32;
        (c.saturating_sub(before), c.saturating_add(after).min(MAX_CP))
    } else {
        let s = get_str(a, "start")?.ok_or("give `around` (with `before`/`after`) or `start` and `end`")?;
        let s = parse_cp(s)?;
        let e = match get_str(a, "end")? {
            Some(e) => parse_cp(e)?,
            None => return Err("`end` is required with `start`".into()),
        };
        if e < s { (e, s) } else { (s, e) }
    };
    let span = hi - lo + 1;
    if span > MAX_RANGE {
        return Err(format!("{span} code points requested ({}..{}); the limit is {MAX_RANGE} per call", cp_str(lo), cp_str(hi)));
    }
    if detail > Detail::Brief && span > detail.max_chars() as u32 {
        return Err(format!("at detail={} the limit is {} code points per call", detail_name(detail), detail.max_chars()));
    }
    let mut rows = Vec::new();
    let mut cp = lo;
    while cp <= hi {
        let kind = gap_kind(u, cp);
        if let Some(k) = kind {
            let mut end = cp;
            while end < hi && gap_kind(u, end + 1) == Some(k) {
                end += 1;
            }
            let mut r = Map::new();
            r.insert("cp".into(), json!(if end > cp { format!("{}..{}", cp_str(cp), cp_str(end)) } else { cp_str(cp) }));
            r.insert("status".into(), json!(k));
            r.insert("count".into(), json!(end - cp + 1));
            if let Some(b) = u.block(cp) {
                r.insert("block".into(), json!(b.name));
            }
            rows.push(Value::Object(r));
            cp = end + 1;
        } else {
            rows.push(char_json(u, cp, detail));
            cp += 1;
        }
    }
    Ok(json!({"unicode_version": UCD_VERSION, "range": format!("{}..{}", cp_str(lo), cp_str(hi)), "rows": rows}))
}

fn gap_kind(u: &Ucd, cp: u32) -> Option<&'static str> {
    if ucd::is_noncharacter(cp) {
        return Some("noncharacter");
    }
    match u.general_category(cp) {
        "Cn" => Some("unassigned"),
        "Co" => Some("private-use"),
        "Cs" => Some("surrogate"),
        _ => None,
    }
}

// ------------------------------------------------------------------ search

fn search(u: &Ucd, a: &Map<String, Value>) -> Result<Value, String> {
    let query = get_str(a, "query")?.ok_or("`query` is required")?;
    let mode = get_str(a, "mode")?.unwrap_or("prefix");
    if !["prefix", "word", "substring"].contains(&mode) {
        return Err("mode must be \"prefix\", \"word\" or \"substring\"".into());
    }
    let limit = (get_uint(a, "limit")?.unwrap_or(50) as usize).clamp(1, MAX_SEARCH_LIMIT);
    let block_f = get_str(a, "block")?.map(ucd::loose);
    let script_f = get_str(a, "script")?.map(|s| s.trim().to_lowercase());
    let cat_f = get_str(a, "category")?.map(|s| s.trim().to_string());
    let han_defs = matches!(a.get("han_definitions"), Some(Value::Bool(true)));
    let words: Vec<String> = query.split_whitespace().map(|w| w.to_uppercase()).collect();
    if words.is_empty() {
        return Err("`query` is empty".into());
    }
    if query.len() > 200 {
        return Err("`query` is longer than 200 bytes".into());
    }
    let qnorm = words.join(" ");

    let filter_ok = |cp: u32| -> bool {
        if let Some(bf) = &block_f {
            match u.block(cp) {
                Some(b) if ucd::loose(b.name).contains(bf.as_str()) => {}
                _ => return false,
            }
        }
        if let Some(sf) = &script_f {
            let s = u.script(cp);
            let in_ext = u.script_extensions(cp).is_some_and(|v| v.iter().any(|x| x.to_lowercase() == *sf));
            if s.to_lowercase() != *sf && !in_ext {
                return false;
            }
        }
        if let Some(cf) = &cat_f {
            let gc = u.general_category(cp);
            let long = u.long_value("gc", gc);
            if !(gc.starts_with(cf.as_str()) || long.eq_ignore_ascii_case(cf)) {
                return false;
            }
        }
        true
    };

    let mut exact: Vec<u32> = Vec::new();
    let mut hits: Vec<u32> = Vec::new();
    for (cp, hay) in &u.search_rows {
        if text_matches(hay, &words, mode) && filter_ok(*cp) {
            if hay.split('\n').any(|l| l == qnorm) {
                exact.push(*cp);
            } else {
                hits.push(*cp);
            }
        }
    }
    if han_defs {
        let lw: Vec<String> = words.iter().map(|w| w.to_lowercase()).collect();
        for cp in u.han_with_definitions() {
            let def = u.unihan(cp).iter().find(|(k, _)| *k == "kDefinition").map(|(_, v)| v.to_lowercase()).unwrap_or_default();
            if text_matches_lower(&def, &lw, mode) && filter_ok(cp) {
                hits.push(cp);
            }
        }
        hits.sort_unstable();
        hits.dedup();
    }
    let total = exact.len() + hits.len();
    let all: Vec<u32> = exact.into_iter().chain(hits).collect();
    let rows: Vec<Value> = all
        .iter()
        .take(limit)
        .map(|&cp| {
            let mut v = char_json(u, cp, Detail::Brief);
            if let Some(h) = u.unihan(cp).iter().find(|(k, _)| *k == "kDefinition").map(|(_, v)| *v) {
                if han_defs {
                    v.as_object_mut().unwrap().insert("definition".into(), json!(h));
                }
            }
            // If the match came from an alias rather than the name, show which.
            if let Some(hay) = u.search_rows.binary_search_by_key(&cp, |r| r.0).ok().map(|i| &u.search_rows[i].1) {
                let first = hay.split('\n').next().unwrap_or("");
                if !text_matches(first, &words, mode) {
                    if let Some(al) = hay.split('\n').skip(1).find(|l| text_matches(l, &words, mode)) {
                        v.as_object_mut().unwrap().insert("matched_alias".into(), json!(al));
                    }
                }
            }
            v
        })
        .collect();

    let mut seq_rows = Vec::new();
    let mut seq_total = 0;
    if block_f.is_none() && script_f.is_none() && cat_f.is_none() {
        for (i, s) in u.sequences.iter().enumerate() {
            if text_matches(&s.name.to_uppercase(), &words, mode) {
                seq_total += 1;
                if seq_rows.len() < limit.min(50) {
                    seq_rows.push(seq_json(u, i));
                }
            }
        }
    }
    // Block names carry vocabulary the character names often lack
    // ("Tai Xuan Jing Symbols" holds characters named TETRAGRAM FOR ...).
    let block_hits: Vec<String> = u
        .blocks
        .iter()
        .filter(|b| text_matches(&b.name.to_uppercase(), &words, mode))
        .map(|b| format!("{}..{} {}", cp_str(b.lo), cp_str(b.hi), b.name))
        .collect();
    let mut out = Map::new();
    out.insert("unicode_version".into(), json!(UCD_VERSION));
    out.insert("query".into(), json!(query));
    out.insert("mode".into(), json!(mode));
    if !block_hits.is_empty() {
        out.insert("blocks_named_like_query".into(), json!(block_hits));
    }
    out.insert("matches_total".into(), json!(total));
    out.insert("returned".into(), json!(rows.len()));
    out.insert("characters".into(), Value::Array(rows));
    if seq_total > 0 {
        out.insert("sequence_matches_total".into(), json!(seq_total));
        out.insert("sequences".into(), Value::Array(seq_rows));
    }
    Ok(Value::Object(out))
}

fn is_word_sep(c: char) -> bool {
    c == ' ' || c == '-' || c == '\n' || c == ':' || c == ',' || c == '(' || c == ')' || c == ';'
}

fn text_matches(hay_upper: &str, words: &[String], mode: &str) -> bool {
    words.iter().all(|w| match mode {
        "substring" => hay_upper.contains(w.as_str()),
        "word" => hay_upper.split(is_word_sep).any(|t| t == w),
        _ => hay_upper.split(is_word_sep).any(|t| t.starts_with(w.as_str())),
    })
}

fn text_matches_lower(hay: &str, words: &[String], mode: &str) -> bool {
    let sep = |c: char| !c.is_alphanumeric();
    words.iter().all(|w| match mode {
        "substring" => hay.contains(w.as_str()),
        "word" => hay.split(sep).any(|t| t == w),
        _ => hay.split(sep).any(|t| t.starts_with(w.as_str())),
    })
}

// ------------------------------------------------------------------ blocks

fn blocks(u: &Ucd, a: &Map<String, Value>) -> Result<Value, String> {
    let q = get_str(a, "query")?.map(str::trim).filter(|s| !s.is_empty());
    let summary = |i: usize| -> String {
        let b = &u.blocks[i];
        format!("{}..{} {}", cp_str(b.lo), cp_str(b.hi), b.name)
    };
    let Some(q) = q else {
        let all: Vec<String> = (0..u.blocks.len()).map(summary).collect();
        return Ok(json!({"unicode_version": UCD_VERSION, "count": all.len(), "blocks": all}));
    };
    let mut matches: Vec<usize> = Vec::new();
    // A glyph or code point?
    if let Ok(cp) = parse_cp(q) {
        if let Some(i) = u.block_index(cp) {
            matches.push(i);
        } else if q.chars().count() == 1 || q.to_ascii_lowercase().starts_with("u+") {
            return Ok(json!({"unicode_version": UCD_VERSION, "query": q, "blocks": [], "note": format!("{} is not in any block (No_Block)", cp_str(cp))}));
        }
    }
    if matches.is_empty() {
        let lq = ucd::loose(q);
        if let Some(i) = u.blocks.iter().position(|b| ucd::loose(b.name) == lq) {
            matches.push(i);
        } else {
            matches = (0..u.blocks.len()).filter(|&i| ucd::loose(u.blocks[i].name).contains(&lq)).collect();
        }
    }
    if matches.len() != 1 {
        return Ok(json!({"unicode_version": UCD_VERSION, "query": q, "blocks": matches.iter().map(|&i| summary(i)).collect::<Vec<_>>()}));
    }
    let b = &u.blocks[matches[0]];
    let assigned = (b.lo..=b.hi).filter(|&c| u.is_assigned(c) && u.general_category(c) != "Co" && u.general_category(c) != "Cs").count();
    let headings: Vec<Value> = b
        .subheaders
        .iter()
        .map(|&s| {
            let sh = &u.subheaders[s];
            let mut m = Map::new();
            m.insert("heading".into(), json!(sh.title));
            m.insert(
                "range".into(),
                json!(if sh.first == sh.last { cp_str(sh.first) } else { format!("{}..{}", cp_str(sh.first), cp_str(sh.last)) }),
            );
            let glyphs: String = (sh.first..=sh.last).filter(|&c| u.is_assigned(c) && printable(u, c)).take(64).filter_map(char::from_u32).collect();
            if !glyphs.is_empty() {
                m.insert("glyphs".into(), json!(glyphs));
            }
            if !sh.notices.is_empty() {
                m.insert("notes".into(), json!(sh.notices));
            }
            Value::Object(m)
        })
        .collect();
    Ok(json!({
        "unicode_version": UCD_VERSION,
        "block": b.name,
        "range": format!("{}..{}", cp_str(b.lo), cp_str(b.hi)),
        "size": b.hi - b.lo + 1,
        "assigned_characters": assigned,
        "chart_headings": headings,
    }))
}

/// Render a tool result for the model: compact JSON, one array element per line.
pub fn render(v: &Value) -> String {
    let Value::Object(m) = v else { return v.to_string() };
    let mut s = String::from("{");
    let mut first = true;
    for (k, val) in m {
        if !first {
            s.push(',');
        }
        first = false;
        s.push('\n');
        s.push_str(&serde_json::to_string(k).unwrap());
        s.push(':');
        match val {
            Value::Array(items) if items.iter().any(|x| x.is_object() || x.is_string()) && items.len() > 1 => {
                s.push('[');
                for (i, it) in items.iter().enumerate() {
                    s.push_str(if i == 0 { "\n  " } else { ",\n  " });
                    s.push_str(&it.to_string());
                }
                s.push_str("\n]");
            }
            o => s.push_str(&o.to_string()),
        }
    }
    s.push_str("\n}");
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::OnceLock;

    fn u() -> &'static Ucd {
        static U: OnceLock<Ucd> = OnceLock::new();
        U.get_or_init(Ucd::load)
    }
    fn obj(v: &Value) -> &Map<String, Value> {
        v.as_object().unwrap()
    }
    fn args(v: Value) -> Map<String, Value> {
        v.as_object().unwrap().clone()
    }

    #[test]
    fn parse_cp_forms() {
        for (s, want) in [
            ("U+2460", 0x2460),
            ("u+2460", 0x2460),
            ("0x1F0A1", 0x1F0A1),
            ("\\u2460", 0x2460),
            ("\\u{1F0A1}", 0x1F0A1),
            ("&#x2460;", 0x2460),
            ("&#9312;", 0x2460),
            ("2460", 0x2460),
            ("41", 0x41),
            ("A", 0x41),
            ("①", 0x2460),
            (" ", 0x20),
        ] {
            assert_eq!(parse_cp(s).unwrap(), want, "{s:?}");
        }
        for bad in ["zz", "U+110000", "", "0x", "①②"] {
            assert!(parse_cp(bad).is_err(), "{bad:?} should fail");
        }
    }

    #[test]
    fn names_and_properties() {
        let u = u();
        assert_eq!(u.name(0x2460).as_deref(), Some("CIRCLED DIGIT ONE"));
        assert_eq!(u.name(0x4E00).as_deref(), Some("CJK UNIFIED IDEOGRAPH-4E00"));
        assert_eq!(u.name(0xAC01).as_deref(), Some("HANGUL SYLLABLE GAG"));
        assert_eq!(u.name(0x0000), None);
        assert_eq!(u.display_name(0x0000), "NULL");
        assert_eq!(u.label(0x0378), "<reserved-0378>");
        assert_eq!(u.label(0xFFFF), "<noncharacter-FFFF>");
        assert_eq!(u.label(0xE000), "<private-use-E000>");
        assert_eq!(u.general_category(0x2460), "No");
        assert_eq!(u.general_category(0x0378), "Cn");
        assert_eq!(u.general_category(0xE123), "Co");
        assert_eq!(u.block(0x1F000).unwrap().name, "Mahjong Tiles");
        assert_eq!(u.script(0x1B53), "Balinese");
        assert_eq!(u.age(0x2460), "1.1");
        assert_eq!(u.age(0x0378), "Unassigned");
        assert_eq!(u.numeric(0x2473), Some(("Numeric", "20")));
        assert_eq!(u.numeric(0x00BD), Some(("Numeric", "1/2")));
        assert_eq!(u.subheader(0x2460).unwrap().title, "Circled numbers");
        assert_eq!(Ucd::hangul_decomposition(0xAC01), Some(vec![0x1100, 0x1161, 0x11A8]));
    }

    #[test]
    fn name_lookup_loose_and_algorithmic() {
        let u = u();
        assert_eq!(u.lookup_name("circled digit one"), Some(NameTarget::Cp(0x2460)));
        assert_eq!(u.lookup_name("CIRCLED_DIGIT_ONE"), Some(NameTarget::Cp(0x2460)));
        assert_eq!(u.lookup_name("nul"), Some(NameTarget::Cp(0x0000)));
        assert_eq!(u.lookup_name("cjk unified ideograph-4e01"), Some(NameTarget::Cp(0x4E01)));
        assert_eq!(u.lookup_name("TANGUT IDEOGRAPH-17000"), Some(NameTarget::Cp(0x17000)));
        assert_eq!(u.lookup_name("cjk unified ideograph-0041"), None);
        assert_eq!(u.lookup_name("HANGUL JUNGSEONG O-E"), Some(NameTarget::Cp(0x1180)));
        assert_eq!(u.lookup_name("HANGUL JUNGSEONG OE"), Some(NameTarget::Cp(0x116C)));
        assert!(matches!(u.lookup_name("flag: Japan"), Some(NameTarget::Seq(_))));
        assert_eq!(u.lookup_name("no such character name"), None);
    }

    #[test]
    fn identify_text_and_sequences() {
        let v = identify(u(), &args(json!({"text": "㏳㏴㏴🇯🇵", "detail": "brief"}))).unwrap();
        let m = obj(&v);
        assert_eq!(m["text_codepoints"], "U+33F3 U+33F4 U+33F4 U+1F1EF U+1F1F5");
        let chars = m["characters"].as_array().unwrap();
        assert_eq!(chars.len(), 4, "distinct code points, first-appearance order");
        assert_eq!(chars[0]["name"], "IDEOGRAPHIC TELEGRAPH SYMBOL FOR DAY TWENTY");
        assert_eq!(m["sequences"][0]["name"], "flag: Japan");
    }

    #[test]
    fn identify_reports_unresolved_without_failing() {
        let v = identify(u(), &args(json!({"codepoints": ["U+2460", "zz"], "names": ["bogus name"]}))).unwrap();
        let m = obj(&v);
        assert_eq!(m["characters"].as_array().unwrap().len(), 1);
        assert_eq!(m["unresolved"].as_array().unwrap().len(), 2);
        assert!(identify(u(), &args(json!({}))).is_err());
    }

    #[test]
    fn standard_detail_carries_chart_context() {
        let v = char_json(u(), 0x25CB, Detail::Standard);
        let m = obj(&v);
        assert_eq!(m["chart_heading"], "Geometric shapes");
        assert!(m["see_also"].as_array().unwrap().iter().any(|s| s.as_str().unwrap().starts_with("U+25EF")));
        let h = char_json(u(), 0x4E00, Detail::Standard);
        assert_eq!(h["han"]["Definition"], "one; a, an; alone");
        assert!(h["han"]["radical"].as_str().unwrap().contains("KANGXI RADICAL ONE"));
    }

    #[test]
    fn range_collapses_gaps_and_bounds_size() {
        let v = range(u(), &args(json!({"start": "U+0376", "end": "U+037A"}))).unwrap();
        let rows = v["rows"].as_array().unwrap();
        assert_eq!(rows.len(), 4);
        assert_eq!(rows[2]["cp"], "U+0378..U+0379");
        assert_eq!(rows[2]["status"], "unassigned");
        let v = range(u(), &args(json!({"around": "🃧", "before": 6}))).unwrap();
        assert_eq!(v["range"], "U+1F0E1..U+1F0E7");
        assert!(range(u(), &args(json!({"start": "U+0000", "end": "U+FFFF"}))).is_err());
        let v = range(u(), &args(json!({"around": "U+10FFFF", "after": 5}))).unwrap();
        assert_eq!(v["range"], "U+10FFFF..U+10FFFF");
    }

    #[test]
    fn search_modes_filters_and_blocks() {
        let v = search(u(), &args(json!({"query": "circ dig", "limit": 500}))).unwrap();
        assert!(v["characters"].as_array().unwrap().iter().any(|c| c["cp"] == "U+2460"));
        let v = search(u(), &args(json!({"query": "digit three", "script": "Balinese", "category": "Nd"}))).unwrap();
        assert_eq!(v["matches_total"], 1);
        let v = search(u(), &args(json!({"query": "tai xuan jing"}))).unwrap();
        assert!(v["blocks_named_like_query"][0].as_str().unwrap().ends_with("Tai Xuan Jing Symbols"));
        let v = search(u(), &args(json!({"query": "bell", "mode": "word"}))).unwrap();
        assert_eq!(v["characters"][0]["cp"], "U+0007", "exact alias match ranks first");
        let v = search(u(), &args(json!({"query": "beauty", "han_definitions": true}))).unwrap();
        assert!(v["matches_total"].as_u64().unwrap() > 0);
    }

    #[test]
    fn blocks_structure() {
        let v = blocks(u(), &args(json!({"query": "mahjong"}))).unwrap();
        assert_eq!(v["block"], "Mahjong Tiles");
        assert_eq!(v["chart_headings"][0]["heading"], "Prevailing wind tiles");
        let v = blocks(u(), &args(json!({}))).unwrap();
        assert!(v["count"].as_u64().unwrap() > 300);
        let v = blocks(u(), &args(json!({"query": "ᬋ"}))).unwrap();
        assert_eq!(v["block"], "Balinese");
    }
}
