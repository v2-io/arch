//! Filetype — one `major/minor` (+trait) per node (design/filetype.md).
//!
//! Ladder, highest first: stat type → empty → exec-bit (marks, does not
//! decide) → magic bytes → shebang → suffix/name map (the tie-breaker
//! `file(1)` refuses) → null-byte sniff → unknown. Magic, shebang, and
//! the sniff share one ≤1 KiB window; they never open the file a second
//! time. The read budget still governs whether a file is read at all.
//!
//! Line-count helpers live here because they consume this fact.

use std::collections::BTreeMap;
use std::path::Path;

/// Closed majors. Minors are open-ended (config + magic).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Major {
    Dir,
    Link,
    Special,
    Text,
    Data,
    Log,
    Exe,
    Image,
    Media,
    Doc,
    Archive,
    Font,
    Object,
    Model,
    Binary,
    Empty,
    Unknown,
}

impl Major {
    pub fn as_str(self) -> &'static str {
        match self {
            Major::Dir => "dir",
            Major::Link => "link",
            Major::Special => "special",
            Major::Text => "text",
            Major::Data => "data",
            Major::Log => "log",
            Major::Exe => "exe",
            Major::Image => "image",
            Major::Media => "media",
            Major::Doc => "doc",
            Major::Archive => "archive",
            Major::Font => "font",
            Major::Object => "object",
            Major::Model => "model",
            Major::Binary => "binary",
            Major::Empty => "empty",
            Major::Unknown => "unknown",
        }
    }

    fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "dir" => Major::Dir,
            "link" => Major::Link,
            "special" => Major::Special,
            "text" => Major::Text,
            "data" => Major::Data,
            "log" => Major::Log,
            "exe" => Major::Exe,
            "image" => Major::Image,
            "media" => Major::Media,
            "doc" => Major::Doc,
            "archive" => Major::Archive,
            "font" => Major::Font,
            "object" => Major::Object,
            "model" => Major::Model,
            "binary" => Major::Binary,
            "empty" => Major::Empty,
            "unknown" => Major::Unknown,
            _ => return None,
        })
    }
}

/// One node's type. `minor` is empty for majors that don't split
/// (`dir`, `empty`, `unknown`). `trait_` is interpreter, binary format,
/// the link target's major, or `+x`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileType {
    pub major: Major,
    pub minor: String,
    pub trait_: Option<String>,
}

impl Default for FileType {
    fn default() -> Self {
        FileType::unknown()
    }
}

impl FileType {
    pub fn new(major: Major, minor: &str) -> Self {
        FileType {
            major,
            minor: minor.to_string(),
            trait_: None,
        }
    }

    pub fn dir() -> Self {
        FileType::new(Major::Dir, "")
    }

    pub fn empty() -> Self {
        FileType::new(Major::Empty, "")
    }

    pub fn unknown() -> Self {
        FileType::new(Major::Unknown, "")
    }

    pub fn link(minor: &str) -> Self {
        FileType::new(Major::Link, minor)
    }

    pub fn special(minor: &str) -> Self {
        FileType::new(Major::Special, minor)
    }

    fn with_trait(mut self, t: impl Into<String>) -> Self {
        let t = t.into();
        if !t.is_empty() {
            self.trait_ = Some(t);
        }
        self
    }

    /// Line-count / mass: `text/*`, text-ish `data/*`, `log/*`,
    /// `exe/script`, `image/svg`. Empty keeps a real `0` (the Open
    /// leaning's "keeps 0 honest for real empty text"). Everything else
    /// omits — never `0`.
    pub fn counts_lines(&self) -> bool {
        match self.major {
            Major::Text | Major::Log | Major::Empty => true,
            Major::Data => !matches!(self.minor.as_str(), "sqlite" | "parquet" | "arrow" | "npz"),
            Major::Exe => self.minor == "script",
            Major::Image => self.minor == "svg",
            _ => false,
        }
    }

    /// Quiet kind-word: the major, absent for unknown (not in statistics).
    pub fn kind_word(&self) -> Option<&'static str> {
        match self.major {
            Major::Unknown => None,
            m => Some(m.as_str()),
        }
    }
}

/// Census bucket grain (design/filetype.md). Default `suffix` keeps
/// today's `md×19` form.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CensusGrain {
    #[default]
    Suffix,
    Minor,
    Major,
}

impl CensusGrain {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "suffix" => Some(CensusGrain::Suffix),
            "minor" => Some(CensusGrain::Minor),
            "major" => Some(CensusGrain::Major),
            _ => None,
        }
    }
}

/// Classify a `[kinds]` value. Bare `text`/`binary` still work
/// (`text/plain`, `binary/plain`).
pub fn classify(spec: &str) -> Option<FileType> {
    let spec = spec.trim();
    match spec {
        "text" => Some(FileType::new(Major::Text, "plain")),
        "binary" => Some(FileType::new(Major::Binary, "plain")),
        "!" => None,
        _ => {
            let (maj, min) = spec.split_once('/')?;
            let major = Major::parse(maj)?;
            Some(FileType::new(major, min))
        }
    }
}

/// The suffix/name map — the ladder's tie-breaker, data not code.
/// Config key `kinds`: `SUFFIX:MAJOR/MINOR` or the legacy `SUFFIX:text`;
/// `!SUFFIX` / `"SUFFIX" = "!"` drops a shipped row (sniff decides).
#[derive(Debug, Default)]
pub struct Map {
    rows: BTreeMap<String, Option<FileType>>,
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
            } else if let Some(ft) = classify(v) {
                insert_kind(&mut rows, k, Some(ft));
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
                    Some(ft) => insert_kind(&mut map.rows, pat, Some(ft)),
                    None if kind.trim() == "!" => insert_kind(&mut map.rows, pat, None),
                    None => {}
                }
            }
        }
        map
    }

    /// Suffix first (case-insensitive), then the extensionless name list.
    pub fn lookup(&self, name: &str) -> Option<FileType> {
        let key = match name.rsplit_once('.') {
            Some((stem, ext)) if !stem.is_empty() && !ext.is_empty() => ext.to_lowercase(),
            _ => name.to_string(),
        };
        if let Some(o) = self.rows.get(&key).or_else(|| self.rows.get(name)) {
            return o.clone();
        }
        if key != name {
            if let Some(o) = self.rows.get(&norm(&key)) {
                return o.clone();
            }
        } else {
            let lower = name.to_lowercase();
            if let Some(o) = self.rows.get(&lower) {
                return o.clone();
            }
        }
        None
    }
}

fn norm(pat: &str) -> String {
    pat.trim().trim_start_matches('.').to_string()
}

fn insert_kind(rows: &mut BTreeMap<String, Option<FileType>>, k: &str, v: Option<FileType>) {
    let key = norm(k);
    let lower = key.to_lowercase();
    if lower != key {
        rows.insert(lower, v.clone());
    }
    rows.insert(key, v);
}

/// Bytes sniffed for magic / shebang / the null-byte floor. The same
/// window the line-count sniff already used; never a second classification
/// read.
pub const SNIFF_BYTES: usize = 1024;

/// Stat type — ladder step 1. `None` means a regular file, continue.
pub fn from_stat(ft: std::fs::FileType) -> Option<FileType> {
    use std::os::unix::fs::FileTypeExt;
    if ft.is_symlink() {
        return Some(FileType::link("file"));
    }
    if ft.is_dir() {
        return Some(FileType::dir());
    }
    if ft.is_fifo() {
        return Some(FileType::special("fifo"));
    }
    if ft.is_socket() {
        return Some(FileType::special("socket"));
    }
    if ft.is_block_device() {
        return Some(FileType::special("block"));
    }
    if ft.is_char_device() {
        return Some(FileType::special("char"));
    }
    None
}

/// Ladder from a ≤1 KiB window plus the suffix-map candidate.
///
/// `mapped` is the tie-breaker, consulted only when bytes don't decide.
/// Shebang records the interpreter as a trait; it does **not** rewrite a
/// suffix-named type to `exe/script` (Open leaning: exe is a trait — a
/// `+x` Python file is still Python to the census). No suffix + shebang
/// → `exe/script`. Native binaries from magic are `exe/binary`.
pub fn from_window(head: &[u8], mapped: Option<FileType>, exec: bool) -> FileType {
    let mut interpreter: Option<String> = None;

    if let Some(ft) = magic(head, &mut interpreter) {
        return with_exec(ft, exec);
    }
    if interpreter.is_none() {
        interpreter = shebang(head);
    }
    if let Some(mut ft) = mapped {
        if let Some(interp) = interpreter {
            ft.trait_ = Some(interp);
        } else if exec && ft.major != Major::Exe {
            ft.trait_ = Some("+x".into());
        }
        return ft;
    }
    if let Some(interp) = interpreter {
        return with_exec(FileType::new(Major::Exe, "script").with_trait(interp), exec);
    }
    if looks_binary(head) {
        with_exec(FileType::new(Major::Binary, "plain"), exec)
    } else {
        with_exec(FileType::new(Major::Text, "plain"), exec)
    }
}

fn with_exec(mut ft: FileType, exec: bool) -> FileType {
    if exec && ft.trait_.is_none() && ft.major != Major::Exe {
        ft.trait_ = Some("+x".into());
    }
    ft
}

/// Magic table. `#!` does not decide — it hands the interpreter to the
/// shebang step via `interpreter`. Anything else that matches decides.
fn magic(head: &[u8], interpreter: &mut Option<String>) -> Option<FileType> {
    if head.starts_with(b"#!") {
        *interpreter = shebang(head);
        return None;
    }
    if head.starts_with(b"\x89PNG\r\n\x1a\n") {
        return Some(FileType::new(Major::Image, "png"));
    }
    if head.len() >= 3 && head[0] == 0xff && head[1] == 0xd8 && head[2] == 0xff {
        return Some(FileType::new(Major::Image, "jpeg"));
    }
    if head.starts_with(b"GIF87a") || head.starts_with(b"GIF89a") {
        return Some(FileType::new(Major::Image, "gif"));
    }
    if head.len() >= 12 && head.starts_with(b"RIFF") && &head[8..12] == b"WEBP" {
        return Some(FileType::new(Major::Image, "webp"));
    }
    if head.starts_with(b"BM") {
        return Some(FileType::new(Major::Image, "bmp"));
    }
    if head.len() >= 4 && head[0] == 0 && head[1] == 0 && head[2] == 1 && head[3] == 0 {
        return Some(FileType::new(Major::Image, "ico"));
    }
    if head.starts_with(b"II*\0") || head.starts_with(b"MM\0*") {
        return Some(FileType::new(Major::Image, "tiff"));
    }
    if let Some(ft) = ftyp(head) {
        return Some(ft);
    }
    if head.starts_with(b"%PDF") {
        return Some(FileType::new(Major::Doc, "pdf"));
    }
    if head.starts_with(b"PK\x03\x04")
        || head.starts_with(b"PK\x05\x06")
        || head.starts_with(b"PK\x07\x08")
    {
        return Some(zip_family(head));
    }
    if head.starts_with(b"\x1f\x8b") {
        return Some(FileType::new(Major::Archive, "gzip"));
    }
    if head.starts_with(b"BZh") {
        return Some(FileType::new(Major::Archive, "bzip2"));
    }
    if head.starts_with(b"\xfd7zXZ\0") {
        return Some(FileType::new(Major::Archive, "xz"));
    }
    if head.len() >= 4 && head[0] == 0x28 && head[1] == 0xb5 && head[2] == 0x2f && head[3] == 0xfd {
        return Some(FileType::new(Major::Archive, "zstd"));
    }
    if head.starts_with(b"7z\xbc\xaf'\x1c") {
        return Some(FileType::new(Major::Archive, "7z"));
    }
    if head.len() >= 262 && &head[257..262] == b"ustar" {
        return Some(FileType::new(Major::Archive, "tar"));
    }
    if head.starts_with(b"SQLite format 3\0") {
        return Some(FileType::new(Major::Data, "sqlite"));
    }
    if head.starts_with(b"\0asm") {
        return Some(FileType::new(Major::Object, "wasm"));
    }
    if let Some(ft) = macho_elf_pe(head) {
        return Some(ft);
    }
    if head.len() >= 12 && head.starts_with(b"RIFF") {
        match &head[8..12] {
            b"WAVE" => return Some(FileType::new(Major::Media, "wav")),
            b"AVI " => return Some(FileType::new(Major::Media, "avi")),
            _ => {}
        }
    }
    if head.starts_with(b"fLaC") {
        return Some(FileType::new(Major::Media, "flac"));
    }
    if head.starts_with(b"OggS") {
        return Some(FileType::new(Major::Media, "ogg"));
    }
    if head.starts_with(b"ID3") || mpeg_audio(head) {
        return Some(FileType::new(Major::Media, "mp3"));
    }
    if head.starts_with(b"wOFF") {
        return Some(FileType::new(Major::Font, "woff"));
    }
    if head.starts_with(b"wOF2") {
        return Some(FileType::new(Major::Font, "woff2"));
    }
    if head.starts_with(b"OTTO") {
        return Some(FileType::new(Major::Font, "otf"));
    }
    if head.len() >= 4 && head[0] == 0 && head[1] == 1 && head[2] == 0 && head[3] == 0 {
        return Some(FileType::new(Major::Font, "ttf"));
    }
    None
}

fn zip_family(head: &[u8]) -> FileType {
    // OOXML typically stores `[Content_Types].xml` as the first zip
    // entry, so it often sits in the 1 KiB window. Distinguishing
    // word/xl/ppt is opportunistic; suffix remains the tie-breaker when
    // the window doesn't say.
    if find(head, b"[Content_Types].xml") {
        if find(head, b"word/") {
            return FileType::new(Major::Doc, "word");
        }
        if find(head, b"xl/") {
            return FileType::new(Major::Doc, "excel");
        }
        if find(head, b"ppt/") {
            return FileType::new(Major::Doc, "powerpoint");
        }
        return FileType::new(Major::Doc, "ooxml");
    }
    FileType::new(Major::Archive, "zip")
}

fn find(hay: &[u8], needle: &[u8]) -> bool {
    hay.windows(needle.len()).any(|w| w == needle)
}

fn ftyp(head: &[u8]) -> Option<FileType> {
    if head.len() < 12 || &head[4..8] != b"ftyp" {
        return None;
    }
    let brand = &head[8..12];
    const HEIC: [&[u8]; 5] = [b"heic", b"heix", b"hevc", b"mif1", b"msf1"];
    if HEIC.contains(&brand) {
        return Some(FileType::new(Major::Image, "heic"));
    }
    if brand == b"qt  " {
        return Some(FileType::new(Major::Media, "mov"));
    }
    Some(FileType::new(Major::Media, "mp4"))
}

fn mpeg_audio(head: &[u8]) -> bool {
    head.len() >= 2 && head[0] == 0xff && (head[1] & 0xe0) == 0xe0
}

fn macho_elf_pe(head: &[u8]) -> Option<FileType> {
    if head.starts_with(b"\x7fELF") {
        return Some(FileType::new(Major::Exe, "binary").with_trait("elf"));
    }
    // Mach-O 32/64, both endians, plus fat. CAFE BABE is also a Java
    // class: nfat_arch in 1..=32 distinguishes a fat header from a
    // classfile (whose version word at the same offset is typically 45–70).
    const MACHO: [&[u8]; 4] = [
        b"\xfe\xed\xfa\xce",
        b"\xfe\xed\xfa\xcf",
        b"\xce\xfa\xed\xfe",
        b"\xcf\xfa\xed\xfe",
    ];
    if MACHO.iter().any(|m| head.starts_with(m)) {
        return Some(FileType::new(Major::Exe, "binary").with_trait("mach-o"));
    }
    if head.len() >= 8 && head.starts_with(b"\xca\xfe\xba\xbe") {
        let narch = u32::from_be_bytes(head[4..8].try_into().ok()?);
        if (1..=32).contains(&narch) {
            return Some(FileType::new(Major::Exe, "binary").with_trait("mach-o"));
        }
        return Some(FileType::new(Major::Object, "class"));
    }
    if head.starts_with(b"\xbe\xba\xfe\xca") {
        return Some(FileType::new(Major::Exe, "binary").with_trait("mach-o"));
    }
    if head.starts_with(b"MZ") && head.len() >= 0x40 {
        let pe = u32::from_le_bytes(head[0x3c..0x40].try_into().ok()?);
        let pe = pe as usize;
        if pe + 4 <= head.len() && &head[pe..pe + 4] == b"PE\0\0" {
            return Some(FileType::new(Major::Exe, "binary").with_trait("pe"));
        }
    }
    None
}

fn shebang(head: &[u8]) -> Option<String> {
    if !head.starts_with(b"#!") {
        return None;
    }
    let line = head[2..].split(|&b| b == b'\n' || b == b'\r').next()?;
    let text = std::str::from_utf8(line).ok()?.trim();
    if text.is_empty() {
        return None;
    }
    let mut toks = text.split_whitespace();
    let first = toks.next()?;
    let name = Path::new(first)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(first);
    let interp = if name == "env" {
        toks.next().unwrap_or(name)
    } else {
        name
    };
    Some(normalize_interp(interp).to_string())
}

fn normalize_interp(name: &str) -> &str {
    let base = name
        .trim_end_matches(|c: char| c.is_ascii_digit() || c == '.')
        .trim_end_matches(|c: char| c.is_ascii_digit());
    match name {
        "python" | "python2" | "python3" | "pypy" | "pypy3" => "python",
        "node" | "nodejs" | "deno" | "bun" => "node",
        "ruby" | "irb" | "jruby" => "ruby",
        "bash" | "sh" | "zsh" | "dash" | "ksh" | "fish" => "sh",
        "perl" => "perl",
        "lua" => "lua",
        "php" => "php",
        "awk" | "gawk" | "nawk" => "awk",
        _ => {
            // python3.12 → python; otherwise keep the basename
            match base {
                "python" | "pypy" => "python",
                "node" => "node",
                other => {
                    if other.is_empty() {
                        name
                    } else {
                        other
                    }
                }
            }
        }
    }
}

fn suffix_label(name: &str) -> String {
    match name.rsplit_once('.') {
        Some((stem, ext)) if !stem.is_empty() && !ext.is_empty() && !ext.contains('/') => {
            ext.to_string()
        }
        _ => "other".to_string(),
    }
}

/// Census bucket label for a name given its type and the caller's grain.
pub fn census_key(name: &str, ft: &FileType, grain: CensusGrain) -> String {
    match grain {
        CensusGrain::Suffix => suffix_label(name),
        CensusGrain::Minor if !ft.minor.is_empty() => ft.minor.clone(),
        CensusGrain::Major if ft.major != Major::Unknown => ft.major.as_str().to_string(),
        _ => suffix_label(name),
    }
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
        assert_eq!(m.lookup("a.md").map(|t| t.major), Some(Major::Binary));
        assert_eq!(m.lookup("x.weird").map(|t| t.major), Some(Major::Text));
        assert!(m.lookup("y.rs").is_none());
        assert_eq!(m.lookup("z.py").map(|t| t.major), Some(Major::Text));
        assert_eq!(m.lookup("z.py").unwrap().minor, "python");
    }

    #[test]
    fn png_magic_beats_md_suffix() {
        let mut png = b"\x89PNG\r\n\x1a\n".to_vec();
        png.extend_from_slice(&[0; 16]);
        let mapped = Some(FileType::new(Major::Text, "markdown"));
        let ft = from_window(&png, mapped, false);
        assert_eq!(ft.major, Major::Image);
        assert_eq!(ft.minor, "png");
        assert!(!ft.counts_lines());
    }

    #[test]
    fn shebang_without_suffix_is_exe_script() {
        let head = b"#!/usr/bin/env python3\nprint(1)\n";
        let ft = from_window(head, None, true);
        assert_eq!(ft.major, Major::Exe);
        assert_eq!(ft.minor, "script");
        assert_eq!(ft.trait_.as_deref(), Some("python"));
        assert!(ft.counts_lines());
    }

    #[test]
    fn shebang_plus_py_suffix_stays_text_python() {
        // Open leaning: exe is a trait. Census still sees Python.
        let head = b"#!/usr/bin/env python3\nprint(1)\n";
        let mapped = Some(FileType::new(Major::Text, "python"));
        let ft = from_window(head, mapped, true);
        assert_eq!(ft.major, Major::Text);
        assert_eq!(ft.minor, "python");
        assert_eq!(ft.trait_.as_deref(), Some("python"));
        assert!(ft.counts_lines());
    }

    #[test]
    fn plusx_markdown_stays_markdown() {
        let head = b"# hi\n";
        let mapped = Some(FileType::new(Major::Text, "markdown"));
        let ft = from_window(head, mapped, true);
        assert_eq!(ft.major, Major::Text);
        assert_eq!(ft.minor, "markdown");
        assert_eq!(ft.trait_.as_deref(), Some("+x"));
    }

    #[test]
    fn elf_magic() {
        let mut elf = b"\x7fELF".to_vec();
        elf.extend_from_slice(&[1, 1, 1, 0, 0, 0, 0, 0]);
        let ft = from_window(&elf, None, true);
        assert_eq!(ft.major, Major::Exe);
        assert_eq!(ft.minor, "binary");
        assert_eq!(ft.trait_.as_deref(), Some("elf"));
        assert!(!ft.counts_lines());
    }

    #[test]
    fn svg_counts_as_image_and_lines() {
        let ft = FileType::new(Major::Image, "svg");
        assert!(ft.counts_lines());
        assert_eq!(ft.kind_word(), Some("image"));
    }

    #[test]
    fn sqlite_data_does_not_count_lines() {
        let ft = FileType::new(Major::Data, "sqlite");
        assert!(!ft.counts_lines());
        let json = FileType::new(Major::Data, "json");
        assert!(json.counts_lines());
    }

    #[test]
    fn empty_counts_zero() {
        assert!(FileType::empty().counts_lines());
        assert_eq!(physical_lines(b""), 0);
    }
}
