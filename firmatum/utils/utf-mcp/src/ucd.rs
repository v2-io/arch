//! The Unicode Character Database, embedded at compile time and parsed once at
//! startup into lookup tables. Every string handed out borrows from the embedded
//! files, so nothing here allocates per query except search haystacks.
//!
//! Sources (all from https://www.unicode.org/Public/18.0.0/, checksums in
//! data/SHA256SUMS-18.0.0): UnicodeData, DerivedName, NameAliases, NamesList,
//! Blocks, Scripts, ScriptExtensions, DerivedAge, EastAsianWidth,
//! DerivedNumericType, DerivedNumericValues, PropertyValueAliases,
//! NamedSequences, emoji-data, emoji-sequences, emoji-zwj-sequences, and a
//! field subset of Unihan (see fetch-ucd).

use std::collections::HashMap;

pub const UCD_VERSION: &str = "18.0.0";

macro_rules! ucd_file {
    ($f:literal) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/18.0.0/", $f))
    };
}

const UNICODE_DATA: &str = ucd_file!("UnicodeData.txt");
const DERIVED_NAME: &str = ucd_file!("DerivedName.txt");
const NAME_ALIASES: &str = ucd_file!("NameAliases.txt");
const NAMES_LIST: &str = ucd_file!("NamesList.txt");
const BLOCKS: &str = ucd_file!("Blocks.txt");
const SCRIPTS: &str = ucd_file!("Scripts.txt");
const SCRIPT_EXT: &str = ucd_file!("ScriptExtensions.txt");
const AGE: &str = ucd_file!("DerivedAge.txt");
const EAW: &str = ucd_file!("EastAsianWidth.txt");
const NUM_TYPE: &str = ucd_file!("DerivedNumericType.txt");
const NUM_VALUE: &str = ucd_file!("DerivedNumericValues.txt");
const PVA: &str = ucd_file!("PropertyValueAliases.txt");
const NAMED_SEQ: &str = ucd_file!("NamedSequences.txt");
const EMOJI_DATA: &str = ucd_file!("emoji-data.txt");
const EMOJI_SEQ: &str = ucd_file!("emoji-sequences.txt");
const EMOJI_ZWJ: &str = ucd_file!("emoji-zwj-sequences.txt");
const UNIHAN: &str = ucd_file!("unihan-subset.txt");
const CJK_RADICALS: &str = ucd_file!("CJKRadicals.txt");
const EMOJI_TEST: &str = ucd_file!("emoji-test.txt");

pub const MAX_CP: u32 = 0x10FFFF;

/// One UnicodeData.txt record (fields that are not recomputed elsewhere).
#[derive(Clone, Copy)]
pub struct Ud {
    pub gc: &'static str,
    pub ccc: &'static str,
    pub bidi: &'static str,
    pub decomp: &'static str,
    pub mirrored: bool,
    pub upper: &'static str,
    pub lower: &'static str,
    pub title: &'static str,
}

#[derive(Clone, Copy)]
pub enum NoteKind {
    /// `=` informal alias
    Alias,
    /// `*` informative note
    Comment,
    /// `x` cross reference
    CrossRef,
    /// `~` standardized variation sequence
    Variation,
    /// `@+` notice printed after a character entry. Positional: it may concern
    /// that character alone or the group the character ends.
    Notice,
    /// `\t\tx` cross-reference belonging to such a notice
    NoticeCrossRef,
}

pub struct NamesListEntry {
    pub notes: Vec<(NoteKind, &'static str)>,
    pub subheader: Option<usize>,
}

pub struct Subheader {
    pub title: &'static str,
    pub notices: Vec<&'static str>,
    pub see_also: Vec<&'static str>,
    pub first: u32,
    pub last: u32,
    pub block: usize,
}

pub struct Block {
    pub lo: u32,
    pub hi: u32,
    pub name: &'static str,
    pub subheaders: Vec<usize>,
    /// Block-level notices from NamesList (printed under the block header).
    pub notices: Vec<&'static str>,
    pub see_also: Vec<&'static str>,
}

pub struct Sequence {
    pub cps: Vec<u32>,
    pub name: &'static str,
    pub kind: &'static str,
}

/// Sorted non-overlapping ranges with a value, plus `@missing` defaults.
struct RangeMap {
    ranges: Vec<(u32, u32, &'static str)>,
    missing: Vec<(u32, u32, &'static str)>,
}

impl RangeMap {
    fn parse(src: &'static str, value_field: usize) -> RangeMap {
        let mut ranges = Vec::new();
        let mut missing = Vec::new();
        for line in src.lines() {
            if let Some(rest) = line.strip_prefix("# @missing:") {
                let f: Vec<&str> = rest.split(';').map(str::trim).collect();
                if f.len() > value_field {
                    let (lo, hi) = parse_range(f[0]);
                    missing.push((lo, hi, f[value_field]));
                }
                continue;
            }
            let Some(data) = data_part(line) else { continue };
            let f: Vec<&str> = data.split(';').map(str::trim).collect();
            if f.len() <= value_field {
                continue;
            }
            let (lo, hi) = parse_range(f[0]);
            ranges.push((lo, hi, f[value_field]));
        }
        ranges.sort_by_key(|r| r.0);
        RangeMap { ranges, missing }
    }

    fn get(&self, cp: u32) -> Option<&'static str> {
        let i = self.ranges.partition_point(|r| r.0 <= cp);
        if i > 0 {
            let r = self.ranges[i - 1];
            if cp <= r.1 {
                return Some(r.2);
            }
        }
        // Later @missing lines take precedence over earlier ones (UAX #44 §4.2.10).
        self.missing.iter().rev().find(|m| m.0 <= cp && cp <= m.1).map(|m| m.2)
    }
}

fn data_part(line: &'static str) -> Option<&'static str> {
    let d = match line.find('#') {
        Some(i) => &line[..i],
        None => line,
    };
    let d = d.trim();
    if d.is_empty() { None } else { Some(d) }
}

fn hex(s: &str) -> u32 {
    u32::from_str_radix(s.trim(), 16).unwrap_or_else(|_| panic!("bad hex in UCD: {s:?}"))
}

fn parse_range(s: &str) -> (u32, u32) {
    match s.split_once("..") {
        Some((a, b)) => (hex(a), hex(b)),
        None => {
            let c = hex(s);
            (c, c)
        }
    }
}

pub struct Ucd {
    ud: HashMap<u32, Ud>,
    ud_ranges: Vec<(u32, u32, Ud)>,
    names: HashMap<u32, &'static str>,
    name_patterns: Vec<(u32, u32, &'static str)>,
    pub aliases: HashMap<u32, Vec<(&'static str, &'static str)>>,
    pub blocks: Vec<Block>,
    pub subheaders: Vec<Subheader>,
    names_list: HashMap<u32, NamesListEntry>,
    scripts: RangeMap,
    script_ext: RangeMap,
    age: RangeMap,
    eaw: RangeMap,
    num_type: RangeMap,
    num_value: RangeMap,
    emoji: Vec<(u32, u32, &'static str)>,
    pva: HashMap<(&'static str, &'static str), &'static str>,
    pub sequences: Vec<Sequence>,
    seq_by_first: HashMap<u32, Vec<usize>>,
    unihan: HashMap<u32, Vec<(&'static str, &'static str)>>,
    han_defined: Vec<u32>,
    /// CJKRadicals.txt: radical number as written in kRSUnicode ("120", "120'") ->
    /// (CJK radical character, corresponding unified ideograph)
    radicals: HashMap<&'static str, (u32, u32)>,
    /// CLDR short names of single-code-point emoji (emoji-test.txt, fully-qualified)
    emoji_names: HashMap<u32, &'static str>,
    /// loose-matched name (UAX44-LM2) → code point or sequence
    name_index: HashMap<String, NameTarget>,
    /// (code point, uppercase searchable text: name, aliases, informal aliases)
    pub search_rows: Vec<(u32, String)>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NameTarget {
    Cp(u32),
    Seq(usize),
}

impl Ucd {
    pub fn load() -> Ucd {
        // --- UnicodeData.txt
        let mut ud = HashMap::new();
        let mut ud_ranges = Vec::new();
        let mut pending_first: Option<(u32, Ud)> = None;
        for line in UNICODE_DATA.lines() {
            let f: Vec<&'static str> = line.split(';').collect();
            if f.len() < 15 {
                continue;
            }
            let cp = hex(f[0]);
            let rec = Ud {
                gc: f[2],
                ccc: f[3],
                bidi: f[4],
                decomp: f[5],
                mirrored: f[9] == "Y",
                upper: f[12],
                lower: f[13],
                title: f[14],
            };
            if f[1].ends_with(", First>") {
                pending_first = Some((cp, rec));
            } else if f[1].ends_with(", Last>") {
                let (lo, r) = pending_first.take().expect("Last without First");
                ud_ranges.push((lo, cp, r));
            } else {
                ud.insert(cp, rec);
            }
        }

        // --- DerivedName.txt
        let mut names = HashMap::new();
        let mut name_patterns = Vec::new();
        for line in DERIVED_NAME.lines() {
            let Some(d) = data_part(line) else { continue };
            let Some((r, n)) = d.split_once(';') else { continue };
            let n = n.trim();
            let (lo, hi) = parse_range(r.trim());
            if n.contains('*') {
                name_patterns.push((lo, hi, n));
            } else {
                for cp in lo..=hi {
                    names.insert(cp, n);
                }
            }
        }
        name_patterns.sort_by_key(|p| p.0);

        // --- NameAliases.txt
        let mut aliases: HashMap<u32, Vec<(&str, &str)>> = HashMap::new();
        for line in NAME_ALIASES.lines() {
            let Some(d) = data_part(line) else { continue };
            let f: Vec<&str> = d.split(';').collect();
            if f.len() >= 3 {
                aliases.entry(hex(f[0])).or_default().push((f[1].trim(), f[2].trim()));
            }
        }

        // --- Blocks.txt
        let mut blocks = Vec::new();
        for line in BLOCKS.lines() {
            let Some(d) = data_part(line) else { continue };
            let Some((r, n)) = d.split_once(';') else { continue };
            let (lo, hi) = parse_range(r.trim());
            blocks.push(Block { lo, hi, name: n.trim(), subheaders: Vec::new(), notices: Vec::new(), see_also: Vec::new() });
        }
        blocks.sort_by_key(|b| b.lo);

        // --- NamesList.txt (annotations, subheaders)
        let mut subheaders: Vec<Subheader> = Vec::new();
        let mut names_list: HashMap<u32, NamesListEntry> = HashMap::new();
        // Block-level notices and their cross-references, keyed by block header start.
        let mut block_notes: HashMap<u32, (Vec<&'static str>, Vec<&'static str>)> = HashMap::new();
        {
            let mut cur_cp: Option<u32> = None;
            let mut cur_sub: Option<usize> = None;
            let mut cur_block: Option<u32> = None;
            for line in NAMES_LIST.lines() {
                if let Some(rest) = line.strip_prefix("@@\t") {
                    cur_cp = None;
                    cur_sub = None;
                    cur_block = rest.split('\t').next().map(hex);
                    continue;
                }
                if let Some(rest) = line.strip_prefix("@\t") {
                    subheaders.push(Subheader {
                        title: rest.trim_start_matches('\t').trim(),
                        notices: Vec::new(),
                        see_also: Vec::new(),
                        first: u32::MAX,
                        last: 0,
                        block: usize::MAX,
                    });
                    cur_sub = Some(subheaders.len() - 1);
                    cur_cp = None;
                    continue;
                }
                if let Some(rest) = line.strip_prefix("\t\t") {
                    // A cross-reference attached to the notice just above it.
                    let Some(xr) = rest.strip_prefix('x') else { continue };
                    let xr = xr.trim();
                    match (cur_cp, cur_sub, cur_block) {
                        (Some(cp), _, _) => {
                            if let Some(e) = names_list.get_mut(&cp) {
                                e.notes.push((NoteKind::NoticeCrossRef, xr));
                            }
                        }
                        (None, Some(s), _) => subheaders[s].see_also.push(xr),
                        (None, None, Some(b)) => block_notes.entry(b).or_default().1.push(xr),
                        _ => {}
                    }
                    continue;
                }
                if let Some(rest) = line.strip_prefix("@+") {
                    let text = rest.trim_start_matches('\t').trim();
                    if text.is_empty() {
                        continue;
                    }
                    match (cur_cp, cur_sub, cur_block) {
                        (Some(cp), _, _) => {
                            if let Some(e) = names_list.get_mut(&cp) {
                                e.notes.push((NoteKind::Notice, text));
                            }
                        }
                        (None, Some(s), _) => subheaders[s].notices.push(text),
                        (None, None, Some(b)) => block_notes.entry(b).or_default().0.push(text),
                        _ => {}
                    }
                    continue;
                }
                if line.starts_with('@') || line.starts_with(';') {
                    continue;
                }
                if let Some(rest) = line.strip_prefix('\t') {
                    let Some(cp) = cur_cp else { continue };
                    let mut chars = rest.chars();
                    let kind = match chars.next() {
                        Some('=') => NoteKind::Alias,
                        Some('*') => NoteKind::Comment,
                        Some('x') => NoteKind::CrossRef,
                        Some('~') => NoteKind::Variation,
                        _ => continue, // ':' '#' decompositions (from UnicodeData instead), '%' formal aliases (from NameAliases)
                    };
                    let text = rest[1..].trim();
                    if let Some(e) = names_list.get_mut(&cp) {
                        e.notes.push((kind, text));
                    }
                    continue;
                }
                // A character entry: HEX \t NAME
                let Some((h, _name)) = line.split_once('\t') else { continue };
                if h.is_empty() || !h.bytes().all(|b| b.is_ascii_hexdigit()) {
                    continue;
                }
                let cp = hex(h);
                cur_cp = Some(cp);
                if let Some(s) = cur_sub {
                    let sh = &mut subheaders[s];
                    sh.first = sh.first.min(cp);
                    sh.last = sh.last.max(cp);
                }
                names_list.insert(cp, NamesListEntry { notes: Vec::new(), subheader: cur_sub });
            }
        }
        for (start, (notes, xrefs)) in block_notes {
            if let Some(b) = blocks.iter_mut().find(|b| b.lo <= start && start <= b.hi) {
                b.notices = notes;
                b.see_also = xrefs;
            }
        }
        for (i, sh) in subheaders.iter_mut().enumerate() {
            if sh.first == u32::MAX {
                continue;
            }
            if let Some(b) = blocks.iter().position(|b| b.lo <= sh.first && sh.first <= b.hi) {
                sh.block = b;
                blocks[b].subheaders.push(i);
            }
        }

        // --- PropertyValueAliases.txt
        let mut pva = HashMap::new();
        for line in PVA.lines() {
            let Some(d) = data_part(line) else { continue };
            let f: Vec<&'static str> = d.split(';').map(str::trim).collect();
            if f.len() < 3 {
                continue;
            }
            if f[0] == "ccc" && f.len() >= 4 {
                pva.insert(("ccc", f[1]), f[3]);
            } else {
                pva.insert((f[0], f[1]), f[2]);
            }
        }

        // --- emoji-data.txt (overlapping ranges of several properties)
        let mut emoji = Vec::new();
        for line in EMOJI_DATA.lines() {
            let Some(d) = data_part(line) else { continue };
            let Some((r, p)) = d.split_once(';') else { continue };
            let (lo, hi) = parse_range(r.trim());
            emoji.push((lo, hi, p.trim()));
        }

        // --- sequences: NamedSequences, emoji-sequences (multi-code-point only), emoji-zwj-sequences
        let mut sequences = Vec::new();
        for line in NAMED_SEQ.lines() {
            let Some(d) = data_part(line) else { continue };
            let Some((n, cps)) = d.split_once(';') else { continue };
            let cps: Vec<u32> = cps.split_whitespace().map(hex).collect();
            sequences.push(Sequence { cps, name: n.trim(), kind: "Named_Sequence" });
        }
        for src in [EMOJI_SEQ, EMOJI_ZWJ] {
            for line in src.lines() {
                let Some(d) = data_part(line) else { continue };
                let f: Vec<&'static str> = d.split(';').map(str::trim).collect();
                if f.len() < 3 || f[0].contains("..") {
                    continue;
                }
                let cps: Vec<u32> = f[0].split_whitespace().map(hex).collect();
                if cps.len() < 2 {
                    continue;
                }
                sequences.push(Sequence { cps, name: f[2], kind: f[1] });
            }
        }
        let mut seq_by_first: HashMap<u32, Vec<usize>> = HashMap::new();
        for (i, s) in sequences.iter().enumerate() {
            seq_by_first.entry(s.cps[0]).or_default().push(i);
        }
        for v in seq_by_first.values_mut() {
            v.sort_by(|a, b| sequences[*b].cps.len().cmp(&sequences[*a].cps.len()));
        }

        // --- Unihan subset
        let mut unihan: HashMap<u32, Vec<(&str, &str)>> = HashMap::new();
        for line in UNIHAN.lines() {
            let mut it = line.splitn(3, '\t');
            let (Some(c), Some(k), Some(v)) = (it.next(), it.next(), it.next()) else { continue };
            let Some(h) = c.strip_prefix("U+") else { continue };
            unihan.entry(hex(h)).or_default().push((k, v));
        }

        let mut u = Ucd {
            ud,
            ud_ranges,
            names,
            name_patterns,
            aliases,
            blocks,
            subheaders,
            names_list,
            scripts: RangeMap::parse(SCRIPTS, 1),
            script_ext: RangeMap::parse(SCRIPT_EXT, 1),
            age: RangeMap::parse(AGE, 1),
            eaw: RangeMap::parse(EAW, 1),
            num_type: RangeMap::parse(NUM_TYPE, 1),
            num_value: RangeMap::parse(NUM_VALUE, 3),
            emoji,
            pva,
            sequences,
            seq_by_first,
            han_defined: {
                let mut v: Vec<u32> = unihan.iter().filter(|(_, f)| f.iter().any(|(k, _)| *k == "kDefinition")).map(|(c, _)| *c).collect();
                v.sort_unstable();
                v
            },
            radicals: {
                let mut m = HashMap::new();
                for line in CJK_RADICALS.lines() {
                    let Some(d) = data_part(line) else { continue };
                    let f: Vec<&'static str> = d.split(';').map(str::trim).collect();
                    if f.len() >= 3 && !f[1].is_empty() {
                        m.insert(f[0], (hex(f[1]), hex(f[2])));
                    }
                }
                m
            },
            emoji_names: {
                let mut m = HashMap::new();
                for line in EMOJI_TEST.lines() {
                    let Some((d, c)) = line.split_once('#') else { continue };
                    let Some((cps, status)) = d.split_once(';') else { continue };
                    if status.trim() != "fully-qualified" {
                        continue;
                    }
                    let cps: Vec<u32> = cps.split_whitespace().map(hex).filter(|&c| c != 0xFE0F).collect();
                    if cps.len() != 1 {
                        continue;
                    }
                    // comment: " 😃 E0.6 grinning face with big eyes"
                    let mut it = c.trim().splitn(3, ' ');
                    let (_, ver, name) = (it.next(), it.next(), it.next());
                    if let (Some(v), Some(n)) = (ver, name) {
                        if v.starts_with('E') {
                            m.insert(cps[0], n.trim());
                        }
                    }
                }
                m
            },
            unihan,
            name_index: HashMap::new(),
            search_rows: Vec::new(),
        };
        u.build_indexes();
        u
    }

    fn build_indexes(&mut self) {
        let mut idx: HashMap<String, NameTarget> = HashMap::new();
        let mut cps: Vec<u32> = self.names.keys().copied().collect();
        cps.sort_unstable();
        for &cp in &cps {
            idx.entry(loose(self.names[&cp])).or_insert(NameTarget::Cp(cp));
        }
        let mut acps: Vec<u32> = self.aliases.keys().copied().collect();
        acps.sort_unstable();
        for &cp in &acps {
            for (a, _) in &self.aliases[&cp] {
                idx.entry(loose(a)).or_insert(NameTarget::Cp(cp));
            }
        }
        for (i, s) in self.sequences.iter().enumerate() {
            idx.entry(loose(s.name)).or_insert(NameTarget::Seq(i));
        }
        let mut ecps: Vec<u32> = self.emoji_names.keys().copied().collect();
        ecps.sort_unstable();
        for cp in ecps {
            idx.entry(loose(self.emoji_names[&cp])).or_insert(NameTarget::Cp(cp));
        }
        self.name_index = idx;

        // Search rows: every code point with an explicit name or alias.
        let mut all: Vec<u32> = cps;
        all.extend(acps.iter().copied().filter(|c| !self.names.contains_key(c)));
        all.sort_unstable();
        all.dedup();
        let mut rows = Vec::with_capacity(all.len());
        for cp in all {
            let mut s = String::new();
            if let Some(n) = self.names.get(&cp) {
                s.push_str(n);
            }
            if let Some(al) = self.aliases.get(&cp) {
                for (a, _) in al {
                    s.push('\n');
                    s.push_str(a);
                }
            }
            if let Some(e) = self.names_list.get(&cp) {
                for (k, t) in &e.notes {
                    if matches!(k, NoteKind::Alias) {
                        s.push('\n');
                        s.push_str(t);
                    }
                }
            }
            if let Some(n) = self.emoji_names.get(&cp) {
                s.push('\n');
                s.push_str(n);
            }
            rows.push((cp, s.to_uppercase()));
        }
        self.search_rows = rows;
    }

    // ---------------------------------------------------------------- queries

    pub fn ud(&self, cp: u32) -> Option<Ud> {
        if let Some(r) = self.ud.get(&cp) {
            return Some(*r);
        }
        self.ud_ranges.iter().find(|r| r.0 <= cp && cp <= r.1).map(|r| r.2)
    }

    pub fn general_category(&self, cp: u32) -> &'static str {
        self.ud(cp).map(|u| u.gc).unwrap_or("Cn")
    }

    /// The Name property (empty for controls, private use, surrogates, unassigned).
    pub fn name(&self, cp: u32) -> Option<String> {
        if let Some(n) = self.names.get(&cp) {
            return Some((*n).to_string());
        }
        self.name_patterns
            .iter()
            .find(|p| p.0 <= cp && cp <= p.1)
            .map(|p| p.2.replace('*', &format!("{cp:04X}")))
    }

    /// Code point label per Unicode §4.8 for code points with no Name.
    pub fn label(&self, cp: u32) -> String {
        let kind = if is_noncharacter(cp) {
            "noncharacter"
        } else {
            match self.general_category(cp) {
                "Cc" => "control",
                "Co" => "private-use",
                "Cs" => "surrogate",
                _ => "reserved",
            }
        };
        format!("<{kind}-{cp:04X}>")
    }

    /// A short display name: the Name, else the first control/figment/abbreviation alias, else the label.
    pub fn display_name(&self, cp: u32) -> String {
        if let Some(n) = self.name(cp) {
            return n;
        }
        if let Some(al) = self.aliases.get(&cp) {
            if let Some((a, _)) = al.iter().find(|(_, t)| *t == "control").or(al.first()) {
                return (*a).to_string();
            }
        }
        self.label(cp)
    }

    pub fn block(&self, cp: u32) -> Option<&Block> {
        let i = self.blocks.partition_point(|b| b.lo <= cp);
        if i > 0 && cp <= self.blocks[i - 1].hi { Some(&self.blocks[i - 1]) } else { None }
    }

    pub fn block_index(&self, cp: u32) -> Option<usize> {
        let i = self.blocks.partition_point(|b| b.lo <= cp);
        if i > 0 && cp <= self.blocks[i - 1].hi { Some(i - 1) } else { None }
    }

    pub fn subheader(&self, cp: u32) -> Option<&Subheader> {
        self.names_list.get(&cp).and_then(|e| e.subheader).map(|i| &self.subheaders[i])
    }

    pub fn notes(&self, cp: u32) -> &[(NoteKind, &'static str)] {
        self.names_list.get(&cp).map(|e| e.notes.as_slice()).unwrap_or(&[])
    }

    pub fn script(&self, cp: u32) -> &'static str {
        self.scripts.get(cp).unwrap_or("Unknown")
    }

    /// Script_Extensions as long names; None when it equals {Script}.
    pub fn script_extensions(&self, cp: u32) -> Option<Vec<&'static str>> {
        let v = self.script_ext.get(cp)?;
        if v.starts_with('<') {
            return None;
        }
        Some(v.split_whitespace().map(|s| self.pva.get(&("sc", s)).copied().unwrap_or(s)).collect())
    }

    pub fn age(&self, cp: u32) -> &'static str {
        self.age.get(cp).unwrap_or("Unassigned")
    }

    pub fn east_asian_width(&self, cp: u32) -> &'static str {
        self.eaw.get(cp).unwrap_or("N")
    }

    pub fn numeric(&self, cp: u32) -> Option<(&'static str, &'static str)> {
        let t = self.num_type.get(cp)?;
        if t == "None" {
            return None;
        }
        Some((t, self.num_value.get(cp).unwrap_or("NaN")))
    }

    pub fn emoji_props(&self, cp: u32) -> Vec<&'static str> {
        self.emoji.iter().filter(|e| e.0 <= cp && cp <= e.1).map(|e| e.2).collect()
    }

    pub fn unihan(&self, cp: u32) -> &[(&'static str, &'static str)] {
        self.unihan.get(&cp).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// (radical character, unified ideograph) for a kRSUnicode radical such as "120'".
    pub fn radical(&self, r: &str) -> Option<(u32, u32)> {
        self.radicals.get(r).copied()
    }

    pub fn emoji_name(&self, cp: u32) -> Option<&'static str> {
        self.emoji_names.get(&cp).copied()
    }

    /// Han ideographs that have a Unihan kDefinition, in code point order.
    pub fn han_with_definitions(&self) -> impl Iterator<Item = u32> + '_ {
        self.han_defined.iter().copied()
    }

    /// A Script filter value (long name or ISO 15924 code, any case) -> the long
    /// name as used in Scripts.txt. None when unknown.
    pub fn resolve_script(&self, v: &str) -> Option<&'static str> {
        let k = loose(v);
        self.pva.iter().find(|((p, short), long)| *p == "sc" && (loose(short) == k || loose(long) == k)).map(|(_, long)| *long)
    }

    /// A General_Category filter value (code or name, any case; one-letter groups
    /// and LC included) -> the two-letter categories it covers. None when unknown.
    pub fn resolve_category(&self, v: &str) -> Option<Vec<&'static str>> {
        let k = loose(v);
        let (short, _) = self.pva.iter().find(|((p, short), long)| *p == "gc" && (loose(short) == k || loose(long) == k)).map(|(k, l)| (k.1, *l))?;
        let leaves = || self.pva.keys().filter(|(p, s)| *p == "gc" && s.len() == 2 && *s != "LC").map(|(_, s)| *s);
        Some(match short {
            "LC" => vec!["Lu", "Ll", "Lt"],
            s if s.len() == 1 => leaves().filter(|x| x.starts_with(s)).collect(),
            s => vec![s],
        })
    }

    pub fn long_value(&self, prop: &'static str, short: &'static str) -> &'static str {
        self.pva.get(&(prop, short)).copied().unwrap_or(short)
    }

    pub fn is_assigned(&self, cp: u32) -> bool {
        self.general_category(cp) != "Cn"
    }

    /// Exact name lookup with UAX44-LM2 loose matching, covering names,
    /// formal aliases, named sequences, emoji sequence names, and the
    /// algorithmic (pattern) names such as CJK UNIFIED IDEOGRAPH-4E00.
    pub fn lookup_name(&self, name: &str) -> Option<NameTarget> {
        let upper = name.trim().to_uppercase();
        // The one LM2 exception: U+1180 HANGUL JUNGSEONG O-E vs U+116C HANGUL JUNGSEONG OE.
        let norm: String = upper.replace('_', " ").split_whitespace().collect::<Vec<_>>().join(" ");
        if norm == "HANGUL JUNGSEONG O-E" {
            return Some(NameTarget::Cp(0x1180));
        }
        let key = loose(&upper);
        if let Some(t) = self.name_index.get(&key) {
            return Some(*t);
        }
        for (lo, hi, pat) in &self.name_patterns {
            let prefix = loose(pat.trim_end_matches('*').trim_end_matches('-'));
            if let Some(h) = key.strip_prefix(&prefix) {
                if (4..=6).contains(&h.len()) && h.bytes().all(|b| b.is_ascii_hexdigit()) {
                    let cp = hex(h);
                    if *lo <= cp && cp <= *hi {
                        return Some(NameTarget::Cp(cp));
                    }
                }
            }
        }
        None
    }

    /// Longest named sequence (named, emoji, ZWJ) starting at `cps[i]`.
    pub fn sequence_at(&self, cps: &[u32], i: usize) -> Option<usize> {
        let cands = self.seq_by_first.get(&cps[i])?;
        cands.iter().copied().find(|&s| {
            let q = &self.sequences[s].cps;
            cps.len() - i >= q.len() && cps[i..i + q.len()] == q[..]
        })
    }

    /// Algorithmic canonical decomposition of a precomposed Hangul syllable.
    pub fn hangul_decomposition(cp: u32) -> Option<Vec<u32>> {
        const S_BASE: u32 = 0xAC00;
        const S_COUNT: u32 = 11172;
        if !(S_BASE..S_BASE + S_COUNT).contains(&cp) {
            return None;
        }
        let s = cp - S_BASE;
        let mut v = vec![0x1100 + s / 588, 0x1161 + (s % 588) / 28];
        if s % 28 != 0 {
            v.push(0x11A7 + s % 28);
        }
        Some(v)
    }
}

pub fn is_noncharacter(cp: u32) -> bool {
    (0xFDD0..=0xFDEF).contains(&cp) || (cp & 0xFFFE) == 0xFFFE
}

/// UAX44-LM2 loose matching key: ignore case, whitespace, underscores, and
/// medial hyphens (a hyphen between two alphanumerics).
pub fn loose(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut out = String::with_capacity(s.len());
    for (i, &c) in chars.iter().enumerate() {
        if c.is_whitespace() || c == '_' {
            continue;
        }
        if c == '-' {
            let prev = i.checked_sub(1).and_then(|j| chars.get(j)).is_some_and(|p| p.is_alphanumeric());
            let next = chars.get(i + 1).is_some_and(|n| n.is_alphanumeric());
            if prev && next {
                continue;
            }
        }
        out.extend(c.to_uppercase());
    }
    out
}
