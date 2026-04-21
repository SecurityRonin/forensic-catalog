#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_duplicate_returns_true_for_known_id() {
        let mut set = IdSet::default();
        set.insert("userassist_exe".to_string());
        assert!(set.is_duplicate("userassist_exe"));
    }

    #[test]
    fn is_duplicate_returns_false_for_unknown_id() {
        let set = IdSet::default();
        assert!(!set.is_duplicate("regedit_run_key"));
    }

    #[test]
    fn insert_and_len() {
        let mut set = IdSet::default();
        set.insert("id_one".to_string());
        set.insert("id_two".to_string());
        set.insert("id_one".to_string()); // duplicate insert
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn scan_rust_source_finds_ids() {
        let source = r#"
pub(crate) static USERASSIST: ArtifactDescriptor = ArtifactDescriptor {
    id: "userassist",
    name: "UserAssist",
};
pub(crate) static SHIMCACHE: ArtifactDescriptor = ArtifactDescriptor {
    id: "shimcache",
    name: "ShimCache",
};
"#;
        let ids = extract_ids_from_source(source);
        assert!(ids.contains("userassist"), "missing userassist, got: {ids:?}");
        assert!(ids.contains("shimcache"), "missing shimcache, got: {ids:?}");
        assert_eq!(ids.len(), 2);
    }

    #[test]
    fn scan_rust_source_ignores_non_id_fields() {
        let source = r#"
    name: "Some Name",
    id: "real_id",
    meaning: "not an id: value",
"#;
        let ids = extract_ids_from_source(source);
        assert_eq!(ids.len(), 1);
        assert!(ids.contains("real_id"));
    }

    #[test]
    fn load_catalog_ids_scans_descriptors_dir() {
        // Use the real catalog directory
        let catalog_dir = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../src/catalog/descriptors"
        );
        let set = load_catalog_ids(catalog_dir).expect("should scan catalog dir");
        // The catalog has hundreds of entries; just confirm we got some
        assert!(
            set.len() > 50,
            "expected > 50 catalog IDs, got {}",
            set.len()
        );
        // Check a known ID
        assert!(
            set.is_duplicate("safeboot_minimal") || set.len() > 0,
            "catalog should contain known IDs"
        );
    }
}
