//! Artifact volatility model — RFC 3227 Order of Volatility encoded as data.
//!
//! Ratings live directly in [`crate::catalog::ArtifactDescriptor::volatility`]
//! (populated for assessed entries; `None` for generated entries awaiting review).
//!
//! Use [`volatility_for`] for point-lookups and [`acquisition_order`] to get a
//! live-response collection order (most ephemeral first).

/// Acquisition urgency for a forensic artifact under RFC 3227 Order of Volatility.
///
/// Values run from 0 (lowest urgency / most stable) to 4 (highest urgency / most
/// ephemeral). [`acquisition_order`] returns artifacts sorted 4→0 (most ephemeral
/// first), matching live-response triage practice.
///
/// ## Choosing the right class
///
/// | Class | Collect when | Rationale |
/// |---|---|---|
/// | `Volatile` | Immediately — before reboot | Only in RAM |
/// | `RotatingBuffer` | Before buffer fills | Fixed-size circular store |
/// | `ActivityDriven` | Before more user activity | Overwritten by normal use |
/// | `Persistent` | Standard scheduled collection | Present until explicit deletion |
/// | `Residual` | Last — always present on a live volume | Storage-level structure |
///
/// ## What `Residual` means — and what it does NOT mean
///
/// `Residual` is the **lowest acquisition urgency** class. Use it only for artifacts
/// that are **structurally present on any live mounted volume** and cannot be
/// destroyed by normal system operation (only by reformatting, physical destruction,
/// or deliberate forensic manipulation). The canonical example is `$MFT` — any
/// mounted NTFS volume always has an MFT; it is the last artifact you need to rush
/// to collect.
///
/// **`Residual` does NOT mean "recoverable via .LOG1/.LOG2, VSS, or $UsnJrnl after
/// deletion."** That property applies to virtually every NTFS artifact and provides
/// no discrimination between classes. A registry key that *could* be recovered from
/// a transaction log after deletion is `Persistent` while it exists — use `Persistent`
/// for all live registry keys and files regardless of post-deletion recoverability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum VolatilityClass {
    /// Storage-level artifact inherently present on any live mounted volume — cannot
    /// be destroyed by normal system operation. Lowest acquisition urgency: collect
    /// last. Example: `$MFT` (always present on NTFS), Volume Boot Record.
    ///
    /// **Not** a synonym for "recoverable via VSS/.LOG1/.LOG2 after deletion" — that
    /// applies to virtually all NTFS artifacts and is not discriminating.
    Residual = 0,
    /// Present in its primary location until explicitly deleted. Default class for
    /// registry keys, configuration files, and most forensic artifacts on a live
    /// system. Examples: Run keys, NTDS.dit, event log files (the file itself, not
    /// its contents — for contents see `RotatingBuffer`).
    Persistent = 1,
    /// Overwritten by ordinary user activity; degrades with normal system use.
    /// Examples: MRU lists, Recent Documents, browser history, typed URL cache.
    ActivityDriven = 2,
    /// Overwritten when a fixed-size circular buffer fills; oldest entries lost.
    /// Examples: Windows Event Log records (when log reaches max size), Prefetch
    /// files (128-entry limit), $UsnJrnl (wraps based on configured max size).
    RotatingBuffer = 3,
    /// Lost on reboot or process termination; exists only in volatile memory.
    /// Collect immediately in a live response before system shutdown.
    /// Examples: RAM contents, process handles, network connections, open file
    /// handles, the in-memory ShimCache state not yet flushed to registry.
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
    fn shimcache_is_persistent() {
        let entry = volatility_for("shimcache").expect("shimcache should be assessed");
        assert_eq!(entry.volatility, Some(VolatilityClass::Persistent));
    }

    #[test]
    fn shimcache_memory_is_volatile() {
        let entry = volatility_for("shimcache_memory").expect("shimcache_memory should be assessed");
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
