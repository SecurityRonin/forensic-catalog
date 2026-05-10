# PLAN: Batch I Catalog Expansion — 19 New Artifact Descriptors

## Pre-flight: What Already Exists (DO NOT DUPLICATE)

The following were originally scoped but **already exist in the catalog** — skip them:

| Static Name | Existing ID | Location |
|---|---|---|
| SETUPAPI_DEV_LOG | `setupapi_dev_log` | `mod.rs` |
| EVTX_TASK_SCHEDULER | `evtx_task_scheduler` | `windows_evtx_ext.rs` |
| EVTX_BITS_CLIENT | `evtx_bits_client` | `windows_evtx_ext.rs` |
| EVTX_APPLOCKER (+SCRIPT) | `evtx_applocker` | `windows_evtx_ext.rs` |
| EVTX_DEFENDER | `evtx_defender` | `windows_evtx_ext.rs` |
| EVTX_FIREWALL | `evtx_firewall` | `windows_evtx_ext.rs` |
| EVTX_WMI_ACTIVITY | `evtx_wmi_activity` | `windows_evtx_ext.rs` |
| LINUX_PROC_MODULES | `linux_proc_modules` | `linux_ext.rs` |
| LINUX_SECURE_LOG | `linux_secure_log` | `linux_ext.rs` |
| LINUX_JOURNAL_DIR | `linux_journal_dir` | `mod.rs` (= LINUX_SYSTEMD_JOURNAL) |
| WER_REPORTS | `wer_reports` | `windows_files_ext.rs` |
| SUM_DB | `sum_db` | `windows_files_ext.rs` |

## Artifacts to Add (19 total)

### Group A — Windows Plaintext Logs (4 new, add to `windows_files_ext.rs`)

| # | Static | ID | File Path | Type | Priority |
|---|---|---|---|---|---|
| 1 | CBS_LOG | `cbs_log` | `%SystemRoot%\Logs\CBS\CBS.log` | File | Medium |
| 2 | PFRO_LOG | `pfro_log` | `%SystemRoot%\PFRO.log` | File | High |
| 3 | SETUPERR_LOG | `setuperr_log` | `%SystemRoot%\setuperr.log` | File | Low |
| 4 | SETUPAPI_UPGRADE_LOG | `setupapi_upgrade_log` | `%SystemRoot%\inf\setupapi.upgrade.log` | File | Low |

### Group B — Windows Error Reporting Split (2 new, add to `windows_files_ext.rs`)

| # | Static | ID | File Path | Type | Priority |
|---|---|---|---|---|---|
| 5 | WER_REPORTS_USER | `wer_reports_user` | `%APPDATA%\Microsoft\Windows\WER\ReportArchive\` | Directory | High |
| 6 | WER_REPORTS_SYSTEM | `wer_reports_system` | `%ProgramData%\Microsoft\Windows\WER\ReportArchive\` | Directory | High |

### Group C — EVTX Channels (3 new, add to `windows_evtx_ext.rs`)

| # | Static | ID | File Path | Priority |
|---|---|---|---|---|
| 7 | EVTX_APPLICATION | `evtx_application` | `%SystemRoot%\System32\winevt\Logs\Application.evtx` | Medium |
| 8 | EVTX_DNS_CLIENT | `evtx_dns_client` | `Microsoft-Windows-DNS-Client%4Operational.evtx` | High |
| 9 | EVTX_TERMINAL_SERVICES | `evtx_terminal_services` | `Microsoft-Windows-TerminalServices-LocalSessionManager%4Operational.evtx` | High |

### Group D — Linux Kernel/Proc (5 new, add to `linux_ext.rs`)

| # | Static | ID | File Path | OsScope | Priority |
|---|---|---|---|---|---|
| 10 | LINUX_KERN_LOG | `linux_kern_log` | `/var/log/kern.log` | LinuxDebian | High |
| 11 | LINUX_DMESG | `linux_dmesg` | `/var/log/dmesg` | Linux | High |
| 12 | LINUX_PROC_NET_TCP | `linux_proc_net_tcp` | `/proc/net/tcp` | Linux | Critical |
| 13 | LINUX_PROC_NET_UNIX | `linux_proc_net_unix` | `/proc/net/unix` | Linux | High |
| 14 | LINUX_BOOT_LOG | `linux_boot_log` | `/var/log/boot.log` | Linux | High |

### Group E — Linux Auth/Binary Logs (1 new, add to `linux_ext.rs`)

| # | Static | ID | File Path | Priority |
|---|---|---|---|---|
| 15 | LINUX_FAILLOG | `linux_faillog` | `/var/log/faillog` | Medium |

### Group F — Windows AppX/Modern App (2 new, add to `windows_files_ext.rs`)

| # | Static | ID | File Path | Priority |
|---|---|---|---|---|
| 16 | APPX_PACKAGES_USER | `appx_packages_user` | `%LocalAppData%\Packages\` | Low |
| 17 | APPX_INSTALL_LOG | `appx_install_log` | `%SystemRoot%\Logs\DISM\dism.log` | Medium |

### Group G — Windows Diagnostic/Telemetry (2 new, add to `windows_files_ext.rs`)

| # | Static | ID | File Path | Priority |
|---|---|---|---|---|
| 18 | DIAGNOSTIC_DATA_DIR | `diagnostic_data_dir` | `%ProgramData%\Microsoft\Diagnosis\ETLLogs\` | Low |
| 19 | WINDOWS_UPDATE_SESSION | `windows_update_session` | `%SystemRoot%\SoftwareDistribution\ReportingEvents.log` | Medium |

---

## Implementation Steps (Strict TDD)

### Step 0 — Environment Setup

```bash
cd ~/src/forensicnomicon
gitsign credential-cache start 2>/dev/null
export GITSIGN_CREDENTIAL_CACHE="$HOME/Library/Caches/sigstore/gitsign/cache.sock"
```

### Step 1 — Research (BEFORE writing any code)

Use WebSearch + WebFetch for each artifact. Required per artifact:
- Exact file path with Windows env var expansion
- File format (encoding, delimiter, timestamp format)
- MITRE ATT&CK technique IDs
- At least 1 authoritative source URL (Microsoft Learn, SANS, AboutDFIR, ForensicsWiki, KAPE GitHub)
- Forensic significance (what does presence/absence prove?)

Key research queries:
- `"CBS.log" forensics format fields Windows Update`
- `"PFRO.log" PendingFileRenameOperations forensics MoveFileEx`
- `"setupapi.upgrade.log" Windows in-place upgrade driver`
- `"setuperr.log" Windows setup error format`
- `WER ReportArchive forensics crash report .wer format`
- `Application.evtx EventID 1000 1026 forensics`
- `"DNS-Client" Operational evtx EventID 3008 forensics`
- `TerminalServices-LocalSessionManager EventID 21 23 25 forensics`
- `"/var/log/kern.log" forensics rootkit taint`
- `"/var/log/dmesg" forensics kernel ring buffer`
- `"/proc/net/tcp" forensics rootkit hiding`
- `"/proc/net/unix" forensics XMRig unix socket`
- `"/var/log/boot.log" forensics timeline`
- `"/var/log/faillog" binary format UID`
- `Windows Store Packages LocalAppData forensics`
- `DISM.log forensics LOLBin`
- `ETLLogs Diagnosis telemetry forensics`
- `ReportingEvents.log Windows Update forensics`

### Step 2 — RED Commit (Failing Tests)

Edit `src/catalog/tests.rs`. Add a new test module after the last existing `#[cfg(test)] mod`:

```rust
#[cfg(test)]
mod batch_i_catalog_presence {
    use super::*;

    #[test]
    fn catalog_has_cbs_log() {
        assert!(crate::catalog::CATALOG.by_id("cbs_log").is_some(),
            "CBS_LOG missing from catalog");
    }

    #[test]
    fn catalog_has_pfro_log() {
        assert!(crate::catalog::CATALOG.by_id("pfro_log").is_some(),
            "PFRO_LOG missing from catalog");
    }

    #[test]
    fn catalog_has_setuperr_log() {
        assert!(crate::catalog::CATALOG.by_id("setuperr_log").is_some(),
            "SETUPERR_LOG missing from catalog");
    }

    #[test]
    fn catalog_has_setupapi_upgrade_log() {
        assert!(crate::catalog::CATALOG.by_id("setupapi_upgrade_log").is_some(),
            "SETUPAPI_UPGRADE_LOG missing from catalog");
    }

    #[test]
    fn catalog_has_wer_reports_user() {
        assert!(crate::catalog::CATALOG.by_id("wer_reports_user").is_some(),
            "WER_REPORTS_USER missing from catalog");
    }

    #[test]
    fn catalog_has_wer_reports_system() {
        assert!(crate::catalog::CATALOG.by_id("wer_reports_system").is_some(),
            "WER_REPORTS_SYSTEM missing from catalog");
    }

    #[test]
    fn catalog_has_evtx_application() {
        assert!(crate::catalog::CATALOG.by_id("evtx_application").is_some(),
            "EVTX_APPLICATION missing from catalog");
    }

    #[test]
    fn catalog_has_evtx_dns_client() {
        assert!(crate::catalog::CATALOG.by_id("evtx_dns_client").is_some(),
            "EVTX_DNS_CLIENT missing from catalog");
    }

    #[test]
    fn catalog_has_evtx_terminal_services() {
        assert!(crate::catalog::CATALOG.by_id("evtx_terminal_services").is_some(),
            "EVTX_TERMINAL_SERVICES missing from catalog");
    }

    #[test]
    fn catalog_has_linux_kern_log() {
        assert!(crate::catalog::CATALOG.by_id("linux_kern_log").is_some(),
            "LINUX_KERN_LOG missing from catalog");
    }

    #[test]
    fn catalog_has_linux_dmesg() {
        assert!(crate::catalog::CATALOG.by_id("linux_dmesg").is_some(),
            "LINUX_DMESG missing from catalog");
    }

    #[test]
    fn catalog_has_linux_proc_net_tcp() {
        assert!(crate::catalog::CATALOG.by_id("linux_proc_net_tcp").is_some(),
            "LINUX_PROC_NET_TCP missing from catalog");
    }

    #[test]
    fn catalog_has_linux_proc_net_unix() {
        assert!(crate::catalog::CATALOG.by_id("linux_proc_net_unix").is_some(),
            "LINUX_PROC_NET_UNIX missing from catalog");
    }

    #[test]
    fn catalog_has_linux_boot_log() {
        assert!(crate::catalog::CATALOG.by_id("linux_boot_log").is_some(),
            "LINUX_BOOT_LOG missing from catalog");
    }

    #[test]
    fn catalog_has_linux_faillog() {
        assert!(crate::catalog::CATALOG.by_id("linux_faillog").is_some(),
            "LINUX_FAILLOG missing from catalog");
    }

    #[test]
    fn catalog_has_appx_packages_user() {
        assert!(crate::catalog::CATALOG.by_id("appx_packages_user").is_some(),
            "APPX_PACKAGES_USER missing from catalog");
    }

    #[test]
    fn catalog_has_appx_install_log() {
        assert!(crate::catalog::CATALOG.by_id("appx_install_log").is_some(),
            "APPX_INSTALL_LOG missing from catalog");
    }

    #[test]
    fn catalog_has_diagnostic_data_dir() {
        assert!(crate::catalog::CATALOG.by_id("diagnostic_data_dir").is_some(),
            "DIAGNOSTIC_DATA_DIR missing from catalog");
    }

    #[test]
    fn catalog_has_windows_update_session() {
        assert!(crate::catalog::CATALOG.by_id("windows_update_session").is_some(),
            "WINDOWS_UPDATE_SESSION missing from catalog");
    }
}
```

Also update the 4 existing `CATALOG.list().len()` assertions from `6554` to `6573` (6554 + 19).

Verify RED:
```bash
cargo test -p forensicnomicon 2>&1 | grep -c FAILED
# Expected: 19
```

Commit:
```bash
git add src/catalog/tests.rs
git commit -m "test(catalog): RED — 19 failing tests for batch I artifacts (Windows logs/WER/EVTX/Linux kernel)"
```

### Step 3 — GREEN Commit (Implementations)

#### 3a. Add descriptors to `windows_files_ext.rs`

Append 8 new statics at end of file (Groups A, B, F, G):
- `CBS_LOG`, `PFRO_LOG`, `SETUPERR_LOG`, `SETUPAPI_UPGRADE_LOG`
- `WER_REPORTS_USER`, `WER_REPORTS_SYSTEM`
- `APPX_PACKAGES_USER`, `APPX_INSTALL_LOG`
- `DIAGNOSTIC_DATA_DIR`, `WINDOWS_UPDATE_SESSION`

Pattern for each (example CBS_LOG):
```rust
pub(crate) static CBS_LOG: ArtifactDescriptor = ArtifactDescriptor {
    id: "cbs_log",
    name: "CBS.log (Component Based Servicing)",
    artifact_type: ArtifactType::File,
    hive: None,
    key_path: "",
    value_name: None,
    file_path: Some(r"%SystemRoot%\Logs\CBS\CBS.log"),
    scope: DataScope::System,
    os_scope: OsScope::All,
    decoder: Decoder::Identity,
    meaning: "...", // Fill from research
    mitre_techniques: &["T1218", "T1562.001"],
    fields: &[...], // Fill from research
    retention: Some("Rotates to CBS.persist.log when exceeds ~50 MB"),
    triage_priority: TriagePriority::Medium,
    related_artifacts: &["windows_update_session"],
    sources: &["https://..."], // Fill from research
};
```

Key rules per descriptor:
- **File artifacts**: `hive: None`, `key_path: ""`, `value_name: None`, populate `file_path`
- **Directory artifacts**: `ArtifactType::Directory`
- **EVTX artifacts**: `ArtifactType::EventLog`, `Decoder::Identity`
- **Linux volatile proc**: note volatility in `meaning` field
- Every `FieldSchema` needs `is_uid_component` and `value_type`
- At least 1 `https://` source URL per descriptor

#### 3b. Add EVTX descriptors to `windows_evtx_ext.rs`

Append 3 new statics (Group C):
- `EVTX_APPLICATION`, `EVTX_DNS_CLIENT`, `EVTX_TERMINAL_SERVICES`

#### 3c. Add Linux descriptors to `linux_ext.rs`

Append 6 new statics (Groups D, E):
- `LINUX_KERN_LOG`, `LINUX_DMESG`, `LINUX_PROC_NET_TCP`, `LINUX_PROC_NET_UNIX`, `LINUX_BOOT_LOG`, `LINUX_FAILLOG`

#### 3d. Register in CATALOG_ENTRIES (`mod.rs`)

Add entries to the `CATALOG_ENTRIES` slice. Insert at the appropriate ext module sections:

After the last `windows_files_ext::` entry (line ~6887):
```rust
    windows_files_ext::CBS_LOG,
    windows_files_ext::PFRO_LOG,
    windows_files_ext::SETUPERR_LOG,
    windows_files_ext::SETUPAPI_UPGRADE_LOG,
    windows_files_ext::WER_REPORTS_USER,
    windows_files_ext::WER_REPORTS_SYSTEM,
    windows_files_ext::APPX_PACKAGES_USER,
    windows_files_ext::APPX_INSTALL_LOG,
    windows_files_ext::DIAGNOSTIC_DATA_DIR,
    windows_files_ext::WINDOWS_UPDATE_SESSION,
```

After the last `windows_evtx_ext::` entry (currently line ~6766, after EVTX_POWERSHELL_CLASSIC):
```rust
    windows_evtx_ext::EVTX_APPLICATION,
    windows_evtx_ext::EVTX_DNS_CLIENT,
    windows_evtx_ext::EVTX_TERMINAL_SERVICES,
```

After the last `linux_ext::` entry:
```rust
    linux_ext::LINUX_KERN_LOG,
    linux_ext::LINUX_DMESG,
    linux_ext::LINUX_PROC_NET_TCP,
    linux_ext::LINUX_PROC_NET_UNIX,
    linux_ext::LINUX_BOOT_LOG,
    linux_ext::LINUX_FAILLOG,
```

#### 3e. Verify

```bash
cargo check -p forensicnomicon   # Compile check first
cargo test -p forensicnomicon    # All tests pass
cargo test -p forensicnomicon 2>&1 | grep -c FAILED
# Expected: 0
```

Commit:
```bash
git add src/catalog/descriptors/mod.rs src/catalog/descriptors/windows_files_ext.rs \
        src/catalog/descriptors/windows_evtx_ext.rs src/catalog/descriptors/linux_ext.rs
git commit -m "feat(catalog): GREEN — 19 new artifact descriptors (Windows logs/WER/EVTX/Linux kernel)"
```

---

## Artifact Research Cheat Sheet

Below is the minimum forensic detail needed per artifact. **Research each via WebSearch before implementing.**

### CBS_LOG
- Format: UTF-8 text, timestamped lines `YYYY-MM-DD HH:MM:SS, <Type> <Component> <Message>`
- Shows KB numbers installed/failed, component store corruption repair
- MITRE: T1218 (LOLBin installs via update), T1562.001 (patch removal to weaken defenses)
- Sources: Microsoft Learn CBS log article, SANS Windows forensics

### PFRO_LOG
- Format: UTF-8/UTF-16 text, lists source→destination file renames pending reboot
- Created by `MoveFileEx` with `MOVEFILE_DELAY_UNTIL_REBOOT` — used by malware for staged deletion
- Presence of this file is itself an indicator; examine entries for suspicious paths
- MITRE: T1036.003 (Rename System Utilities), T1070.004 (File Deletion)
- Sources: Microsoft MoveFileEx docs, SANS DFIR

### SETUPERR_LOG
- Format: same as setupapi.dev.log but only errors
- Shows driver/hardware failures during Windows install — useful for timeline of OS deployment
- MITRE: none directly; informational
- Sources: Microsoft Learn setuperr.log

### SETUPAPI_UPGRADE_LOG
- Format: same as setupapi.dev.log
- Records driver migration during Windows in-place upgrade
- MITRE: T1200 (if driver issues during upgrade reveal hardware additions)
- Sources: Microsoft Learn

### WER_REPORTS_USER / WER_REPORTS_SYSTEM
- Directory containing `Report.wer` files (INI-like format)
- Fields: EventType, AppName, AppPath, AppVersion, ExceptionCode, ModuleName
- Crash reports reveal: malware crashes (unusual paths), exploitation (0xc0000005 access violations), injected DLLs
- User scope vs System scope
- MITRE: T1059 (script crashes), T1055 (injection failures)
- Sources: Microsoft WER docs, Harlan Carvey Windows Forensic Analysis

### EVTX_APPLICATION
- Standard Windows event log channel
- Key EventIDs: 1000 (app crash), 1001 (WER), 1026 (.NET runtime error), 11707/11708 (MSI install success/fail)
- MITRE: T1218 (LOLBin evidence), T1059.007 (script execution evidence)
- Sources: Microsoft event documentation, SANS Windows Forensics poster

### EVTX_DNS_CLIENT
- Win10+ only, DISABLED by default (must be enabled via GPO or auditpol)
- EventID 3008: DNS query sent; 3020: DNS response received
- Reveals C2 domain lookups even without network capture
- MITRE: T1071.004 (DNS protocol for C2)
- Sources: Microsoft DNS client event docs, Palantir Windows DNS logging

### EVTX_TERMINAL_SERVICES
- `TerminalServices-LocalSessionManager/Operational`
- EventID 21=logon, 23=logoff, 24=disconnect, 25=reconnect
- Source IP in EventData — RDP lateral movement destination artifact
- MITRE: T1021.001 (Remote Desktop Protocol)
- Sources: 13cubed RDP forensics, SANS FOR508

### LINUX_KERN_LOG
- Debian/Ubuntu: `/var/log/kern.log`; RHEL: kernel messages go to `/var/log/messages`
- Contains kernel module load/unload, taint bit changes (value 4 = unsigned module = rootkit indicator)
- Father rootkit case: kernel taint message at module load time
- MITRE: T1014, T1547.006
- OsScope: LinuxDebian (RHEL equivalent is linux_messages_log which already exists)
- Sources: kernel.org documentation

### LINUX_DMESG
- `/var/log/dmesg` or captured via `dmesg` command
- Boot-time kernel ring buffer — non-persistent on most systems (cleared at boot)
- Contains hardware init, module load at boot, taint flags
- MITRE: T1014, T1547.006
- Sources: man dmesg, SANS Linux forensics

### LINUX_PROC_NET_TCP
- `/proc/net/tcp` and `/proc/net/tcp6`
- Kernel-level network connection table (hex-encoded local/remote addr:port, state, inode)
- Rootkits modify this to hide connections — discrepancy with `ss`/`netstat` = rootkit
- VOLATILE: must capture from live system
- MITRE: T1014 (rootkit network hiding)
- Sources: kernel.org /proc/net/tcp documentation

### LINUX_PROC_NET_UNIX
- `/proc/net/unix`
- Lists Unix domain sockets with path, type, state, inode
- XMRig and other miners use Unix sockets for local API; C2 implants use them for IPC
- VOLATILE
- MITRE: T1071 (local IPC for C2)
- Sources: kernel.org proc(5) man page

### LINUX_BOOT_LOG
- `/var/log/boot.log`
- Captured by `bootlogd` or `plymouth` — records service start/stop during boot
- Critical for rootkit timeline: boot.log timestamp vs file modification timestamps reveals deployment window
- MITRE: T1014 (temporal anchor for rootkit analysis)
- Sources: SANS Linux forensics

### LINUX_FAILLOG
- `/var/log/faillog`
- Binary format: fixed-size records indexed by UID (struct faillog: fail_cnt, fail_max, fail_locktime)
- Read with `faillog -a` command
- Brute force indicator: high fail_cnt values
- MITRE: T1110 (Brute Force)
- Sources: man faillog(5), SANS

### APPX_PACKAGES_USER
- `%LocalAppData%\Packages\` — each UWP app gets a subdirectory
- Contains LocalCache, LocalState, Settings, AC (AppContainer)
- Forensic value: app-specific data including caches, databases, settings
- MITRE: T1059.007 (JavaScript in packaged apps), T1036 (masquerading via store names)
- Sources: Microsoft UWP app data docs

### APPX_INSTALL_LOG (DISM)
- `%SystemRoot%\Logs\DISM\dism.log`
- Records DISM operations: feature enable/disable, package install, image servicing
- Shows LOLBin installs (e.g., enabling .NET features, WSL)
- MITRE: T1218 (system binary proxy execution), T1562.001 (feature manipulation)
- Sources: Microsoft DISM log docs

### DIAGNOSTIC_DATA_DIR
- `%ProgramData%\Microsoft\Diagnosis\ETLLogs\`
- Contains Windows telemetry ETL (Event Trace Log) files
- AutoLogger, ShutdownLogger, DiagTrack subdirectories
- Low priority but can contain app usage, connectivity, crash data
- MITRE: T1005 (data from local system, if exfiltrated)
- Sources: Microsoft diagnostic data docs

### WINDOWS_UPDATE_SESSION
- `%SystemRoot%\SoftwareDistribution\ReportingEvents.log`
- Tab-delimited: timestamp, agent, status, update title, KB number
- Correlate patch state with compromise timeline — was system unpatched at time of exploitation?
- MITRE: T1190 (exploit public-facing app — unpatched), T1562.001 (update suppression)
- Sources: Microsoft Windows Update log docs

---

## Critical Reminders

1. **Start gitsign credential cache** before committing
2. **Update the 4 `CATALOG.list().len()` assertions** from `6554` → `6573`
3. **`related_artifacts` must reference existing IDs** — the `all_related_artifacts_exist` test enforces this
4. **All sources must be `https://` URLs** — enforced by `all_sources_are_https_urls` test
5. **MITRE IDs must match `T\d{4}(.\d{3})?` pattern** — enforced by `all_mitre_ids_match_pattern` test
6. **`cargo check -p forensicnomicon`** before committing GREEN to catch compile errors
7. **Do NOT re-add** any of the 12 artifacts listed in the "Already Exists" table
8. For `related_artifacts`, only reference IDs that exist. Safe references include:
   - `setupapi_dev_log`, `wer_reports`, `sum_db`, `linux_proc_modules`, `linux_secure_log`
   - `linux_journal_dir`, `linux_auth_log`, `linux_syslog`, `linux_messages_log`
   - `evtx_security`, `evtx_system`, `evtx_task_scheduler`, `evtx_defender`
   - `evtx_rdp_client`, `evtx_rdp_inbound`, `evtx_bits_client`
   - Plus any new IDs from this batch (they'll all be in catalog together)
