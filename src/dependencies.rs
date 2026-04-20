//! Artifact dependency graph.
//!
//! Models structural and interpretive dependencies between catalog artifacts.
//! Enables collection tools to compute minimal artifact sets and analysts
//! to understand which artifacts require others for correct interpretation.

/// How one artifact depends on another.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DependencyKind {
    /// This artifact is physically contained within another (e.g., registry value inside a hive file).
    ContainedIn,
    /// Interpreting this artifact requires context from another (e.g., UserAssist needs NTUSER.DAT path).
    ContextFrom,
    /// Timestamps should be compared/correlated with another artifact.
    TemporalCorrelation,
    /// Another artifact provides the same evidence from a different source.
    AlternativeSource,
    /// This artifact is a prerequisite for decoding another.
    DecodingPrerequisite,
}

/// One directed dependency between two catalog artifacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ArtifactDependency {
    /// The artifact that has the dependency.
    pub artifact_id: &'static str,
    /// The artifact it depends on.
    pub depends_on: &'static str,
    pub relationship: DependencyKind,
    /// Human-readable explanation of why this dependency exists.
    pub explanation: &'static str,
}

pub static ARTIFACT_DEPENDENCIES: &[ArtifactDependency] = &[];

/// Returns all dependencies for a given artifact ID (things it depends on).
pub fn dependencies_of(_artifact_id: &str) -> Vec<&'static ArtifactDependency> {
    todo!()
}

/// Returns all artifacts that depend on the given artifact ID.
pub fn dependents_of(_artifact_id: &str) -> Vec<&'static ArtifactDependency> {
    todo!()
}

/// Returns the full dependency graph as a slice.
pub fn dependency_graph() -> &'static [ArtifactDependency] {
    todo!()
}

/// Given a set of target artifact IDs, computes the minimal collection set
/// including all DecodingPrerequisite and ContainedIn dependencies.
pub fn full_collection_set(_artifact_ids: &[&str]) -> Vec<&'static str> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::CATALOG;

    #[test]
    fn userassist_depends_on_ntuser() {
        let deps = dependencies_of("userassist_exe");
        assert!(!deps.is_empty());
        assert!(
            deps.iter().any(|d| d.depends_on == "ntuser_dat"),
            "userassist_exe should depend on ntuser_dat"
        );
    }

    #[test]
    fn dpapi_cred_has_decoding_prerequisite() {
        let deps = dependencies_of("dpapi_cred_user");
        assert!(
            deps.iter()
                .any(|d| d.relationship == DependencyKind::DecodingPrerequisite),
            "dpapi_cred_user should have a DecodingPrerequisite"
        );
    }

    #[test]
    fn full_collection_set_includes_prerequisites() {
        let set = full_collection_set(&["dpapi_cred_user"]);
        assert!(
            set.contains(&"dpapi_masterkey_user"),
            "Collection set for dpapi_cred_user should include dpapi_masterkey_user"
        );
    }

    #[test]
    fn full_collection_set_deduplicates() {
        let set = full_collection_set(&["userassist_exe", "run_key_hkcu"]);
        let ntuser_count = set.iter().filter(|&&id| id == "ntuser_dat").count();
        assert_eq!(ntuser_count, 1, "ntuser_dat should appear only once");
    }

    #[test]
    fn dependency_graph_nonempty() {
        assert!(dependency_graph().len() >= 20);
    }

    #[test]
    fn dependents_of_ntuser_is_nonempty() {
        let dependents = dependents_of("ntuser_dat");
        assert!(
            !dependents.is_empty(),
            "Several artifacts depend on ntuser_dat"
        );
    }

    #[test]
    fn all_dependency_artifact_ids_exist_in_catalog() {
        // Only check artifact_id (the dependent), not depends_on (may be container artifacts)
        for dep in ARTIFACT_DEPENDENCIES {
            assert!(
                CATALOG.by_id(dep.artifact_id).is_some(),
                "dependency graph references unknown artifact_id: {}",
                dep.artifact_id
            );
        }
    }

    #[test]
    fn all_explanations_nonempty() {
        for dep in ARTIFACT_DEPENDENCIES {
            assert!(
                !dep.explanation.is_empty(),
                "dependency {}->{} has empty explanation",
                dep.artifact_id,
                dep.depends_on
            );
        }
    }
}
