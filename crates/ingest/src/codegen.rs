#[cfg(test)]
mod tests {
    use super::*;
    use crate::record::{IngestRecord, IngestType};

    fn sample_registry_record() -> IngestRecord {
        IngestRecord {
            id: "regedit_portproxy_v4tov4_tcp".to_string(),
            name: "PortProxy v4ToV4 TCP Mapping".to_string(),
            source_name: "regedit",
            artifact_type: IngestType::RegistryKey,
            hive: Some("HKLM\\SYSTEM".to_string()),
            key_path: r"CurrentControlSet\Services\PortProxy\v4tov4\tcp".to_string(),
            value_name: None,
            file_path: None,
            meaning: "Records IPv4-to-IPv4 port forwarding rules; commonly abused for lateral movement.".to_string(),
            mitre_techniques: vec!["T1090".to_string()],
            triage_priority: "High".to_string(),
            sources: vec!["https://example.com/portproxy".to_string()],
        }
    }

    fn sample_file_record() -> IngestRecord {
        IngestRecord {
            id: "kape_file_chrome_history".to_string(),
            name: "Chrome History".to_string(),
            source_name: "kape",
            artifact_type: IngestType::File,
            hive: None,
            key_path: String::new(),
            value_name: None,
            file_path: Some(r"C:\Users\%user%\AppData\Local\Google\Chrome\User Data\Default\History".to_string()),
            meaning: "Chrome browsing history SQLite database.".to_string(),
            mitre_techniques: vec![],
            triage_priority: "Medium".to_string(),
            sources: vec![],
        }
    }

    #[test]
    fn generate_static_contains_pub_crate_static() {
        let rec = sample_registry_record();
        let output = generate_static(&rec);
        assert!(output.contains("pub(crate) static"), "missing pub(crate) static in:\n{output}");
    }

    #[test]
    fn generate_static_contains_artifact_descriptor() {
        let rec = sample_registry_record();
        let output = generate_static(&rec);
        assert!(output.contains("ArtifactDescriptor"), "missing ArtifactDescriptor in:\n{output}");
    }

    #[test]
    fn generate_static_uses_correct_id() {
        let rec = sample_registry_record();
        let output = generate_static(&rec);
        assert!(output.contains(r#"id: "regedit_portproxy_v4tov4_tcp""#), "missing id field in:\n{output}");
    }

    #[test]
    fn generate_static_has_uppercase_static_name() {
        let rec = sample_registry_record();
        let output = generate_static(&rec);
        assert!(output.contains("REGEDIT_PORTPROXY_V4TOV4_TCP"), "missing uppercase static name in:\n{output}");
    }

    #[test]
    fn generate_static_registry_key_type() {
        let rec = sample_registry_record();
        let output = generate_static(&rec);
        assert!(output.contains("ArtifactType::RegistryKey"), "wrong artifact type in:\n{output}");
    }

    #[test]
    fn generate_static_hive_some() {
        let rec = sample_registry_record();
        let output = generate_static(&rec);
        assert!(output.contains("Some(HiveTarget::HklmSystem)"), "wrong hive in:\n{output}");
    }

    #[test]
    fn generate_static_file_type_and_path() {
        let rec = sample_file_record();
        let output = generate_static(&rec);
        assert!(output.contains("ArtifactType::File"), "wrong type in:\n{output}");
        assert!(output.contains("file_path: Some("), "missing file_path in:\n{output}");
        assert!(output.contains("Chrome"), "missing path content in:\n{output}");
    }

    #[test]
    fn generate_static_mitre_techniques() {
        let rec = sample_registry_record();
        let output = generate_static(&rec);
        assert!(output.contains(r#""T1090""#), "missing MITRE technique in:\n{output}");
    }

    #[test]
    fn generate_static_triage_high() {
        let rec = sample_registry_record();
        let output = generate_static(&rec);
        assert!(output.contains("TriagePriority::High"), "wrong triage priority in:\n{output}");
    }

    #[test]
    fn generate_static_empty_mitre_and_sources() {
        let rec = sample_file_record();
        let output = generate_static(&rec);
        assert!(output.contains("mitre_techniques: &[]"), "missing empty mitre in:\n{output}");
        assert!(output.contains("sources: &[]"), "missing empty sources in:\n{output}");
    }

    #[test]
    fn generate_module_header() {
        let header = generate_module_header("regedit", 42);
        assert!(header.contains("regedit"), "missing source name in header");
        assert!(header.contains("42"), "missing count in header");
        assert!(header.contains("#![allow(clippy::too_many_lines)]"), "missing clippy allow");
    }
}
