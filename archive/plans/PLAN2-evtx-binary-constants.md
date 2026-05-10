# PLAN2.md — forensicnomicon: EVTX Binary Format Constants Module

**Date:** 2026-05-11
**Status:** ACTIVE — unblocks Phase 7 of `winevt-forensic`

---

## Background

`winevt-core` (in the `winevt-forensic` workspace) currently defines EVTX binary format
constants inline in `crates/winevt-core/src/binary.rs`. The agreed architecture is that
all format constants belong in `forensicnomicon` as the single source of truth — parser
crates import from here rather than defining their own copies.

`memf-windows` (in `memory-forensic`) already imports from `winevt_core::binary::*`, so
once `winevt-core` re-exports from `forensicnomicon::evtx`, both repos are updated
automatically without touching their call sites.

---

## What to Build

### New file: `src/evtx.rs`

Zero external deps. `no_std`-compatible. Follow the same
`#[cfg_attr(feature = "serde", derive(serde::Serialize))]` pattern as `eventids.rs`.

**Constants:**

```rust
// ── File header ──────────────────────────────────────────────────────────────
pub const ELFFILE_MAGIC: [u8; 8]      = *b"ElfFile\0";
pub const FILE_HEADER_SIZE: u64       = 0x80;
pub const FILE_HEADER_BLOCK_SIZE: u64 = 0x1000;   // 4 KiB

// ── Chunk ─────────────────────────────────────────────────────────────────────
pub const ELFCHNK_MAGIC: [u8; 8]      = *b"ElfChnk\0";
pub const CHUNK_SIZE: u64             = 0x1_0000;  // 64 KiB
pub const CHUNK_HEADER_SIZE: u64      = 0x80;
pub const CHUNK_RECORDS_OFFSET: u64   = 0x200;     // records start here within each chunk

/// Byte range covered by the chunk header CRC32 (stored at CHUNK_HEADER_CRC_OFFSET).
pub const CHUNK_HEADER_CRC_RANGE: core::ops::Range<usize> = 0..0x78;
pub const CHUNK_HEADER_CRC_OFFSET: usize    = 0x78;
pub const EVENT_RECORDS_CRC_OFFSET: usize   = 0x34;

// ── Event record ──────────────────────────────────────────────────────────────
pub const RECORD_MAGIC: [u8; 4]       = [0x2A, 0x2A, 0x00, 0x00];
pub const RECORD_HEADER_SIZE: u64     = 0x18;  // magic(4)+size(4)+record_id(8)+timestamp(8)

// ── File flags ────────────────────────────────────────────────────────────────
pub const FILE_FLAG_DIRTY: u32 = 0x0001;
pub const FILE_FLAG_FULL: u32  = 0x0002;
```

**Offset layout structs** (documents the format in the type system, cross-checks offsets
at compile time via the const initialisers):

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct EvtxFileHeaderOffsets {
    pub magic: u64,               // 0x00
    pub first_chunk_number: u64,  // 0x08
    pub last_chunk_number: u64,   // 0x10
    pub next_record_id: u64,      // 0x18
    pub header_size: u64,         // 0x20
    pub minor_version: u64,       // 0x24
    pub major_version: u64,       // 0x26
    pub header_block_size: u64,   // 0x28
    pub chunk_count: u64,         // 0x2A
    pub file_flags: u64,          // 0x78
    pub checksum: u64,            // 0x7C
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct EvtxChunkHeaderOffsets {
    pub magic: u64,                          // 0x00
    pub first_event_record_number: u64,      // 0x08
    pub last_event_record_number: u64,       // 0x10
    pub first_event_record_id: u64,          // 0x18
    pub last_event_record_id: u64,           // 0x20
    pub header_size: u64,                    // 0x28
    pub last_event_record_data_offset: u64,  // 0x2C
    pub free_space_offset: u64,              // 0x30
    pub event_records_checksum: u64,         // 0x34
    pub header_checksum: u64,               // 0x78
}

pub const EVTX_FILE_HEADER_OFFSETS: EvtxFileHeaderOffsets = EvtxFileHeaderOffsets {
    magic: 0x00, first_chunk_number: 0x08, last_chunk_number: 0x10,
    next_record_id: 0x18, header_size: 0x20, minor_version: 0x24,
    major_version: 0x26, header_block_size: 0x28, chunk_count: 0x2A,
    file_flags: 0x78, checksum: 0x7C,
};

pub const EVTX_CHUNK_HEADER_OFFSETS: EvtxChunkHeaderOffsets = EvtxChunkHeaderOffsets {
    magic: 0x00, first_event_record_number: 0x08, last_event_record_number: 0x10,
    first_event_record_id: 0x18, last_event_record_id: 0x20, header_size: 0x28,
    last_event_record_data_offset: 0x2C, free_space_offset: 0x30,
    event_records_checksum: 0x34, header_checksum: 0x78,
};
```

### Edit: `src/lib.rs`

Add one line alongside the other `pub mod` declarations:

```rust
pub mod evtx;
```

---

## TDD Steps (strict RED → GREEN, two commits)

### RED commit — failing tests only

Add a `#[cfg(test)]` module at the bottom of `src/evtx.rs` (file does not exist yet —
tests will fail to compile, which counts as RED):

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn file_magic_is_correct()   { assert_eq!(&ELFFILE_MAGIC, b"ElfFile\0"); }
    #[test] fn chunk_magic_is_correct()  { assert_eq!(&ELFCHNK_MAGIC, b"ElfChnk\0"); }
    #[test] fn record_magic_is_correct() { assert_eq!(RECORD_MAGIC, [0x2A, 0x2A, 0x00, 0x00]); }
    #[test] fn chunk_size_is_64kib()     { assert_eq!(CHUNK_SIZE, 65536); }
    #[test] fn records_start_at_0x200()  { assert_eq!(CHUNK_RECORDS_OFFSET, 0x200); }
    #[test] fn header_crc_covers_first_120_bytes() {
        assert_eq!(CHUNK_HEADER_CRC_RANGE, 0..0x78);
        assert_eq!(CHUNK_HEADER_CRC_OFFSET, 0x78);
    }
    #[test] fn file_header_offsets_are_correct() {
        assert_eq!(EVTX_FILE_HEADER_OFFSETS.magic,              0x00);
        assert_eq!(EVTX_FILE_HEADER_OFFSETS.next_record_id,     0x18);
        assert_eq!(EVTX_FILE_HEADER_OFFSETS.chunk_count,        0x2A);
        assert_eq!(EVTX_FILE_HEADER_OFFSETS.checksum,           0x7C);
    }
    #[test] fn chunk_header_offsets_are_correct() {
        assert_eq!(EVTX_CHUNK_HEADER_OFFSETS.first_event_record_number, 0x08);
        assert_eq!(EVTX_CHUNK_HEADER_OFFSETS.event_records_checksum,    0x34);
        assert_eq!(EVTX_CHUNK_HEADER_OFFSETS.header_checksum,           0x78);
    }
}
```

### GREEN commit — implementation

Write `src/evtx.rs` with all constants and structs as listed above. Add `pub mod evtx;`
to `src/lib.rs`. Run `cargo test -p forensicnomicon` — all 8 tests pass.

---

## Downstream: `winevt-core` re-exports (work in `winevt-forensic` repo)

After this is done (published or added as a `path` dep):

1. `winevt-core/Cargo.toml` — add `forensicnomicon = { path = "../../forensicnomicon" }` (or version dep)
2. `winevt-core/src/binary.rs` — replace the inline const blocks with re-exports:

```rust
pub use forensicnomicon::evtx::{
    ELFFILE_MAGIC, ELFCHNK_MAGIC, RECORD_MAGIC,
    CHUNK_SIZE, CHUNK_RECORDS_OFFSET, CHUNK_HEADER_SIZE,
    CHUNK_HEADER_CRC_RANGE, CHUNK_HEADER_CRC_OFFSET,
    EVENT_RECORDS_CRC_OFFSET, RECORD_HEADER_SIZE,
    FILE_FLAG_DIRTY, FILE_FLAG_FULL,
    EVTX_FILE_HEADER_OFFSETS, EVTX_CHUNK_HEADER_OFFSETS,
    EvtxFileHeaderOffsets, EvtxChunkHeaderOffsets,
};
```

All existing callers (`winevt-carver`, `winevt-integrity`, `memf-windows`) import from
`winevt_core::binary::*` and require zero changes — the re-export preserves every
existing public path.
