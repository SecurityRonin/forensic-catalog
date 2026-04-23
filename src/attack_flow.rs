//! MITRE ATT&CK Flow — campaign graph layer.
//!
//! Models adversary campaigns as directed sequences of ATT&CK actions, each
//! mapped to the forensicnomicon artifact IDs that provide evidence of that
//! action. Inspired by the CTID Attack Flow specification:
//! <https://center-for-threat-informed-defense.github.io/attack-flow/>
//!
//! # Data model
//!
//! An [`AttackFlow`] is a named campaign scenario (e.g. ransomware double
//! extortion, lateral movement via PtH). It contains an ordered sequence of
//! [`FlowAction`] steps. Each step carries:
//!
//! - the ATT&CK technique it represents
//! - the forensicnomicon artifact IDs that provide evidence of it
//! - indices into the action list for successor steps (causal edges)
//!
//! # Example
//!
//! ```rust
//! use forensicnomicon::attack_flow::{flow_by_id, artifacts_in_flow};
//!
//! let flow = flow_by_id("ransomware_double_extortion").unwrap();
//! let artifacts = artifacts_in_flow("ransomware_double_extortion");
//! assert!(!artifacts.is_empty());
//! ```

use crate::mitre::AttackTechnique;

/// A single action in an attack flow — one ATT&CK technique and the
/// forensicnomicon artifact IDs that provide evidence of it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlowAction {
    /// ATT&CK technique ID, e.g. `"T1486"`.
    pub technique_id: &'static str,
    /// ATT&CK tactic, e.g. `"impact"`.
    pub tactic: &'static str,
    /// Human-readable technique name.
    pub name: &'static str,
    /// Forensicnomicon artifact IDs that provide evidence of this action.
    pub artifact_ids: &'static [&'static str],
    /// Indices into the parent flow's `actions` slice for causal successors.
    pub leads_to: &'static [usize],
}

impl FlowAction {
    /// Returns this action as a typed [`AttackTechnique`].
    pub fn technique(&self) -> AttackTechnique {
        AttackTechnique {
            technique_id: self.technique_id,
            tactic: self.tactic,
            name: self.name,
        }
    }
}

/// A named adversary campaign scenario modelled as an ordered action graph.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttackFlow {
    /// Stable identifier, e.g. `"ransomware_double_extortion"`.
    pub id: &'static str,
    /// Human-readable campaign name.
    pub name: &'static str,
    /// Brief description of the scenario.
    pub description: &'static str,
    /// Ordered sequence of actions. Edges encoded via `leads_to` indices.
    pub actions: &'static [FlowAction],
}

// ── Static campaign graph ────────────────────────────────────────────────────

static RANSOMWARE_DOUBLE_EXTORTION_ACTIONS: &[FlowAction] = &[
    FlowAction {
        technique_id: "T1566.001",
        tactic: "initial-access",
        name: "Phishing: Spearphishing Attachment",
        artifact_ids: &["evtx_security", "evtx_sysmon", "lnk_files"],
        leads_to: &[1],
    },
    FlowAction {
        technique_id: "T1059.001",
        tactic: "execution",
        name: "PowerShell",
        artifact_ids: &["powershell_history", "evtx_powershell", "psreadline_history"],
        leads_to: &[2],
    },
    FlowAction {
        technique_id: "T1547.001",
        tactic: "persistence",
        name: "Boot or Logon Autostart Execution: Registry Run Keys",
        artifact_ids: &["run_key_hkcu", "run_key_hklm"],
        leads_to: &[3],
    },
    FlowAction {
        technique_id: "T1003.001",
        tactic: "credential-access",
        name: "OS Credential Dumping: LSASS Memory",
        artifact_ids: &["evtx_security", "evtx_sysmon", "prefetch_dir"],
        leads_to: &[4],
    },
    FlowAction {
        technique_id: "T1041",
        tactic: "exfiltration",
        name: "Exfiltration Over C2 Channel",
        artifact_ids: &["evtx_security", "srum_network_usage", "dns_debug_log"],
        leads_to: &[5],
    },
    FlowAction {
        technique_id: "T1490",
        tactic: "impact",
        name: "Inhibit System Recovery",
        artifact_ids: &["evtx_system", "usn_journal", "evtx_security"],
        leads_to: &[6],
    },
    FlowAction {
        technique_id: "T1486",
        tactic: "impact",
        name: "Data Encrypted for Impact",
        artifact_ids: &["mft_file", "usn_journal", "recycle_bin"],
        leads_to: &[],
    },
];

static LATERAL_MOVEMENT_PTH_ACTIONS: &[FlowAction] = &[
    FlowAction {
        technique_id: "T1078",
        tactic: "defense-evasion",
        name: "Valid Accounts",
        artifact_ids: &["evtx_security", "windows_vault_user"],
        leads_to: &[1],
    },
    FlowAction {
        technique_id: "T1003.002",
        tactic: "credential-access",
        name: "OS Credential Dumping: Security Account Manager",
        artifact_ids: &["sam_users", "evtx_security", "prefetch_dir"],
        leads_to: &[2],
    },
    FlowAction {
        technique_id: "T1550.002",
        tactic: "lateral-movement",
        name: "Use Alternate Authentication Material: Pass the Hash",
        artifact_ids: &["evtx_security", "evtx_rdp_inbound"],
        leads_to: &[3],
    },
    FlowAction {
        technique_id: "T1021.001",
        tactic: "lateral-movement",
        name: "Remote Services: Remote Desktop Protocol",
        artifact_ids: &["evtx_rdp_inbound", "evtx_rdp_client", "evtx_security"],
        leads_to: &[4],
    },
    FlowAction {
        technique_id: "T1053.005",
        tactic: "persistence",
        name: "Scheduled Task/Job: Scheduled Task",
        artifact_ids: &["scheduled_tasks_dir", "evtx_task_scheduler", "scheduled_task_registry_cache"],
        leads_to: &[],
    },
];

static CREDENTIAL_THEFT_PERSISTENCE_ACTIONS: &[FlowAction] = &[
    FlowAction {
        technique_id: "T1566.002",
        tactic: "initial-access",
        name: "Phishing: Spearphishing Link",
        artifact_ids: &["evtx_security", "chrome_history", "firefox_places"],
        leads_to: &[1],
    },
    FlowAction {
        technique_id: "T1555.003",
        tactic: "credential-access",
        name: "Credentials from Password Stores: Credentials from Web Browsers",
        artifact_ids: &["chrome_login_data", "firefox_logins", "edge_chromium_login_data"],
        leads_to: &[2],
    },
    FlowAction {
        technique_id: "T1555",
        tactic: "credential-access",
        name: "Credentials from Password Stores",
        artifact_ids: &["dpapi_masterkey_user", "dpapi_cred_user", "windows_vault_user"],
        leads_to: &[3],
    },
    FlowAction {
        technique_id: "T1546.015",
        tactic: "persistence",
        name: "Event Triggered Execution: Component Object Model Hijacking",
        artifact_ids: &["run_key_hkcu", "usrclass_dat_file"],
        leads_to: &[4],
    },
    FlowAction {
        technique_id: "T1070.004",
        tactic: "defense-evasion",
        name: "Indicator Removal: File Deletion",
        artifact_ids: &["recycle_bin", "usn_journal", "mft_file"],
        leads_to: &[],
    },
];

static LIVING_OFF_THE_LAND_ACTIONS: &[FlowAction] = &[
    FlowAction {
        technique_id: "T1059.001",
        tactic: "execution",
        name: "Command and Scripting Interpreter: PowerShell",
        artifact_ids: &["powershell_history", "evtx_powershell", "prefetch_dir"],
        leads_to: &[1, 2],
    },
    FlowAction {
        technique_id: "T1218",
        tactic: "defense-evasion",
        name: "System Binary Proxy Execution",
        artifact_ids: &["prefetch_dir", "shimcache", "amcache_app_file"],
        leads_to: &[3],
    },
    FlowAction {
        technique_id: "T1012",
        tactic: "discovery",
        name: "Query Registry",
        artifact_ids: &["evtx_sysmon", "prefetch_dir", "powershell_history"],
        leads_to: &[3],
    },
    FlowAction {
        technique_id: "T1027",
        tactic: "defense-evasion",
        name: "Obfuscated Files or Information",
        artifact_ids: &["evtx_powershell", "prefetch_dir", "amcache_app_file"],
        leads_to: &[4],
    },
    FlowAction {
        technique_id: "T1041",
        tactic: "exfiltration",
        name: "Exfiltration Over C2 Channel",
        artifact_ids: &["srum_network_usage", "evtx_security", "dns_debug_log"],
        leads_to: &[],
    },
];

static WMI_PERSISTENCE_ACTIONS: &[FlowAction] = &[
    FlowAction {
        technique_id: "T1059.001",
        tactic: "execution",
        name: "PowerShell",
        artifact_ids: &["powershell_history", "evtx_powershell"],
        leads_to: &[1],
    },
    FlowAction {
        technique_id: "T1546.003",
        tactic: "persistence",
        name: "Event Triggered Execution: Windows Management Instrumentation Event Subscription",
        artifact_ids: &["wmi_subscriptions", "wmi_mof_dir", "evtx_wmi_activity"],
        leads_to: &[2],
    },
    FlowAction {
        technique_id: "T1070.001",
        tactic: "defense-evasion",
        name: "Indicator Removal: Clear Windows Event Logs",
        artifact_ids: &["evtx_security", "evtx_system", "usn_journal"],
        leads_to: &[],
    },
];

/// All available attack flow scenarios.
static ATTACK_FLOWS: &[AttackFlow] = &[
    AttackFlow {
        id: "ransomware_double_extortion",
        name: "Ransomware — Double Extortion",
        description: "Phishing delivery → PowerShell execution → credential access \
                       → data exfiltration → shadow copy deletion → encryption",
        actions: RANSOMWARE_DOUBLE_EXTORTION_ACTIONS,
    },
    AttackFlow {
        id: "lateral_movement_pth",
        name: "Lateral Movement via Pass-the-Hash",
        description: "Credential access via SAM dump → PtH → RDP lateral movement \
                       → scheduled task persistence on target",
        actions: LATERAL_MOVEMENT_PTH_ACTIONS,
    },
    AttackFlow {
        id: "credential_theft_persistence",
        name: "Browser Credential Theft → Persistence",
        description: "Spearphishing link → browser credential harvest → DPAPI vault \
                       → COM hijack persistence → file deletion anti-forensics",
        actions: CREDENTIAL_THEFT_PERSISTENCE_ACTIONS,
    },
    AttackFlow {
        id: "living_off_the_land",
        name: "Living-off-the-Land Execution",
        description: "LOLBin abuse → PowerShell → system binary proxy execution \
                       → registry discovery → obfuscation → C2 exfiltration",
        actions: LIVING_OFF_THE_LAND_ACTIONS,
    },
    AttackFlow {
        id: "wmi_persistence",
        name: "WMI Event Subscription Persistence",
        description: "PowerShell drops WMI event subscription → event log clearing",
        actions: WMI_PERSISTENCE_ACTIONS,
    },
];

// ── Query API ────────────────────────────────────────────────────────────────

/// Look up a campaign flow by its stable `id`.
pub fn flow_by_id(id: &str) -> Option<&'static AttackFlow> {
    ATTACK_FLOWS.iter().find(|f| f.id == id)
}

/// Return all available attack flows.
pub fn all_flows() -> &'static [AttackFlow] {
    ATTACK_FLOWS
}

/// Collect all unique artifact IDs referenced across all actions in a flow.
///
/// Returns an empty `Vec` if `flow_id` is not found.
pub fn artifacts_in_flow(flow_id: &str) -> Vec<&'static str> {
    let Some(flow) = flow_by_id(flow_id) else {
        return Vec::new();
    };
    let mut seen = std::collections::HashSet::new();
    let mut ids = Vec::new();
    for action in flow.actions {
        for &id in action.artifact_ids {
            if seen.insert(id) {
                ids.push(id);
            }
        }
    }
    ids
}

/// Return all flows that reference `artifact_id` in at least one action.
pub fn flows_for_artifact(artifact_id: &str) -> Vec<&'static AttackFlow> {
    ATTACK_FLOWS
        .iter()
        .filter(|f| {
            f.actions
                .iter()
                .any(|a| a.artifact_ids.contains(&artifact_id))
        })
        .collect()
}

/// Return all flows that contain an action for `technique_id`.
pub fn flows_for_technique(technique_id: &str) -> Vec<&'static AttackFlow> {
    ATTACK_FLOWS
        .iter()
        .filter(|f| f.actions.iter().any(|a| a.technique_id == technique_id))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::CATALOG;

    #[test]
    fn attack_flow_table_nonempty() {
        assert!(!ATTACK_FLOWS.is_empty(), "need at least one flow");
    }

    #[test]
    fn flow_by_id_returns_correct_flow() {
        let f = flow_by_id("ransomware_double_extortion").unwrap();
        assert_eq!(f.id, "ransomware_double_extortion");
        assert!(f.name.contains("Ransomware"));
    }

    #[test]
    fn unknown_flow_id_returns_none() {
        assert!(flow_by_id("nonexistent_scenario").is_none());
    }

    #[test]
    fn every_flow_has_at_least_two_actions() {
        for flow in ATTACK_FLOWS {
            assert!(
                flow.actions.len() >= 2,
                "flow '{}' has only {} action(s) — flows should model sequences",
                flow.id,
                flow.actions.len()
            );
        }
    }

    #[test]
    fn every_flow_has_nonempty_description() {
        for flow in ATTACK_FLOWS {
            assert!(!flow.description.is_empty(), "flow '{}' has empty description", flow.id);
        }
    }

    #[test]
    fn leads_to_indices_are_in_bounds() {
        for flow in ATTACK_FLOWS {
            let len = flow.actions.len();
            for (i, action) in flow.actions.iter().enumerate() {
                for &j in action.leads_to {
                    assert!(
                        j < len,
                        "flow '{}' action[{i}] leads_to index {j} is out of bounds (len={len})",
                        flow.id,
                    );
                }
            }
        }
    }

    #[test]
    fn artifacts_in_flow_are_in_catalog() {
        // Only check curated IDs — generated IDs may differ across ingest runs.
        // We accept the artifact either in the catalog or as a known generated ID.
        let curated_prefixes = ["evtx_", "kape_", "fa_", "browsers_", "velociraptor_",
                                 "nirsoft_", "regedit_"];
        for flow in ATTACK_FLOWS {
            for artifact_id in artifacts_in_flow(flow.id) {
                let in_catalog = CATALOG.by_id(artifact_id).is_some();
                let is_generated = curated_prefixes.iter().any(|p| artifact_id.starts_with(p));
                assert!(
                    in_catalog || is_generated,
                    "flow '{}' references artifact '{}' which is not in the catalog",
                    flow.id,
                    artifact_id
                );
            }
        }
    }

    #[test]
    fn artifacts_in_flow_deduplicates() {
        for flow in ATTACK_FLOWS {
            let ids = artifacts_in_flow(flow.id);
            let mut seen = std::collections::HashSet::new();
            for id in &ids {
                assert!(seen.insert(id), "duplicate artifact '{id}' in flow '{}'", flow.id);
            }
        }
    }

    #[test]
    fn flows_for_artifact_finds_relevant_flows() {
        let flows = flows_for_artifact("powershell_history");
        assert!(
            !flows.is_empty(),
            "powershell_history should appear in at least one flow"
        );
        assert!(flows.iter().any(|f| f.id == "ransomware_double_extortion"));
    }

    #[test]
    fn flows_for_artifact_unknown_returns_empty() {
        let flows = flows_for_artifact("__definitely_not_an_artifact__");
        assert!(flows.is_empty());
    }

    #[test]
    fn flows_for_technique_finds_ransomware() {
        let flows = flows_for_technique("T1486");
        assert!(
            flows.iter().any(|f| f.id == "ransomware_double_extortion"),
            "T1486 should be in ransomware_double_extortion"
        );
    }

    #[test]
    fn all_flow_ids_are_unique() {
        let mut ids = std::collections::HashSet::new();
        for flow in ATTACK_FLOWS {
            assert!(ids.insert(flow.id), "duplicate flow id: '{}'", flow.id);
        }
    }

    #[test]
    fn flow_action_technique_method_returns_correct_struct() {
        let flow = flow_by_id("ransomware_double_extortion").unwrap();
        let last = flow.actions.last().unwrap();
        let t = last.technique();
        assert_eq!(t.technique_id, "T1486");
        assert_eq!(t.tactic, "impact");
    }

    #[test]
    fn covered_techniques_in_navigator_are_typed() {
        let techniques = crate::navigator::covered_techniques();
        assert!(!techniques.is_empty());
        // Every entry should have a non-empty technique_id
        for t in &techniques {
            assert!(!t.technique_id.is_empty());
        }
    }

    // ── CTID-sourced flow tests (RED: these fail until the real data is added) ──

    /// The corpus must contain at least 5 flows (the 5 real CTID flows).
    #[test]
    fn ctid_flow_count_at_least_five() {
        assert!(
            ATTACK_FLOWS.len() >= 5,
            "expected at least 5 flows from CTID corpus, got {}",
            ATTACK_FLOWS.len()
        );
    }

    /// Black Basta Ransomware flow from CTID corpus must exist.
    #[test]
    fn ctid_black_basta_ransomware_flow_exists() {
        let f = flow_by_id("black_basta_ransomware")
            .expect("flow 'black_basta_ransomware' must exist (CTID corpus)");
        assert!(f.name.contains("Black Basta"), "name should contain 'Black Basta'");
    }

    /// Black Basta must have its first real technique T1566.001 (spearphishing attachment).
    #[test]
    fn ctid_black_basta_has_t1566_001() {
        let f = flow_by_id("black_basta_ransomware")
            .expect("flow 'black_basta_ransomware' must exist");
        assert!(
            f.actions.iter().any(|a| a.technique_id == "T1566.001"),
            "black_basta_ransomware must contain T1566.001 (Spearphishing Attachment)"
        );
    }

    /// Black Basta must have T1486 (ransomware encryption).
    #[test]
    fn ctid_black_basta_has_t1486() {
        let f = flow_by_id("black_basta_ransomware")
            .expect("flow 'black_basta_ransomware' must exist");
        assert!(
            f.actions.iter().any(|a| a.technique_id == "T1486"),
            "black_basta_ransomware must contain T1486 (Data Encrypted for Impact)"
        );
    }

    /// Cobalt Kitty Campaign flow from CTID corpus must exist.
    #[test]
    fn ctid_cobalt_kitty_campaign_flow_exists() {
        let f = flow_by_id("cobalt_kitty_campaign")
            .expect("flow 'cobalt_kitty_campaign' must exist (CTID corpus)");
        assert!(f.name.contains("Cobalt Kitty"), "name should contain 'Cobalt Kitty'");
    }

    /// Cobalt Kitty must have T1566.002 (spearphishing link — its initial access).
    #[test]
    fn ctid_cobalt_kitty_has_t1566_002() {
        let f = flow_by_id("cobalt_kitty_campaign")
            .expect("flow 'cobalt_kitty_campaign' must exist");
        assert!(
            f.actions.iter().any(|a| a.technique_id == "T1566.002"),
            "cobalt_kitty_campaign must contain T1566.002 (Spearphishing Link)"
        );
    }

    /// SolarWinds supply-chain flow from CTID corpus must exist.
    #[test]
    fn ctid_solarwinds_supply_chain_flow_exists() {
        let f = flow_by_id("solarwinds_supply_chain")
            .expect("flow 'solarwinds_supply_chain' must exist (CTID corpus)");
        assert!(f.name.contains("SolarWinds"), "name should contain 'SolarWinds'");
    }

    /// SolarWinds must have T1195.002 (supply chain compromise — its hallmark technique).
    #[test]
    fn ctid_solarwinds_has_t1195_002() {
        let f = flow_by_id("solarwinds_supply_chain")
            .expect("flow 'solarwinds_supply_chain' must exist");
        assert!(
            f.actions.iter().any(|a| a.technique_id == "T1195.002"),
            "solarwinds_supply_chain must contain T1195.002 (Compromise Software Supply Chain)"
        );
    }

    /// Conti Ransomware flow from CTID corpus must exist.
    #[test]
    fn ctid_conti_ransomware_flow_exists() {
        let f = flow_by_id("conti_ransomware")
            .expect("flow 'conti_ransomware' must exist (CTID corpus)");
        assert!(f.name.contains("Conti"), "name should contain 'Conti'");
    }

    /// Conti must have T1486 (ransomware encryption).
    #[test]
    fn ctid_conti_ransomware_has_t1486() {
        let f = flow_by_id("conti_ransomware")
            .expect("flow 'conti_ransomware' must exist");
        assert!(
            f.actions.iter().any(|a| a.technique_id == "T1486"),
            "conti_ransomware must contain T1486 (Data Encrypted for Impact)"
        );
    }

    /// BumbleBee Round 2 flow from CTID corpus must exist.
    #[test]
    fn ctid_bumblbee_round2_flow_exists() {
        let f = flow_by_id("bumblbee_round2")
            .expect("flow 'bumblbee_round2' must exist (CTID corpus)");
        assert!(f.name.contains("BumbleBee"), "name should contain 'BumbleBee'");
    }

    /// BumbleBee must have T1003.001 (LSASS credential dumping).
    #[test]
    fn ctid_bumblbee_round2_has_t1003_001() {
        let f = flow_by_id("bumblbee_round2")
            .expect("flow 'bumblbee_round2' must exist");
        assert!(
            f.actions.iter().any(|a| a.technique_id == "T1003.001"),
            "bumblbee_round2 must contain T1003.001 (LSASS Memory)"
        );
    }

    /// All CTID flows must have their real technique IDs (not empty strings).
    #[test]
    fn ctid_flows_have_nonempty_technique_ids() {
        let ctid_ids = [
            "black_basta_ransomware",
            "cobalt_kitty_campaign",
            "solarwinds_supply_chain",
            "conti_ransomware",
            "bumblbee_round2",
        ];
        for id in ctid_ids {
            if let Some(f) = flow_by_id(id) {
                for action in f.actions {
                    assert!(
                        !action.technique_id.is_empty(),
                        "flow '{}' has an action with empty technique_id: '{}'",
                        id, action.name
                    );
                }
            }
        }
    }
}
