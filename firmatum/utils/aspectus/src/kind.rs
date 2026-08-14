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

const TEXT_SUFFIXES: &[&str] = &[
    "md", "txt", "rs", "py", "rb", "toml", "json", "yaml", "yml", "sh", "bash", "zsh",
    "c", "h", "cpp", "cc", "hpp", "js", "ts", "tsx", "jsx", "mjs", "html", "htm", "css",
    "scss", "tex", "bib", "ex", "exs", "erl", "hrl", "hs", "go", "java", "kt", "swift",
    "lock", "cfg", "ini", "conf", "xml", "svg", "csv", "tsv", "sql", "el", "lua", "vim",
    "zig", "pl", "pm", "r", "jl", "org", "rst", "adoc", "udon", "log", "diff", "patch",
    "env", "sum", "mod", "proto", "graphql", "sty", "cls", "typ", "nix", "d", "s", "asm",
];

const BINARY_SUFFIXES: &[&str] = &[
    "png", "jpg", "jpeg", "gif", "webp", "bmp", "ico", "pdf", "zip", "gz", "bz2", "xz",
    "zst", "tar", "tgz", "7z", "dylib", "so", "a", "o", "rlib", "rmeta", "class", "jar",
    "wasm", "ttf", "otf", "woff", "woff2", "eot", "mp3", "mp4", "mov", "avi", "wav",
    "flac", "ogg", "sqlite", "sqlite3", "db", "bin", "exe", "dmg", "pkg", "iso", "heic",
    "pyc", "pyo", "keynote", "pages", "numbers", "doc", "docx", "xls", "xlsx", "ppt",
    "pptx", "parquet", "arrow", "pt", "onnx", "gguf", "safetensors",
];

/// Extensionless names the estate reads as text every day.
const TEXT_NAMES: &[&str] = &[
    "Makefile", "makefile", "GNUmakefile", "LICENSE", "LICENCE", "README", "CHANGELOG",
    "CHANGES", "TODO", "NOTICE", "AUTHORS", "CONTRIBUTORS", "COPYING", "VERSION",
    "Dockerfile", "Rakefile", "Gemfile", "Justfile", "justfile", "Procfile", "CODEOWNERS",
    "SOURCE_REV",
];

/// The suffix-map with config overrides applied. Config key `kinds`:
/// `SUFFIX:text` / `SUFFIX:binary` / `NAME:text` comma-separated;
/// `!SUFFIX` drops a shipped row back to unknown (sniff decides).
#[derive(Debug, Default)]
pub struct Map {
    overrides: BTreeMap<String, Option<Kind>>,
}

impl Map {
    pub fn shipped() -> Self {
        Map::default()
    }

    pub fn with_config(rules: &str) -> Self {
        let mut overrides = BTreeMap::new();
        for rule in rules.split(',') {
            let rule = rule.trim();
            if rule.is_empty() {
                continue;
            }
            if let Some(dropped) = rule.strip_prefix('!') {
                overrides.insert(norm(dropped), None);
                continue;
            }
            if let Some((pat, kind)) = rule.rsplit_once(':') {
                let k = match kind.trim() {
                    "text" => Kind::Text,
                    "binary" => Kind::Binary,
                    _ => continue, // an unknown kind word claims nothing
                };
                overrides.insert(norm(pat), Some(k));
            }
        }
        Map { overrides }
    }

    /// Judge a basename. Suffix match first (case-insensitive), then the
    /// extensionless name list.
    pub fn kind(&self, name: &str) -> Kind {
        let key = match name.rsplit_once('.') {
            Some((stem, ext)) if !stem.is_empty() && !ext.is_empty() => ext.to_lowercase(),
            _ => name.to_string(),
        };
        if let Some(o) = self.overrides.get(&key).or_else(|| self.overrides.get(name)) {
            return o.unwrap_or(Kind::Unknown);
        }
        let k = key.as_str();
        if TEXT_SUFFIXES.contains(&k) || TEXT_NAMES.contains(&name) {
            Kind::Text
        } else if BINARY_SUFFIXES.contains(&k) {
            Kind::Binary
        } else {
            Kind::Unknown
        }
    }
}

fn norm(pat: &str) -> String {
    pat.trim().trim_start_matches('.').to_string()
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
