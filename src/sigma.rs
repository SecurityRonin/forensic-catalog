//! Sigma rule cross-references for catalog artifacts.
//!
//! Maps forensic catalog artifact IDs to known Sigma rules from SigmaHQ,
//! enabling analysts to correlate artifact-based triage with detection logic.

/// Reference to a Sigma rule that detects activity related to a catalog artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct SigmaRef {
    /// Catalog artifact ID this rule is associated with.
    pub artifact_id: &'static str,
    /// Sigma rule ID (UUID string as used in the Sigma rule `id:` field).
    pub rule_id: &'static str,
    /// Human-readable rule title.
    pub title: &'static str,
    /// Sigma logsource category (e.g. `"process_creation"`, `"registry_set"`).
    pub logsource_category: &'static str,
    /// MITRE ATT&CK technique IDs this rule covers.
    pub mitre_techniques: &'static [&'static str],
}

/// All Sigma rule cross-references known to the catalog.
pub static SIGMA_TABLE: &[SigmaRef] = &[];

/// Return all [`SigmaRef`] entries associated with the given artifact ID.
pub fn sigma_refs_for(_artifact_id: &str) -> &'static [SigmaRef] {
    todo!("sigma_refs_for: not yet implemented")
}

/// Return a sorted, deduplicated list of artifact IDs that have at least one
/// Sigma rule cross-reference.
pub fn artifacts_covered_by_sigma() -> Vec<&'static str> {
    todo!("artifacts_covered_by_sigma: not yet implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sigma_table_nonempty() {
        assert!(!SIGMA_TABLE.is_empty());
    }

    #[test]
    fn prefetch_has_sigma_rule() {
        let refs = sigma_refs_for("prefetch_dir");
        assert!(!refs.is_empty(), "prefetch_dir should have at least one sigma ref");
    }

    #[test]
    fn evtx_security_has_sigma_rules() {
        let refs = sigma_refs_for("evtx_security");
        assert!(!refs.is_empty());
        assert!(refs.iter().any(|r| !r.mitre_techniques.is_empty()));
    }

    #[test]
    fn unknown_artifact_returns_empty() {
        assert!(sigma_refs_for("this_does_not_exist").is_empty());
    }

    #[test]
    fn artifacts_covered_nonempty() {
        let covered = artifacts_covered_by_sigma();
        assert!(covered.len() >= 5);
    }

    #[test]
    fn all_sigma_artifact_ids_valid() {
        use crate::catalog::CATALOG;
        let ids: std::collections::HashSet<&str> = CATALOG.list().iter().map(|d| d.id).collect();
        for r in SIGMA_TABLE {
            assert!(
                ids.contains(r.artifact_id),
                "Unknown artifact_id in SIGMA_TABLE: {}",
                r.artifact_id
            );
        }
    }
}
