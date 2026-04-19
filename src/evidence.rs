//! Evidence strength / confidence model for forensic artifacts.
//!
//! Maps each catalog artifact to an [`EvidenceStrength`] rating and known
//! interpretation caveats, helping analysts assess the weight of evidence
//! and communicate findings in reports.

/// How strongly an artifact proves a fact in isolation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum EvidenceStrength {
    /// Known false-positive generator; use only with strong corroboration.
    Unreliable = 0,
    /// Suggestive but easily explained by benign activity.
    Circumstantial = 1,
    /// Useful with other evidence; not standalone proof.
    Corroborative = 2,
    /// Strong evidence; edge-case alternative explanations exist.
    Strong = 3,
    /// Definitive proof of the claimed activity (e.g., Prefetch = execution occurred).
    Definitive = 4,
}

/// Evidence strength entry for one catalog artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct EvidenceEntry {
    pub artifact_id: &'static str,
    pub strength: EvidenceStrength,
    /// Known caveats, edge cases, or false-positive scenarios.
    pub caveats: &'static [&'static str],
}

pub static EVIDENCE_TABLE: &[EvidenceEntry] = &[];

/// Returns the evidence entry for a given artifact ID, or None if unknown.
pub fn evidence_for(_artifact_id: &str) -> Option<&'static EvidenceEntry> {
    todo!("implement evidence lookup")
}

/// Returns all artifacts at or above the given strength threshold.
pub fn artifacts_with_strength(
    _min_strength: EvidenceStrength,
) -> Vec<&'static EvidenceEntry> {
    todo!("implement strength filter")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::{CATALOG, TriagePriority};

    #[test]
    fn prefetch_is_definitive() {
        let e = evidence_for("prefetch_file").expect("prefetch_file must be in table");
        assert_eq!(e.strength, EvidenceStrength::Definitive);
    }

    #[test]
    fn bash_history_is_circumstantial() {
        let e = evidence_for("linux_bash_history").expect("linux_bash_history must be in table");
        assert_eq!(e.strength, EvidenceStrength::Circumstantial);
    }

    #[test]
    fn definitive_entries_have_caveats() {
        // Even definitive evidence should document its limitations
        for entry in EVIDENCE_TABLE {
            if entry.strength == EvidenceStrength::Definitive {
                assert!(
                    !entry.caveats.is_empty(),
                    "{} is Definitive but has no caveats documented",
                    entry.artifact_id
                );
            }
        }
    }

    #[test]
    fn strength_ordering_is_consistent() {
        assert!(EvidenceStrength::Definitive > EvidenceStrength::Strong);
        assert!(EvidenceStrength::Strong > EvidenceStrength::Corroborative);
        assert!(EvidenceStrength::Corroborative > EvidenceStrength::Circumstantial);
        assert!(EvidenceStrength::Circumstantial > EvidenceStrength::Unreliable);
    }

    #[test]
    fn filter_by_strength_returns_subset() {
        let definitive = artifacts_with_strength(EvidenceStrength::Definitive);
        let all = artifacts_with_strength(EvidenceStrength::Unreliable);
        assert!(definitive.len() < all.len());
        assert!(definitive
            .iter()
            .all(|e| e.strength == EvidenceStrength::Definitive));
    }

    #[test]
    fn unknown_artifact_returns_none() {
        assert!(evidence_for("this_does_not_exist").is_none());
    }

    #[test]
    fn all_table_ids_exist_in_catalog() {
        let catalog_ids: std::collections::HashSet<&str> =
            CATALOG.list().iter().map(|d| d.id).collect();
        for entry in EVIDENCE_TABLE {
            assert!(
                catalog_ids.contains(entry.artifact_id),
                "evidence table references unknown catalog id: {}",
                entry.artifact_id
            );
        }
    }

    #[test]
    fn critical_triage_artifacts_have_evidence_entries() {
        let missing: Vec<&str> = CATALOG
            .for_triage()
            .into_iter()
            .filter(|d| d.triage_priority == TriagePriority::Critical)
            .filter(|d| evidence_for(d.id).is_none())
            .map(|d| d.id)
            .collect();
        assert!(
            missing.is_empty(),
            "Critical-priority artifacts missing from evidence table: {:?}",
            missing
        );
    }
}
