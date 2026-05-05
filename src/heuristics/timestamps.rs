//! NTFS timestamp precision and MACB consistency heuristics.

// NTFS stores 100-nanosecond intervals. A timestamp divisible by 10_000_000
// (= 1 second in 100-ns ticks) was written with only second-level precision —
// a common timestomping artifact.

/// Returns `true` if the FILETIME has sub-second precision.
/// Legitimate NTFS timestamps almost always have non-zero sub-second components.
#[must_use]
pub fn has_subsecond_precision(filetime: u64) -> bool {
    filetime % 10_000_000 != 0
}

/// Returns `true` if the FILETIME was written with only second-level precision
/// (sub-second component is exactly zero). Common timestomping indicator.
#[must_use]
pub fn is_low_precision_timestamp(filetime: u64) -> bool {
    filetime != 0 && !has_subsecond_precision(filetime)
}

/// Returns `true` if all four MACB timestamps are identical.
/// Timestomping tools commonly set all four to the same value.
#[must_use]
pub fn is_all_macb_identical(m: i64, a: i64, c: i64, b: i64) -> bool {
    m == a && a == c && c == b
}

/// Returns `true` if the timestamp (Unix nanoseconds) falls on an exact UTC hour boundary.
/// Manually-set timestamps are often rounded to whole hours — a human-set indicator.
#[must_use]
pub fn is_round_hour_timestamp(ts_ns: i64) -> bool {
    ts_ns > 0 && ts_ns % 3_600_000_000_000 == 0
}

/// Minimum plausible Windows install date (Unix seconds).
/// 2001-09-09 01:46:40 UTC = 1_000_000_000 — no legitimate install predates Windows XP.
pub const MIN_PLAUSIBLE_INSTALL_DATE_SECS: u32 = 1_000_000_000;

/// Returns `true` if the registry InstallDate is within a plausible range.
/// Values below MIN_PLAUSIBLE_INSTALL_DATE_SECS are spoofed or garbage.
#[must_use]
pub fn is_plausible_install_date(unix_secs: u32) -> bool {
    unix_secs >= MIN_PLAUSIBLE_INSTALL_DATE_SECS
}

// ── Tests ─────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subsecond_precision_nonzero_sub_second() {
        // filetime with non-zero sub-second component → true
        let filetime = 10_000_001u64; // 1 second + 1 tick
        assert!(has_subsecond_precision(filetime));
    }

    #[test]
    fn subsecond_precision_zero_sub_second() {
        // filetime % 10_000_000 == 0, != 0 → false
        let filetime = 10_000_000u64; // exactly 1 second
        assert!(!has_subsecond_precision(filetime));
    }

    #[test]
    fn low_precision_whole_second() {
        // filetime divisible by 10_000_000, not zero → true
        let filetime = 10_000_000u64;
        assert!(is_low_precision_timestamp(filetime));
    }

    #[test]
    fn low_precision_zero_filetime_returns_false() {
        // zero is not "low precision" (it's null)
        assert!(!is_low_precision_timestamp(0));
    }

    #[test]
    fn low_precision_with_sub_second_returns_false() {
        let filetime = 10_000_001u64;
        assert!(!is_low_precision_timestamp(filetime));
    }

    #[test]
    fn macb_identical_all_same() {
        assert!(is_all_macb_identical(100, 100, 100, 100));
    }

    #[test]
    fn macb_not_identical_m_differs() {
        assert!(!is_all_macb_identical(99, 100, 100, 100));
    }

    #[test]
    fn macb_not_identical_a_differs() {
        assert!(!is_all_macb_identical(100, 99, 100, 100));
    }

    #[test]
    fn round_hour_on_exact_hour() {
        // ts_ns = 3_600_000_000_000 (1970-01-01 01:00:00 UTC) → true
        assert!(is_round_hour_timestamp(3_600_000_000_000));
    }

    #[test]
    fn round_hour_not_on_hour() {
        assert!(!is_round_hour_timestamp(3_600_000_000_001));
    }

    #[test]
    fn round_hour_zero_is_not_round() {
        // ts_ns = 0 → false (guard: ts_ns > 0)
        assert!(!is_round_hour_timestamp(0));
    }

    #[test]
    fn round_hour_negative_is_not_round() {
        assert!(!is_round_hour_timestamp(-3_600_000_000_000));
    }

    #[test]
    fn plausible_install_date_above_threshold() {
        assert!(is_plausible_install_date(1_100_000_000));
    }

    #[test]
    fn plausible_install_date_at_threshold() {
        // exactly 1_000_000_000 → true
        assert!(is_plausible_install_date(MIN_PLAUSIBLE_INSTALL_DATE_SECS));
    }

    #[test]
    fn plausible_install_date_below_threshold() {
        // 999_999_999 → false
        assert!(!is_plausible_install_date(999_999_999));
    }
}
