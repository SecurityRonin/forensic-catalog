//! EVTX binary format constants and offset layouts.
//!
//! Single source of truth for all magic bytes, sizes, and field offsets
//! used by EVTX parsers and carvers. Parser crates re-export from here
//! rather than defining their own copies.

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
