//! Header facts that locate a look: absolute root, time of the look.

use std::io;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn absolute_root(path: &Path) -> io::Result<PathBuf> {
    let abs = std::path::absolute(path)?;
    Ok(abs)
}

/// ISO-8601 / RFC-3339 UTC, second resolution: `YYYY-MM-DDTHH:MM:SSZ`.
pub fn stamp_utc(now: SystemTime) -> String {
    let secs = now
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let (y, m, d, hh, mm, ss) = civil_utc(secs);
    format!("{y:04}-{m:02}-{d:02}T{hh:02}:{mm:02}:{ss:02}Z")
}

/// Howard Hinnant civil-from-days, UTC.
fn civil_utc(unix_secs: u64) -> (i32, u32, u32, u32, u32, u32) {
    let z = (unix_secs / 86400) as i64 + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i32 + era as i32 * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = (if mp < 10 { mp + 3 } else { mp - 9 }) as u32;
    let y = y + if m <= 2 { 1 } else { 0 };
    let rem = (unix_secs % 86400) as u32;
    let hh = rem / 3600;
    let mm = (rem % 3600) / 60;
    let ss = rem % 60;
    (y, m, d, hh, mm, ss)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn unix_epoch_is_1970() {
        assert_eq!(stamp_utc(UNIX_EPOCH), "1970-01-01T00:00:00Z");
    }

    #[test]
    fn known_instant() {
        // 2026-08-14 00:00:00 UTC
        let t = UNIX_EPOCH + Duration::from_secs(1_786_665_600);
        assert_eq!(stamp_utc(t), "2026-08-14T00:00:00Z");
    }
}
