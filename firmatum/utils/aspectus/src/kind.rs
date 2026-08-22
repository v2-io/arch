//! Text-vs-binary judgment for line counting (design/linecount.md).
//!
//! Kind comes from a config suffix-map, not magic (lattice), shipped with
//! the well-known suffixes and extensionless text names. An unknown suffix
//! falls to a cheap null-byte sniff of the first block — the design's
//! recorded leaning; flagged there for ratification since "not magic" was
//! the lattice's word for *kind*, and the sniff decides only count-vs-omit,
//! never a rendered kind word.

use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Text,
    Binary,
    /// Not in the map — the sniff decides at read time.
    Unknown,
}

/// Classify a `[kinds]` value. Bare `text`/`binary` still work; `MAJOR/MINOR`
/// follows design/defaults.md for this slice: `text/*`, `data/*`, `log/*`,
/// `image/svg` → text, the rest → binary. The full ladder is design/filetype.md.
pub fn classify(spec: &str) -> Option<Kind> {
    let spec = spec.trim();
    match spec {
        "text" => Some(Kind::Text),
        "binary" => Some(Kind::Binary),
        "!" => None,
        _ => match spec.split_once('/') {
            Some(("text", _)) | Some(("data", _)) | Some(("log", _)) => Some(Kind::Text),
            Some(("image", "svg")) => Some(Kind::Text),
            Some((_, _)) => Some(Kind::Binary),
            None => None, // unknown bare word claims nothing
        },
    }
}

/// The suffix-map with config overlays applied. Config key `kinds`:
/// `SUFFIX:text` / `SUFFIX:binary` / `NAME:text` comma-separated, or a
/// `[kinds]` table of `SUFFIX = "major/minor"`; `!SUFFIX` / `"SUFFIX" = "!"`
/// drops a shipped row back to unknown (sniff decides).
#[derive(Debug, Default)]
pub struct Map {
    /// Present `None` is an explicit drop (unknown / sniff).
    rows: BTreeMap<String, Option<Kind>>,
}

impl Map {
    pub fn shipped() -> Self {
        Self::from_pairs(
            crate::config::embedded()
                .kinds
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_str())),
        )
    }

    pub fn from_pairs<'a, I>(pairs: I) -> Self
    where
        I: IntoIterator<Item = (&'a str, &'a str)>,
    {
        let mut rows = BTreeMap::new();
        for (k, v) in pairs {
            if v.trim() == "!" {
                insert_kind(&mut rows, k, None);
            } else if let Some(kind) = classify(v) {
                insert_kind(&mut rows, k, Some(kind));
            }
        }
        Map { rows }
    }

    pub fn from_sourced(rows: &[crate::config::Sourced]) -> Self {
        Self::from_pairs(rows.iter().map(|r| (r.key.as_str(), r.value.as_str())))
    }

    pub fn with_config(rules: &str) -> Self {
        let mut map = Map::shipped();
        for rule in rules.split(',') {
            let rule = rule.trim();
            if rule.is_empty() {
                continue;
            }
            if let Some(dropped) = rule.strip_prefix('!') {
                insert_kind(&mut map.rows, dropped, None);
                continue;
            }
            if let Some((pat, kind)) = rule.rsplit_once(':') {
                match classify(kind) {
                    Some(k) => insert_kind(&mut map.rows, pat, Some(k)),
                    None if kind.trim() == "!" => insert_kind(&mut map.rows, pat, None),
                    None => {} // an unknown kind word claims nothing
                }
            }
        }
        map
    }

    /// Judge a basename. Suffix match first (case-insensitive), then the
    /// extensionless name list.
    pub fn kind(&self, name: &str) -> Kind {
        let key = match name.rsplit_once('.') {
            Some((stem, ext)) if !stem.is_empty() && !ext.is_empty() => ext.to_lowercase(),
            _ => name.to_string(),
        };
        if let Some(o) = self.rows.get(&key).or_else(|| self.rows.get(name)) {
            return o.unwrap_or(Kind::Unknown);
        }
        // Extensionless names in the file are stored as written (Makefile)
        // and sometimes lowercased (makefile); try the other case too.
        if key != name {
            if let Some(o) = self.rows.get(&norm(&key)) {
                return o.unwrap_or(Kind::Unknown);
            }
        } else {
            let lower = name.to_lowercase();
            if let Some(o) = self.rows.get(&lower) {
                return o.unwrap_or(Kind::Unknown);
            }
        }
        Kind::Unknown
    }
}

fn norm(pat: &str) -> String {
    pat.trim().trim_start_matches('.').to_string()
}

fn insert_kind(rows: &mut BTreeMap<String, Option<Kind>>, k: &str, v: Option<Kind>) {
    let key = norm(k);
    let lower = key.to_lowercase();
    if lower != key {
        rows.insert(lower, v);
    }
    rows.insert(key, v);
}

/// Physical lines: newline-terminated, plus a final unterminated line
/// counts as one (the reader's units — design/linecount.md).
pub fn physical_lines(bytes: &[u8]) -> u64 {
    if bytes.is_empty() {
        return 0;
    }
    let n = bytes.iter().filter(|&&b| b == b'\n').count() as u64;
    if bytes.last() == Some(&b'\n') {
        n
    } else {
        n + 1
    }
}

/// Non-blank: whitespace-only lines excluded.
pub fn non_blank_lines(bytes: &[u8]) -> u64 {
    bytes
        .split(|&b| b == b'\n')
        .filter(|l| l.iter().any(|b| !b.is_ascii_whitespace()))
        .count() as u64
}

/// A NUL in the first block means binary (the sniff for unknown suffixes).
pub fn looks_binary(first_block: &[u8]) -> bool {
    first_block.contains(&0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn physical_counts_unterminated_final_line() {
        assert_eq!(physical_lines(b"a\nb"), 2);
        assert_eq!(physical_lines(b"a\nb\n"), 2);
        assert_eq!(physical_lines(b""), 0);
    }

    #[test]
    fn config_overrides_shipped() {
        let m = Map::with_config(".md:binary, weird:text, !rs");
        assert_eq!(m.kind("a.md"), Kind::Binary);
        assert_eq!(m.kind("x.weird"), Kind::Text);
        assert_eq!(m.kind("y.rs"), Kind::Unknown);
        assert_eq!(m.kind("z.py"), Kind::Text);
    }
}
