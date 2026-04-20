//! STIX 2.1 observable mapping.
//!
//! Maps catalog artifacts to STIX 2.1 Cyber Observable types and
//! indicator patterns, enabling integration with threat intelligence
//! platforms and SIEM systems.

/// STIX 2.1 Cyber Observable type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StixObservableType {
    /// file (SCO).
    File,
    /// windows-registry-key (SCO).
    WindowsRegistryKey,
    /// process (SCO).
    Process,
    /// network-traffic (SCO).
    NetworkTraffic,
    /// user-account (SCO).
    UserAccount,
    /// directory (SCO).
    Directory,
    /// software (SCO).
    Software,
    /// artifact (SCO) — binary content.
    Artifact,
    /// domain-name (SCO).
    DomainName,
    /// email-message (SCO).
    EmailMessage,
}

/// STIX mapping for one catalog artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct StixMapping {
    pub artifact_id: &'static str,
    pub stix_type: StixObservableType,
    /// STIX Indicator pattern template (use {value} as placeholder for extracted value).
    pub stix_pattern: Option<&'static str>,
    /// Human description of the mapping.
    pub mapping_notes: &'static str,
}

pub static STIX_MAPPINGS: &[StixMapping] = &[];

/// Returns the STIX mapping for a given artifact ID.
pub fn stix_mapping_for(_artifact_id: &str) -> Option<&'static StixMapping> {
    todo!("implement stix_mapping_for")
}

/// Returns all artifacts mapping to a given STIX observable type.
pub fn artifacts_for_stix_type(_stix_type: StixObservableType) -> Vec<&'static StixMapping> {
    todo!("implement artifacts_for_stix_type")
}

/// Returns all artifact IDs that have STIX patterns defined.
pub fn artifacts_with_patterns() -> Vec<&'static StixMapping> {
    todo!("implement artifacts_with_patterns")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::CATALOG;

    #[test]
    fn userassist_maps_to_registry_key() {
        let m = stix_mapping_for("userassist_exe").expect("userassist_exe must have STIX mapping");
        assert_eq!(m.stix_type, StixObservableType::WindowsRegistryKey);
        assert!(m.stix_pattern.is_some());
    }

    #[test]
    fn prefetch_maps_to_file() {
        let m = stix_mapping_for("prefetch_file").expect("prefetch_file must have STIX mapping");
        assert_eq!(m.stix_type, StixObservableType::File);
    }

    #[test]
    fn registry_type_has_multiple_mappings() {
        let mappings = artifacts_for_stix_type(StixObservableType::WindowsRegistryKey);
        assert!(
            mappings.len() >= 3,
            "Should have multiple registry key mappings"
        );
    }

    #[test]
    fn artifacts_with_patterns_nonempty() {
        let patterns = artifacts_with_patterns();
        assert!(!patterns.is_empty());
        assert!(patterns.iter().all(|m| m.stix_pattern.is_some()));
    }

    #[test]
    fn unknown_artifact_returns_none() {
        assert!(stix_mapping_for("nonexistent").is_none());
    }

    #[test]
    fn all_table_ids_exist_in_catalog() {
        for mapping in STIX_MAPPINGS {
            assert!(
                CATALOG.by_id(mapping.artifact_id).is_some(),
                "stix table references unknown artifact: {}",
                mapping.artifact_id
            );
        }
    }

    #[test]
    fn all_mappings_have_notes() {
        for mapping in STIX_MAPPINGS {
            assert!(
                !mapping.mapping_notes.is_empty(),
                "stix mapping for '{}' has empty mapping_notes",
                mapping.artifact_id
            );
        }
    }
}
