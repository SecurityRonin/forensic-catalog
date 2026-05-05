//! Userland LD_PRELOAD rootkit forensic indicators.
//!
//! Sources:
//! - Father rootkit source code and README (github.com/mav8557/Father):
//!   <https://github.com/mav8557/Father>
//! - MITRE ATT&CK T1574.006 — Dynamic Linker Hijacking (LD_PRELOAD abuse):
//!   <https://attack.mitre.org/techniques/T1574/006/>
//! - MITRE ATT&CK T1548.001 — Setuid and Setgid / GID manipulation:
//!   <https://attack.mitre.org/techniques/T1548/001/>

/// Father rootkit (github.com/mav8557/Father) forensic indicators.
///
/// Father is a userland LD_PRELOAD rootkit that hides processes, files, network
/// connections, and provides backdoor shell access via a magic GID, a specific
/// SSH source-port, and a PAM module hook.
///
/// Sources:
/// - Father rootkit source code and README:
///   <https://github.com/mav8557/Father>
/// - MITRE ATT&CK T1574.006 — Dynamic Linker Hijacking:
///   <https://attack.mitre.org/techniques/T1574/006/>
pub mod father {
    /// GID that Father grants to rootkit-controlled processes.
    ///
    /// Any process carrying GID 7823 (`magic_gid` in Father's config) is
    /// hidden from `ps`, `/proc`, and `ls` output by the rootkit's hooked libc
    /// functions.
    ///
    /// Source: Father source — `config.h` `#define MAGIC_GID 7823`:
    /// <https://github.com/mav8557/Father>
    pub const MAGIC_GID: u32 = 7823;

    /// Source port Father monitors in `accept()` to grant backdoor shells.
    ///
    /// When sshd calls `accept()` and the connecting client's source port is
    /// 48411, Father intercepts and spawns an authenticated shell regardless
    /// of credentials.
    ///
    /// Source: Father source — `config.h` `#define SOURCEPORT 48411`:
    /// <https://github.com/mav8557/Father>
    pub const BACKDOOR_SOURCE_PORT: u16 = 48411;

    /// Default backdoor shell password.
    ///
    /// Father's PAM hook accepts this password for any user account, bypassing
    /// normal PAM authentication.
    ///
    /// Source: Father source — `config.h` `#define SHELL_PASS "ymv"`:
    /// <https://github.com/mav8557/Father>
    pub const SHELL_PASS: &str = "ymv";

    /// Default `LD_PRELOAD` path for the rootkit shared library.
    ///
    /// The library masquerades as a legitimate x86-64 shared object inside
    /// `/usr/lib/x86_64-linux-gnu/` to evade casual `ls` inspection.
    ///
    /// Source: Father source — `install.sh` and `config.h` `PRELOAD` path:
    /// <https://github.com/mav8557/Father>
    pub const DEFAULT_PRELOAD_PATH: &str = "/usr/lib/x86_64-linux-gnu/libymv.so.3";

    /// Port Father hides from `netstat` / `ss` (0xD431 = 53297 decimal).
    ///
    /// Father's hooked `readdir` and `/proc/net/tcp` filter suppress entries
    /// for this port, making it invisible to standard tools.
    ///
    /// Source: Father source — `config.h` `#define HIDDENPORT 0xD431`:
    /// <https://github.com/mav8557/Father>
    pub const HIDDEN_PORT: u16 = 0xD431;

    /// PAM hook artifact file path.
    ///
    /// Father's PAM module writes this file when loaded, providing a reliable
    /// forensic indicator of PAM-level compromise on the host.
    ///
    /// Source: Father source — `pam_father.c`:
    /// <https://github.com/mav8557/Father>
    pub const PAM_ARTIFACT_PATH: &str = "/tmp/silly.txt";

    /// Binary config file magic bytes (`"FATHER"` in ASCII / UTF-8).
    ///
    /// Father's binary configuration blob begins with these six bytes, enabling
    /// signature-based detection in memory scans, disk carving, and YARA rules.
    ///
    /// Source: Father source — `config.h` magic header definition:
    /// <https://github.com/mav8557/Father>
    pub const CONFIG_MAGIC: &[u8] = b"\x46\x41\x54\x48\x45\x52";
}

/// Known Linux userland LD_PRELOAD rootkit library name substrings.
///
/// Each entry is a substring of the `.so` filename. Detection logic should
/// match against the basename of any path found in `/etc/ld.so.preload` or
/// the `LD_PRELOAD` environment variable.
///
/// Sources:
/// - Father (libymv): <https://github.com/mav8557/Father>
/// - libprocesshider: <https://github.com/gianlucaborello/libprocesshider>
/// - Reptile: <https://github.com/f0rb1dd3n/Reptile>
/// - Azazel: <https://github.com/chokepoint/azazel>
/// - Jynx / Jynx2: <https://github.com/chokepoint/Jynx2>
/// - MITRE ATT&CK T1574.006 — Dynamic Linker Hijacking:
///   <https://attack.mitre.org/techniques/T1574/006/>
pub const KNOWN_LD_PRELOAD_ROOTKITS: &[&str] = &[
    "libprocesshider",
    "libymv",
    "reptile",
    "azazel",
    "jynx",
    "jynx2",
];

/// Returns `true` if `filename` matches a known LD_PRELOAD rootkit library
/// name (case-insensitive substring match).
///
/// Pass the basename of the shared object (e.g. `"libymv.so.3"`) or a full
/// path — both work because the check is a substring search.
///
/// # Examples
///
/// ```rust
/// use forensicnomicon::rootkit::is_known_rootkit_lib;
/// assert!(is_known_rootkit_lib("libymv.so.3"));
/// assert!(!is_known_rootkit_lib("libc.so.6"));
/// ```
///
/// Sources:
/// - MITRE ATT&CK T1574.006 — Dynamic Linker Hijacking:
///   <https://attack.mitre.org/techniques/T1574/006/>
pub fn is_known_rootkit_lib(filename: &str) -> bool {
    let lower = filename.to_ascii_lowercase();
    KNOWN_LD_PRELOAD_ROOTKITS
        .iter()
        .any(|k| lower.contains(*k))
}

/// Returns `true` if `gid` matches a known rootkit magic GID.
///
/// Currently checks against Father's `MAGIC_GID` (7823). Additional
/// rootkit GIDs should be added here as they are discovered.
///
/// Sources:
/// - Father source — `config.h`:
///   <https://github.com/mav8557/Father>
/// - MITRE ATT&CK T1548.001 — Abuse Elevation Control Mechanism: Setuid and Setgid:
///   <https://attack.mitre.org/techniques/T1548/001/>
pub fn is_rootkit_gid(gid: u32) -> bool {
    gid == father::MAGIC_GID
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
    fn father_hidden_port_decimal_equals_54321() {
        assert_eq!(father::HIDDEN_PORT, 54321);
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
