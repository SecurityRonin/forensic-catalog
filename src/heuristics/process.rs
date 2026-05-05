//! Process, session, and Linux capability heuristics.

// ── Windows process heuristics ─────────────────────────────────────────────

/// Returns `true` if the PID is a valid Windows process ID.
/// Windows PIDs are always multiples of 4 and non-zero.
#[must_use]
pub fn is_valid_windows_pid(pid: u32) -> bool { pid != 0 && pid % 4 == 0 }

/// Returns `true` if the child process appears to have been created before its parent.
/// This is impossible under normal conditions and indicates PPID spoofing (T1134.004).
#[must_use]
pub fn is_child_born_before_parent(child_create_ns: i64, parent_create_ns: i64) -> bool {
    child_create_ns < parent_create_ns
}

// ── Windows logon type constants (Event ID 4624) ──────────────────────────

pub const LOGON_INTERACTIVE: u32       = 2;
pub const LOGON_NETWORK: u32           = 3;
pub const LOGON_BATCH: u32             = 4;
pub const LOGON_SERVICE: u32           = 5;
pub const LOGON_NETWORK_CLEARTEXT: u32 = 8;
pub const LOGON_NEW_CREDENTIALS: u32   = 9;  // pass-the-hash / pass-the-ticket
pub const LOGON_REMOTE_INTERACTIVE: u32 = 10; // RDP

/// Returns `true` for network-originating logon types (lateral movement candidates).
#[must_use]
pub fn is_remote_logon(logon_type: u32) -> bool {
    matches!(logon_type, LOGON_NETWORK | LOGON_REMOTE_INTERACTIVE | LOGON_NETWORK_CLEARTEXT)
}

/// Returns `true` for logon types commonly used in lateral movement / credential abuse.
#[must_use]
pub fn is_lateral_movement_logon(logon_type: u32) -> bool {
    matches!(logon_type, LOGON_NEW_CREDENTIALS | LOGON_NETWORK | LOGON_NETWORK_CLEARTEXT)
}

/// Returns `true` if the Windows session ID indicates Session 0 (system/service, non-interactive).
#[must_use]
pub fn is_system_session(session_id: u32) -> bool { session_id == 0 }

/// Token elevation type 3 = full admin token (UAC bypassed or already elevated).
pub const TOKEN_ELEVATION_FULL: u32 = 3;

/// Returns `true` if the token elevation type indicates a fully elevated (admin) token.
#[must_use]
pub fn is_elevated_token(elevation_type: u32) -> bool { elevation_type == TOKEN_ELEVATION_FULL }

// ── Linux process heuristics ────────────────────────────────────────────────

/// Suspicious PID gap: gaps larger than this in a sorted /proc PID list
/// suggest a rootkit is hiding processes.
pub const SUSPICIOUS_PID_GAP: u32 = 50;

/// Returns `true` if the sorted PID list contains a gap larger than `max_gap`.
/// A gap in consecutive /proc entries means processes were hidden.
#[must_use]
pub fn has_pid_gap(sorted_pids: &[u32], max_gap: u32) -> bool {
    sorted_pids.windows(2).any(|w| w[1].saturating_sub(w[0]) > max_gap)
}

// Linux POSIX capability numbers (from linux/capability.h)
pub const CAP_DAC_OVERRIDE: u32 = 1;  // bypass file permission checks
pub const CAP_NET_RAW: u32      = 13; // raw sockets / packet capture
pub const CAP_SYS_PTRACE: u32   = 19; // ptrace any process
pub const CAP_SYS_ADMIN: u32    = 21; // broad system administration

/// Capabilities that grant near-root privileges or forensic evasion ability.
pub const DANGEROUS_CAPS: &[u32] = &[CAP_DAC_OVERRIDE, CAP_NET_RAW, CAP_SYS_PTRACE, CAP_SYS_ADMIN];

/// Returns `true` if `cap` is in the dangerous capabilities list.
#[must_use]
pub fn is_dangerous_capability(cap: u32) -> bool { DANGEROUS_CAPS.contains(&cap) }

// ── Tests ─────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_windows_pid_4() {
        assert!(is_valid_windows_pid(4));
    }

    #[test]
    fn valid_windows_pid_1000() {
        assert!(is_valid_windows_pid(1000));
    }

    #[test]
    fn invalid_windows_pid_zero() {
        assert!(!is_valid_windows_pid(0));
    }

    #[test]
    fn invalid_windows_pid_odd() {
        assert!(!is_valid_windows_pid(1001));
    }

    #[test]
    fn child_born_before_parent_returns_true() {
        assert!(is_child_born_before_parent(100, 200));
    }

    #[test]
    fn child_born_after_parent_returns_false() {
        assert!(!is_child_born_before_parent(200, 100));
    }

    #[test]
    fn child_same_time_as_parent_returns_false() {
        assert!(!is_child_born_before_parent(100, 100));
    }

    #[test]
    fn remote_logon_network() {
        assert!(is_remote_logon(LOGON_NETWORK));
    }

    #[test]
    fn remote_logon_rdp() {
        assert!(is_remote_logon(LOGON_REMOTE_INTERACTIVE));
    }

    #[test]
    fn remote_logon_cleartext() {
        assert!(is_remote_logon(LOGON_NETWORK_CLEARTEXT));
    }

    #[test]
    fn interactive_logon_is_not_remote() {
        assert!(!is_remote_logon(LOGON_INTERACTIVE));
    }

    #[test]
    fn lateral_movement_new_credentials() {
        assert!(is_lateral_movement_logon(LOGON_NEW_CREDENTIALS));
    }

    #[test]
    fn lateral_movement_network() {
        assert!(is_lateral_movement_logon(LOGON_NETWORK));
    }

    #[test]
    fn service_logon_is_not_lateral() {
        assert!(!is_lateral_movement_logon(LOGON_SERVICE));
    }

    #[test]
    fn system_session_zero() {
        assert!(is_system_session(0));
    }

    #[test]
    fn user_session_one() {
        assert!(!is_system_session(1));
    }

    #[test]
    fn elevated_token_type_3() {
        assert!(is_elevated_token(TOKEN_ELEVATION_FULL));
    }

    #[test]
    fn non_elevated_token_type_1() {
        assert!(!is_elevated_token(1));
    }

    #[test]
    fn pid_gap_detected() {
        // gap between 2 and 100 is 98, > 50
        assert!(has_pid_gap(&[1, 2, 100, 101], 50));
    }

    #[test]
    fn pid_gap_not_detected_small_gaps() {
        assert!(!has_pid_gap(&[1, 2, 3, 4], 50));
    }

    #[test]
    fn pid_gap_empty_slice() {
        assert!(!has_pid_gap(&[], 50));
    }

    #[test]
    fn dangerous_cap_sys_admin() {
        assert!(is_dangerous_capability(CAP_SYS_ADMIN));
    }

    #[test]
    fn dangerous_cap_net_raw() {
        assert!(is_dangerous_capability(CAP_NET_RAW));
    }

    #[test]
    fn non_dangerous_cap_chown() {
        // CAP_CHOWN = 0
        assert!(!is_dangerous_capability(0));
    }
}
