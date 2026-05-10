//! Artifact volatility model — RFC 3227 Order of Volatility encoded as data.
//!
//! Ratings live directly in [`crate::catalog::ArtifactDescriptor::volatility`]
//! (populated for assessed entries; `None` for generated entries awaiting review).
//!
//! Use [`volatility_for`] for point-lookups and [`acquisition_order`] to get a
//! live-response collection order (most ephemeral first).

/// How quickly an artifact is overwritten or lost.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum VolatilityClass {
    /// Survives deletion (journal, shadow copies, slack space).
    Residual = 0,
    /// Persistent until explicit deletion (registry keys, most files).
    Persistent = 1,
    /// Overwritten by user activity (MRU, recent docs, browser history).
    ActivityDriven = 2,
    /// Overwritten on rotation (event logs, prefetch circular buffer).
    RotatingBuffer = 3,
    /// Lost on reboot (RAM contents, process handles, network state).
    Volatile = 4,
}

/// Returns the descriptor for a given artifact ID if it has been assessed.
///
/// Returns `None` when the artifact ID is unknown or has no volatility assessment.
/// On `Some(d)`, `d.volatility` and `d.volatility_rationale` are guaranteed non-None/non-empty.
pub fn volatility_for(artifact_id: &str) -> Option<&'static crate::catalog::ArtifactDescriptor> {
    crate::catalog::CATALOG
        .by_id(artifact_id)
        .filter(|d| d.volatility.is_some())
}

/// Returns all assessed descriptors sorted most-volatile-first (Volatile → Residual).
/// Ties broken by artifact ID for determinism.
///
/// Use this for live-response triage: collect in the returned order to capture
/// the most ephemeral evidence before it is lost (RFC 3227).
pub fn acquisition_order() -> Vec<&'static crate::catalog::ArtifactDescriptor> {
    let mut entries: Vec<&crate::catalog::ArtifactDescriptor> = crate::catalog::CATALOG
        .list()
        .iter()
        .filter(|d| d.volatility.is_some())
        .collect();
    entries.sort_by(|a, b| b.volatility.cmp(&a.volatility).then(a.id.cmp(b.id)));
    entries
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::CATALOG;

    #[test]
    fn volatility_for_returns_descriptor() {
        let result: Option<&crate::catalog::ArtifactDescriptor> = volatility_for("shimcache");
        assert!(result.is_some());
    }

    #[test]
    fn shimcache_is_volatile() {
        let entry = volatility_for("shimcache").expect("shimcache should be assessed");
        assert_eq!(entry.volatility, Some(VolatilityClass::Volatile));
    }

    #[test]
    fn mft_is_residual() {
        let entry = volatility_for("mft_file").expect("mft_file should be assessed");
        assert_eq!(entry.volatility, Some(VolatilityClass::Residual));
    }

    #[test]
    fn evtx_security_is_rotating_buffer() {
        let entry = volatility_for("evtx_security").expect("evtx_security should be assessed");
        assert_eq!(entry.volatility, Some(VolatilityClass::RotatingBuffer));
    }

    #[test]
    fn acquisition_order_volatile_first() {
        let order = acquisition_order();
        assert!(!order.is_empty());
        assert_eq!(
            order[0].volatility,
            Some(VolatilityClass::Volatile),
            "acquisition_order should start with Volatile class"
        );
        assert_eq!(
            order.last().unwrap().volatility,
            Some(VolatilityClass::Residual),
            "acquisition_order should end with Residual class"
        );
    }

    #[test]
    fn unknown_artifact_returns_none() {
        assert!(volatility_for("this_does_not_exist").is_none());
    }

    #[test]
    fn table_covers_critical_triage_artifacts() {
        let missing: Vec<&str> = CATALOG
            .for_triage()
            .into_iter()
            .filter(|d| d.triage_priority == crate::catalog::TriagePriority::Critical)
            .filter(|d| volatility_for(d.id).is_none())
            .map(|d| d.id)
            .collect();
        assert!(
            missing.is_empty(),
            "Critical-priority artifacts missing from volatility table: {missing:?}"
        );
    }

    #[test]
    fn volatility_ordering_is_consistent() {
        assert!(VolatilityClass::Volatile > VolatilityClass::RotatingBuffer);
        assert!(VolatilityClass::RotatingBuffer > VolatilityClass::ActivityDriven);
        assert!(VolatilityClass::ActivityDriven > VolatilityClass::Persistent);
        assert!(VolatilityClass::Persistent > VolatilityClass::Residual);
    }
}
