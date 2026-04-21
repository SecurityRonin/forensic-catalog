#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_hklm_software_path() {
        let id = normalize_registry_id(
            r"HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Run",
            "kape",
        );
        assert_eq!(id, "kape_windows_currentversion_run");
    }

    #[test]
    fn normalize_hkcu_path() {
        let id = normalize_registry_id(
            r"HKCU\Software\Microsoft\Terminal Server Client",
            "regedit",
        );
        assert_eq!(id, "regedit_microsoft_terminal_server_client");
    }

    #[test]
    fn normalize_hkey_local_machine_system_path() {
        let id = normalize_registry_id(
            r"HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\PortProxy\v4tov4\tcp",
            "regedit",
        );
        assert_eq!(id, "regedit_portproxy_v4tov4_tcp");
    }

    #[test]
    fn normalize_strips_hive_prefix() {
        let id = normalize_registry_id(r"HKLM\SYSTEM\Select", "kape");
        assert_eq!(id, "kape_system_select");
    }

    #[test]
    fn normalize_replaces_special_chars() {
        let id = normalize_registry_id(
            r"HKCU\Software\Classes\.exe\OpenWithProgids",
            "fa",
        );
        // dots become underscores, backslashes become underscores
        assert!(id.starts_with("fa_"));
        assert!(!id.contains('\\'));
        assert!(!id.contains('.'));
    }

    #[test]
    fn normalize_truncates_long_ids() {
        let long_path = format!(
            r"HKLM\SOFTWARE\{}\{}\{}\{}\{}",
            "A".repeat(20),
            "B".repeat(20),
            "C".repeat(20),
            "D".repeat(20),
            "E".repeat(20)
        );
        let id = normalize_registry_id(&long_path, "test");
        assert!(id.len() <= 60, "ID too long: {} chars", id.len());
    }

    #[test]
    fn normalize_file_id_basic() {
        let id = normalize_file_id(
            r"C:\Windows\System32\winevt\Logs\Security.evtx",
            "fa",
            false,
        );
        assert_eq!(id, "fa_file_logs_security_evtx");
    }

    #[test]
    fn normalize_file_id_directory() {
        let id = normalize_file_id(
            r"C:\Users\%user%\AppData\Local\Google\Chrome\User Data",
            "kape",
            true,
        );
        assert_eq!(id, "kape_dir_chrome_user_data");
    }

    #[test]
    fn normalize_makes_snake_case() {
        let id = normalize_registry_id(r"HKLM\SOFTWARE\Microsoft Office\16.0", "regedit");
        assert!(id.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_'),
            "ID contains non-snake-case chars: {id}");
    }

    #[test]
    fn unique_suffix_on_collision() {
        let existing: std::collections::HashSet<String> =
            ["kape_windows_run".to_string()].into_iter().collect();
        let id = normalize_registry_id_unique(
            r"HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Run",
            "kape",
            &existing,
        );
        assert_eq!(id, "kape_windows_run_2");
    }
}
