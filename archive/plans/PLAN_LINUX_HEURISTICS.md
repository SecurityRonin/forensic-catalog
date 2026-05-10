# forensicnomicon — Malware Profile Framework Plan

A generic, profile-driven malware detection framework. Father is the first concrete
profile; every other LD_PRELOAD rootkit, crypto miner, LKM rootkit, and future family
slots in by adding one profile file.

The framework has three parts, all zero-dependency compile-time data:
1. **Signal taxonomy** — vocabulary of atomic observable facts (`threat_intel/signals.rs`)
2. **Profile database** — per-family weighted signal tables (`threat_intel/profiles/`)
3. **Scoring engine** — `score_all_profiles(signals) -> Vec<ProfileMatch>` (`threat_intel/engine.rs`)

Runtime extensibility (YAML profile files) is a follow-on sprint and is called out separately.

Builds on existing forensicnomicon modules: `evidence.rs` (strength levels),
`mitre.rs` (technique IDs), `scoring.rs` (aggregate risk), `stix.rs` (STIX output).

---

## Module Layout

```
forensicnomicon/src/threat_intel/
  mod.rs             — re-exports; MalwareClass enum
  signals.rs         — signal ID constants + SignalMeta table
  profile.rs         — MalwareProfile, ProfileSignal, WeightedExclusion types
  engine.rs          — score_all_profiles(), score_against_profile()
  profiles/
    mod.rs            — ALL_PROFILES: &[&'static MalwareProfile]
    father.rs
    jynx.rs
    azazel.rs
    bdvl.rs
    libprocesshider.rs
    xmrig.rs
    lkm_generic.rs
    ld_preload_generic.rs  — capability-class profile (no family attribution)
```

Register in `forensicnomicon/src/lib.rs`:
```rust
pub mod threat_intel;
```

---

## 1. Signal Taxonomy (`threat_intel/signals.rs`)

Signals are atomic observable facts emitted by detection functions. A signal ID is a
dotted-namespace string constant. Detection code and profiles reference the same ID; the
compiler enforces no misspelling because both sides use the `const` from this module.

### Design rules
- One signal = one observable fact. If two things must both be true, the profile requires
  two signals — do not compound them into one signal.
- Signals carry no interpretation. `elf.hooks.pam_credential_theft` means "the ELF imports
  a PAM hook symbol". Whether that is malicious is the profile's job.
- Signals may carry metadata (e.g., the thread name that fired `process.thread.miner`),
  but the ID itself is always a `&'static str`.

### Signal ID constants

```rust
// ── ELF capability signals (from memf-linux::elf_analysis) ──────────────────
pub const ELF_HOOKS_PROCESS_HIDING:    &str = "elf.hooks.process_hiding";
pub const ELF_HOOKS_FILE_HIDING:       &str = "elf.hooks.file_hiding";
pub const ELF_HOOKS_PAM_CREDENTIAL:    &str = "elf.hooks.pam_credential_theft";
pub const ELF_HOOKS_NETWORK_HIDING:    &str = "elf.hooks.network_hiding";
pub const ELF_HOOKS_UID_SPOOFING:      &str = "elf.hooks.uid_spoofing";
pub const ELF_HOOKS_IO_INTERCEPTION:   &str = "elf.hooks.io_interception";
pub const ELF_GLOBALLY_LOADED:         &str = "elf.globally_loaded";
pub const ELF_NOT_IN_PKG_DB:           &str = "elf.not_in_package_db";
pub const ELF_LIBC_SHADOW_EXPORTS:     &str = "elf.libc_shadow_exports";
pub const ELF_STRING_FATHER_FORMAT:    &str = "elf.string.father_format";
pub const ELF_STRING_STAGING_PATH:     &str = "elf.string.staging_path";

// ── Artifact signals (from issen-parser-uac) ─────────────────────────────────
pub const ARTIFACT_PAM_STAGING_STRUCTURAL: &str = "artifact.pam_staging.uid_field_value_format";
pub const ARTIFACT_PAM_STAGING_FATHER:     &str = "artifact.pam_staging.father_exact_format";
pub const ARTIFACT_LD_PRELOAD_FOREIGN:     &str = "artifact.ld_preload.foreign_library";
pub const ARTIFACT_LD_PRELOAD_IN_HASH_LIST:&str = "artifact.ld_preload.path_in_hash_list";

// ── Process signals (from issen-parser-uac::parsers::mod, mem_sockstat) ──────
pub const PROCESS_HIDDEN_FROM_PS:      &str = "process.hidden_from_ps";
pub const PROCESS_THREAD_MINER_XMRIG:  &str = "process.thread.miner.xmrig";
pub const PROCESS_THREAD_MINER_GENERIC:&str = "process.thread.miner.generic";
pub const PROCESS_ANOMALOUS_THREAD_COUNT: &str = "process.anomalous_thread_count";
pub const PROCESS_MASQUERADE:          &str = "process.masquerade_as_system_proc";
pub const PROCESS_SHELL_UPGRADE_CHAIN: &str = "process.shell_upgrade_chain";

// ── Network signals ───────────────────────────────────────────────────────────
pub const NETWORK_STRATUM_CONNECTION:  &str = "network.stratum_connection";
pub const NETWORK_STRATUM_LISTEN:      &str = "network.stratum_listen";
pub const NETWORK_SSH_STRATUM_TUNNEL:  &str = "network.ssh_stratum_tunnel";
pub const NETWORK_REVERSE_SHELL:       &str = "network.reverse_shell";
pub const NETWORK_MAGIC_PACKET_KNOCK:  &str = "network.magic_packet_knock";

// ── System / OS signals ───────────────────────────────────────────────────────
pub const SYSTEM_KERNEL_TAINT_OOT:     &str = "system.kernel_taint.out_of_tree_module";
pub const SYSTEM_KERNEL_TAINT_FORCED:  &str = "system.kernel_taint.forced_load";
pub const SYSTEM_CPU_ANOMALY_HIGH:     &str = "system.cpu.anomaly_high";
pub const SYSTEM_LD_PRELOAD_SET:       &str = "system.ld_preload.set";
pub const SYSTEM_PROC_MODULES_SUSPECT: &str = "system.proc_modules.suspect_entry";

// ── Temporal / correlation signals (from issen-correlation rules) ─────────────
pub const TEMPORAL_LDPRELOAD_SSHD_RESTART: &str = "temporal.ld_preload_then_sshd_restart";
pub const TEMPORAL_ACTIVATION_SEQUENCE:    &str = "temporal.activation_sequence";
```

### Signal metadata table

```rust
pub struct SignalMeta {
    pub id: &'static str,
    pub description: &'static str,
    pub mitre_technique: Option<&'static str>,
}

pub const SIGNAL_META: &[SignalMeta] = &[
    SignalMeta { id: ELF_HOOKS_PROCESS_HIDING,    description: "ELF exports readdir64/getdents64 — hides PIDs from /proc",        mitre_technique: Some("T1014") },
    SignalMeta { id: ELF_HOOKS_FILE_HIDING,       description: "ELF exports stat/open/access — hides paths from filesystem calls",  mitre_technique: Some("T1564.001") },
    SignalMeta { id: ELF_HOOKS_PAM_CREDENTIAL,    description: "ELF exports pam_get_item/pam_authenticate — intercepts credentials",mitre_technique: Some("T1556.003") },
    SignalMeta { id: ELF_HOOKS_NETWORK_HIDING,    description: "ELF exports recvmsg/recvfrom — hides network connections",          mitre_technique: Some("T1014") },
    SignalMeta { id: ELF_HOOKS_UID_SPOOFING,      description: "ELF exports getuid/geteuid — spoofs UID in userspace",              mitre_technique: Some("T1548") },
    SignalMeta { id: ELF_HOOKS_IO_INTERCEPTION,   description: "ELF exports write — intercepts I/O for keystroke logging",          mitre_technique: Some("T1056.001") },
    SignalMeta { id: ELF_GLOBALLY_LOADED,         description: "Library loaded in ≥90% of running processes — LD_PRELOAD infection", mitre_technique: Some("T1574.006") },
    SignalMeta { id: ELF_NOT_IN_PKG_DB,           description: "Library path not owned by any installed package",                   mitre_technique: Some("T1574.006") },
    SignalMeta { id: ELF_LIBC_SHADOW_EXPORTS,     description: "Library exports symbols with same names as libc functions",         mitre_technique: Some("T1574.006") },
    SignalMeta { id: ELF_STRING_FATHER_FORMAT,    description: "ELF .rodata contains ':password:' format string fragment",          mitre_technique: Some("T1556.003") },
    SignalMeta { id: ELF_STRING_STAGING_PATH,     description: "ELF .rodata contains reference to a /tmp staging path",            mitre_technique: Some("T1074") },
    SignalMeta { id: ARTIFACT_PAM_STAGING_STRUCTURAL, description: "File with UID:counter:field:value format found in temp dir",   mitre_technique: Some("T1074") },
    SignalMeta { id: ARTIFACT_PAM_STAGING_FATHER, description: "File with exact Father format (UID:N:password:V) found",           mitre_technique: Some("T1556.003") },
    SignalMeta { id: ARTIFACT_LD_PRELOAD_FOREIGN, description: "ld.so.preload contains path not from package manager",             mitre_technique: Some("T1574.006") },
    SignalMeta { id: PROCESS_HIDDEN_FROM_PS,      description: "PID visible in /proc but absent from ps output",                   mitre_technique: Some("T1014") },
    SignalMeta { id: PROCESS_THREAD_MINER_XMRIG,  description: "Thread name matches XMRig worker pattern",                         mitre_technique: Some("T1496") },
    SignalMeta { id: PROCESS_THREAD_MINER_GENERIC,description: "Thread name matches generic crypto miner pattern",                  mitre_technique: Some("T1496") },
    SignalMeta { id: PROCESS_ANOMALOUS_THREAD_COUNT, description: "Single-threaded utility has unexpected thread pool",            mitre_technique: Some("T1055") },
    SignalMeta { id: PROCESS_MASQUERADE,          description: "Process name matches common system utility but behaviour differs",  mitre_technique: Some("T1036.005") },
    SignalMeta { id: PROCESS_SHELL_UPGRADE_CHAIN, description: "sh+interpreter+bash triple on shared external endpoint (pty.spawn)",mitre_technique: Some("T1059.006") },
    SignalMeta { id: NETWORK_STRATUM_CONNECTION,  description: "Connection to/from a known Stratum mining pool port",               mitre_technique: Some("T1496") },
    SignalMeta { id: NETWORK_STRATUM_LISTEN,      description: "Process listening on a Stratum port (local relay)",                 mitre_technique: Some("T1496") },
    SignalMeta { id: NETWORK_SSH_STRATUM_TUNNEL,  description: "SSH process forwarding Stratum traffic to external host",          mitre_technique: Some("T1572") },
    SignalMeta { id: NETWORK_REVERSE_SHELL,       description: "Process with outbound connection and interactive shell child",      mitre_technique: Some("T1059") },
    SignalMeta { id: NETWORK_MAGIC_PACKET_KNOCK,  description: "Unusual UDP sequence consistent with port knocking activation",     mitre_technique: Some("T1205.001") },
    SignalMeta { id: SYSTEM_KERNEL_TAINT_OOT,     description: "Kernel taint bit 2 set — out-of-tree module loaded",               mitre_technique: Some("T1215") },
    SignalMeta { id: SYSTEM_KERNEL_TAINT_FORCED,  description: "Kernel taint bit 4 set — module force-loaded",                     mitre_technique: Some("T1215") },
    SignalMeta { id: SYSTEM_CPU_ANOMALY_HIGH,     description: "CPU user time ≥90% sustained — likely stolen by hidden process",   mitre_technique: Some("T1496") },
    SignalMeta { id: SYSTEM_LD_PRELOAD_SET,       description: "/etc/ld.so.preload is non-empty",                                  mitre_technique: Some("T1574.006") },
    SignalMeta { id: SYSTEM_PROC_MODULES_SUSPECT, description: "/proc/modules contains a known-suspect module name",               mitre_technique: Some("T1215") },
    SignalMeta { id: TEMPORAL_LDPRELOAD_SSHD_RESTART, description: "ld.so.preload written then sshd restarted within 120 s",      mitre_technique: Some("T1574.006") },
];
```

---

## 2. Profile Types (`threat_intel/profile.rs`)

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MalwareClass {
    LdPreloadProcessHider,         // readdir64/getdents64 only
    LdPreloadPamHooker,            // process hiding + PAM credential theft
    LdPreloadNetworkHider,         // process hiding + network socket hiding
    LdPreloadFullRootkit,          // multiple capability dimensions
    LkmRootkit,                    // kernel module (not .so)
    CryptoMiner,                   // resource hijacking
    GenericLdPreload,              // preloaded but capability unclear
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Classification {
    NoMatch,       // score = 0 or required signal absent
    LowConfidence, // > 0 but below class_threshold
    ClassMatch,    // capability class confirmed; family unclear
    Probable,      // likely this family
    Confirmed,     // strong multi-signal attribution
}

pub struct ProfileSignal {
    /// Signal ID from `signals.rs`.
    pub id: &'static str,
    /// Contribution to total score when signal is present.
    pub weight: u32,
    /// If true, signal MUST be present for any score > 0.
    /// Absence of a required signal → `Classification::NoMatch`.
    pub required: bool,
}

pub struct WeightedExclusion {
    /// Signal ID that reduces score when present.
    pub id: &'static str,
    /// Amount subtracted from score.
    pub penalty: u32,
}

pub struct MalwareProfile {
    pub id: &'static str,
    pub family: &'static str,
    pub aliases: &'static [&'static str],
    pub description: &'static str,
    pub malware_class: MalwareClass,
    pub mitre_techniques: &'static [&'static str],
    /// Weighted positive signals.
    pub signals: &'static [ProfileSignal],
    /// Signals whose presence reduces confidence (differentiates from similar families).
    pub exclusions: &'static [WeightedExclusion],
    /// Thresholds (inclusive lower bounds).
    pub class_threshold:     u32,
    pub probable_threshold:  u32,
    pub confirmed_threshold: u32,
}
```

---

## 3. Scoring Engine (`threat_intel/engine.rs`)

```rust
use std::collections::HashSet;
use super::{profile::*, signals::*};

#[derive(Debug)]
pub struct DetectedSignal {
    pub id: &'static str,
    pub confidence: f32,    // 0.0–1.0; 1.0 = certain
    pub evidence: String,   // human-readable description for output
}

#[derive(Debug)]
pub struct FiredSignal {
    pub id: &'static str,
    pub weight: u32,
}

#[derive(Debug)]
pub struct ProfileMatch {
    pub profile: &'static MalwareProfile,
    pub score: u32,
    pub classification: Classification,
    pub fired: Vec<FiredSignal>,
    pub missed_required: Vec<&'static str>, // required signals not present (explains NoMatch)
}

pub fn score_against_profile(
    signals: &[DetectedSignal],
    profile: &'static MalwareProfile,
) -> ProfileMatch {
    let present: HashSet<&str> = signals.iter().map(|s| s.id).collect();

    // Required-signal gate
    let missed_required: Vec<&'static str> = profile.signals.iter()
        .filter(|ps| ps.required && !present.contains(ps.id))
        .map(|ps| ps.id)
        .collect();
    if !missed_required.is_empty() {
        return ProfileMatch {
            profile, score: 0, classification: Classification::NoMatch,
            fired: vec![], missed_required,
        };
    }

    // Weighted sum of present signals
    let mut fired = Vec::new();
    let raw_score: u32 = profile.signals.iter()
        .filter(|ps| present.contains(ps.id))
        .map(|ps| { fired.push(FiredSignal { id: ps.id, weight: ps.weight }); ps.weight })
        .sum();

    // Apply exclusion penalties
    let penalty: u32 = profile.exclusions.iter()
        .filter(|ex| present.contains(ex.id))
        .map(|ex| ex.penalty)
        .sum();
    let score = raw_score.saturating_sub(penalty);

    let classification = if score == 0 {
        Classification::LowConfidence
    } else if score >= profile.confirmed_threshold {
        Classification::Confirmed
    } else if score >= profile.probable_threshold {
        Classification::Probable
    } else if score >= profile.class_threshold {
        Classification::ClassMatch
    } else {
        Classification::LowConfidence
    };

    ProfileMatch { profile, score, classification, fired, missed_required: vec![] }
}

/// Score signals against every known profile. Returns all profiles with score > 0,
/// sorted descending by score.
pub fn score_all_profiles(signals: &[DetectedSignal]) -> Vec<ProfileMatch> {
    use super::profiles::ALL_PROFILES;
    let mut matches: Vec<ProfileMatch> = ALL_PROFILES.iter()
        .map(|p| score_against_profile(signals, p))
        .filter(|m| m.score > 0)
        .collect();
    matches.sort_by(|a, b| b.score.cmp(&a.score).then(a.profile.id.cmp(b.profile.id)));
    matches
}

/// Return the top ClassMatch-or-better result, if any.
pub fn top_match(signals: &[DetectedSignal]) -> Option<ProfileMatch> {
    score_all_profiles(signals)
        .into_iter()
        .find(|m| m.classification >= Classification::ClassMatch)
}
```

---

## 4. Malware Profiles (`threat_intel/profiles/`)

### `profiles/mod.rs`
```rust
mod father;          pub use father::FATHER;
mod jynx;            pub use jynx::JYNX;
mod azazel;          pub use azazel::AZAZEL;
mod bdvl;            pub use bdvl::BDVL;
mod libprocesshider; pub use libprocesshider::LIB_PROCESS_HIDER;
mod xmrig;           pub use xmrig::XMRIG;
mod lkm_generic;     pub use lkm_generic::LKM_GENERIC;
mod ld_preload_generic; pub use ld_preload_generic::LD_PRELOAD_GENERIC;

pub const ALL_PROFILES: &[&MalwareProfile] = &[
    &FATHER, &JYNX, &AZAZEL, &BDVL, &LIB_PROCESS_HIDER,
    &XMRIG, &LKM_GENERIC, &LD_PRELOAD_GENERIC,
];
```

### `profiles/father.rs`
```rust
use crate::threat_intel::{profile::*, signals::*};

pub static FATHER: MalwareProfile = MalwareProfile {
    id: "father",
    family: "Father",
    aliases: &["libymv", "father-rootkit"],
    description: "LD_PRELOAD rootkit: process hiding via readdir64 + PAM credential theft. \
                  Writes credentials to a staging file in /tmp (UID:N:password:V format).",
    malware_class: MalwareClass::LdPreloadPamHooker,
    mitre_techniques: &["T1574.006", "T1014", "T1556.003", "T1074"],
    signals: &[
        // Required — both must be present for any Father score
        ProfileSignal { id: ELF_HOOKS_PROCESS_HIDING, weight: 20, required: true  },
        ProfileSignal { id: ELF_HOOKS_PAM_CREDENTIAL, weight: 30, required: true  },
        // Bonus — Father-specific behavioral artifacts
        ProfileSignal { id: ARTIFACT_PAM_STAGING_STRUCTURAL, weight: 25, required: false },
        ProfileSignal { id: ARTIFACT_PAM_STAGING_FATHER,     weight: 15, required: false },
        ProfileSignal { id: ELF_STRING_FATHER_FORMAT,         weight: 10, required: false },
        ProfileSignal { id: ELF_STRING_STAGING_PATH,          weight:  5, required: false },
        ProfileSignal { id: ELF_GLOBALLY_LOADED,              weight: 10, required: false },
        ProfileSignal { id: ELF_NOT_IN_PKG_DB,                weight: 10, required: false },
        ProfileSignal { id: TEMPORAL_LDPRELOAD_SSHD_RESTART,  weight: 15, required: false },
    ],
    // bdvl and Azazel also have process hiding + PAM; strong network hiding tips toward bdvl
    exclusions: &[
        WeightedExclusion { id: ELF_HOOKS_NETWORK_HIDING, penalty: 20 },
    ],
    class_threshold:     50, // process hiding + PAM = Father-class capability
    probable_threshold:  75, // + staging artifact or ELF strings
    confirmed_threshold: 90, // + multiple corroborating signals
};
```

### `profiles/jynx.rs`
```rust
pub static JYNX: MalwareProfile = MalwareProfile {
    id: "jynx",
    family: "Jynx",
    aliases: &["jynx2", "jynxkit"],
    description: "LD_PRELOAD rootkit: process + file hiding. No PAM hooking. \
                  Simpler than Father — only hooks readdir and stat family.",
    malware_class: MalwareClass::LdPreloadProcessHider,
    mitre_techniques: &["T1574.006", "T1014", "T1564.001"],
    signals: &[
        ProfileSignal { id: ELF_HOOKS_PROCESS_HIDING, weight: 30, required: true  },
        ProfileSignal { id: ELF_HOOKS_FILE_HIDING,    weight: 30, required: true  },
        ProfileSignal { id: ELF_GLOBALLY_LOADED,      weight: 15, required: false },
        ProfileSignal { id: ELF_NOT_IN_PKG_DB,        weight: 10, required: false },
        ProfileSignal { id: ELF_LIBC_SHADOW_EXPORTS,  weight:  5, required: false },
    ],
    // PAM hooks → not Jynx; Jynx has no credential theft capability
    exclusions: &[
        WeightedExclusion { id: ELF_HOOKS_PAM_CREDENTIAL, penalty: 40 },
        WeightedExclusion { id: ARTIFACT_PAM_STAGING_STRUCTURAL, penalty: 30 },
    ],
    class_threshold:     45,
    probable_threshold:  60,
    confirmed_threshold: 75,
};
```

### `profiles/azazel.rs`
```rust
pub static AZAZEL: MalwareProfile = MalwareProfile {
    id: "azazel",
    family: "Azazel",
    aliases: &["azazel-rootkit"],
    description: "LD_PRELOAD rootkit similar to Father: process hiding + PAM credential theft. \
                  Distinguishable from Father by absence of the Father staging format and \
                  presence of magic-packet backdoor activation.",
    malware_class: MalwareClass::LdPreloadPamHooker,
    mitre_techniques: &["T1574.006", "T1014", "T1556.003", "T1205.001"],
    signals: &[
        ProfileSignal { id: ELF_HOOKS_PROCESS_HIDING,      weight: 20, required: true  },
        ProfileSignal { id: ELF_HOOKS_PAM_CREDENTIAL,      weight: 30, required: true  },
        ProfileSignal { id: ELF_GLOBALLY_LOADED,           weight: 10, required: false },
        ProfileSignal { id: ELF_NOT_IN_PKG_DB,             weight: 10, required: false },
        ProfileSignal { id: NETWORK_MAGIC_PACKET_KNOCK,    weight: 25, required: false },
    ],
    // Father's exact staging format tips away from Azazel
    exclusions: &[
        WeightedExclusion { id: ARTIFACT_PAM_STAGING_FATHER, penalty: 20 },
        WeightedExclusion { id: ELF_STRING_FATHER_FORMAT,    penalty: 15 },
    ],
    class_threshold:     50,
    probable_threshold:  70,
    confirmed_threshold: 85,
};
```

### `profiles/bdvl.rs`
```rust
pub static BDVL: MalwareProfile = MalwareProfile {
    id: "bdvl",
    family: "bdvl",
    aliases: &["bedevil"],
    description: "LD_PRELOAD rootkit specialising in network socket hiding. \
                  Hooks readdir64 (process) and recvmsg/recvfrom (network). \
                  No PAM credential theft. Provides a backdoor shell via magic packet.",
    malware_class: MalwareClass::LdPreloadNetworkHider,
    mitre_techniques: &["T1574.006", "T1014", "T1205.001"],
    signals: &[
        ProfileSignal { id: ELF_HOOKS_PROCESS_HIDING,  weight: 25, required: true  },
        ProfileSignal { id: ELF_HOOKS_NETWORK_HIDING,  weight: 30, required: true  },
        ProfileSignal { id: ELF_GLOBALLY_LOADED,       weight: 10, required: false },
        ProfileSignal { id: ELF_NOT_IN_PKG_DB,         weight: 10, required: false },
        ProfileSignal { id: NETWORK_MAGIC_PACKET_KNOCK,weight: 25, required: false },
    ],
    // PAM hooks → not bdvl; bdvl has no credential theft
    exclusions: &[
        WeightedExclusion { id: ELF_HOOKS_PAM_CREDENTIAL,    penalty: 40 },
        WeightedExclusion { id: ARTIFACT_PAM_STAGING_STRUCTURAL, penalty: 30 },
    ],
    class_threshold:     55,
    probable_threshold:  70,
    confirmed_threshold: 85,
};
```

### `profiles/libprocesshider.rs`
```rust
pub static LIB_PROCESS_HIDER: MalwareProfile = MalwareProfile {
    id: "libprocesshider",
    family: "libprocesshider",
    aliases: &["libshow.so", "libhide", "libproc.a"],
    description: "Minimal LD_PRELOAD rootkit: only hooks readdir/readdir64 to hide \
                  processes by PID prefix. No file hiding, no PAM, no network hiding. \
                  Very small library; commonly used as a base by custom implants.",
    malware_class: MalwareClass::LdPreloadProcessHider,
    mitre_techniques: &["T1574.006", "T1014"],
    signals: &[
        ProfileSignal { id: ELF_HOOKS_PROCESS_HIDING, weight: 30, required: true  },
        ProfileSignal { id: ELF_GLOBALLY_LOADED,      weight: 15, required: false },
        ProfileSignal { id: ELF_NOT_IN_PKG_DB,        weight: 10, required: false },
    ],
    // Any additional capability tips away from this minimal family
    exclusions: &[
        WeightedExclusion { id: ELF_HOOKS_PAM_CREDENTIAL, penalty: 50 },
        WeightedExclusion { id: ELF_HOOKS_FILE_HIDING,    penalty: 30 },
        WeightedExclusion { id: ELF_HOOKS_NETWORK_HIDING, penalty: 40 },
    ],
    class_threshold:     30,
    probable_threshold:  45,
    confirmed_threshold: 55,
};
```

### `profiles/xmrig.rs`
```rust
pub static XMRIG: MalwareProfile = MalwareProfile {
    id: "xmrig",
    family: "XMRig",
    aliases: &["xmr-stak", "cryptonight-miner"],
    description: "Open-source Monero/crypto miner. Often deployed by rootkit-concealed \
                  threat actors. Identified by libuv worker thread names and Stratum \
                  protocol connections. Frequently disguised with a system process name.",
    malware_class: MalwareClass::CryptoMiner,
    mitre_techniques: &["T1496", "T1036.005"],
    signals: &[
        // At least one miner thread signal required
        ProfileSignal { id: PROCESS_THREAD_MINER_XMRIG,   weight: 40, required: false },
        ProfileSignal { id: PROCESS_THREAD_MINER_GENERIC, weight: 20, required: false },
        ProfileSignal { id: NETWORK_STRATUM_CONNECTION,   weight: 30, required: false },
        ProfileSignal { id: NETWORK_STRATUM_LISTEN,       weight: 20, required: false },
        ProfileSignal { id: SYSTEM_CPU_ANOMALY_HIGH,      weight: 15, required: false },
        ProfileSignal { id: PROCESS_MASQUERADE,           weight: 10, required: false },
        ProfileSignal { id: PROCESS_ANOMALOUS_THREAD_COUNT, weight: 15, required: false },
        ProfileSignal { id: PROCESS_HIDDEN_FROM_PS,       weight: 20, required: false },
    ],
    exclusions: &[],
    class_threshold:     30,  // any one strong miner signal
    probable_threshold:  55,
    confirmed_threshold: 75,
};
```

### `profiles/lkm_generic.rs`
```rust
pub static LKM_GENERIC: MalwareProfile = MalwareProfile {
    id: "lkm_generic",
    family: "LKM Rootkit",
    aliases: &["diamorphine", "reptile", "suterusu", "knark", "adore"],
    description: "Loadable Kernel Module rootkit. Sets kernel taint bits. May hide \
                  itself from /proc/modules. Includes Diamorphine, Reptile, Suterusu, \
                  and similar kernel-space implants.",
    malware_class: MalwareClass::LkmRootkit,
    mitre_techniques: &["T1215", "T1014"],
    signals: &[
        ProfileSignal { id: SYSTEM_KERNEL_TAINT_OOT,     weight: 30, required: true  },
        ProfileSignal { id: SYSTEM_PROC_MODULES_SUSPECT, weight: 40, required: false },
        ProfileSignal { id: PROCESS_HIDDEN_FROM_PS,      weight: 20, required: false },
        ProfileSignal { id: SYSTEM_KERNEL_TAINT_FORCED,  weight: 15, required: false },
    ],
    exclusions: &[],
    class_threshold:     30,
    probable_threshold:  55,
    confirmed_threshold: 75,
};
```

### `profiles/ld_preload_generic.rs`
```rust
/// Catch-all profile for LD_PRELOAD preloaded libraries that don't match a known
/// family. Confirms capability class even when no family attribution is possible.
pub static LD_PRELOAD_GENERIC: MalwareProfile = MalwareProfile {
    id: "ld_preload_generic",
    family: "Unknown LD_PRELOAD Rootkit",
    aliases: &[],
    description: "An LD_PRELOAD library with rootkit-class capability hooks that does \
                  not match any known family profile. Novel or heavily modified variant.",
    malware_class: MalwareClass::GenericLdPreload,
    mitre_techniques: &["T1574.006", "T1014"],
    signals: &[
        ProfileSignal { id: ELF_HOOKS_PROCESS_HIDING, weight: 30, required: false },
        ProfileSignal { id: ELF_HOOKS_PAM_CREDENTIAL, weight: 30, required: false },
        ProfileSignal { id: ELF_HOOKS_NETWORK_HIDING, weight: 25, required: false },
        ProfileSignal { id: ELF_HOOKS_FILE_HIDING,    weight: 20, required: false },
        ProfileSignal { id: ELF_GLOBALLY_LOADED,      weight: 20, required: true  },
        ProfileSignal { id: ELF_NOT_IN_PKG_DB,        weight: 10, required: false },
        ProfileSignal { id: ARTIFACT_LD_PRELOAD_FOREIGN, weight: 15, required: false },
    ],
    exclusions: &[],
    class_threshold:     30, // globally-loaded + any hook = generic rootkit-class
    probable_threshold:  60,
    confirmed_threshold: 80,
};
```

---

## 5. Runtime Profile Loading (Follow-on Sprint)

Compile-time profiles cover known families. For field-added profiles (threat intel feeds,
incident-specific indicators), add a YAML loader in `issen-cli` (not `forensicnomicon`,
which stays zero-dependency):

```yaml
# Example runtime profile: ~/.issen/profiles/novel_rootkit.yml
id: "novel_rootkit_2026"
family: "NovelRootkit"
description: "LD_PRELOAD rootkit seen in Q2 2026 campaign"
malware_class: "ld_preload_pam_hooker"
mitre_techniques: ["T1574.006", "T1556.003"]
signals:
  - id: "elf.hooks.process_hiding"
    weight: 20
    required: true
  - id: "artifact.pam_staging.uid_field_value_format"
    weight: 30
    required: false
class_threshold: 50
probable_threshold: 70
confirmed_threshold: 85
```

The YAML profile is deserialized into a heap-allocated `MalwareProfile` equivalent and
passed to the same `score_against_profile()` engine. No special handling required.

---

## 6. Linux-Specific Constants (from previous plan, integrated here)

### Hook Symbol Table → Signal Mapping

`heuristics/linux_rootkit.rs` maps ELF imported/exported symbol names to signal IDs:

```rust
pub struct HookSymbol {
    pub name: &'static str,
    pub emits_signal: &'static str,  // signal ID from threat_intel/signals.rs
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
```

### Other constants from previous plan (unchanged)

`SHELL_UPGRADE_INTERPRETERS`, `MINER_THREAD_PATTERNS`, `STRATUM_PORTS`,
`PAM_STAGING_LINE_PATTERN`, `FATHER_CLASS_ELF_PATTERNS`, `SINGLE_THREADED_PROCESSES`
— all remain as documented in the previous revision of this file. They are now used
by signal-emitting functions rather than by ad-hoc scoring code.

---

## TDD Tests

### `threat_intel/engine.rs`
```rust
fn score_zero_signals_returns_no_match_for_all_profiles()
fn score_father_required_signals_returns_father_class_match()
fn score_father_full_signals_returns_father_confirmed()
fn score_jynx_with_pam_signal_excluded_by_penalty()
fn score_bdvl_required_signals_returns_bdvl_class_match()
fn score_xmrig_thread_signal_returns_xmrig_class_match()
fn score_all_profiles_sorted_descending_by_score()
fn score_all_profiles_filters_zero_score()
fn top_match_returns_highest_classification()
fn score_missing_required_signal_is_no_match()
fn score_exclusion_reduces_score()
fn score_exclusion_cannot_exceed_raw_score() // saturating_sub
```

### `threat_intel/profiles/*.rs`
```rust
// father.rs
fn father_process_hiding_plus_pam_reaches_class_threshold()
fn father_with_staging_file_reaches_probable_threshold()
fn father_with_all_signals_reaches_confirmed_threshold()
fn father_network_hiding_signal_reduces_score()
fn father_requires_both_process_hiding_and_pam()

// jynx.rs
fn jynx_process_and_file_hiding_reaches_class_threshold()
fn jynx_pam_signal_strongly_excluded()
fn jynx_without_file_hiding_is_no_match()

// bdvl.rs
fn bdvl_process_and_network_hiding_reaches_class_threshold()
fn bdvl_pam_signal_strongly_excluded()

// xmrig.rs
fn xmrig_thread_signal_alone_reaches_class_threshold()
fn xmrig_thread_plus_stratum_reaches_probable()

// ld_preload_generic.rs
fn generic_catches_novel_library_with_unknown_hooks()
fn generic_requires_globally_loaded_signal()
```

---

## Implementation Order

| Priority | Item | Effort |
|----------|------|--------|
| 1 | `threat_intel/signals.rs` — signal ID constants | XS |
| 2 | `threat_intel/profile.rs` — types | XS |
| 3 | `threat_intel/engine.rs` — scoring engine | S |
| 4 | `heuristics/linux_rootkit.rs` — hook symbol table emitting signal IDs | XS |
| 5 | `profiles/father.rs`, `jynx.rs`, `bdvl.rs`, `libprocesshider.rs` | S |
| 6 | `profiles/xmrig.rs`, `lkm_generic.rs`, `ld_preload_generic.rs` | S |
| 7 | `profiles/azazel.rs` + tests for all profiles | S |
| 8 | Runtime YAML profile loader (in `issen-cli`, not here) | M |

Items 1–4 must land before anything in `issen-parser-uac` or `memf-linux` can emit signals.
Items 5–7 are independent of each other and can be parallelised.
