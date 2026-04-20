//! Investigation playbook engine.
//!
//! Provides directed investigation paths: given a trigger artifact or MITRE
//! technique, what artifacts to examine next, in what order, and why.

/// One step in an investigation playbook.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct InvestigationStep {
    /// Catalog artifact ID to examine.
    pub artifact_id: &'static str,
    /// Why this step matters in context.
    pub rationale: &'static str,
    /// What specific indicators or values to look for.
    pub look_for: &'static str,
    /// Artifact IDs that become relevant if this step yields results.
    pub unlocks: &'static [&'static str],
}

/// A directed investigation path for a specific scenario.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct InvestigationPath {
    /// Unique playbook identifier.
    pub id: &'static str,
    /// What triggers this path (artifact ID or MITRE technique).
    pub trigger: &'static str,
    /// Human-readable scenario name.
    pub name: &'static str,
    /// Ordered investigation steps.
    pub steps: &'static [InvestigationStep],
    /// ATT&CK tactics this path covers.
    pub tactics_covered: &'static [&'static str],
    /// Brief description of the scenario.
    pub description: &'static str,
}

pub static PLAYBOOKS: &[InvestigationPath] = &[
    InvestigationPath {
        id: "lateral_movement_rdp",
        trigger: "rdp_client_servers",
        name: "Lateral Movement via RDP",
        description: "Investigate RDP-based lateral movement: source, destination, credentials used, and post-exploitation activity.",
        tactics_covered: &["TA0008", "TA0003"],
        steps: &[
            InvestigationStep {
                artifact_id: "rdp_client_servers",
                rationale: "Establishes which hosts were connected TO from this machine.",
                look_for: "Unfamiliar internal IPs, jump hosts, sequential targeting pattern.",
                unlocks: &["evtx_security", "lnk_files"],
            },
            InvestigationStep {
                artifact_id: "networklist_profiles",
                rationale: "Identifies networks the machine connected to; corroborates RDP destinations.",
                look_for: "Network names matching target subnets.",
                unlocks: &[],
            },
            InvestigationStep {
                artifact_id: "evtx_security",
                rationale: "Event 4624 Type 10 = RemoteInteractive logon; 4648 = explicit credential use.",
                look_for: "4624 with LogonType=10, 4648 with target server matching RDP MRU entries.",
                unlocks: &["dpapi_masterkey_user", "lsa_secrets"],
            },
            InvestigationStep {
                artifact_id: "prefetch_file",
                rationale: "mstsc.exe prefetch proves local RDP client was run and when.",
                look_for: "MSTSC.EXE-*.pf with timestamps matching logon events.",
                unlocks: &["jump_list_auto"],
            },
            InvestigationStep {
                artifact_id: "jump_list_auto",
                rationale: "mstsc.exe jump list may contain target hostnames.",
                look_for: "Recent items in mstsc jump list; correlate with Security log.",
                unlocks: &[],
            },
            InvestigationStep {
                artifact_id: "bam_user",
                rationale: "Background Activity Monitor records last execution time for mstsc.exe.",
                look_for: "mstsc.exe entry with timestamp matching attack window.",
                unlocks: &[],
            },
        ],
    },
    InvestigationPath {
        id: "credential_harvesting",
        trigger: "lsa_secrets",
        name: "Credential Harvesting",
        description: "Investigate credential theft: which credentials were targeted, how they were extracted, and what access was gained.",
        tactics_covered: &["TA0006"],
        steps: &[
            InvestigationStep {
                artifact_id: "lsa_secrets",
                rationale: "Service account passwords and cached domain credentials stored in SYSTEM hive.",
                look_for: "Unexpected service credentials, recently modified LSA secrets.",
                unlocks: &["dpapi_masterkey_user", "dcc2_cache"],
            },
            InvestigationStep {
                artifact_id: "dpapi_masterkey_user",
                rationale: "Master keys protect DPAPI-encrypted credentials; attacker may target these.",
                look_for: "Master key access events, unusual modification timestamps.",
                unlocks: &["dpapi_cred_user", "chrome_login_data"],
            },
            InvestigationStep {
                artifact_id: "dpapi_cred_user",
                rationale: "Encrypted credential blobs for Windows Credential Manager.",
                look_for: "Credential blobs accessed/modified outside normal user session.",
                unlocks: &[],
            },
            InvestigationStep {
                artifact_id: "dcc2_cache",
                rationale: "Cached domain credentials allow offline cracking without DC access.",
                look_for: "Presence of domain admin account hashes in cache.",
                unlocks: &["ntds_dit"],
            },
            InvestigationStep {
                artifact_id: "evtx_security",
                rationale: "4672=Special logon, 4768/4769=Kerberos TGT/service ticket requests.",
                look_for: "4672 for admin accounts, 4768 with RC4 encryption (downgrade attack).",
                unlocks: &[],
            },
            InvestigationStep {
                artifact_id: "sam_users",
                rationale: "Local account hashes; attacker with SYSTEM can extract all local credentials.",
                look_for: "Unusual local admin accounts, recently created accounts.",
                unlocks: &[],
            },
        ],
    },
    InvestigationPath {
        id: "persistence_hunt",
        trigger: "run_key_hklm",
        name: "Persistence Mechanism Hunt",
        description: "Systematic enumeration of persistence mechanisms: registry autoruns, services, scheduled tasks, and boot persistence.",
        tactics_covered: &["TA0003"],
        steps: &[
            InvestigationStep {
                artifact_id: "run_key_hklm",
                rationale: "HKLM Run key executes entries for all users at login.",
                look_for: "Unsigned executables, unusual paths (Temp, AppData, Downloads).",
                unlocks: &["run_key_hkcu", "prefetch_file"],
            },
            InvestigationStep {
                artifact_id: "run_key_hkcu",
                rationale: "HKCU Run key executes entries for the current user at login.",
                look_for: "Same suspicious patterns; note which user profile.",
                unlocks: &["shellbags_user"],
            },
            InvestigationStep {
                artifact_id: "active_setup_hklm",
                rationale: "Active Setup runs per-user first login; abused for persistence after privilege escalation.",
                look_for: "StubPath values pointing to unusual executables.",
                unlocks: &[],
            },
            InvestigationStep {
                artifact_id: "winlogon_shell",
                rationale: "Shell value replaces or supplements explorer.exe at login.",
                look_for: "Any value other than 'explorer.exe' is highly suspicious.",
                unlocks: &[],
            },
            InvestigationStep {
                artifact_id: "boot_execute",
                rationale: "Runs before Windows subsystem initializes; used by rootkits.",
                look_for: "Anything other than 'autocheck autochk *' is suspicious.",
                unlocks: &[],
            },
            InvestigationStep {
                artifact_id: "appinit_dlls",
                rationale: "DLLs injected into every process loading user32.dll.",
                look_for: "Any non-empty value; verify each DLL is signed.",
                unlocks: &[],
            },
            InvestigationStep {
                artifact_id: "ifeo_debugger",
                rationale: "IFEO Debugger hijacks process execution at launch.",
                look_for: "Debugger pointing to malware or cmd.exe for accessibility binary hijack.",
                unlocks: &[],
            },
            InvestigationStep {
                artifact_id: "com_hijack_clsid_hkcu",
                rationale: "HKCU CLSID overrides load user-controlled DLLs without UAC.",
                look_for: "CLSIDs in HKCU overriding HKLM entries; unsigned DLL paths.",
                unlocks: &[],
            },
            InvestigationStep {
                artifact_id: "scheduled_tasks_dir",
                rationale: "Scheduled tasks provide persistence with flexible triggers.",
                look_for: "Tasks with random/GUID names, tasks running from Temp or AppData.",
                unlocks: &["evtx_security"],
            },
            InvestigationStep {
                artifact_id: "services_imagepath",
                rationale: "Services run as SYSTEM; malware frequently registers services.",
                look_for: "Services with ImagePath in non-standard locations, recently created.",
                unlocks: &[],
            },
        ],
    },
    InvestigationPath {
        id: "data_exfiltration",
        trigger: "chrome_login_data",
        name: "Data Exfiltration Investigation",
        description: "Investigate data exfiltration: what was accessed, staged, and sent where.",
        tactics_covered: &["TA0010", "TA0009"],
        steps: &[
            InvestigationStep {
                artifact_id: "chrome_login_data",
                rationale: "Browser credentials stolen first to access additional resources.",
                look_for: "Access timestamps on login.db outside normal business hours.",
                unlocks: &["firefox_logins"],
            },
            InvestigationStep {
                artifact_id: "network_drives",
                rationale: "Mapped network shares may be staging areas or exfiltration targets.",
                look_for: "Unusual drive mappings, external/cloud share paths.",
                unlocks: &[],
            },
            InvestigationStep {
                artifact_id: "usn_journal",
                rationale: "USN journal records file create/modify/delete; reveals staging activity.",
                look_for: "Bulk file copy operations, archive creation (zip/rar/7z), large file moves.",
                unlocks: &["recycle_bin"],
            },
            InvestigationStep {
                artifact_id: "recycle_bin",
                rationale: "Deleted files after exfiltration may still be in Recycle Bin.",
                look_for: "Deleted archives, bulk deletions after staging window.",
                unlocks: &[],
            },
            InvestigationStep {
                artifact_id: "lnk_files",
                rationale: "LNK files created when files are opened; proves attacker accessed specific files.",
                look_for: "LNK files pointing to sensitive documents, unusual file paths.",
                unlocks: &["jump_list_auto"],
            },
            InvestigationStep {
                artifact_id: "evtx_security",
                rationale: "5140/5145 = network share access events; proves remote file access.",
                look_for: "5145 with sensitive share names accessed by unfamiliar accounts.",
                unlocks: &[],
            },
        ],
    },
    InvestigationPath {
        id: "execution_trace",
        trigger: "prefetch_file",
        name: "Malware Execution Trace",
        description: "Reconstruct malware execution: what ran, when, from where, and what it accessed.",
        tactics_covered: &["TA0002"],
        steps: &[
            InvestigationStep {
                artifact_id: "prefetch_file",
                rationale: "Definitive execution proof; records run count, last run time, and loaded DLLs.",
                look_for: "Unknown executables, tools run from Temp/Downloads, single-run counts.",
                unlocks: &["shimcache", "amcache_app_file"],
            },
            InvestigationStep {
                artifact_id: "shimcache",
                rationale: "Records all executables that touched shimcache; proves file existed on disk.",
                look_for: "Files in Prefetch not in Shimcache (deleted after run).",
                unlocks: &[],
            },
            InvestigationStep {
                artifact_id: "amcache_app_file",
                rationale: "Records SHA1 hash of executed files; enables hash lookup even if file deleted.",
                look_for: "Hash lookups on VirusTotal for any matches in Prefetch.",
                unlocks: &[],
            },
            InvestigationStep {
                artifact_id: "userassist_exe",
                rationale: "GUI application launches with run count and timestamp.",
                look_for: "Unusual GUI tools (network scanners, dumpers) launched by user.",
                unlocks: &[],
            },
            InvestigationStep {
                artifact_id: "bam_user",
                rationale: "Background Activity Monitor; precise last execution time per binary.",
                look_for: "Execution times outside business hours, correlation with other timestamps.",
                unlocks: &[],
            },
            InvestigationStep {
                artifact_id: "evtx_security",
                rationale: "4688 = process creation (if audit policy enabled); 4689 = process exit.",
                look_for: "4688 events for malware executables; CommandLine field if enabled.",
                unlocks: &["evtx_sysmon"],
            },
            InvestigationStep {
                artifact_id: "evtx_sysmon",
                rationale: "Sysmon Event 1 = process creation with full command line and hashes.",
                look_for: "Sysmon 1 for malware hashes; Sysmon 3 for network connections; Sysmon 11 for file drops.",
                unlocks: &[],
            },
        ],
    },
    InvestigationPath {
        id: "defense_evasion",
        trigger: "usn_journal",
        name: "Defense Evasion Detection",
        description: "Detect anti-forensic actions: log clearing, timestomping, prefetch disabling, and tool deletion.",
        tactics_covered: &["TA0005"],
        steps: &[
            InvestigationStep {
                artifact_id: "evtx_system",
                rationale: "Event 104 = Security log cleared (or any channel cleared from System).",
                look_for: "Event 104 or 1102; gap in event timestamps indicates clearing.",
                unlocks: &["evtx_security"],
            },
            InvestigationStep {
                artifact_id: "evtx_security",
                rationale: "Event 1102 = audit log cleared; compare log size and oldest event timestamp.",
                look_for: "Event 1102; compare oldest event time against known attack window.",
                unlocks: &[],
            },
            InvestigationStep {
                artifact_id: "usn_journal",
                rationale: "USN journal deletions may catch tool cleanup (attacker deleting malware).",
                look_for: "File deletions immediately after execution events; *.exe, *.ps1 in Temp.",
                unlocks: &["recycle_bin"],
            },
            InvestigationStep {
                artifact_id: "mft_file",
                rationale: "Compare $STANDARD_INFORMATION vs $FILE_NAME timestamps; differences indicate timestomping.",
                look_for: "$SI Create before $FN Create; $SI earlier than Volume Create = timestomped.",
                unlocks: &[],
            },
            InvestigationStep {
                artifact_id: "prefetch_file",
                rationale: "Missing Prefetch for known tools may indicate anti-forensics (folder cleared or disabled).",
                look_for: "Check HKLM\\SYSTEM\\...\\PrefetchParameters\\EnablePrefetcher = 0 (disabled).",
                unlocks: &[],
            },
        ],
    },
];

/// Returns the playbook with the given ID.
pub fn playbook_by_id(id: &str) -> Option<&'static InvestigationPath> {
    PLAYBOOKS.iter().find(|p| p.id == id)
}

/// Returns all playbooks whose trigger matches the given artifact ID or MITRE technique.
pub fn playbooks_for_trigger(trigger: &str) -> Vec<&'static InvestigationPath> {
    PLAYBOOKS.iter().filter(|p| p.trigger == trigger).collect()
}

/// Returns all playbooks that reference the given artifact ID in any step.
pub fn playbooks_for_artifact(artifact_id: &str) -> Vec<&'static InvestigationPath> {
    PLAYBOOKS
        .iter()
        .filter(|p| p.steps.iter().any(|s| s.artifact_id == artifact_id))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::CATALOG;

    #[test]
    fn six_playbooks_defined() {
        assert_eq!(PLAYBOOKS.len(), 6, "Expected 6 playbooks");
    }

    #[test]
    fn playbook_by_id_works() {
        let pb = playbook_by_id("lateral_movement_rdp")
            .expect("lateral_movement_rdp playbook must exist");
        assert!(!pb.steps.is_empty());
        assert!(!pb.tactics_covered.is_empty());
    }

    #[test]
    fn playbooks_for_trigger_rdp() {
        let pbs = playbooks_for_trigger("rdp_client_servers");
        assert!(
            !pbs.is_empty(),
            "Should find playbooks triggered by rdp_client_servers"
        );
    }

    #[test]
    fn playbooks_for_artifact_evtx_security() {
        let pbs = playbooks_for_artifact("evtx_security");
        assert!(
            pbs.len() >= 2,
            "evtx_security should appear in multiple playbooks"
        );
    }

    #[test]
    fn all_step_artifact_ids_exist_in_catalog() {
        for pb in PLAYBOOKS {
            for step in pb.steps {
                assert!(
                    CATALOG.by_id(step.artifact_id).is_some(),
                    "playbook '{}' step references unknown artifact: {}",
                    pb.id,
                    step.artifact_id
                );
            }
        }
    }

    #[test]
    fn all_unlocks_reference_valid_artifacts() {
        for pb in PLAYBOOKS {
            for step in pb.steps {
                for unlocked_id in step.unlocks {
                    assert!(
                        CATALOG.by_id(unlocked_id).is_some(),
                        "playbook '{}' step '{}' unlocks unknown artifact: {}",
                        pb.id,
                        step.artifact_id,
                        unlocked_id
                    );
                }
            }
        }
    }

    #[test]
    fn all_playbooks_have_nonempty_steps_and_tactics() {
        for pb in PLAYBOOKS {
            assert!(!pb.steps.is_empty(), "Playbook '{}' has no steps", pb.id);
            assert!(
                !pb.tactics_covered.is_empty(),
                "Playbook '{}' has no tactics",
                pb.id
            );
            assert!(
                !pb.description.is_empty(),
                "Playbook '{}' has no description",
                pb.id
            );
        }
    }

    #[test]
    fn unknown_playbook_returns_none() {
        assert!(playbook_by_id("does_not_exist").is_none());
    }
}
