//! Artifact volatility model — RFC 3227 Order of Volatility encoded as data.
//!
//! Maps each catalog artifact to a [`VolatilityClass`], enabling tools to
//! sort collection order from most-volatile to least-volatile.

use crate::catalog::CATALOG;

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

/// Volatility entry for one catalog artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct VolatilityEntry {
    /// Catalog artifact ID.
    pub artifact_id: &'static str,
    pub volatility: VolatilityClass,
    /// One-line rationale for this classification.
    pub rationale: &'static str,
}

/// Volatility table for all catalog artifacts.
/// Sorted by artifact_id for determinism.
pub static VOLATILITY_TABLE: &[VolatilityEntry] = &[];

/// Returns the volatility entry for a given artifact ID, or `None` if unknown.
pub fn volatility_for(_artifact_id: &str) -> Option<&'static VolatilityEntry> {
    todo!()
}

/// Returns all artifacts sorted most-volatile-first (Volatile → Residual).
/// Ties broken by artifact ID for determinism.
pub fn acquisition_order() -> Vec<&'static VolatilityEntry> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shimcache_is_volatile() {
        let entry = volatility_for("shimcache").expect("shimcache should be in table");
        assert_eq!(entry.volatility, VolatilityClass::Volatile);
    }

    #[test]
    fn mft_is_residual() {
        let entry = volatility_for("mft_file").expect("mft_file should be in table");
        assert_eq!(entry.volatility, VolatilityClass::Residual);
    }

    #[test]
    fn evtx_security_is_rotating_buffer() {
        let entry = volatility_for("evtx_security").expect("evtx_security should be in table");
        assert_eq!(entry.volatility, VolatilityClass::RotatingBuffer);
    }

    #[test]
    fn acquisition_order_volatile_first() {
        let order = acquisition_order();
        assert!(!order.is_empty());
        // First entry must be the most volatile
        assert_eq!(
            order[0].volatility,
            VolatilityClass::Volatile,
            "acquisition_order should start with Volatile class"
        );
        // Last entry must be Residual
        assert_eq!(
            order.last().unwrap().volatility,
            VolatilityClass::Residual,
            "acquisition_order should end with Residual class"
        );
    }

    #[test]
    fn unknown_artifact_returns_none() {
        assert!(volatility_for("this_does_not_exist").is_none());
    }

    #[test]
    fn table_covers_critical_triage_artifacts() {
        // All Critical triage artifacts should be in the volatility table
        let missing: Vec<&str> = CATALOG
            .for_triage()
            .into_iter()
            .filter(|d| d.triage_priority == crate::catalog::TriagePriority::Critical)
            .filter(|d| volatility_for(d.id).is_none())
            .map(|d| d.id)
            .collect();
        assert!(
            missing.is_empty(),
            "Critical-priority artifacts missing from volatility table: {:?}",
            missing
        );
    }

    #[test]
    fn all_table_artifact_ids_exist_in_catalog() {
        let catalog_ids: std::collections::HashSet<&str> =
            CATALOG.list().iter().map(|d| d.id).collect();
        for entry in VOLATILITY_TABLE {
            assert!(
                catalog_ids.contains(entry.artifact_id),
                "volatility table references unknown catalog id: {}",
                entry.artifact_id
            );
        }
    }

    #[test]
    fn volatility_ordering_is_consistent() {
        // Volatile > RotatingBuffer > ActivityDriven > Persistent > Residual
        assert!(VolatilityClass::Volatile > VolatilityClass::RotatingBuffer);
        assert!(VolatilityClass::RotatingBuffer > VolatilityClass::ActivityDriven);
        assert!(VolatilityClass::ActivityDriven > VolatilityClass::Persistent);
        assert!(VolatilityClass::Persistent > VolatilityClass::Residual);
    }
}
