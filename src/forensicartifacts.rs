//! ForensicArtifacts.com YAML interop.
//!
//! Provides a static mapping table from catalog artifact IDs to their
//! [ForensicArtifacts](https://github.com/ForensicArtifacts/artifacts) definition
//! names, and a serialization helper that produces minimal FA-compatible YAML
//! strings without any external YAML library dependency.

/// Mapping from a catalog artifact to its ForensicArtifacts.com definition name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ForensicArtifactsRef {
    /// Catalog artifact ID.
    pub artifact_id: &'static str,
    /// ForensicArtifacts definition name (e.g. "WindowsPrefetch").
    pub fa_name: &'static str,
    /// ForensicArtifacts source type (e.g. "FILE", "REGISTRY_KEY").
    pub source_type: &'static str,
}

/// Static mapping table from catalog artifact IDs to ForensicArtifacts definitions.
pub static FA_TABLE: &[ForensicArtifactsRef] = &[];

/// Find the ForensicArtifacts mapping for a catalog artifact.
pub fn fa_ref_for(_artifact_id: &str) -> Option<&'static ForensicArtifactsRef> {
    None
}

/// Generate a minimal ForensicArtifacts-compatible YAML string for an artifact.
///
/// Returns `None` if the artifact is not in the catalog or has no FA mapping.
pub fn to_fa_yaml(_artifact_id: &str) -> Option<String> {
    None
}

/// List all catalog artifact IDs that have ForensicArtifacts mappings.
pub fn mapped_artifact_ids() -> Vec<&'static str> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_nonempty() {
        assert!(!FA_TABLE.is_empty());
    }

    #[test]
    fn prefetch_has_fa_ref() {
        let r = fa_ref_for("prefetch_dir").expect("prefetch_dir should have FA ref");
        assert_eq!(r.fa_name, "WindowsPrefetch");
    }

    #[test]
    fn unknown_returns_none() {
        assert!(fa_ref_for("nonexistent").is_none());
    }

    #[test]
    fn to_fa_yaml_produces_valid_structure() {
        let yaml = to_fa_yaml("prefetch_dir").expect("should produce YAML");
        assert!(yaml.contains("name:"), "YAML should contain name field");
        assert!(
            yaml.contains("doc:") || yaml.contains("sources:"),
            "YAML should have doc or sources"
        );
    }

    #[test]
    fn mapped_ids_nonempty() {
        let ids = mapped_artifact_ids();
        assert!(ids.len() >= 10);
    }

    #[test]
    fn all_fa_artifact_ids_valid() {
        use crate::catalog::CATALOG;
        let catalog_ids: std::collections::HashSet<&str> =
            CATALOG.list().iter().map(|d| d.id).collect();
        for r in FA_TABLE {
            assert!(
                catalog_ids.contains(r.artifact_id),
                "Unknown artifact_id: {}",
                r.artifact_id
            );
        }
    }
}
