// RED: Tests only — implementation to follow
#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_REB: &str = r#"Description: RECmd batch file
Author: Mike Cary
Version: 1
Id: 4eec0ce6-d1c3-4b65-9f0e-3ccd429d506c
Keys:
    -
        Description: Network
        HiveType: NTUSER
        Category: Devices
        KeyPath: Network
        Recursive: true
        Comment: Network Drives
    -
        Description: User Run Key
        HiveType: NTUSER
        Category: Autoruns
        KeyPath: Software\Microsoft\Windows\CurrentVersion\Run
        Recursive: false
        Comment: User Run Key
    -
        Description: System Run Key
        HiveType: HKLM
        Category: Autoruns
        KeyPath: Software\Microsoft\Windows\CurrentVersion\Run
        Recursive: false
        Comment: System Run Key
    -
        Description: PortProxy v4ToV4
        HiveType: HKLM
        Category: Network
        KeyPath: SYSTEM\CurrentControlSet\Services\PortProxy\v4tov4
        Recursive: true
        Comment: Port proxying - lateral movement indicator
"#;

    #[test]
    fn parse_returns_records() {
        let records = parse_reb(SAMPLE_REB);
        assert!(!records.is_empty(), "should return at least one record");
    }

    #[test]
    fn parse_correct_record_count() {
        let records = parse_reb(SAMPLE_REB);
        assert_eq!(records.len(), 4, "expected 4 entries, got {}", records.len());
    }

    #[test]
    fn parse_ntuser_hive_detected() {
        let records = parse_reb(SAMPLE_REB);
        let network = records.iter().find(|r| r.key_path == "Network").expect("no Network record");
        assert_eq!(network.hive.as_deref(), Some("NTUSER"));
        assert_eq!(network.name, "Network");
    }

    #[test]
    fn parse_hklm_hive_detected() {
        let records = parse_reb(SAMPLE_REB);
        let run = records.iter().find(|r| r.name == "System Run Key").expect("no System Run Key");
        assert_eq!(run.hive.as_deref(), Some("HKLM"));
        assert_eq!(run.key_path, r"Software\Microsoft\Windows\CurrentVersion\Run");
    }

    #[test]
    fn parse_meaning_from_comment() {
        let records = parse_reb(SAMPLE_REB);
        let portproxy = records.iter().find(|r| r.name == "PortProxy v4ToV4").expect("no PortProxy record");
        assert!(
            portproxy.meaning.contains("Port proxy") || portproxy.meaning.contains("lateral"),
            "meaning should include comment text, got: {}",
            portproxy.meaning
        );
    }

    #[test]
    fn parse_source_name_is_regedit() {
        let records = parse_reb(SAMPLE_REB);
        for rec in &records {
            assert_eq!(rec.source_name, "regedit", "wrong source_name for {}", rec.id);
        }
    }

    #[test]
    fn parse_ids_are_unique() {
        let records = parse_reb(SAMPLE_REB);
        let mut ids = std::collections::HashSet::new();
        for rec in &records {
            assert!(ids.insert(rec.id.clone()), "duplicate ID: {}", rec.id);
        }
    }

    #[test]
    fn parse_ids_are_snake_case() {
        let records = parse_reb(SAMPLE_REB);
        for rec in &records {
            assert!(
                rec.id.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_'),
                "ID not snake_case: {}",
                rec.id
            );
        }
    }

    #[test]
    fn parse_from_url_returns_records() {
        // This test hits the network — skip in offline CI if needed
        let result = parse_reb_url("https://raw.githubusercontent.com/EricZimmerman/RECmd/master/BatchExamples/RECmd_Batch_MC.reb");
        match result {
            Ok(records) => {
                assert!(!records.is_empty(), "expected records from URL");
                assert!(records.len() > 10, "expected many records from real file");
            }
            Err(e) => {
                // Network failure is acceptable in CI
                eprintln!("WARN: network fetch failed (acceptable in offline CI): {e}");
            }
        }
    }
}
