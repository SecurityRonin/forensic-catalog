use crate::threat_intel::signals::*;

pub struct HookSymbol {
    pub name: &'static str,
    /// Signal ID from `threat_intel/signals.rs` emitted when this symbol is found.
    pub emits_signal: &'static str,
    pub mitre_technique: &'static str,
}

pub const ROOTKIT_HOOK_SYMBOLS: &[HookSymbol] = &[
    HookSymbol { name: "readdir",          emits_signal: ELF_HOOKS_PROCESS_HIDING,  mitre_technique: "T1014" },
    HookSymbol { name: "readdir64",        emits_signal: ELF_HOOKS_PROCESS_HIDING,  mitre_technique: "T1014" },
    HookSymbol { name: "getdents",         emits_signal: ELF_HOOKS_PROCESS_HIDING,  mitre_technique: "T1014" },
    HookSymbol { name: "getdents64",       emits_signal: ELF_HOOKS_PROCESS_HIDING,  mitre_technique: "T1014" },
    HookSymbol { name: "openat",           emits_signal: ELF_HOOKS_FILE_HIDING,     mitre_technique: "T1564.001" },
    HookSymbol { name: "stat",             emits_signal: ELF_HOOKS_FILE_HIDING,     mitre_technique: "T1564.001" },
    HookSymbol { name: "lstat",            emits_signal: ELF_HOOKS_FILE_HIDING,     mitre_technique: "T1564.001" },
    HookSymbol { name: "access",           emits_signal: ELF_HOOKS_FILE_HIDING,     mitre_technique: "T1564.001" },
    HookSymbol { name: "pam_get_item",     emits_signal: ELF_HOOKS_PAM_CREDENTIAL,  mitre_technique: "T1556.003" },
    HookSymbol { name: "pam_authenticate", emits_signal: ELF_HOOKS_PAM_CREDENTIAL,  mitre_technique: "T1556.003" },
    HookSymbol { name: "pam_open_session", emits_signal: ELF_HOOKS_PAM_CREDENTIAL,  mitre_technique: "T1556.003" },
    HookSymbol { name: "recvfrom",         emits_signal: ELF_HOOKS_NETWORK_HIDING,  mitre_technique: "T1014" },
    HookSymbol { name: "recvmsg",          emits_signal: ELF_HOOKS_NETWORK_HIDING,  mitre_technique: "T1014" },
    HookSymbol { name: "getuid",           emits_signal: ELF_HOOKS_UID_SPOOFING,    mitre_technique: "T1548" },
    HookSymbol { name: "geteuid",          emits_signal: ELF_HOOKS_UID_SPOOFING,    mitre_technique: "T1548" },
    HookSymbol { name: "getpwuid",         emits_signal: ELF_HOOKS_UID_SPOOFING,    mitre_technique: "T1548" },
    HookSymbol { name: "write",            emits_signal: ELF_HOOKS_IO_INTERCEPTION, mitre_technique: "T1056.001" },
];
