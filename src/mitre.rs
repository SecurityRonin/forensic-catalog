//! MITRE ATT&CK integration.
//!
//! Canonical home for ATT&CK-typed data shared across forensicnomicon modules:
//!
//! - [`AttackTechnique`] — the shared ATT&CK technique struct (re-used by
//!   [`crate::navigator`] and [`crate::attack_flow`])
//! - [`lookup_attack_for_rule_name`] — map a YARA rule name prefix to its
//!   ATT&CK technique, for enriching YARA scan results without embedding
//!   forensic knowledge in the calling tool (used by blazehash)
//!
//! Re-exported as `forensicnomicon::attack` for backwards compatibility.

/// A resolved MITRE ATT&CK technique entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttackTechnique {
    /// ATT&CK technique ID, e.g. `"T1486"` or `"T1059.001"`.
    pub technique_id: &'static str,
    /// ATT&CK tactic (lowercase kebab-case), e.g. `"impact"`.
    pub tactic: &'static str,
    /// Human-readable technique name, e.g. `"Data Encrypted for Impact"`.
    pub name: &'static str,
}

/// YARA rule name prefix → ATT&CK technique mapping.
///
/// Entries are matched case-insensitively against the start of the rule name.
/// The first matching entry wins.
static ATTACK_PREFIXES: &[(&str, &str, &str, &str)] = &[
    (
        "rat_",
        "T1219",
        "command-and-control",
        "Remote Access Software",
    ),
    (
        "ransomware_",
        "T1486",
        "impact",
        "Data Encrypted for Impact",
    ),
    ("wiper_", "T1485", "impact", "Data Destruction"),
    (
        "creddump_",
        "T1003",
        "credential-access",
        "OS Credential Dumping",
    ),
    (
        "keylogger_",
        "T1056.001",
        "collection",
        "Input Capture: Keylogging",
    ),
    ("rootkit_", "T1014", "defense-evasion", "Rootkit"),
    (
        "backdoor_",
        "T1505",
        "persistence",
        "Server Software Component",
    ),
    (
        "dropper_",
        "T1105",
        "command-and-control",
        "Ingress Tool Transfer",
    ),
    ("miner_", "T1496", "impact", "Resource Hijacking"),
    (
        "stealer_",
        "T1041",
        "exfiltration",
        "Exfiltration Over C2 Channel",
    ),
    (
        "exploit_",
        "T1203",
        "execution",
        "Exploitation for Client Execution",
    ),
    ("loader_", "T1129", "execution", "Shared Modules"),
    (
        "persistence_",
        "T1547",
        "persistence",
        "Boot or Logon Autostart Execution",
    ),
    (
        "injection_",
        "T1055",
        "defense-evasion",
        "Process Injection",
    ),
    (
        "shellcode_",
        "T1059",
        "execution",
        "Command and Scripting Interpreter",
    ),
    (
        "webshell_",
        "T1505.003",
        "persistence",
        "Server Software Component: Web Shell",
    ),
    ("powershell_", "T1059.001", "execution", "PowerShell"),
    (
        "maldoc_",
        "T1566.001",
        "initial-access",
        "Phishing: Spearphishing Attachment",
    ),
    (
        "botnet_",
        "T1571",
        "command-and-control",
        "Non-Standard Port",
    ),
    ("antiav_", "T1562", "defense-evasion", "Impair Defenses"),
];

/// Look up a MITRE ATT&CK technique by matching the start of `rule_name`
/// case-insensitively against the known prefix table.
///
/// Returns `None` if no prefix matches.
pub fn lookup_attack_for_rule_name(rule_name: &str) -> Option<AttackTechnique> {
    let lower = rule_name.to_lowercase();
    for &(prefix, technique_id, tactic, name) in ATTACK_PREFIXES {
        if lower.starts_with(prefix) {
            return Some(AttackTechnique {
                technique_id,
                tactic,
                name,
            });
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ransomware_prefix_maps_to_t1486() {
        let r = lookup_attack_for_rule_name("ransomware_locky").unwrap();
        assert_eq!(r.technique_id, "T1486");
        assert_eq!(r.tactic, "impact");
        assert_eq!(r.name, "Data Encrypted for Impact");
    }

    #[test]
    fn rootkit_prefix_maps_to_t1014() {
        let r = lookup_attack_for_rule_name("rootkit_necurs").unwrap();
        assert_eq!(r.technique_id, "T1014");
        assert_eq!(r.tactic, "defense-evasion");
    }

    #[test]
    fn case_insensitive_match() {
        let r = lookup_attack_for_rule_name("Ransomware_petya").unwrap();
        assert_eq!(r.technique_id, "T1486");
    }

    #[test]
    fn powershell_prefix_maps_to_t1059_001() {
        let r = lookup_attack_for_rule_name("powershell_empire").unwrap();
        assert_eq!(r.technique_id, "T1059.001");
    }

    #[test]
    fn webshell_maps_to_t1505_003() {
        let r = lookup_attack_for_rule_name("webshell_china_chopper").unwrap();
        assert_eq!(r.technique_id, "T1505.003");
    }

    #[test]
    fn unknown_prefix_returns_none() {
        assert!(lookup_attack_for_rule_name("generic_malware").is_none());
        assert!(lookup_attack_for_rule_name("").is_none());
    }

    #[test]
    fn all_20_prefixes_are_reachable() {
        let probes = [
            "rat_",
            "ransomware_",
            "wiper_",
            "creddump_",
            "keylogger_",
            "rootkit_",
            "backdoor_",
            "dropper_",
            "miner_",
            "stealer_",
            "exploit_",
            "loader_",
            "persistence_",
            "injection_",
            "shellcode_",
            "webshell_",
            "powershell_",
            "maldoc_",
            "botnet_",
            "antiav_",
        ];
        for prefix in probes {
            let name = format!("{prefix}test");
            assert!(
                lookup_attack_for_rule_name(&name).is_some(),
                "prefix '{prefix}' returned None"
            );
        }
    }
}
