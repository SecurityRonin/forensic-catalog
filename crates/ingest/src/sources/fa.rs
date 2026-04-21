// RED: ForensicArtifacts YAML parser tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::record::IngestType;

    const SAMPLE_FA_YAML: &str = r#"---
name: WindowsRunKeys
doc: Windows Run and RunOnce persistence keys.
sources:
- type: REGISTRY_KEY
  attributes:
    keys:
    - 'HKEY_LOCAL_MACHINE\Software\Microsoft\Windows\CurrentVersion\Run'
    - 'HKEY_LOCAL_MACHINE\Software\Microsoft\Windows\CurrentVersion\RunOnce'
    - 'HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Run'
supported_os: [Windows]
urls:
- 'https://attack.mitre.org/techniques/T1547/001/'
---
name: WindowsEventLogs
doc: Windows Event logs.
sources:
- type: FILE
  attributes:
    paths:
    - '%%environ_systemroot%%\System32\winevt\Logs\*.evtx'
supported_os: [Windows]
urls:
- 'https://docs.microsoft.com/en-us/windows/win32/eventlog/event-logs'
---
name: WindowsNtdsDatabase
doc: Active Directory database file.
sources:
- type: FILE
  attributes:
    paths:
    - '%%environ_systemroot%%\ntds\ntds.dit'
supported_os: [Windows]
urls: []
"#;

    #[test]
    fn parse_returns_records() {
        let records = parse_fa_yaml(SAMPLE_FA_YAML);
        assert!(!records.is_empty(), "should return records");
    }

    #[test]
    fn parse_registry_key_type() {
        let records = parse_fa_yaml(SAMPLE_FA_YAML);
        let run = records.iter().find(|r| r.name == "WindowsRunKeys");
        assert!(run.is_some(), "no WindowsRunKeys record");
        let run = run.unwrap();
        assert_eq!(run.artifact_type, IngestType::RegistryKey);
    }

    #[test]
    fn parse_file_type() {
        let records = parse_fa_yaml(SAMPLE_FA_YAML);
        let evtx = records.iter().find(|r| r.name == "WindowsEventLogs");
        assert!(evtx.is_some(), "no WindowsEventLogs record");
        let evtx = evtx.unwrap();
        assert_eq!(evtx.artifact_type, IngestType::File);
    }

    #[test]
    fn parse_source_name_is_fa() {
        let records = parse_fa_yaml(SAMPLE_FA_YAML);
        for rec in &records {
            assert_eq!(rec.source_name, "fa", "wrong source_name: {}", rec.source_name);
        }
    }

    #[test]
    fn parse_meaning_from_doc() {
        let records = parse_fa_yaml(SAMPLE_FA_YAML);
        let ntds = records.iter().find(|r| r.name == "WindowsNtdsDatabase").expect("no ntds");
        assert!(ntds.meaning.contains("Active Directory"), "meaning: {}", ntds.meaning);
    }

    #[test]
    fn parse_ids_are_unique_and_snake_case() {
        let records = parse_fa_yaml(SAMPLE_FA_YAML);
        let mut ids = std::collections::HashSet::new();
        for rec in &records {
            assert!(ids.insert(rec.id.clone()), "duplicate ID: {}", rec.id);
            assert!(
                rec.id.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_'),
                "ID not snake_case: {}",
                rec.id
            );
        }
    }

    #[test]
    fn parse_multiple_keys_produces_multiple_records() {
        let records = parse_fa_yaml(SAMPLE_FA_YAML);
        // WindowsRunKeys has 3 keys → expect >= 3 registry records
        let registry_count = records.iter().filter(|r| r.artifact_type == IngestType::RegistryKey).count();
        assert!(registry_count >= 3, "expected >= 3 registry records, got {registry_count}");
    }

    #[test]
    fn fetch_fa_windows_returns_records() {
        let result = fetch_fa_artifacts("https://raw.githubusercontent.com/forensicartifacts/artifacts/main/artifacts/data/windows.yaml");
        match result {
            Ok(records) => {
                assert!(!records.is_empty(), "expected FA windows records");
                assert!(records.len() > 20, "expected many FA records");
            }
            Err(e) => {
                eprintln!("WARN: network fetch failed (acceptable in offline CI): {e}");
            }
        }
    }
}
