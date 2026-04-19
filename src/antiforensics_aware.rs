//! Anti-forensics awareness layer.
//!
//! Maps each catalog artifact to known anti-forensic techniques that can
//! affect it, along with detection hints. Helps analysts know what has
//! *not* been tampered with and what evidence gaps might be intentional.

/// A specific anti-forensic technique class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AntiForensicTechnique {
    /// NTFS timestamp manipulation ($STANDARD_INFORMATION modified).
    Timestomping,
    /// Event log channel cleared (Security 1102, System 104).
    LogClearing,
    /// File securely overwritten before deletion.
    SecureOverwrite,
    /// Registry key deleted or value removed.
    RegistryDeletion,
    /// Volume Shadow Copies deleted (vssadmin delete shadows).
    ShadowCopyDeletion,
    /// Prefetch disabled via registry to suppress execution traces.
    PrefetchDisable,
    /// History/cache file manually deleted or cleared.
    HistoryClearing,
    /// Artifact encrypted so contents are inaccessible.
    Encryption,
    /// Process list manipulated to hide entries (DKOM).
    ProcessHiding,
    /// Shellbag entries deleted to hide folder browsing.
    ShellbagDeletion,
}

/// One anti-forensic risk for a specific artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct AntiForensicRisk {
    pub technique: AntiForensicTechnique,
    /// What the attacker does to suppress this artifact.
    pub attacker_action: &'static str,
    /// What to look for to detect the anti-forensic action itself.
    pub detection_hint: &'static str,
}

/// Anti-forensic risks for one catalog artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ArtifactAntiForensics {
    pub artifact_id: &'static str,
    pub risks: &'static [AntiForensicRisk],
}

// ---------------------------------------------------------------------------
// Risk tables per artifact
// ---------------------------------------------------------------------------

static PREFETCH_FILE_RISKS: &[AntiForensicRisk] = &[
    AntiForensicRisk {
        technique: AntiForensicTechnique::PrefetchDisable,
        attacker_action: "Set HKLM\\SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management\\PrefetchParameters\\EnablePrefetcher to 0 to disable prefetch creation.",
        detection_hint: "Check EnablePrefetcher registry value; 0 indicates prefetch was disabled. Absence of any .pf files on a running system is suspicious.",
    },
    AntiForensicRisk {
        technique: AntiForensicTechnique::HistoryClearing,
        attacker_action: "Manually delete files from %SystemRoot%\\Prefetch\\ or use tools such as CCleaner to wipe the folder.",
        detection_hint: "Look for gaps in prefetch file timestamps or absence of .pf files for known-executed binaries. USN journal may record deletions from the Prefetch directory.",
    },
];

static EVTX_SECURITY_RISKS: &[AntiForensicRisk] = &[
    AntiForensicRisk {
        technique: AntiForensicTechnique::LogClearing,
        attacker_action: "Run `wevtutil cl Security` or use the Windows Event Viewer Clear Log action to wipe the Security channel.",
        detection_hint: "Event ID 1102 (Security log cleared) is itself written to the Security log before clearing. Also check System log for event 104 if the Security log is entirely absent.",
    },
];

static EVTX_SYSMON_RISKS: &[AntiForensicRisk] = &[
    AntiForensicRisk {
        technique: AntiForensicTechnique::LogClearing,
        attacker_action: "Run `wevtutil cl Microsoft-Windows-Sysmon/Operational` to erase Sysmon telemetry.",
        detection_hint: "Sysmon event 255 (error) or complete absence of the channel on a system with Sysmon installed is suspicious. Cross-reference with Security log 1102 timestamps.",
    },
];

static EVTX_SYSTEM_RISKS: &[AntiForensicRisk] = &[
    AntiForensicRisk {
        technique: AntiForensicTechnique::LogClearing,
        attacker_action: "Run `wevtutil cl System` to clear the System event log channel.",
        detection_hint: "Event ID 104 (System log cleared) is written to the System log itself before the wipe. A System log that begins only recently on a long-running host is suspicious.",
    },
];

static EVTX_POWERSHELL_RISKS: &[AntiForensicRisk] = &[
    AntiForensicRisk {
        technique: AntiForensicTechnique::LogClearing,
        attacker_action: "Run `wevtutil cl Microsoft-Windows-PowerShell/Operational` to remove PowerShell script block and module logging events.",
        detection_hint: "Absence of the PowerShell/Operational channel on a system where PowerShell has been used is suspicious. Check for event 1102 in the Security log correlating with the gap.",
    },
    AntiForensicRisk {
        technique: AntiForensicTechnique::PrefetchDisable,
        attacker_action: "Disable ScriptBlock logging via Group Policy or registry (HKLM\\SOFTWARE\\Policies\\Microsoft\\Windows\\PowerShell\\ScriptBlockLogging\\EnableScriptBlockLogging = 0).",
        detection_hint: "Check EnableScriptBlockLogging registry value. PowerShell transcripts or AMSI telemetry may still record activity even when event log logging is off.",
    },
];

static MFT_FILE_RISKS: &[AntiForensicRisk] = &[
    AntiForensicRisk {
        technique: AntiForensicTechnique::Timestomping,
        attacker_action: "Modify $STANDARD_INFORMATION timestamps using tools such as Timestomp or SetFileTime to make a file appear older or newer than it is.",
        detection_hint: "Compare $STANDARD_INFORMATION (SI) timestamps against $FILE_NAME (FN) timestamps in the MFT record. SI timestamps earlier than FN timestamps or with suspicious 0-nanosecond precision indicate timestomping.",
    },
];

static USN_JOURNAL_RISKS: &[AntiForensicRisk] = &[
    AntiForensicRisk {
        technique: AntiForensicTechnique::LogClearing,
        attacker_action: "Delete the USN journal with `fsutil usn deletejournal /D C:` to eliminate the change log for the volume.",
        detection_hint: "Absence of a USN journal on a live NTFS volume is unusual. The MFT may still retain residual USN data in $UsnJrnl:$J. Check for deletejournal events in Security or Sysmon logs.",
    },
];

static LINUX_BASH_HISTORY_RISKS: &[AntiForensicRisk] = &[
    AntiForensicRisk {
        technique: AntiForensicTechnique::HistoryClearing,
        attacker_action: "Run `history -c` to clear in-memory history, set HISTFILE=/dev/null to prevent writing, or manually delete ~/.bash_history.",
        detection_hint: "A missing or zero-byte ~/.bash_history file on an active account is suspicious. Check for HISTFILE=/dev/null in shell profiles (.bashrc, .bash_profile). Auth logs and syslog may still record command-driven activity.",
    },
];

static SHIMCACHE_RISKS: &[AntiForensicRisk] = &[
    AntiForensicRisk {
        technique: AntiForensicTechnique::RegistryDeletion,
        attacker_action: "Delete the AppCompatCache key under HKLM\\SYSTEM\\CurrentControlSet\\Control\\Session Manager\\AppCompatCache to remove shimcache execution evidence.",
        detection_hint: "Absence of the AppCompatCache registry value on a Windows system is highly suspicious. Residual evidence may exist in VSS snapshots or offline SYSTEM hive backups.",
    },
];

static USERASSIST_EXE_RISKS: &[AntiForensicRisk] = &[
    AntiForensicRisk {
        technique: AntiForensicTechnique::RegistryDeletion,
        attacker_action: "Delete the UserAssist key from NTUSER.DAT under Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\UserAssist to remove GUI execution history.",
        detection_hint: "Absence of UserAssist subkeys in NTUSER.DAT is suspicious for an active user profile. VSS snapshots or forensic copies of prior NTUSER.DAT versions may retain the data.",
    },
];

static LNK_FILES_RISKS: &[AntiForensicRisk] = &[
    AntiForensicRisk {
        technique: AntiForensicTechnique::HistoryClearing,
        attacker_action: "Manually delete files from %APPDATA%\\Microsoft\\Windows\\Recent\\ or use CCleaner/similar tools to erase LNK shortcuts.",
        detection_hint: "Absence of recent LNK files in a user profile that has clearly been active is suspicious. Shellbags and jumplists may independently corroborate file access even after LNK deletion.",
    },
];

static JUMP_LIST_AUTO_RISKS: &[AntiForensicRisk] = &[
    AntiForensicRisk {
        technique: AntiForensicTechnique::HistoryClearing,
        attacker_action: "Delete jump list files from %APPDATA%\\Microsoft\\Windows\\Recent\\AutomaticDestinations\\ to remove application file-access history.",
        detection_hint: "Absence of AutomaticDestinations files for commonly used applications is suspicious. LNK files and shellbags may still corroborate access to the same files.",
    },
];

static RECYCLE_BIN_RISKS: &[AntiForensicRisk] = &[
    AntiForensicRisk {
        technique: AntiForensicTechnique::SecureOverwrite,
        attacker_action: "Use secure-erase tools such as Eraser or SDelete to overwrite file data before or after deletion, preventing carving recovery.",
        detection_hint: "Presence of known secure-deletion tool artifacts (prefetch entries, registry MRU, or event log entries for Eraser/SDelete) alongside missing Recycle Bin entries may indicate deliberate destruction.",
    },
];

static SHELLBAGS_USER_RISKS: &[AntiForensicRisk] = &[
    AntiForensicRisk {
        technique: AntiForensicTechnique::ShellbagDeletion,
        attacker_action: "Use ShellBagsView, SBEDelete, or manual registry editing to remove shellbag entries from NTUSER.DAT and UsrClass.dat.",
        detection_hint: "Gaps in folder hierarchy within shellbags or a recently created BagMRU root with no history may indicate prior deletion. VSS snapshots of the hive may retain older shellbag state.",
    },
];

static SAM_USERS_RISKS: &[AntiForensicRisk] = &[
    AntiForensicRisk {
        technique: AntiForensicTechnique::Encryption,
        attacker_action: "Enable BitLocker on the system volume to prevent offline extraction of the SAM hive without the recovery key.",
        detection_hint: "BitLocker status can be verified with `manage-bde -status`. Network-based attacks (Pass-the-Hash) may target SAM in-memory via LSASS even when offline access is blocked.",
    },
];

static NTDS_DIT_RISKS: &[AntiForensicRisk] = &[
    AntiForensicRisk {
        technique: AntiForensicTechnique::Encryption,
        attacker_action: "Enable BitLocker on the Domain Controller system volume to prevent offline extraction of ntds.dit without the recovery key.",
        detection_hint: "BitLocker status can be checked with `manage-bde -status`. In-memory attacks via DCSync (Mimikatz) bypass disk encryption entirely; monitor for AD replication events from non-DC hosts.",
    },
    AntiForensicRisk {
        technique: AntiForensicTechnique::ShadowCopyDeletion,
        attacker_action: "Run `vssadmin delete shadows /all /quiet` to delete Volume Shadow Copies, preventing ntds.dit recovery from VSS snapshots.",
        detection_hint: "Check for event ID 8222 (VSS) or Sysmon process create events for vssadmin.exe with 'delete' arguments. Absence of VSS snapshots on a DC that should have them is suspicious.",
    },
];

static LINUX_AUTH_LOG_RISKS: &[AntiForensicRisk] = &[
    AntiForensicRisk {
        technique: AntiForensicTechnique::LogClearing,
        attacker_action: "As root, delete or truncate /var/log/auth.log to remove SSH, sudo, and PAM authentication records.",
        detection_hint: "A truncated or recently re-created auth.log with a creation time more recent than the system boot time is suspicious. Remote syslog forwarding or auditd may preserve copies of deleted entries.",
    },
];

static LINUX_WTMP_RISKS: &[AntiForensicRisk] = &[
    AntiForensicRisk {
        technique: AntiForensicTechnique::LogClearing,
        attacker_action: "Use `utmpdump /var/log/wtmp > tmp && edit && utmpdump -r < tmp > /var/log/wtmp` to surgically remove login records, or simply truncate the file.",
        detection_hint: "Gaps in wtmp login sequences or a wtmp mtime that predates the most recent entry timestamp indicate tampering. Cross-reference with auth.log and lastlog for consistency.",
    },
];

// ---------------------------------------------------------------------------
// Full risk table
// ---------------------------------------------------------------------------

/// Static table mapping catalog artifact IDs to their anti-forensic risks.
pub static AF_RISKS_TABLE: &[ArtifactAntiForensics] = &[
    ArtifactAntiForensics {
        artifact_id: "prefetch_file",
        risks: PREFETCH_FILE_RISKS,
    },
    ArtifactAntiForensics {
        artifact_id: "evtx_security",
        risks: EVTX_SECURITY_RISKS,
    },
    ArtifactAntiForensics {
        artifact_id: "evtx_sysmon",
        risks: EVTX_SYSMON_RISKS,
    },
    ArtifactAntiForensics {
        artifact_id: "evtx_system",
        risks: EVTX_SYSTEM_RISKS,
    },
    ArtifactAntiForensics {
        artifact_id: "evtx_powershell",
        risks: EVTX_POWERSHELL_RISKS,
    },
    ArtifactAntiForensics {
        artifact_id: "mft_file",
        risks: MFT_FILE_RISKS,
    },
    ArtifactAntiForensics {
        artifact_id: "usn_journal",
        risks: USN_JOURNAL_RISKS,
    },
    ArtifactAntiForensics {
        artifact_id: "linux_bash_history",
        risks: LINUX_BASH_HISTORY_RISKS,
    },
    ArtifactAntiForensics {
        artifact_id: "shimcache",
        risks: SHIMCACHE_RISKS,
    },
    ArtifactAntiForensics {
        artifact_id: "userassist_exe",
        risks: USERASSIST_EXE_RISKS,
    },
    ArtifactAntiForensics {
        artifact_id: "lnk_files",
        risks: LNK_FILES_RISKS,
    },
    ArtifactAntiForensics {
        artifact_id: "jump_list_auto",
        risks: JUMP_LIST_AUTO_RISKS,
    },
    ArtifactAntiForensics {
        artifact_id: "recycle_bin",
        risks: RECYCLE_BIN_RISKS,
    },
    ArtifactAntiForensics {
        artifact_id: "shellbags_user",
        risks: SHELLBAGS_USER_RISKS,
    },
    ArtifactAntiForensics {
        artifact_id: "sam_users",
        risks: SAM_USERS_RISKS,
    },
    ArtifactAntiForensics {
        artifact_id: "ntds_dit",
        risks: NTDS_DIT_RISKS,
    },
    ArtifactAntiForensics {
        artifact_id: "linux_auth_log",
        risks: LINUX_AUTH_LOG_RISKS,
    },
    ArtifactAntiForensics {
        artifact_id: "linux_wtmp",
        risks: LINUX_WTMP_RISKS,
    },
];

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Return the anti-forensic risks for a catalog artifact by ID, or `None` if
/// the artifact is not in the awareness table.
pub fn anti_forensics_for(artifact_id: &str) -> Option<&'static ArtifactAntiForensics> {
    AF_RISKS_TABLE.iter().find(|e| e.artifact_id == artifact_id)
}

/// Return all artifacts vulnerable to a specific anti-forensic technique.
pub fn artifacts_vulnerable_to(
    technique: AntiForensicTechnique,
) -> Vec<&'static ArtifactAntiForensics> {
    AF_RISKS_TABLE
        .iter()
        .filter(|e| e.risks.iter().any(|r| r.technique == technique))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefetch_has_prefetch_disable_risk() {
        let af = anti_forensics_for("prefetch_file").expect("prefetch_file must be in table");
        assert!(
            af.risks
                .iter()
                .any(|r| r.technique == AntiForensicTechnique::PrefetchDisable),
            "prefetch_file should list PrefetchDisable as a risk"
        );
    }

    #[test]
    fn evtx_security_has_log_clearing_risk() {
        let af = anti_forensics_for("evtx_security").expect("evtx_security must be in table");
        assert!(
            af.risks
                .iter()
                .any(|r| r.technique == AntiForensicTechnique::LogClearing),
            "evtx_security should list LogClearing as a risk"
        );
    }

    #[test]
    fn mft_has_timestomping_risk() {
        let af = anti_forensics_for("mft_file").expect("mft_file must be in table");
        assert!(
            af.risks
                .iter()
                .any(|r| r.technique == AntiForensicTechnique::Timestomping),
            "mft_file should list Timestomping as a risk"
        );
    }

    #[test]
    fn unknown_artifact_returns_none() {
        assert!(anti_forensics_for("nonexistent_artifact_xyz").is_none());
    }

    #[test]
    fn log_clearing_affects_multiple_evtx_artifacts() {
        let vulnerable = artifacts_vulnerable_to(AntiForensicTechnique::LogClearing);
        let evtx_count = vulnerable
            .iter()
            .filter(|e| e.artifact_id.starts_with("evtx_"))
            .count();
        assert!(
            evtx_count >= 3,
            "At least 3 evtx_ artifacts should be vulnerable to LogClearing"
        );
    }

    #[test]
    fn all_risks_have_non_empty_detection_hints() {
        for entry in AF_RISKS_TABLE {
            for risk in entry.risks {
                assert!(
                    !risk.detection_hint.is_empty(),
                    "artifact '{}' has empty detection_hint for {:?}",
                    entry.artifact_id,
                    risk.technique
                );
            }
        }
    }

    #[test]
    fn all_table_ids_exist_in_catalog() {
        use crate::catalog::CATALOG;
        let catalog_ids: std::collections::HashSet<&str> =
            CATALOG.list().iter().map(|d| d.id).collect();
        for entry in AF_RISKS_TABLE {
            assert!(
                catalog_ids.contains(entry.artifact_id),
                "AF table references unknown catalog id: {}",
                entry.artifact_id
            );
        }
    }
}
