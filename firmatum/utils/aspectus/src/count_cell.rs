//! The 12-cell count grammar (design/grid-cleanup.md §The count cell).
//!
//! ```text
//! col  1   2   3   4   5   6-8   9    10   11   12
//!      g   ␠   m   T   ·   NNN   .    f    s    u
//! ```
//!
//! One function, one field, every number in the look. Alignment is free
//! because every cell is 12 columns; the `.` is the structural anchor.
//! Scale starts at ≥ 10,000 (not 1,000): below that the value is exact to
//! the unit; at and above, three significant digits and one fraction digit.
//! Bytes divide by 1024; counts, lines, and tokens by 1000.
//!
//! Under a heading that names the subject and/or the unit, pass `subject =
//! None` and/or `show_unit = false`. The slots stay (a space), so a `3.3P`
//! under `bytes` sits where `3.3PB` would beside a glyph.
//!
//! Marks: `≥` floor, `~` estimated, `≈` exact-but-grouped (the formatter
//! applies `≈` itself when an exact value scales), blank when exact and
//! ungrouped. Callers never drop a mark they earned.

/// What the number counts (col 1). Blank when a heading names it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Subject {
    /// ● files
    Files,
    /// □ dirs
    Dirs,
    /// ▣ files+dirs
    Both,
}

impl Subject {
    fn glyph(self) -> char {
        match self {
            Subject::Files => '\u{25CF}', // ●
            Subject::Dirs => '\u{25A1}',  // □
            Subject::Both => '\u{25A3}',  // ▣
        }
    }
}

/// The unit slot (col 12). `𝓃` count · `B` bytes · `𝓁` lines · `𝓉` tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unit {
    Count,
    Bytes,
    Lines,
    Tokens,
}

impl Unit {
    fn glyph(self) -> char {
        match self {
            Unit::Count => '\u{1D4C3}', // 𝓃
            Unit::Bytes => 'B',
            Unit::Lines => '\u{1D4C1}',  // 𝓁
            Unit::Tokens => '\u{1D4C9}', // 𝓉
        }
    }

    fn base(self) -> f64 {
        match self {
            Unit::Bytes => 1024.0,
            Unit::Count | Unit::Lines | Unit::Tokens => 1000.0,
        }
    }
}

/// Honesty mark (col 3). `Exact` becomes `≈` when the formatter scales.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mark {
    Exact,
    Floor,
    Estimated,
}

const SCALES: [char; 5] = ['K', 'M', 'G', 'T', 'P'];
const MIDDOT: char = '\u{00B7}'; // ·
const SCALE_AT: u64 = 10_000;

/// Right-align a heading over the count cell so the word ends at the `.`
/// (cell 9). Cells 10–12 stay reserved: the field is still 12, so a
/// following column (heat) does not move. Under a heading the value's
/// `f s u` are blank, and a word ending at cell 12 sat three cells right
/// of the digits it names.
pub fn heading(word: &str) -> String {
    let mut c = [' '; 12];
    let chars: Vec<char> = word.chars().collect();
    // Cell 9 is index 8. Words longer than 9 would overwrite the reserved
    // tail; the shipped headings (`lines`, `bytes`) are five.
    debug_assert!(
        chars.len() <= 9,
        "count-cell heading longer than the `.` column: {word}"
    );
    let n = chars.len().min(9);
    let start = 9 - n;
    for (i, ch) in chars.into_iter().take(n).enumerate() {
        c[start + i] = ch;
    }
    c.iter().collect()
}

/// Render `value` as a 12-cell field. Always 12 `char`s, always a `.` at
/// col 9. `subject = None` blanks col 1; `show_unit = false` blanks col 12.
pub fn count_cell(
    value: u64,
    mark: Mark,
    subject: Option<Subject>,
    unit: Unit,
    show_unit: bool,
) -> String {
    let mut c = [' '; 12];
    if let Some(s) = subject {
        c[0] = s.glyph();
    }
    // c[1] is the always-space
    c[8] = '.';
    if show_unit {
        c[11] = unit.glyph();
    }

    if value < SCALE_AT {
        place_exact(&mut c, value);
        c[2] = match mark {
            Mark::Floor => '\u{2265}', // ≥
            Mark::Estimated => '~',
            Mark::Exact => ' ',
        };
    } else {
        let (int_part, frac, scale_idx) = scale(value, unit.base());
        c[2] = match mark {
            Mark::Floor => '\u{2265}',
            Mark::Estimated => '~',
            Mark::Exact => '\u{2248}', // ≈ — grouped by scaling
        };
        place_scaled(&mut c, int_part, frac, scale_idx);
    }
    c.iter().collect()
}

fn place_exact(c: &mut [char; 12], n: u64) {
    if n >= 1000 {
        c[3] = char::from(b'0' + (n / 1000) as u8);
        c[4] = MIDDOT;
        write3(c, 5, n % 1000, true);
    } else {
        write3(c, 5, n, false);
    }
}

fn place_scaled(c: &mut [char; 12], int_part: u64, frac: u8, scale_idx: usize) {
    c[9] = char::from(b'0' + frac);
    c[10] = SCALES[scale_idx];
    if int_part >= 1000 {
        // Only reachable at P when rounding still leaves four integer
        // digits — T·NNN.fP keeps the 12-cell field.
        c[3] = char::from(b'0' + (int_part / 1000) as u8);
        c[4] = MIDDOT;
        write3(c, 5, int_part % 1000, true);
    } else {
        write3(c, 5, int_part, false);
    }
}

/// Write `n` into three cells starting at `at`. `zero_pad` fills 048;
/// otherwise spaces (912, 14, 1) — a token-reader sees leading zeros as
/// content, the same reason the field will not use dotted leaders.
fn write3(c: &mut [char; 12], at: usize, n: u64, zero_pad: bool) {
    let h = (n / 100) as u8;
    let t = ((n / 10) % 10) as u8;
    let o = (n % 10) as u8;
    if zero_pad || h > 0 {
        c[at] = char::from(b'0' + h);
        c[at + 1] = char::from(b'0' + t);
        c[at + 2] = char::from(b'0' + o);
    } else if t > 0 || n >= 10 {
        c[at + 1] = char::from(b'0' + t);
        c[at + 2] = char::from(b'0' + o);
    } else {
        c[at + 2] = char::from(b'0' + o);
    }
}

/// Mantissa in `[1, 1000)` with one decimal, plus the scale index into
/// `SCALES`. First division is required (`value ≥ 10_000`); further
/// divisions keep the integer part to three digits (`1.0M` not `1·000.0K`).
fn scale(n: u64, base: f64) -> (u64, u8, usize) {
    let mut v = n as f64;
    let mut idx = 0usize;
    while idx < SCALES.len() {
        v /= base;
        idx += 1;
        if v < 1000.0 {
            break;
        }
    }
    let mut tenths = (v * 10.0).round() as u64;
    if tenths >= 10_000 && idx < SCALES.len() {
        v = tenths as f64 / 10.0 / base;
        idx += 1;
        tenths = (v * 10.0).round() as u64;
    }
    (tenths / 10, (tenths % 10) as u8, idx.saturating_sub(1))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn show(s: &str) -> String {
        s.replace(' ', "␣")
    }

    fn assert_cell(s: &str, want: &str) {
        assert_eq!(
            s.chars().count(),
            12,
            "width {}: {}",
            s.chars().count(),
            show(s)
        );
        assert_eq!(s, want, "\n  got  {}\n  want {}", show(s), show(want));
    }

    #[test]
    fn exact_below_ten_thousand() {
        // 1·099, 9·021, 912, 0 — design/grid-cleanup.md specimens.
        // Blank mark still occupies col 3, so a heading-blanked 1·099 is
        // three leading spaces (g, always-sp, m) not two.
        assert_cell(
            &count_cell(1_099, Mark::Exact, Some(Subject::Files), Unit::Count, true),
            "●  1·099.  𝓃",
        );
        assert_cell(
            &count_cell(9_021, Mark::Floor, Some(Subject::Dirs), Unit::Bytes, true),
            "□ ≥9·021.  B",
        );
        assert_cell(
            &count_cell(912, Mark::Exact, None, Unit::Lines, false),
            "     912.   ",
        );
        assert_cell(
            &count_cell(0, Mark::Exact, None, Unit::Lines, false),
            "       0.   ",
        );
        assert_cell(
            &count_cell(1, Mark::Exact, None, Unit::Lines, false),
            "       1.   ",
        );
        assert_cell(
            &count_cell(10, Mark::Exact, None, Unit::Lines, false),
            "      10.   ",
        );
        assert_cell(
            &count_cell(999, Mark::Exact, None, Unit::Count, true),
            "     999.  𝓃",
        );
        assert_cell(
            &count_cell(1_000, Mark::Exact, None, Unit::Count, false),
            "   1·000.   ",
        );
        assert_cell(
            &count_cell(9_999, Mark::Exact, None, Unit::Count, false),
            "   9·999.   ",
        );
    }

    #[test]
    fn scaled_three_sig_digits() {
        // 190.0K, 14.3G, 1.0M, 3.3P — and 61.2K / 2.4M from the specimens.
        // Exact + scaled ⇒ ≈ occupies col 3; T/· stay blank (mantissa < 1000).
        assert_cell(
            &count_cell(190_000, Mark::Exact, None, Unit::Count, false),
            "  ≈  190.0K ",
        );
        assert_cell(
            &count_cell(1_000_000, Mark::Exact, None, Unit::Count, false),
            "  ≈    1.0M ",
        );
        assert_cell(
            &count_cell(61_200, Mark::Exact, Some(Subject::Files), Unit::Lines, true),
            "● ≈   61.2K𝓁",
        );
        assert_cell(
            &count_cell(
                2_400_000,
                Mark::Estimated,
                Some(Subject::Files),
                Unit::Tokens,
                true,
            ),
            "● ~    2.4M𝓉",
        );
        // 14.3 GiB = 14.3 × 1024³.
        let gib = (14.3_f64 * 1024.0 * 1024.0 * 1024.0).round() as u64;
        assert_cell(
            &count_cell(gib, Mark::Exact, Some(Subject::Both), Unit::Bytes, true),
            "▣ ≈   14.3GB",
        );
        // 3.3 PiB = 3.3 × 1024⁵. Exact+scaled ⇒ ≈ even under a heading;
        // the design specimen omitted the mark to show g-blanking.
        let pib = (3.3 * 1024.0_f64.powi(5)).round() as u64;
        assert_cell(
            &count_cell(pib, Mark::Exact, None, Unit::Bytes, true),
            "  ≈    3.3PB",
        );
        assert_cell(
            &count_cell(pib, Mark::Exact, None, Unit::Bytes, false),
            "  ≈    3.3P ",
        );
        assert_cell(
            &count_cell(10_000, Mark::Exact, None, Unit::Count, false),
            "  ≈   10.0K ",
        );
        assert_cell(
            &count_cell(999_950, Mark::Exact, None, Unit::Count, false),
            "  ≈    1.0M ",
        );
    }

    #[test]
    fn marks_never_dropped_and_exact_ungrouped_is_blank() {
        assert_eq!(
            count_cell(6, Mark::Exact, None, Unit::Lines, false)
                .chars()
                .nth(2),
            Some(' ')
        );
        assert_eq!(
            count_cell(6, Mark::Floor, None, Unit::Lines, false)
                .chars()
                .nth(2),
            Some('\u{2265}')
        );
        assert_eq!(
            count_cell(6, Mark::Estimated, None, Unit::Lines, false)
                .chars()
                .nth(2),
            Some('~')
        );
        // Scaling groups an exact value; floor/estimate keep their face.
        assert_eq!(
            count_cell(61_200, Mark::Exact, None, Unit::Lines, false)
                .chars()
                .nth(2),
            Some('\u{2248}')
        );
        assert_eq!(
            count_cell(61_200, Mark::Floor, None, Unit::Lines, false)
                .chars()
                .nth(2),
            Some('\u{2265}')
        );
        assert_eq!(
            count_cell(61_200, Mark::Estimated, None, Unit::Lines, false)
                .chars()
                .nth(2),
            Some('~')
        );
    }

    #[test]
    fn heading_ends_at_the_dot_column() {
        // "lines" / "bytes" occupy cells 5–9; cells 10–12 reserved.
        assert_cell(&heading("lines"), "    lines   ");
        assert_cell(&heading("bytes"), "    bytes   ");
        assert_eq!(heading("lines").chars().nth(8), Some('s'));
        assert_eq!(heading("lines").chars().nth(9), Some(' '));
        // The `.` of a heading-blanked value is the same column.
        let v = count_cell(11, Mark::Exact, None, Unit::Lines, false);
        assert_eq!(v.chars().nth(8), Some('.'));
        assert_eq!(heading("lines").chars().count(), v.chars().count());
    }

    #[test]
    fn heading_blanking_keeps_width() {
        let full = count_cell(3, Mark::Exact, Some(Subject::Files), Unit::Lines, true);
        let blanked = count_cell(3, Mark::Exact, None, Unit::Lines, false);
        assert_eq!(full.chars().count(), 12);
        assert_eq!(blanked.chars().count(), 12);
        // Same `.` column either way.
        assert_eq!(full.chars().nth(8), Some('.'));
        assert_eq!(blanked.chars().nth(8), Some('.'));
        assert_eq!(blanked.chars().next(), Some(' '));
        assert_eq!(blanked.chars().nth(11), Some(' '));
    }

    #[test]
    fn bytes_scale_by_1024() {
        // 2048 B is below 10,000 — exact, not 2.0K.
        assert_cell(
            &count_cell(2048, Mark::Exact, None, Unit::Bytes, true),
            "   2·048.  B",
        );
        // 80 MiB = 80 × 1024² — scaled, so Exact wears ≈.
        let mib = 80u64 << 20;
        assert_cell(
            &count_cell(mib, Mark::Exact, None, Unit::Bytes, false),
            "  ≈   80.0M ",
        );
    }
}
