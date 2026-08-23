//! SIGNA — zoetica's visual time notation (design/phenom-format.md).
//!
//! Mixed-radix run, largest unit first. The column shows the two
//! most-significant units (≤14 cells); seconds-grain glyphs are omitted
//! whenever a minute or larger unit is present. ≥10 years is `⬤` ×9
//! capped. Pure function of elapsed seconds (mtime vs the look's stamp).

/// Month and year match `ready::rel_age` so the two spellings of one
/// duration agree on the calendar (365.2425-day year / 12).
const YEAR: i64 = 31_556_952;
const MONTH: i64 = 2_629_746;
const WEEK: i64 = 604_800;
const DAY: i64 = 86_400;
const HOUR: i64 = 3_600;
const MIN: i64 = 60;

/// Largest first. Max-count is the table in
/// `~/src/_core/zoetica/docs/messaging/06-temporal-coherence.md` §SIGNA.
const UNITS: &[(char, i64, i64)] = &[
    ('⬤', YEAR, 9),
    ('◉', 2 * MONTH, 5),
    ('◎', WEEK, 7),
    ('○', DAY, 6),
    ('⚬', 4 * HOUR, 7),
    ('═', HOUR, 3),
    ('━', 10 * MIN, 5),
    ('╍', MIN, 9),
    ('╌', 10, 5),
    ('╶', 5, 1),
    ('·', 1, 4),
];

const SECONDS_GRAIN: &[char] = &['·', '╶', '╌'];

/// Elapsed seconds → SIGNA run. `·` at 0 (just now); `"future"` when `secs < 0`.
pub fn format(secs: i64) -> String {
    if secs < 0 {
        return "future".to_string();
    }
    if secs == 0 {
        // Just now: the 1-second grain, so a speaking mtime that's "now"
        // still has a glyph (blank would look silent).
        return "·".to_string();
    }
    if secs >= 10 * YEAR {
        return "⬤".repeat(9);
    }
    let mut remaining = secs;
    let mut parts: Vec<(char, usize)> = Vec::new();
    for &(glyph, unit, max) in UNITS {
        let n = (remaining / unit).min(max) as usize;
        if n > 0 {
            parts.push((glyph, n));
            remaining -= n as i64 * unit;
        }
    }
    let has_minute = parts.iter().any(|(g, _)| !SECONDS_GRAIN.contains(g));
    if has_minute {
        parts.retain(|(g, _)| !SECONDS_GRAIN.contains(g));
    }
    parts.truncate(2);
    parts
        .into_iter()
        .flat_map(|(g, n)| std::iter::repeat_n(g, n))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_is_one_tick() {
        assert_eq!(format(0), "·");
    }

    #[test]
    fn future_word() {
        assert_eq!(format(-1), "future");
    }

    #[test]
    fn seconds_mixed_radix() {
        assert_eq!(format(1), "·");
        assert_eq!(format(4), "····");
        assert_eq!(format(5), "╶");
        // 7 = 5 + 2. The primary's `·······` illustration exceeds ·'s max
        // of 4; mixed-radix with the table's max-counts is the law.
        assert_eq!(format(7), "╶··");
        assert_eq!(format(10), "╌");
        assert_eq!(format(23), "╌╌···"); // 20 + 3
    }

    #[test]
    fn seconds_omitted_when_minute_present() {
        assert_eq!(format(60), "╍");
        assert_eq!(format(83), "╍"); // 1m 23s — seconds dropped
        assert_eq!(format(9 * 60), "╍╍╍╍╍╍╍╍╍");
    }

    #[test]
    fn primary_examples_that_match_the_table() {
        // 3h 15m full run is ═══━╍╍╍╍╍ (three unit ranks); the column
        // keeps two, so the 5 minutes drop: ═══━ = 3h 10m.
        assert_eq!(format(3 * HOUR + 15 * MIN), "═══━");
        // 1d 8h = ○ + ⚬×2
        assert_eq!(format(DAY + 8 * HOUR), "○⚬⚬");
        // 2w 3d
        assert_eq!(format(2 * WEEK + 3 * DAY), "◎◎○○○");
        // 1y 5mo → two-unit truncation: ⬤ + ◉×2 (4 of 5 months)
        assert_eq!(format(YEAR + 5 * MONTH), "⬤◉◉");
    }

    #[test]
    fn two_unit_bound() {
        // 3h 59m would be three unit ranks; the third is dropped.
        let s = format(3 * HOUR + 59 * MIN);
        assert_eq!(s, "═══━━━━━"); // 3h + 50m
        assert!(s.chars().count() <= 14);
    }

    #[test]
    fn ten_years_caps() {
        assert_eq!(format(10 * YEAR), "⬤⬤⬤⬤⬤⬤⬤⬤⬤");
        assert_eq!(format(14 * YEAR), "⬤⬤⬤⬤⬤⬤⬤⬤⬤");
        assert_eq!(format(9 * YEAR).chars().count(), 9);
    }
}
