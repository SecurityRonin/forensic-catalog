/// Userland LD_PRELOAD rootkit forensic indicators.
///
/// Sources:
/// - Father rootkit source code and README (github.com/mav8557/Father):
///   <https://github.com/mav8557/Father>
/// - MITRE ATT&CK T1574.006 — Dynamic Linker Hijacking (LD_PRELOAD abuse):
///   <https://attack.mitre.org/techniques/T1574/006/>
/// - MITRE ATT&CK T1548.001 — Setuid and Setgid / GID manipulation:
///   <https://attack.mitre.org/techniques/T1548/001/>

/// Father rootkit (github.com/mav8557/Father) forensic indicators.
pub mod father {
    // RED: constants not yet defined — tests below will fail to compile.
}

/// Known Linux userland LD_PRELOAD rootkit library name substrings.
pub const KNOWN_LD_PRELOAD_ROOTKITS: &[&str] = &[];

/// Returns `true` if `filename` matches a known LD_PRELOAD rootkit library
/// name (case-insensitive substring match).
pub fn is_known_rootkit_lib(_filename: &str) -> bool {
    todo!("not yet implemented")
}

/// Returns `true` if `gid` matches a known rootkit magic GID.
pub fn is_rootkit_gid(_gid: u32) -> bool {
    todo!("not yet implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- father:: constant assertions ---

    #[test]
    fn father_magic_gid_is_7823() {
        assert_eq!(father::MAGIC_GID, 7823);
    }

    #[test]
    fn father_backdoor_source_port_is_48411() {
        assert_eq!(father::BACKDOOR_SOURCE_PORT, 48411);
    }

    #[test]
    fn father_hidden_port_is_0xd431() {
        assert_eq!(father::HIDDEN_PORT, 0xD431);
    }

    #[test]
    fn father_hidden_port_decimal_equals_53297() {
        assert_eq!(father::HIDDEN_PORT, 53297);
    }

    #[test]
    fn father_pam_artifact_path_is_tmp_silly_txt() {
        assert_eq!(father::PAM_ARTIFACT_PATH, "/tmp/silly.txt");
    }

    #[test]
    fn father_config_magic_starts_with_father_bytes() {
        assert!(father::CONFIG_MAGIC.starts_with(b"FATHER"));
    }

    // --- is_known_rootkit_lib ---

    #[test]
    fn libymv_so3_is_known_rootkit() {
        assert!(is_known_rootkit_lib("libymv.so.3"));
    }

    #[test]
    fn libc_so6_is_not_rootkit() {
        assert!(!is_known_rootkit_lib("libc.so.6"));
    }

    #[test]
    fn libprocesshider_is_known_rootkit() {
        assert!(is_known_rootkit_lib("libprocesshider.so"));
    }

    #[test]
    fn reptile_so_is_known_rootkit() {
        assert!(is_known_rootkit_lib("reptile.so"));
    }

    #[test]
    fn azazel_is_known_rootkit() {
        assert!(is_known_rootkit_lib("azazel.so.1"));
    }

    #[test]
    fn jynx2_is_known_rootkit() {
        assert!(is_known_rootkit_lib("jynx2.so"));
    }

    #[test]
    fn libssl_so_is_not_rootkit() {
        assert!(!is_known_rootkit_lib("libssl.so.3"));
    }

    #[test]
    fn empty_string_is_not_rootkit() {
        assert!(!is_known_rootkit_lib(""));
    }

    #[test]
    fn case_insensitive_libymv_uppercase() {
        assert!(is_known_rootkit_lib("LIBYMV.SO.3"));
    }

    // --- is_rootkit_gid ---

    #[test]
    fn gid_7823_is_rootkit_gid() {
        assert!(is_rootkit_gid(7823));
    }

    #[test]
    fn gid_1000_is_not_rootkit_gid() {
        assert!(!is_rootkit_gid(1000));
    }

    #[test]
    fn gid_0_is_not_rootkit_gid() {
        assert!(!is_rootkit_gid(0));
    }
}
