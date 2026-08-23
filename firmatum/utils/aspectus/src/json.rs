//! JSON — the same look, serialized instead of drawn (design/json.md).
//!
//! Same walk, same budget, same censuses, same marks: this module receives
//! the tree *after* the allocator and sort have shaped it, so a machine
//! caller sees exactly the cuts and honesty a human caller would — never a
//! deeper walk or extra facts for being machine-shaped. Transport-
//! guaranteed, not picture-true.
//!
//! Everything the text look confesses with glyphs exists here as fields:
//! censuses as objects, `denied` / `walk_bound` / `cycle` booleans, `≥` as
//! `bounded`, a look-level `truncated`. Formats collapse to canonical:
//! sizes always bytes (integers), times one canonical form (iso-8601 UTC,
//! the overview invariant's), permissions octal strings, no SIGNA, no
//! color, no alignment. Key order is fixed in the emitter, node order is
//! Sort's — two aspecta diff as data. Facts ride regardless of quiet:
//! quiet is a text-rendering law, not a data cut (machine formats get
//! facts, not affordances).

use crate::config::Drift;
use crate::n_level::{Census, Mass, Node};

/// Schema version, from birth: one field, one saved migration.
pub const SCHEMA: u32 = 1;

fn esc(s: &str, out: &mut String) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
}

/// One field into an object body (comma handled by the caller pattern).
struct Obj {
    buf: String,
    first: bool,
}

impl Obj {
    fn new() -> Obj {
        Obj {
            buf: String::from("{"),
            first: true,
        }
    }

    fn key(&mut self, k: &str) {
        if !self.first {
            self.buf.push(',');
        }
        self.first = false;
        esc(k, &mut self.buf);
        self.buf.push(':');
    }

    fn str(&mut self, k: &str, v: &str) {
        self.key(k);
        esc(v, &mut self.buf);
    }

    fn raw(&mut self, k: &str, v: &str) {
        self.key(k);
        self.buf.push_str(v);
    }

    fn bool_true(&mut self, k: &str, v: bool) {
        // Marks are present-when-true; an absent mark is the honest false.
        if v {
            self.raw(k, "true");
        }
    }

    fn done(mut self) -> String {
        self.buf.push('}');
        self.buf
    }
}

fn iso(secs: i64) -> String {
    if secs < 0 {
        // Pre-epoch mtimes exist; the epoch integer is the exact form.
        return secs.to_string();
    }
    crate::overview::stamp_utc(std::time::UNIX_EPOCH + std::time::Duration::from_secs(secs as u64))
}

fn mass_obj(m: &Mass, include_lines: bool) -> String {
    let mut o = Obj::new();
    o.raw("files", &m.files.to_string());
    if include_lines {
        o.raw("lines", &m.lines.to_string());
        o.bool_true("estimated", m.est);
    }
    o.bool_true("bounded", m.bounded);
    o.done()
}

fn census_obj(c: &Census) -> String {
    let mut o = Obj::new();
    let files: usize = c.buckets.iter().map(|(_, n)| n).sum();
    o.raw("total", &(c.dirs + files).to_string());
    if c.ignored > 0 {
        // Gitignored files among the censused names — outside `total`
        // and the buckets, never silent (design/gitignore-bodies.md).
        o.raw("ignored", &c.ignored.to_string());
    }
    if c.dirs > 0 {
        o.raw("dirs", &c.dirs.to_string());
        if let Some(n) = &c.dir_name {
            o.str("dir_name", n);
        }
        if let Some(m) = &c.dir_files {
            o.raw("dir_files", &mass_obj(m, false));
        }
    }
    o.key("buckets");
    o.buf.push('[');
    for (i, (k, n)) in c.buckets.iter().enumerate() {
        if i > 0 {
            o.buf.push(',');
        }
        let mut b = Obj::new();
        b.str("kind", k);
        b.raw("n", &n.to_string());
        o.buf.push_str(&b.done());
    }
    o.buf.push(']');
    if let Some(s) = &c.single {
        o.str("single", s);
    }
    o.bool_true("bounded", c.bounded);
    o.done()
}

fn type_obj(ft: &crate::filetype::FileType) -> String {
    let mut o = Obj::new();
    o.str("major", ft.major.as_str());
    if !ft.minor.is_empty() {
        o.str("minor", &ft.minor);
    }
    if let Some(t) = &ft.trait_ {
        o.str("trait", t);
    }
    o.done()
}

fn node_obj(n: &Node) -> String {
    let mut o = Obj::new();
    o.str("name", &n.name);
    o.raw("dir", if n.is_dir { "true" } else { "false" });
    o.raw("type", &type_obj(&n.filetype));
    if let Some(l) = n.lines {
        o.raw("lines", &l.to_string());
    }
    if let Some(s) = n.size {
        o.raw("size", &s.to_string()); // always bytes
    }
    if let Some(m) = n.mode {
        // Octal string: the canonical spelling of the permission fact.
        let s = if m & 0o7000 != 0 {
            format!("{m:04o}")
        } else {
            format!("{:03o}", m & 0o777)
        };
        o.str("mode", &s);
    }
    if let Some(u) = n.uid {
        o.raw("uid", &u.to_string());
    }
    if let Some(g) = n.gid {
        o.raw("gid", &g.to_string());
    }
    if let Some(t) = n.mtime {
        o.str("mtime", &iso(t));
    }
    if let Some(h) = n.heat {
        // Four decimals, trailing zeros trimmed: 0.35220434162551345 was
        // spurious precision for a decayed sum (grok, 2026-08-14) — the
        // text look shows two; four keeps machine sort-stability margin.
        let s = format!("{h:.4}");
        let s = s.trim_end_matches('0').trim_end_matches('.');
        o.raw("heat", if s.is_empty() { "0" } else { s });
    }
    if let Some((sha, n)) = &n.intro {
        o.str("initial_sha", sha); // full — JSON gets the canonical form
        o.raw("initial_behind", &n.to_string());
    }
    if let Some((sha, n)) = &n.touch {
        o.str("latest_sha", sha);
        o.raw("latest_behind", &n.to_string());
    }
    if let Some(t) = &n.title {
        o.str("title", t);
    }
    // A collapsed name-series (design/globify.md): structured, members
    // recoverable in principle — never a lossy string.
    if let Some(g) = &n.glob {
        let mut b = Obj::new();
        b.raw("min", &g.lo.to_string());
        b.raw("max", &g.hi.to_string());
        b.raw("width", &g.width.to_string());
        b.raw("count", &g.count.to_string());
        b.str("kind", if n.is_dir { "dir" } else { "file" });
        o.raw("glob", &b.done());
    }
    // The `[has: …]` contents-claim (lattice row `has`; renamed from
    // `kind` the same day this schema was born, so the field follows).
    if !n.kinds.is_empty() {
        o.key("has");
        o.buf.push('[');
        for (i, k) in n.kinds.iter().enumerate() {
            if i > 0 {
                o.buf.push(',');
            }
            esc(k, &mut o.buf);
        }
        o.buf.push(']');
    }
    // Hidden furniture magnitudes (presence survives hiding): additive
    // beside `has`, keyed by the claiming kind.
    if !n.has_counts.is_empty() {
        o.key("hidden");
        o.buf.push('[');
        for (i, (k, files, bounded)) in n.has_counts.iter().enumerate() {
            if i > 0 {
                o.buf.push(',');
            }
            let mut h = Obj::new();
            h.str("kind", k);
            h.raw("files", &files.to_string());
            h.bool_true("bounded", *bounded);
            o.buf.push_str(&h.done());
        }
        o.buf.push(']');
    }
    if !n.facets.is_empty() {
        o.key("facets");
        o.buf.push('[');
        for (i, f) in n.facets.iter().enumerate() {
            if i > 0 {
                o.buf.push(',');
            }
            esc(f, &mut o.buf);
        }
        o.buf.push(']');
    }
    if let Some(l) = &n.link {
        o.str("link", l);
        o.bool_true("broken", n.link_broken);
    }
    // The caller's explicit ask, marked where it landed (design/focus.md:
    // `matched` is a field on matched nodes; the ask does not change the
    // schema's shape).
    o.bool_true("matched", n.matched);
    o.bool_true("gitignored", n.ignored);
    if n.ignored_files > 0 {
        o.raw("ignored_files", &n.ignored_files.to_string());
    }
    o.bool_true("denied", n.denied);
    o.bool_true("walk_bound", n.cut);
    o.bool_true("cycle", n.cycle);
    o.bool_true("other_fs", n.other_fs);
    o.bool_true("iter_err", n.iter_err);
    // The dir census of an unexpanded directory, with the subtree's mass —
    // the same lines where the text look renders them.
    if let Some(c) = &n.leftover {
        o.raw("census", &census_obj(c));
        if let Some(m) = &n.mass {
            o.raw("mass", &mass_obj(m, true));
        }
    }
    if !n.children.is_empty() {
        o.key("children");
        o.buf.push('[');
        for (i, c) in n.children.iter().enumerate() {
            if i > 0 {
                o.buf.push(',');
            }
            o.buf.push_str(&node_obj(c));
        }
        o.buf.push(']');
    }
    // Leaf census: the siblings the line budget could not list.
    if let Some(c) = &n.omitted {
        o.raw("omitted", &census_obj(c));
    }
    o.done()
}

/// Any cut or denial anywhere in the look — so a caller can branch without
/// walking the tree. Depth cutoffs and line-budget folds are the look's
/// honest *shape* (confessed by censuses), not truncation; what counts is
/// denial, the walk bound, mid-iteration errors, mount stops, and any `≥`
/// floor an aggregate carries.
fn truncated(n: &Node) -> bool {
    n.denied
        || n.cut
        || n.iter_err
        || n.other_fs
        || n.leftover.as_ref().is_some_and(|c| c.bounded)
        || (n.leftover.is_some() && n.mass.as_ref().is_some_and(|m| m.bounded))
        // A hidden-furniture count that hit its cap is a `≥` floor like
        // any other (close audit 2026-08-14: the code missed what the
        // impl note already promised).
        || n.has_counts.iter().any(|(_, _, bounded)| *bounded)
        || n.children.iter().any(truncated)
}

fn drift_arr(items: &[Drift]) -> String {
    let mut buf = String::from("[");
    for (i, d) in items.iter().enumerate() {
        if i > 0 {
            buf.push(',');
        }
        let mut o = Obj::new();
        o.str("key", &d.key);
        o.str("value", &d.value);
        o.str("source", d.source);
        buf.push_str(&o.done());
    }
    buf.push(']');
    buf
}

/// The whole look as one JSON document (one line, trailing newline).
/// `config_drift` is omitted when nothing differs (absent, never faked).
pub fn render(root: &str, stamp: &str, tree: &Node, drift: &[Drift]) -> String {
    let mut o = Obj::new();
    o.str("aspectus", env!("CARGO_PKG_VERSION"));
    o.raw("schema", &SCHEMA.to_string());
    o.str("time", stamp);
    o.str("root", root);
    o.raw("truncated", if truncated(tree) { "true" } else { "false" });
    if !drift.is_empty() {
        o.raw("config_drift", &drift_arr(drift));
    }
    // The steward's feedback solicitation rides on stderr (main.rs), not
    // in the document — stdout is data (Joseph, 2026-08-22).
    o.raw("tree", &node_obj(tree));
    let mut out = o.done();
    out.push('\n');
    out
}

/// A refusal in machine mode: the class, the observed token, the next
/// action — as data, on stderr (stdout stays data-or-nothing; channel
/// choice provisional, design Open).
pub fn refusal(class: &str, observed: &str, detail: Option<&str>) -> String {
    let mut e = Obj::new();
    e.str("class", class);
    e.str("observed", observed);
    if let Some(d) = detail {
        e.str("detail", d);
    }
    e.key("next");
    e.buf.push('[');
    esc("aspectus help", &mut e.buf);
    e.buf.push(']');
    let mut o = Obj::new();
    o.raw("error", &e.done());
    let mut out = o.done();
    out.push('\n');
    out
}
