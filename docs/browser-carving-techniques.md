# Browser Artifact Carving Techniques

> **Purpose:** Reference for implementing `browser-carve` crate in browser-forensic.
> Covers SQLite internals, browser-specific binary formats, and recovery algorithms.
> Companion to `browser-data-structures.md`.

---

## 1. SQLite Page Internals Primer

Every SQLite database is a sequence of fixed-size **pages** (default 4096 bytes). The page size is stored at offset 16 of the database header (2 bytes, big-endian).

```
Database file layout:
  Page 1:  DB header (100 bytes) + root page of sqlite_master
  Page 2+: B-tree interior/leaf pages, overflow pages, freelist trunk/leaf pages

Page header (8 bytes for leaf, 12 bytes for interior):
  Offset 0: Page type byte
    0x02 = interior index b-tree
    0x05 = interior table b-tree
    0x0a = leaf index b-tree
    0x0d = leaf table b-tree
  Offset 1: First freeblock offset (0 if none)
  Offset 3: Number of cells on page
  Offset 5: Cell content area start offset (0 means 65536)
  Offset 7: Fragmented free bytes
  Offset 8: (interior only) Right-most pointer
```

**Key forensic insight:** Even after `DELETE`, SQLite does not zero out cell content. The cell pointer array is updated and space is marked as a freeblock, but the data bytes remain until overwritten by a new INSERT.

---

## 2. Freelist Analysis

SQLite maintains a **freelist** of pages no longer in use (table dropped, page emptied).

### Database Header Fields

```
Offset 32: Freelist trunk page number (0 if empty)
Offset 36: Total freelist page count
```

### Freelist Trunk Page Structure

```
Offset 0:  Next trunk page number (4 bytes, big-endian)
Offset 4:  Number of leaf page pointers on this trunk (4 bytes)
Offset 8+: Array of leaf page numbers (4 bytes each)
```

### Recovery Algorithm

```rust
fn recover_freelist_pages(db_path: &Path) -> Vec<Vec<u8>> {
    let raw = std::fs::read(db_path).unwrap();
    let page_size = u16::from_be_bytes([raw[16], raw[17]]) as usize;
    let page_size = if page_size == 1 { 65536 } else { page_size };

    let trunk_num = u32::from_be_bytes([raw[32], raw[33], raw[34], raw[35]]) as usize;
    let mut pages = Vec::new();

    let mut trunk = trunk_num;
    while trunk != 0 {
        let offset = (trunk - 1) * page_size;
        let next = u32::from_be_bytes([raw[offset], raw[offset+1], raw[offset+2], raw[offset+3]]) as usize;
        let count = u32::from_be_bytes([raw[offset+4], raw[offset+5], raw[offset+6], raw[offset+7]]) as usize;
        for i in 0..count {
            let leaf_num = u32::from_be_bytes([
                raw[offset+8+i*4], raw[offset+9+i*4],
                raw[offset+10+i*4], raw[offset+11+i*4]
            ]) as usize;
            let leaf_offset = (leaf_num - 1) * page_size;
            pages.push(raw[leaf_offset..leaf_offset+page_size].to_vec());
        }
        trunk = next;
    }
    pages
}
```

**Target tables:** Chrome `urls`, `visits`, `downloads`; Firefox `moz_places`, `moz_historyvisits`; Safari `history_items`, `history_visits`.

---

## 3. Freeblock (Within-Page) Analysis

When a row is deleted from a non-empty page, the cell is converted to a **freeblock** in the page's freeblock chain.

### Freeblock Chain Structure

```
Page header offset 1: Offset of first freeblock (0 = none)

Freeblock entry:
  Offset +0: Offset of next freeblock (2 bytes, 0 if last)
  Offset +2: Size of this freeblock including header (2 bytes)
  Offset +4: Former cell content (varies)
```

### Recovery Algorithm

```rust
fn walk_freeblocks(page: &[u8]) -> Vec<&[u8]> {
    let first = u16::from_be_bytes([page[1], page[2]]) as usize;
    let mut offset = first;
    let mut recovered = Vec::new();

    while offset != 0 && offset + 4 <= page.len() {
        let next = u16::from_be_bytes([page[offset], page[offset+1]]) as usize;
        let size = u16::from_be_bytes([page[offset+2], page[offset+3]]) as usize;
        if size >= 4 && offset + size <= page.len() {
            recovered.push(&page[offset+4..offset+size]);
        }
        offset = next;
    }
    recovered
}
```

**Notes:**
- Freeblocks smaller than 4 bytes are "fragments" counted in page header offset 7.
- The former cell content is a SQLite record: starts with a varint payload size, then a varint row-id, then the record body.
- Parse the record header (varint serial types) to extract field values.

---

## 4. WAL (Write-Ahead Log) Analysis

SQLite WAL mode writes all changes to a `-wal` file before committing to the main database. This creates a forensic goldmine: **uncommitted, rolled-back, or checkpoint-pending frames remain in the WAL until overwritten**.

### WAL File Binary Layout

```
WAL Header (32 bytes):
  Offset 0:  Magic 0x377f0682 (big-endian) or 0x377f0683 (native)
  Offset 4:  File format version (currently 3007000)
  Offset 8:  Database page size (4 bytes)
  Offset 12: Checkpoint sequence number
  Offset 16: Salt-1 (random, changes on each restart)
  Offset 20: Salt-2 (random, changes on each restart)
  Offset 24: Checksum-1 (big-endian)
  Offset 28: Checksum-2 (big-endian)

WAL Frame (frame_header + page_content):
  Frame Header (24 bytes):
    Offset 0:  Page number (4 bytes, 1-based)
    Offset 4:  For commit frame: size of db after commit (pages); else 0
    Offset 8:  Salt-1 (must match WAL header)
    Offset 12: Salt-2 (must match WAL header)
    Offset 16: Checksum-1 (cumulative)
    Offset 20: Checksum-2 (cumulative)
  Page content: page_size bytes
```

### WAL Analysis Procedure

```rust
fn analyze_wal(wal_path: &Path, page_size: usize) -> Vec<WalFrame> {
    let raw = std::fs::read(wal_path).unwrap();
    // Validate magic
    let magic = u32::from_be_bytes([raw[0], raw[1], raw[2], raw[3]]);
    assert!(magic == 0x377f0682 || magic == 0x377f0683);

    let frame_size = 24 + page_size;
    let mut frames = Vec::new();
    let mut pos = 32; // skip WAL header

    while pos + frame_size <= raw.len() {
        let page_num = u32::from_be_bytes([raw[pos], raw[pos+1], raw[pos+2], raw[pos+3]]);
        let db_size = u32::from_be_bytes([raw[pos+4], raw[pos+5], raw[pos+6], raw[pos+7]]);
        let is_commit = db_size != 0;
        frames.push(WalFrame {
            page_num,
            is_commit,
            data: raw[pos+24..pos+frame_size].to_vec(),
        });
        pos += frame_size;
    }
    frames
}
```

**Integrity indicator:** WAL file exists with valid frames not yet checkpointed → browser closed abnormally or WAL deletion was prevented.

---

## 5. Unallocated Space Carving

SQLite's **unallocated space** is the gap between the end of the cell pointer array and the start of the cell content area within each page.

```
Page layout:
  [page header: 8 or 12 bytes]
  [cell pointer array: 2 bytes × cell_count]
  [UNALLOCATED SPACE]        ← carved from here
  [cell content area]
  [fragmented free bytes at end]
```

The cell content area start offset is at page header byte 5 (2 bytes). The unallocated region starts at `8 + 2*cell_count` and ends at the content area start.

**Pattern matching in unallocated space:**
- Look for SQLite varint sequences that parse into plausible row-ids
- Look for known string patterns: `https://`, `http://`, domain patterns
- Apply serial type decoding after locating candidate record headers

---

## 6. Chrome SimpleCache Entry Recovery

Chrome's disk cache (after ~Chrome 61) uses the **SimpleCache** format: one file per resource, named by URL hash.

### Entry File Structure

```
SimpleCache entry file layout:
  [HTTP headers + response body]
  EOF Record (at end of file):
    Offset -24: Magic 0xD1E0F4A6 (4 bytes, little-endian)
    Offset -20: Flags (4 bytes)
    Offset -16: CRC32 of key (4 bytes)
    Offset -12: Key length (4 bytes)
    Offset -8:  Data stream sizes (8 bytes)
    At key_offset = eof - 24 - key_length:
      Raw URL bytes (key_length bytes)
```

### Recovery Algorithm

```rust
fn extract_url_from_cache_entry(data: &[u8]) -> Option<String> {
    const MAGIC: u32 = 0xD1E0F4A6;
    if data.len() < 24 { return None; }

    let magic_offset = data.len() - 24;
    let found_magic = u32::from_le_bytes([
        data[magic_offset], data[magic_offset+1],
        data[magic_offset+2], data[magic_offset+3]
    ]);
    if found_magic != MAGIC { return None; }

    let key_len = u32::from_le_bytes([
        data[magic_offset+8], data[magic_offset+9],
        data[magic_offset+10], data[magic_offset+11]
    ]) as usize;

    if key_len + 24 > data.len() { return None; }
    let key_start = data.len() - 24 - key_len;
    std::str::from_utf8(&data[key_start..key_start+key_len]).ok().map(|s| s.to_string())
}
```

**Carved metadata:** URL, file size (= response body size), cache directory timestamps from filesystem metadata.

---

## 7. Firefox Cache2 Entry Recovery

Firefox Cache2 uses `~/.mozilla/firefox/<profile>/cache2/entries/`. Files are named by SHA-1 of URL.

### Entry File Structure

```
Cache2 entry:
  [response headers and body — variable length]
  Metadata at end:
    Offset -4:  Metadata start offset from beginning (4 bytes, network byte order)
    At metadata_start:
      version (4 bytes)
      fetch_count (4 bytes)
      last_fetched (4 bytes, unix seconds)
      last_modified (4 bytes, unix seconds)
      frecency (4 bytes)
      expire_time (4 bytes)
      key_size (4 bytes)
      flags (4 bytes)
      key bytes (key_size bytes) ← URL
      element_count (4 bytes)
      [name\0value\0 pairs]
```

**Recovery:** Read last 4 bytes, seek to metadata_start, parse key_size, extract URL.

---

## 8. mozLz4 Partial Recovery (Firefox Session Store)

Firefox compresses `sessionstore.jsonlz4` with a custom LZ4 format (NOT LZ4 frame format):

```
mozLz4 layout:
  Magic: "mozLz40\0"  (8 bytes, literal)
  Uncompressed size: 4 bytes, little-endian
  LZ4 block data: remainder of file (raw LZ4 block, NOT framed)
```

### Recovery

```rust
use lz4_flex::block::decompress;

fn decompress_mozlz4(data: &[u8]) -> anyhow::Result<Vec<u8>> {
    const MAGIC: &[u8] = b"mozLz40\0";
    anyhow::ensure!(data.starts_with(MAGIC), "not mozLz4");
    let uncompressed_size = u32::from_le_bytes(data[8..12].try_into()?) as usize;
    let block = &data[12..];
    decompress(block, uncompressed_size).map_err(Into::into)
}
```

**Partial recovery:** If file is truncated after the magic+size header, attempt decompression of the partial block — lz4_flex may recover partial JSON. Parse partial JSON with a lenient parser (look for `"url":` patterns even in incomplete JSON).

---

## 9. Safari History Tombstone Timeline Reconstruction

Safari's `History.db` has a `history_tombstones` table that is the most explicit deletion log in any major browser:

```sql
SELECT
    url,
    datetime(start_time + 978307200, 'unixepoch') AS deleted_range_start,
    datetime(end_time   + 978307200, 'unixepoch') AS deleted_range_end
FROM history_tombstones
ORDER BY start_time DESC;
```

**Cross-table correlation:** URLs in `history_tombstones` that also appear in `history_items` (not yet purged from B-tree unallocated space) indicate incomplete deletion — the history clear was interrupted or the SQLite vacuum did not run.

```sql
-- Find tombstoned URLs still physically present in B-tree
SELECT t.url, t.start_time, t.end_time, i.visit_count
FROM history_tombstones t
JOIN history_items i ON i.url = t.url
WHERE t.end_time > t.start_time;
-- Any result here is a high-confidence integrity indicator
```

---

## 10. Chrome Sync LevelDB Analysis

Chrome stores sync metadata in `<profile>/Sync Data/LevelDB/`. LevelDB uses a log-structured merge (LSM) tree with:
- `.log` files: append-only write-ahead log (uncommitted records)
- `.ldb` / `.sst` files: sorted string tables (committed data)
- `MANIFEST-*`: version log

### LevelDB Record Format (Log Files)

```
Log block (32768 bytes each):
  Record header (7 bytes):
    checksum: 4 bytes (CRC32, masked)
    length:   2 bytes
    type:     1 byte (1=FULL, 2=FIRST, 3=MIDDLE, 4=LAST)
  Record data: length bytes
    [protobuf-encoded sync entity]
```

**Key forensic value:** `.log` files contain recent sync writes including deleted bookmark sync entries, cleared history sync tombstones, and device sync tokens that reveal which other devices shared this Chrome profile.

**Pattern matching fallback:** If protobuf parsing is unavailable, scan `.log` files for `https://` byte patterns — sync entities embed the raw URL string.

---

## 11. SQLite B-Tree Structural Anomaly Detection

These SQL queries detect integrity indicators directly from SQLite metadata:

```sql
-- Gap detection in auto-increment IDs (deleted rows leave gaps)
SELECT
    id,
    id - LAG(id, 1, id-1) OVER (ORDER BY id) AS gap
FROM visits
WHERE gap > 1;

-- Timestamp ordering anomalies (rows inserted out of chronological order)
SELECT
    id,
    visit_time,
    LAG(visit_time) OVER (ORDER BY id) AS prev_time
FROM visits
WHERE visit_time < LAG(visit_time) OVER (ORDER BY id);

-- Visit count vs. actual visit records mismatch (Chrome)
SELECT u.id, u.url, u.visit_count, COUNT(v.id) AS actual_visits
FROM urls u
LEFT JOIN visits v ON v.url = u.id
GROUP BY u.id
HAVING u.visit_count != actual_visits;
-- visit_count > actual_visits: visits were deleted
-- visit_count < actual_visits: counter was manipulated
```

---

## 12. Safari Cookies.binarycookies Carving

`Cookies.binarycookies` is a proprietary binary format (not SQLite). Partial recovery is possible by scanning for page magic bytes.

### Binary Layout

```
File header (16 bytes, big-endian):
  Magic:      "cook"  (4 bytes)
  Page count: 4 bytes
  Page sizes: page_count × 4 bytes (each page size in bytes)

Per page (mixed-endian!):
  Page magic: 0x00000100  (4 bytes, big-endian)
  Cookie count: 4 bytes (little-endian)
  Cookie offsets: cookie_count × 4 bytes (little-endian, relative to page start)
  Per cookie record:
    Total size:   4 bytes (little-endian)
    Unknown:      4 bytes
    Flags:        4 bytes (little-endian) -- 1=Secure, 4=HttpOnly
    Unknown:      4 bytes
    Domain offset:  4 bytes (little-endian, from record start)
    Name offset:    4 bytes
    Path offset:    4 bytes
    Value offset:   4 bytes
    End offset:     8 bytes (reserved)
    Expiry date:    8 bytes f64 (little-endian, Core Data epoch = Jan 1, 2001)
    Create date:    8 bytes f64 (little-endian, Core Data epoch)
    Domain string:  null-terminated
    Name string:    null-terminated
    Path string:    null-terminated
    Value string:   null-terminated
```

### Magic Byte Scanning for Carving

Scan raw file (or unallocated disk space) for page magic `\x00\x00\x01\x00`:

```rust
fn scan_for_cookie_pages(data: &[u8]) -> Vec<usize> {
    let magic = [0x00u8, 0x00, 0x01, 0x00];
    data.windows(4)
        .enumerate()
        .filter(|(_, w)| *w == magic)
        .map(|(i, _)| i)
        .collect()
}
```

At each candidate offset, validate cookie_count (must be < 1000 as sanity check) and attempt to parse cookies. Partial records yield domain + name strings even if value is truncated.

---

## 13. Chrome DPAPI Key Recovery Chain

Chrome stores the AES-256-GCM encryption key for v10/v11 cookies in `Local State` under `os_crypt.encrypted_key`:
- Base64-encoded bytes starting with `DPAPI`
- The DPAPI-protected blob follows the 5-byte `DPAPI` prefix

Recovery chain:
```
Local State encrypted_key
  → base64 decode
  → strip "DPAPI" prefix (5 bytes)
  → CryptUnprotectData (Windows, requires user context)
    OR
  → memf-windows::dpapi_keys (extract master key from LSASS memory dump)
  → reconstruct DPAPI session key
  → decrypt key blob
  → AES-256-GCM key (32 bytes)
  → decrypt cookie values from Cookies.db (ciphertext = value column, nonce = bytes 3..15)
```

**v20 (Chrome 127+) app-bound encryption:** Requires SYSTEM-level `elevation_service.exe` for decryption. Recovery from memory: scan Chrome renderer process heap for decrypted AES key material using memf-windows pattern scan.

---

## References

1. SQLite File Format: https://www.sqlite.org/fileformat2.html
2. SQLite WAL: https://www.sqlite.org/wal.html
3. Chrome SimpleCache: https://chromium.googlesource.com/chromium/src/+/refs/heads/main/net/disk_cache/simple/
4. Firefox Cache2: https://searchfox.org/mozilla-central/source/netwerk/cache2/
5. mozLz4 format: https://github.com/nicowillis/firefox-session-recovery
6. LevelDB log format: https://github.com/google/leveldb/blob/main/doc/log_format.md
7. DPAPI internals: https://docs.microsoft.com/en-us/windows/win32/api/dpapi/nf-dpapi-cryptunprotectdata
8. Chrome Cookie Encryption v20: https://security.googleblog.com/2024/07/improving-security-of-chrome-cookies-on.html
9. Binary Cookies format: https://github.com/as0ler/BinaryCookieReader
10. Recovering SQLite deleted data: https://www.forensicmag.com/articles/2012/02/sqlite-forensics
