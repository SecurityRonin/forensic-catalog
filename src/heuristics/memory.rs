//! Windows virtual address space and VAD/VMA protection heuristics.

// x64 canonical address space: user 0x0..=0x00007FFF_FFFFFFFF,
// kernel 0xFFFF0000_00000000..=0xFFFFFFFF_FFFFFFFF.
// Addresses in between (bits 48-63 not sign-extended) are non-canonical.

/// Upper bound of x64 user space (inclusive).
pub const USER_SPACE_MAX_X64: u64 = 0x0000_7FFF_FFFF_FFFF;

/// Lower bound of x64 kernel space (inclusive).
pub const KERNEL_SPACE_MIN_X64: u64 = 0xFFFF_0000_0000_0000;

/// Returns `true` if `addr` is in the x64 user-mode address range.
#[must_use]
pub fn is_user_address_x64(addr: u64) -> bool {
    addr <= USER_SPACE_MAX_X64
}

/// Returns `true` if `addr` is in the x64 kernel-mode address range.
#[must_use]
pub fn is_kernel_address_x64(addr: u64) -> bool {
    addr >= KERNEL_SPACE_MIN_X64
}

/// Returns `true` if `addr` is a canonical x64 address (user or kernel space).
/// Non-canonical addresses in VAD/EPROCESS structures indicate corruption or injection.
#[must_use]
pub fn is_canonical_x64(addr: u64) -> bool {
    is_user_address_x64(addr) || is_kernel_address_x64(addr)
}

// Windows VAD protection constants (from wdm.h / ntifs.h)
/// Read-Write-Execute page protection — always suspicious in private VAD regions.
pub const PAGE_EXECUTE_READWRITE: u32 = 0x40;
/// Execute-WriteCopy page protection — also suspicious (used by packers).
pub const PAGE_EXECUTE_WRITECOPY: u32 = 0x80;

/// Returns `true` if the VAD protection flags indicate RWX or execute-writecopy pages.
/// RWX private regions are the canonical shellcode/reflective-loader indicator.
#[must_use]
pub fn is_rwx_page(protection: u32) -> bool {
    matches!(protection, PAGE_EXECUTE_READWRITE | PAGE_EXECUTE_WRITECOPY)
}

/// Page size on x86/x64/ARM64.
pub const PAGE_SIZE_BYTES: u64 = 4096;

/// Returns `true` if the region size is below one page — shellcode-sized allocation.
/// Legitimate code sections are always at least one page.
#[must_use]
pub fn is_shellcode_candidate_size(size_bytes: u64) -> bool {
    size_bytes > 0 && size_bytes < PAGE_SIZE_BYTES
}

/// Returns `true` if the region is large enough to be a memory dump, container, or
/// injected PE (>= 1 GiB).
#[must_use]
pub fn is_large_private_region(size_bytes: u64) -> bool {
    size_bytes >= 1_073_741_824 // 1 GiB
}

// ── Tests ─────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_address_zero_is_user() {
        assert!(is_user_address_x64(0));
    }

    #[test]
    fn user_address_max_boundary() {
        assert!(is_user_address_x64(USER_SPACE_MAX_X64));
    }

    #[test]
    fn user_address_just_above_max() {
        assert!(!is_user_address_x64(USER_SPACE_MAX_X64 + 1));
    }

    #[test]
    fn kernel_address_min_boundary() {
        assert!(is_kernel_address_x64(KERNEL_SPACE_MIN_X64));
    }

    #[test]
    fn kernel_address_max() {
        assert!(is_kernel_address_x64(u64::MAX));
    }

    #[test]
    fn kernel_address_just_below() {
        assert!(!is_kernel_address_x64(KERNEL_SPACE_MIN_X64 - 1));
    }

    #[test]
    fn canonical_user_address() {
        assert!(is_canonical_x64(0x0000_1234_5678_9ABC));
    }

    #[test]
    fn canonical_kernel_address() {
        assert!(is_canonical_x64(0xFFFF_8000_0000_0000));
    }

    #[test]
    fn non_canonical_address() {
        // between user max and kernel min — non-canonical
        assert!(!is_canonical_x64(0x0001_0000_0000_0000));
    }

    #[test]
    fn rwx_page_execute_readwrite() {
        assert!(is_rwx_page(PAGE_EXECUTE_READWRITE));
    }

    #[test]
    fn rwx_page_execute_writecopy() {
        assert!(is_rwx_page(PAGE_EXECUTE_WRITECOPY));
    }

    #[test]
    fn rwx_readonly_is_not_rwx() {
        assert!(!is_rwx_page(0x02));
    }

    #[test]
    fn shellcode_size_below_page() {
        assert!(is_shellcode_candidate_size(1024));
    }

    #[test]
    fn shellcode_size_zero_is_not_candidate() {
        assert!(!is_shellcode_candidate_size(0));
    }

    #[test]
    fn shellcode_size_full_page_is_not_candidate() {
        assert!(!is_shellcode_candidate_size(4096));
    }

    #[test]
    fn large_region_one_gib() {
        assert!(is_large_private_region(1_073_741_824));
    }

    #[test]
    fn large_region_below_gib() {
        assert!(!is_large_private_region(1_073_741_823));
    }
}
