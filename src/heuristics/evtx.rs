//! EVTX event log "hiding" anomaly heuristics.
//!
//! Detection predicates for the technique described in Harlan Carvey's
//! [*Hiding In The Windows Event Log*](https://windowsir.blogspot.com/2023/07/hiding-in-windows-event-log.html)
//! (windowsir, 2023-07-08).
//!
//! RED-phase: tests reference predicates that do not yet exist; this file
//! intentionally fails to compile until the GREEN-phase implementation lands.

// ── Tests ─────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    // ── Constants ────────────────────────────────────────────────────────────

    #[test]
    fn reserved_event_id_max_is_zero() {
        assert_eq!(RESERVED_EVENT_ID_MAX, 0);
    }

    #[test]
    fn low_volume_channel_max_records_is_ten() {
        assert_eq!(LOW_VOLUME_CHANNEL_MAX_RECORDS, 10);
    }

    #[test]
    fn rare_source_id_pair_max_is_three() {
        assert_eq!(RARE_SOURCE_ID_PAIR_MAX, 3);
    }

    #[test]
    fn big_three_contains_security_system_application() {
        assert!(BIG_THREE_CHANNELS.contains(&"Security"));
        assert!(BIG_THREE_CHANNELS.contains(&"System"));
        assert!(BIG_THREE_CHANNELS.contains(&"Application"));
        assert_eq!(BIG_THREE_CHANNELS.len(), 3);
    }

    // ── is_reserved_event_id ─────────────────────────────────────────────────

    #[test]
    fn event_id_zero_is_reserved() {
        // Per Carvey 2023: threat-actor tools emit every record as ID 0.
        assert!(is_reserved_event_id(0));
    }

    #[test]
    fn event_id_one_is_not_reserved() {
        assert!(!is_reserved_event_id(1));
    }

    #[test]
    fn event_id_4624_logon_is_not_reserved() {
        // The classic "successful login" event — must not flag.
        assert!(!is_reserved_event_id(4624));
    }

    #[test]
    fn event_id_max_is_not_reserved() {
        assert!(!is_reserved_event_id(u32::MAX));
    }

    // ── is_low_volume_channel ────────────────────────────────────────────────

    #[test]
    fn empty_channel_is_low_volume() {
        // The post's KMS example: "not populated on any of them".
        assert!(is_low_volume_channel(0));
    }

    #[test]
    fn channel_with_ten_records_is_low_volume() {
        assert!(is_low_volume_channel(10));
    }

    #[test]
    fn channel_with_eleven_records_is_not_low_volume() {
        assert!(!is_low_volume_channel(11));
    }

    #[test]
    fn busy_channel_is_not_low_volume() {
        // Security.evtx with thousands of records — must not flag.
        assert!(!is_low_volume_channel(50_000));
    }

    // ── is_rare_source_id_pair ───────────────────────────────────────────────

    #[test]
    fn zero_occurrence_pair_is_not_rare() {
        // A pair seen zero times doesn't exist in the collection.
        assert!(!is_rare_source_id_pair(0));
    }

    #[test]
    fn single_occurrence_pair_is_rare() {
        assert!(is_rare_source_id_pair(1));
    }

    #[test]
    fn three_occurrence_pair_is_rare() {
        assert!(is_rare_source_id_pair(3));
    }

    #[test]
    fn four_occurrence_pair_is_not_rare() {
        assert!(!is_rare_source_id_pair(4));
    }

    #[test]
    fn very_common_pair_is_not_rare() {
        assert!(!is_rare_source_id_pair(10_000));
    }

    // ── is_big_three_channel / is_overlooked_channel ─────────────────────────

    #[test]
    fn security_is_big_three() {
        assert!(is_big_three_channel("Security"));
    }

    #[test]
    fn system_is_big_three() {
        assert!(is_big_three_channel("System"));
    }

    #[test]
    fn application_is_big_three() {
        assert!(is_big_three_channel("Application"));
    }

    #[test]
    fn big_three_match_is_case_insensitive() {
        assert!(is_big_three_channel("security"));
        assert!(is_big_three_channel("SYSTEM"));
        assert!(is_big_three_channel("ApPlIcAtIoN"));
    }

    #[test]
    fn key_management_service_is_overlooked() {
        // The post's headline example.
        assert!(is_overlooked_channel("Key Management Service"));
    }

    #[test]
    fn microsoft_windows_powershell_operational_is_overlooked() {
        assert!(is_overlooked_channel(
            "Microsoft-Windows-PowerShell/Operational"
        ));
    }

    #[test]
    fn security_is_not_overlooked() {
        assert!(!is_overlooked_channel("Security"));
    }

    #[test]
    fn empty_channel_name_is_overlooked() {
        // Defensive: empty string is not in the Big Three.
        assert!(is_overlooked_channel(""));
    }
}
