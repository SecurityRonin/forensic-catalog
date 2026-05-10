//! EVTX binary format constants and offset layouts.
//!
//! Single source of truth for all magic bytes, sizes, and field offsets
//! used by EVTX parsers and carvers. Parser crates re-export from here
//! rather than defining their own copies.
//!
//! ```rust
//! use forensicnomicon::evtx::{ELFFILE_MAGIC, EVTX_FILE_HEADER_OFFSETS};
//! // magic at byte 0, checksum at 0x7C
//! let _ = EVTX_FILE_HEADER_OFFSETS.checksum;
//! ```

// ── File header ───────────────────────────────────────────────────────────────

pub const ELFFILE_MAGIC: [u8; 8]      = *b"ElfFile\0";
pub const FILE_HEADER_SIZE: u64       = 0x80;
pub const FILE_HEADER_BLOCK_SIZE: u64 = 0x1000;  // 4 KiB

// ── Chunk ─────────────────────────────────────────────────────────────────────

pub const ELFCHNK_MAGIC: [u8; 8]      = *b"ElfChnk\0";
pub const CHUNK_SIZE: u64             = 0x1_0000; // 64 KiB
pub const CHUNK_HEADER_SIZE: u64      = 0x80;
pub const CHUNK_RECORDS_OFFSET: u64   = 0x200;   // records start here within each chunk

/// Byte range covered by the chunk header CRC32 (stored at `CHUNK_HEADER_CRC_OFFSET`).
pub const CHUNK_HEADER_CRC_RANGE: core::ops::Range<usize>  = 0..0x78;
pub const CHUNK_HEADER_CRC_OFFSET: usize                   = 0x78;
pub const EVENT_RECORDS_CRC_OFFSET: usize                  = 0x34;

// ── Event record ──────────────────────────────────────────────────────────────

/// `**\0\0` — marks the start of every event record.
pub const RECORD_MAGIC: [u8; 4] = [0x2A, 0x2A, 0x00, 0x00];
/// magic(4) + size(4) + record_id(8) + timestamp(8) = 24 bytes.
pub const RECORD_HEADER_SIZE: u64 = 0x18;

// ── File flags ────────────────────────────────────────────────────────────────

pub const FILE_FLAG_DIRTY: u32 = 0x0001;
pub const FILE_FLAG_FULL: u32  = 0x0002;

// ── Offset layout structs ─────────────────────────────────────────────────────

/// Field offsets within the 128-byte EVTX file header.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct EvtxFileHeaderOffsets {
    pub magic:              u64, // 0x00
    pub first_chunk_number: u64, // 0x08
    pub last_chunk_number:  u64, // 0x10
    pub next_record_id:     u64, // 0x18
    pub header_size:        u64, // 0x20
    pub minor_version:      u64, // 0x24
    pub major_version:      u64, // 0x26
    pub header_block_size:  u64, // 0x28
    pub chunk_count:        u64, // 0x2A
    pub file_flags:         u64, // 0x78
    pub checksum:           u64, // 0x7C
}

/// Field offsets within the 128-byte EVTX chunk header.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct EvtxChunkHeaderOffsets {
    pub magic:                         u64, // 0x00
    pub first_event_record_number:     u64, // 0x08
    pub last_event_record_number:      u64, // 0x10
    pub first_event_record_id:         u64, // 0x18
    pub last_event_record_id:          u64, // 0x20
    pub header_size:                   u64, // 0x28
    pub last_event_record_data_offset: u64, // 0x2C
    pub free_space_offset:             u64, // 0x30
    pub event_records_checksum:        u64, // 0x34
    pub header_checksum:               u64, // 0x78
}

pub const EVTX_FILE_HEADER_OFFSETS: EvtxFileHeaderOffsets = EvtxFileHeaderOffsets {
    magic:              0x00,
    first_chunk_number: 0x08,
    last_chunk_number:  0x10,
    next_record_id:     0x18,
    header_size:        0x20,
    minor_version:      0x24,
    major_version:      0x26,
    header_block_size:  0x28,
    chunk_count:        0x2A,
    file_flags:         0x78,
    checksum:           0x7C,
};

pub const EVTX_CHUNK_HEADER_OFFSETS: EvtxChunkHeaderOffsets = EvtxChunkHeaderOffsets {
    magic:                         0x00,
    first_event_record_number:     0x08,
    last_event_record_number:      0x10,
    first_event_record_id:         0x18,
    last_event_record_id:          0x20,
    header_size:                   0x28,
    last_event_record_data_offset: 0x2C,
    free_space_offset:             0x30,
    event_records_checksum:        0x34,
    header_checksum:               0x78,
};

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
        assert_eq!(EVTX_FILE_HEADER_OFFSETS.magic,          0x00);
        assert_eq!(EVTX_FILE_HEADER_OFFSETS.next_record_id, 0x18);
        assert_eq!(EVTX_FILE_HEADER_OFFSETS.chunk_count,    0x2A);
        assert_eq!(EVTX_FILE_HEADER_OFFSETS.checksum,       0x7C);
    }
    #[test] fn chunk_header_offsets_are_correct() {
        assert_eq!(EVTX_CHUNK_HEADER_OFFSETS.first_event_record_number, 0x08);
        assert_eq!(EVTX_CHUNK_HEADER_OFFSETS.event_records_checksum,    0x34);
        assert_eq!(EVTX_CHUNK_HEADER_OFFSETS.header_checksum,           0x78);
    }
}
