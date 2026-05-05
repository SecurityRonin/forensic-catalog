# Browser Data Structure Specifications

Authoritative reference for DFIR artifact knowledge covering Chrome/Chromium, Firefox, and Safari browser data structures. Parser implementations in `browser-forensic` should be validated against these specs.

**Last updated**: 2026-05-04

---

## Timestamp Epoch Reference

| Browser | Epoch Name | Zero Point | Unit | Conversion to Unix ns |
|---------|-----------|------------|------|-----------------------|
| Chrome/Chromium | WebKit | 1601-01-01 00:00:00 UTC | Microseconds (integer) | `(webkit_us - 11_644_473_600_000_000) * 1_000` |
| Firefox (history/cookies) | PRTime (Unix) | 1970-01-01 00:00:00 UTC | Microseconds (integer) | `unix_us * 1_000` |
| Firefox (logins/extensions/session) | Unix milliseconds | 1970-01-01 00:00:00 UTC | Milliseconds (integer) | `unix_ms * 1_000_000` |
| Chrome autofill (`Web Data`) | Unix seconds | 1970-01-01 00:00:00 UTC | Seconds (integer) | `unix_s * 1_000_000_000` |
| Safari | Core Data / Mac Absolute Time | 2001-01-01 00:00:00 UTC | Seconds (float64) | `(core_data_secs + 978_307_200) * 1_000_000_000` |
| Safari (Cookies.binarycookies) | Mac Absolute Time | 2001-01-01 00:00:00 UTC | Seconds (float64) | Same as Core Data |

**Critical pitfall**: Chrome's `autofill` table in `Web Data` uses Unix epoch **seconds**, NOT WebKit microseconds. Every other Chrome SQLite timestamp uses WebKit microseconds. Misapplying the epoch conversion yields dates in the 1600s.

---

## Chrome/Chromium

### 1. History SQLite (`History`)

**Location (per-profile)**:
- Windows: `%LOCALAPPDATA%\Google\Chrome\User Data\{Profile}\History`
- macOS: `~/Library/Application Support/Google/Chrome/{Profile}/History`
- Linux: `~/.config/google-chrome/{Profile}/History`

**Format**: SQLite 3, WAL mode
**Epoch**: WebKit microseconds (since 1601-01-01 UTC)

#### `urls` Table

```sql
CREATE TABLE urls (
    id              INTEGER PRIMARY KEY,
    url             LONGVARCHAR,
    title           LONGVARCHAR,
    visit_count     INTEGER DEFAULT 0 NOT NULL,
    typed_count     INTEGER DEFAULT 0 NOT NULL,
    last_visit_time INTEGER NOT NULL,
    hidden          INTEGER DEFAULT 0 NOT NULL
);
CREATE INDEX urls_url_index ON urls (url);
```

| Column | Type | Description | Forensic Significance |
|--------|------|-------------|----------------------|
| `id` | INTEGER | Primary key, auto-increment | Gap analysis for deleted records |
| `url` | LONGVARCHAR | Full URL | Primary browsing evidence |
| `title` | LONGVARCHAR | Page title | Context for visited pages |
| `visit_count` | INTEGER | Total visits across all sessions | Frequency analysis |
| `typed_count` | INTEGER | Times URL was typed (not clicked) | User intent indicator |
| `last_visit_time` | INTEGER | WebKit microseconds | Timeline anchor |
| `hidden` | INTEGER | 1 = hidden from default history view | Subframe/redirect pages |

#### `visits` Table

```sql
CREATE TABLE visits (
    id              INTEGER PRIMARY KEY,
    url             INTEGER NOT NULL,
    visit_time      INTEGER NOT NULL,
    from_visit      INTEGER DEFAULT 0 NOT NULL,
    transition      INTEGER DEFAULT 0 NOT NULL,
    segment_id      INTEGER DEFAULT 0 NOT NULL,
    visit_duration  INTEGER DEFAULT 0 NOT NULL,
    incremented_omnibox_typed_score INTEGER DEFAULT 0 NOT NULL,
    opener_visit    INTEGER DEFAULT 0 NOT NULL,
    originator_cache_guid TEXT,
    originator_visit_id INTEGER DEFAULT 0 NOT NULL,
    originator_from_visit INTEGER DEFAULT 0 NOT NULL,
    originator_opener_visit INTEGER DEFAULT 0 NOT NULL,
    is_known_to_sync INTEGER DEFAULT 0 NOT NULL,
    consider_for_ntp_most_visited INTEGER DEFAULT 1 NOT NULL,
    publicly_routable INTEGER DEFAULT 0 NOT NULL
);
CREATE INDEX visits_url_index ON visits (url);
CREATE INDEX visits_time_index ON visits (visit_time);
CREATE INDEX visits_from_index ON visits (from_visit);
```

| Column | Description | Forensic Significance |
|--------|-------------|----------------------|
| `id` | Visit ID | Gap analysis for deletions |
| `url` | FK to `urls.id` | Join to get actual URL |
| `visit_time` | WebKit microseconds | Per-visit timestamp |
| `from_visit` | ID of referring visit (0 = none) | Navigation chain reconstruction |
| `transition` | Bitmask (see below) | How user arrived at page |
| `segment_id` | Typed URL segment | URL bar autocomplete tracking |
| `visit_duration` | Microseconds on page | Engagement metric |
| `originator_cache_guid` | Sync origin device GUID | Cross-device attribution |

##### Transition Type Bitmask

The `transition` column is a 32-bit bitmask. Low 8 bits = core type, high 24 bits = qualifiers.

**Core types** (mask: `transition & 0xFF`):

| Value | Name | Description |
|-------|------|-------------|
| 0 | `LINK` | User followed a hyperlink |
| 1 | `TYPED` | User typed URL in address bar |
| 2 | `AUTO_BOOKMARK` | Navigation via UI suggestion (e.g., top sites) |
| 3 | `AUTO_SUBFRAME` | Automatic subframe navigation (ads, iframes) |
| 4 | `MANUAL_SUBFRAME` | User-initiated subframe navigation |
| 5 | `GENERATED` | URL generated by command (e.g., search result) |
| 6 | `AUTO_TOPLEVEL` | Top-level auto navigation (e.g., pre-render) |
| 7 | `FORM_SUBMIT` | Form submission |
| 8 | `RELOAD` | Page reload (also used for session restore) |
| 9 | `KEYWORD` | Omnibox keyword/tab-to-search |
| 10 | `KEYWORD_GENERATED` | Keyword-generated visit |

**Qualifier flags** (mask: `transition & 0xFFFFFF00`):

| Hex Value | Name | Description |
|-----------|------|-------------|
| `0x00800000` | `BLOCKED` | Navigation was blocked |
| `0x01000000` | `FORWARD_BACK` | Forward/back button navigation |
| `0x02000000` | `FROM_ADDRESS_BAR` | Initiated from address bar |
| `0x04000000` | `HOME_PAGE` | Home page navigation |
| `0x08000000` | `FROM_API` | Initiated by extension/API |
| `0x10000000` | `CHAIN_START` | First in a redirect chain |
| `0x20000000` | `CHAIN_END` | Last in a redirect chain |
| `0x40000000` | `CLIENT_REDIRECT` | Client-side redirect (JS/meta refresh) |
| `0x80000000` | `SERVER_REDIRECT` | Server-side redirect (301/302) |
| `0xC0000000` | `IS_REDIRECT_MASK` | Either client or server redirect |

**Forensic example**: `transition = 0x80000001` means TYPED (`& 0xFF == 1`) + SERVER_REDIRECT (`& 0x80000000`).

#### `visit_source` Table

```sql
CREATE TABLE visit_source (
    id     INTEGER PRIMARY KEY,
    source INTEGER NOT NULL
);
```

`id` is a foreign key to `visits.id`. A visit absent from this table was browsed locally (source 1 is omitted for efficiency).

| Source Value | Name | Meaning |
|-------------|------|---------|
| 0 | `SOURCE_SYNCED` | Synchronized from another device |
| 1 | `SOURCE_BROWSED` | Locally browsed (NOT STORED -- absence implies this) |
| 2 | `SOURCE_EXTENSION` | Added by a browser extension |
| 3 | `SOURCE_FIREFOX_IMPORTED` | Imported from Firefox |
| 4 | `SOURCE_IE_IMPORTED` | Imported from Internet Explorer |
| 5 | `SOURCE_SAFARI_IMPORTED` | Imported from Safari |

**Anti-forensic note**: A URL present in `urls`/`visits` but absent from `visit_source` was locally browsed. A URL present in `visit_source` with `source=0` was synced and may not have been visited on this device.

#### `keyword_search_terms` Table

```sql
CREATE TABLE keyword_search_terms (
    keyword_id      INTEGER NOT NULL,
    url_id          INTEGER NOT NULL,
    term            LONGVARCHAR NOT NULL,
    normalized_term LONGVARCHAR NOT NULL
);
```

Recovers search queries entered via the omnibox. `keyword_id` references the `keywords` table in `Web Data` (search engine definitions). One of the highest-value history artifacts for determining user intent.

#### `downloads` Table (Modern Schema)

```sql
CREATE TABLE downloads (
    id                  INTEGER PRIMARY KEY,
    guid                VARCHAR NOT NULL,
    current_path        LONGVARCHAR NOT NULL,
    target_path         LONGVARCHAR NOT NULL,
    start_time          INTEGER NOT NULL,
    received_bytes      INTEGER NOT NULL,
    total_bytes         INTEGER NOT NULL,
    state               INTEGER NOT NULL,
    danger_type         INTEGER NOT NULL,
    interrupt_reason    INTEGER NOT NULL,
    hash                BLOB NOT NULL,
    end_time            INTEGER NOT NULL,
    opened              INTEGER NOT NULL,
    last_access_time    INTEGER NOT NULL,
    transient           INTEGER NOT NULL,
    referrer            VARCHAR NOT NULL,
    site_url            VARCHAR NOT NULL,
    tab_url             VARCHAR NOT NULL,
    tab_referrer_url    VARCHAR NOT NULL,
    http_method         VARCHAR NOT NULL,
    by_ext_id           VARCHAR NOT NULL,
    by_ext_name         VARCHAR NOT NULL,
    etag                VARCHAR NOT NULL,
    last_modified       VARCHAR NOT NULL,
    mime_type           VARCHAR(255) NOT NULL,
    original_mime_type  VARCHAR(255) NOT NULL
);
```

All timestamps (`start_time`, `end_time`, `last_access_time`) are WebKit microseconds.

`hash` is a BLOB containing the SHA-256 hash of the downloaded file content (added in Chrome 26+).

**`state` values**:

| Value | Name |
|-------|------|
| 0 | `IN_PROGRESS` |
| 1 | `COMPLETE` |
| 2 | `CANCELLED` |
| 3 | `INTERRUPTED` |

**`danger_type` values**:

| Value | Name | Description |
|-------|------|-------------|
| 0 | `NOT_DANGEROUS` | Safe |
| 1 | `DANGEROUS_FILE` | Dangerous file type (e.g., `.exe`) |
| 2 | `DANGEROUS_URL` | URL flagged by Safe Browsing |
| 3 | `DANGEROUS_CONTENT` | Content flagged by Safe Browsing |
| 4 | `MAYBE_DANGEROUS_CONTENT` | Potentially dangerous content |
| 5 | `UNCOMMON_CONTENT` | Uncommon download |
| 6 | `USER_VALIDATED` | User explicitly overrode a danger warning |
| 7 | `DANGEROUS_HOST` | Host flagged by Safe Browsing |
| 8 | `POTENTIALLY_UNWANTED` | PUA/PUP classification |
| 9 | `ALLOWLISTED_BY_POLICY` | Enterprise policy allowlisted |
| 10 | `ASYNC_SCANNING` | Asynchronous deep scanning in progress |
| 11 | `BLOCKED_PASSWORD_PROTECTED` | Password-protected archive blocked |
| 12 | `BLOCKED_TOO_LARGE` | File too large to scan |
| 13 | `SENSITIVE_CONTENT_WARNING` | Contains sensitive content (warning) |
| 14 | `SENSITIVE_CONTENT_BLOCK` | Blocked due to sensitive content |
| 15 | `DEEP_SCANNED_SAFE` | Passed deep scan, safe |
| 16 | `DEEP_SCANNED_OPENED_DANGEROUS` | Dangerous result from deep scan |
| 17 | `PROMPT_FOR_SCANNING` | Prompting user for scan decision |
| 18 | `BLOCKED_UNSUPPORTED_FILETYPE` | Unsupported file type blocked by policy |

**`interrupt_reason` ranges**:

| Range | Category |
|-------|----------|
| 0 | None (no interruption) |
| 1–9 | File errors (access denied, no space, path too long) |
| 10–19 | File errors continued (virus detected, blocked by policy) |
| 20–29 | Network errors (failed, timeout, disconnected) |
| 30–39 | Server errors (bad content, no range support) |
| 40–49 | User actions (cancelled, shutdown) |
| 50 | Crash |

#### `downloads_url_chains` Table

```sql
CREATE TABLE downloads_url_chains (
    id          INTEGER NOT NULL,
    chain_index INTEGER NOT NULL,
    url         TEXT NOT NULL,
    PRIMARY KEY (id, chain_index)
);
```

Full redirect chain for each download. `chain_index=0` is the initial URL.

**MITRE ATT&CK**: T1217 (Browser Information Discovery), T1005 (Data from Local System)

---

### 2. Cookies SQLite (`Cookies`)

**Location**: `{Profile}/Network/Cookies` (Chrome 96+; previously `{Profile}/Cookies`)
**Format**: SQLite 3, WAL mode
**Epoch**: WebKit microseconds for all timestamp columns

```sql
CREATE TABLE cookies (
    creation_utc            INTEGER NOT NULL,
    host_key                TEXT NOT NULL,
    top_frame_site_key      TEXT NOT NULL DEFAULT '',
    name                    TEXT NOT NULL,
    value                   TEXT NOT NULL,
    encrypted_value         BLOB DEFAULT '',
    path                    TEXT NOT NULL,
    expires_utc             INTEGER NOT NULL,
    is_secure               INTEGER NOT NULL,
    is_httponly             INTEGER NOT NULL,
    last_access_utc         INTEGER NOT NULL,
    has_expires             INTEGER NOT NULL DEFAULT 1,
    is_persistent           INTEGER NOT NULL DEFAULT 1,
    priority                INTEGER NOT NULL DEFAULT 1,
    samesite                INTEGER NOT NULL DEFAULT -1,
    source_scheme           INTEGER NOT NULL DEFAULT 0,
    source_port             INTEGER NOT NULL DEFAULT -1,
    last_update_utc         INTEGER NOT NULL DEFAULT 0,
    source_type             INTEGER NOT NULL DEFAULT 0,
    has_cross_site_ancestor INTEGER NOT NULL DEFAULT 0
);
```

`samesite` values: -1 = unspecified, 0 = no restriction, 1 = lax, 2 = strict.

#### Encryption of `encrypted_value`

In modern Chrome, `value` is empty and `encrypted_value` holds the actual cookie value.

**Encrypted blob structure**: `[3-byte version prefix][12-byte nonce/IV][ciphertext (variable)][16-byte GCM auth tag]`

| Prefix (ASCII) | Hex | Chrome Version | Key Source | Algorithm |
|----------------|-----|----------------|------------|-----------|
| (none/raw) | -- | Pre-80 | Windows DPAPI directly | `CryptProtectData` |
| `v10` | `76 31 30` | 80+ | `Local State` -> `os_crypt.encrypted_key` (DPAPI-wrapped) | AES-256-GCM |
| `v11` | `76 31 31` | 80+ (variant) | Same as v10 | AES-256-GCM |
| `v20` | `76 32 30` | 127+ | `Local State` -> `os_crypt.app_bound_encrypted_key` (SYSTEM+User DPAPI) | AES-256-GCM |

**Key recovery chain (v10/v11) on Windows**:
1. Read `Local State` JSON -> `os_crypt.encrypted_key` (Base64 string)
2. Base64-decode the value
3. Strip first 5 bytes (`DPAPI` ASCII prefix: `44 50 41 50 49`)
4. Call `CryptUnprotectData` on the remaining bytes to get the 32-byte AES-256 key
5. For each cookie: strip 3-byte prefix, extract 12-byte nonce, AES-256-GCM decrypt remainder

**macOS key derivation**: Key is stored in macOS Keychain under service name "Chrome Safe Storage". Encryption is PBKDF2-HMAC-SHA1 (1003 iterations, salt "saltysalt") -> AES-128-CBC.

**Linux key derivation**: Hardcoded password `"peanuts"`, PBKDF2-HMAC-SHA1 (1 iteration, salt `"saltysalt"`), AES-128-CBC.

**MITRE ATT&CK**: T1539 (Steal Web Session Cookie), T1555.003 (Credentials from Web Browsers)

---

### 3. Login Data SQLite (`Login Data`)

**Location**: `{Profile}/Login Data`
**Format**: SQLite 3
**Epoch**: WebKit microseconds for all date columns

```sql
CREATE TABLE logins (
    origin_url                      VARCHAR NOT NULL,
    action_url                      VARCHAR,
    username_element                VARCHAR,
    username_value                  VARCHAR,
    password_element                VARCHAR,
    password_value                  BLOB,
    submit_element                  VARCHAR,
    signon_realm                    VARCHAR NOT NULL,
    date_created                    INTEGER NOT NULL,
    blacklisted_by_user             INTEGER NOT NULL,
    scheme                          INTEGER NOT NULL,
    password_type                   INTEGER,
    times_used                      INTEGER,
    form_data                       BLOB,
    display_name                    VARCHAR,
    icon_url                        VARCHAR,
    federation_url                  VARCHAR,
    skip_zero_click                 INTEGER,
    generation_upload_status        INTEGER,
    possible_username_pairs         BLOB,
    id                              INTEGER PRIMARY KEY,
    date_last_used                  INTEGER NOT NULL DEFAULT 0,
    date_password_modified          INTEGER NOT NULL DEFAULT 0,
    keychain_identifier             BLOB DEFAULT ''
);
```

`password_value` uses identical encryption to cookie `encrypted_value` (v10/v20 prefix + AES-256-GCM on Windows).

**Forensic note**: `username_value` is stored in **plaintext** regardless of whether the user saves the password. When `blacklisted_by_user=1`, the user declined to save, but `origin_url` and `username_value` are still recorded — revealing login attempts for credentials never saved.

**MITRE ATT&CK**: T1555.003 (Credentials from Web Browsers)

---

### 4. Web Data SQLite (`Web Data`) — Autofill

**Location**: `{Profile}/Web Data`
**Format**: SQLite 3
**Epoch**: Unix **seconds** (NOT WebKit) for autofill timestamps — critical difference

#### `autofill` Table

```sql
CREATE TABLE autofill (
    name            VARCHAR,
    value           VARCHAR,
    value_lower     VARCHAR,
    date_created    INTEGER DEFAULT 0,
    date_last_used  INTEGER DEFAULT 0,
    count           INTEGER DEFAULT 1
);
```

`date_created` and `date_last_used` are Unix epoch **seconds**. Convert: `ts_ns = date_created * 1_000_000_000`.

#### `autofill_profiles` Table

```sql
CREATE TABLE autofill_profiles (
    guid            VARCHAR PRIMARY KEY,
    company_name    VARCHAR,
    street_address  VARCHAR,
    city            VARCHAR,
    state           VARCHAR,
    zipcode         VARCHAR,
    country_code    VARCHAR,
    date_modified   INTEGER NOT NULL DEFAULT 0,
    use_count       INTEGER NOT NULL DEFAULT 0,
    use_date        INTEGER NOT NULL DEFAULT 0,
    label           VARCHAR
);
```

All timestamps are Unix seconds. Related sub-tables: `autofill_profile_names`, `autofill_profile_emails`, `autofill_profile_phones`.

#### `credit_cards` Table

```sql
CREATE TABLE credit_cards (
    guid                  VARCHAR PRIMARY KEY,
    name_on_card          VARCHAR,
    expiration_month      INTEGER,
    expiration_year       INTEGER,
    card_number_encrypted BLOB,
    date_modified         INTEGER NOT NULL DEFAULT 0,
    use_count             INTEGER NOT NULL DEFAULT 0,
    use_date              INTEGER NOT NULL DEFAULT 0,
    billing_address_id    VARCHAR,
    nickname              VARCHAR
);
```

`card_number_encrypted` uses the same v10/v20 encryption as cookies.

---

### 5. Bookmarks JSON (`Bookmarks`)

**Location**: `{Profile}/Bookmarks` (also `Bookmarks.bak` backup)
**Format**: JSON (UTF-8)
**Epoch**: WebKit microseconds encoded as **decimal strings** (not JSON numbers)

```json
{
    "checksum": "a3f1b2c4d5e6f7a8b9c0d1e2f3a4b5c6",
    "roots": {
        "bookmark_bar": { "...": "..." },
        "other":        { "...": "..." },
        "synced":       { "...": "..." }
    },
    "version": 1
}
```

`checksum` is an MD5 hex string over the concatenation of node ids, titles, and URLs in tree-walk order. A mismatch indicates external modification.

URL node fields: `date_added` (WebKit us as string), `date_last_used`, `guid`, `id`, `name`, `type: "url"`, `url`.
Folder node fields: `children`, `date_added`, `date_modified`, `guid`, `id`, `name`, `type: "folder"`.

`date_added` and `date_modified` are WebKit microseconds encoded as **strings** — parsers must call `s.parse::<i64>()` before epoch conversion.

IDs are assigned sequentially. Gaps in the ID sequence indicate deleted bookmarks.

**MITRE ATT&CK**: T1217 (Browser Bookmark Discovery)

---

### 6. Extensions Directory (`Extensions/`)

**Location**: `{Profile}/Extensions/{extension_id}/{version}/manifest.json`

Key `manifest.json` fields:

| Field | Forensic Significance |
|-------|-----------------------|
| `manifest_version` | 2 (legacy MV2) or 3 (current MV3) |
| `permissions` | API permissions: `"cookies"`, `"tabs"`, `"webRequest"` |
| `host_permissions` | URL match patterns; `"<all_urls>"` = access to all sites |
| `background.service_worker` | MV3 background script |
| `content_scripts[].matches` | URL patterns where scripts inject |

**High-risk combinations**: `"cookies"` + `"<all_urls>"` = all cookies. `"webRequest"` + `"<all_urls>"` = all traffic interception. `"nativeMessaging"` = native process communication.

**Install time**: Not in `manifest.json`. From `{Profile}/Preferences` -> `extensions.settings.{id}.install_time` (WebKit us as string).

---

### 7. Cache (SimpleCache Format)

**Location**: `{Profile}/Cache/Cache_Data/` (Chrome 97+)
**Format**: Proprietary binary (Very Simple Backend)

Entry filenames: `{16-hex-digit-hash}_{stream_number}`. Stream 0 = HTTP response headers. Stream 1 = response body.

**EOF Record** (last 20-28 bytes of each entry file):
- Bytes 0–3: final magic number
- Bytes 4–7: flags
- Bytes 8–11: `key_size` (u32 LE) — URL length
- Bytes 12–15: key CRC32
- Bytes 16–19: stream 0 size

**URL extraction**: Read `key_size` from bytes 8–11, then read `key_size` bytes immediately preceding the EOF record.

---

### 8. Local State JSON

**Location**: `{Chrome User Data}/Local State`

Key forensic fields:

| JSON Path | Description |
|-----------|-------------|
| `os_crypt.encrypted_key` | Base64-encoded DPAPI-wrapped AES-256 key (v10 decryption) |
| `os_crypt.app_bound_encrypted_key` | Base64-encoded app-bound key (v20 decryption) |
| `profile.info_cache.{ProfileDir}.name` | Profile display name |
| `profile.info_cache.{ProfileDir}.gaia_id` | Google Account ID |
| `profile.info_cache.{ProfileDir}.user_name` | Google Account email |
| `profile.last_used` | Last-used profile directory name |
| `browser.last_known_google_account` | Signed-in Google account email |
| `sync.has_setup_completed` | Whether Chrome Sync is configured |
| `uninstall_metrics.installation_date2` | Chrome installation timestamp |

---

### 9. Preferences JSON

**Location**: `{Profile}/Preferences`

Key forensic fields:

| JSON Path | Description |
|-----------|-------------|
| `extensions.settings.{id}.install_time` | Extension install time (WebKit us as string) |
| `extensions.settings.{id}.state` | 1 = enabled, 0 = disabled |
| `download.default_directory` | Default download location |
| `partition.per_host_zoom_levels` | Per-site zoom levels (reveals visited domains) |
| `profile.created_by_version` | Chrome version that created this profile |

---

### 10. Session / Tab Recovery (SNSS Format)

**Location (Chrome 86+)**: `{Profile}/Sessions/Session_{timestamp}` and `{Profile}/Sessions/Tabs_{timestamp}`
**Location (Chrome < 86)**: `Current Session`, `Last Session`, `Current Tabs`, `Last Tabs`
**Format**: SNSS binary (NOT SQLite, NOT JSON)

```
Offset  Size  Endian  Field
0       4     --      Magic: "SNSS" (0x53 0x4E 0x53 0x53)
4       4     LE      Version: always 1 (u32)

--- Repeated command records ---
N       2     LE      Command payload size in bytes (u16)
N+2     1     --      Command ID (u8)
N+3     P     --      Command payload (Pickle-serialized, P = payload size - 1)
```

Key Command IDs:

| ID | Name | Payload |
|----|------|---------|
| 6 | `UpdateTabNavigation` | URL, title, transition, referrer, page state |
| 7 | `SetSelectedNavigationIndex` | Active back/forward position |
| 8 | `SetSelectedTabInIndex` | Active tab in window |
| 21 | `SetPinnedState` | Whether tab is pinned |

**Forensic significance**: Session files capture the full back/forward navigation stack per tab. Persist even after the user clears browsing history. `Last Session`/`Last Tabs` capture state at last clean shutdown.

---

## Firefox

### 11. places.sqlite (History + Bookmarks + Downloads)

**Location**:
- Windows: `%APPDATA%\Mozilla\Firefox\Profiles\{profile}.default\places.sqlite`
- macOS: `~/Library/Application Support/Firefox/Profiles/{profile}.default/places.sqlite`
- Linux: `~/.mozilla/firefox/{profile}.default/places.sqlite`

**Format**: SQLite 3, WAL mode
**Epoch**: Unix microseconds (PRTime format)

#### `moz_places` Table

```sql
CREATE TABLE moz_places (
    id                  INTEGER PRIMARY KEY,
    url                 LONGVARCHAR,
    title               LONGVARCHAR,
    rev_host            LONGVARCHAR,
    visit_count         INTEGER DEFAULT 0,
    hidden              INTEGER DEFAULT 0 NOT NULL,
    typed               INTEGER DEFAULT 0 NOT NULL,
    frecency            INTEGER DEFAULT -1 NOT NULL,
    last_visit_date     INTEGER,
    guid                TEXT,
    foreign_count       INTEGER DEFAULT 0 NOT NULL,
    url_hash            INTEGER DEFAULT 0 NOT NULL,
    description         TEXT,
    preview_image_url   TEXT,
    origin_id           INTEGER
);
```

| Column | Forensic Significance |
|--------|----------------------|
| `rev_host` | Reversed+dotted hostname (`moc.elpmaxe.` for `example.com`) |
| `typed` | 1 = user typed this URL directly |
| `frecency` | Frequency+recency score for omnibox ranking |
| `foreign_count` | Count of bookmarks referencing this place; `> 0` = currently bookmarked |
| `hidden` | 1 = redirect or subframe (not visited by user directly) |

#### `moz_historyvisits` Table

```sql
CREATE TABLE moz_historyvisits (
    id          INTEGER PRIMARY KEY,
    from_visit  INTEGER,
    place_id    INTEGER,
    visit_date  INTEGER,
    visit_type  INTEGER,
    session     INTEGER DEFAULT 0
);
```

##### `visit_type` Enum

| Value | Name | Description |
|-------|------|-------------|
| 1 | `TRANSITION_LINK` | User clicked a hyperlink |
| 2 | `TRANSITION_TYPED` | User typed the URL |
| 3 | `TRANSITION_BOOKMARK` | User clicked a bookmark |
| 4 | `TRANSITION_EMBED` | Embedded content (not a top-level visit) |
| 5 | `TRANSITION_REDIRECT_PERMANENT` | HTTP 301 redirect |
| 6 | `TRANSITION_REDIRECT_TEMPORARY` | HTTP 302 redirect |
| 7 | `TRANSITION_DOWNLOAD` | URL was downloaded |
| 8 | `TRANSITION_FRAMED_LINK` | User followed link in a frame |
| 9 | `TRANSITION_RELOAD` | Page was reloaded |

#### `moz_bookmarks` Table

```sql
CREATE TABLE moz_bookmarks (
    id                INTEGER PRIMARY KEY,
    type              INTEGER,
    fk                INTEGER DEFAULT NULL,
    parent            INTEGER,
    position          INTEGER,
    title             LONGVARCHAR,
    keyword_id        INTEGER,
    dateAdded         INTEGER,
    lastModified      INTEGER,
    guid              TEXT,
    syncStatus        INTEGER NOT NULL DEFAULT 0,
    syncChangeCounter INTEGER NOT NULL DEFAULT 1
);
```

| `type` Value | Meaning |
|-------------|---------|
| 1 | `TYPE_BOOKMARK` — URL bookmark; `fk` -> `moz_places.id` |
| 2 | `TYPE_FOLDER` — Container |
| 3 | `TYPE_SEPARATOR` — Visual separator |

#### `moz_bookmarks_deleted` Table

```sql
CREATE TABLE moz_bookmarks_deleted (
    guid        TEXT PRIMARY KEY,
    dateRemoved INTEGER NOT NULL DEFAULT 0
);
```

Persists GUIDs and deletion timestamps for bookmarks removed since last sync. An explicit anti-forensic detection artifact — deletion is recorded even after removal from `moz_bookmarks`.

#### `moz_inputhistory` Table

```sql
CREATE TABLE moz_inputhistory (
    place_id  INTEGER NOT NULL,
    input     LONGVARCHAR NOT NULL,
    use_count INTEGER,
    PRIMARY KEY (place_id, input)
);
```

Stores literal text the user typed in the address bar. Reveals actual user input even when it differs from the final URL.

#### Downloads via `moz_annos` / `moz_anno_attributes`

Downloads are recorded as annotations on `moz_places` entries:
- `downloads/destinationFileURI` — local file path
- `downloads/metaData` — JSON: `{"state":1,"endTime":1648000000000,"fileSize":1048576}`

---

### 12. cookies.sqlite

**Format**: SQLite 3, WAL mode
**Epoch**: `creationTime` and `lastAccessed` = Unix **microseconds**; `expiry` = Unix **seconds**

```sql
CREATE TABLE moz_cookies (
    id                        INTEGER PRIMARY KEY,
    originAttributes          TEXT NOT NULL DEFAULT '',
    name                      TEXT,
    value                     TEXT,
    host                      TEXT,
    path                      TEXT,
    expiry                    INTEGER,
    lastAccessed              INTEGER,
    creationTime              INTEGER,
    isSecure                  INTEGER,
    isHttpOnly                INTEGER,
    sameSite                  INTEGER DEFAULT 0,
    rawSameSite               INTEGER DEFAULT 0,
    schemeMap                 INTEGER DEFAULT 0,
    isPartitionedAttributeSet INTEGER DEFAULT 0,
    CONSTRAINT moz_uniqueid UNIQUE (name, host, path, originAttributes)
);
```

**Critical mixed-unit alert**: `expiry` is Unix **seconds**, while `creationTime` and `lastAccessed` are Unix **microseconds**. The unit difference is 1,000,000x.

**Firefox cookie values are stored in plaintext** in the `value` column. No encryption.

`originAttributes`: Firefox container context. Empty = default. Format: `^userContextId=N` where N is container ID.

**MITRE ATT&CK**: T1539 (Steal Web Session Cookie)

---

### 13. formhistory.sqlite (Autofill)

**Epoch**: Unix microseconds for `firstUsed` and `lastUsed`

```sql
CREATE TABLE moz_formhistory (
    id          INTEGER PRIMARY KEY,
    fieldname   TEXT NOT NULL,
    value       TEXT NOT NULL,
    timesUsed   INTEGER,
    firstUsed   INTEGER,
    lastUsed    INTEGER,
    guid        TEXT
);
```

`fieldname` is the HTML `name` attribute. High forensic value: recovers search queries, usernames, and other typed PII.

---

### 14. logins.json (Credentials)

**Epoch**: Unix milliseconds for all timestamp fields

```json
{
    "logins": [{
        "id": 1,
        "hostname": "https://example.com",
        "formSubmitURL": "https://example.com/login",
        "usernameField": "email",
        "passwordField": "password",
        "encryptedUsername": "MDIEEPg...",
        "encryptedPassword": "MDIEEPg...",
        "guid": "{xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx}",
        "encType": 1,
        "timeCreated": 1648000000000,
        "timeLastUsed": 1650000000000,
        "timePasswordChanged": 1648000000000,
        "timesUsed": 5
    }]
}
```

**Encryption**: 3DES-CBC, ASN.1 DER encoded, then Base64. Key stored in `key4.db` (SQLite, Firefox 58+). If user has a Primary Password, key4.db is additionally encrypted via PBKDF2 with the master password.

**Firefox 144+**: Updated NSS encryption. Older decryption tools may fail without updating to libnss3 >= 3.113.

**MITRE ATT&CK**: T1555.003 (Credentials from Web Browsers)

---

### 15. extensions.json

**Epoch**: Unix milliseconds for `installDate`, `updateDate`

Key fields per addon: `id`, `version`, `type`, `location`, `active`, `userDisabled`, `installDate`, `sourceURI`, `signedState`, `foreignInstall`, `userPermissions.permissions`, `userPermissions.origins`.

**`location` values**:

| Value | Meaning |
|-------|---------|
| `app-profile` | User-installed from AMO or local file |
| `winreg-app-global` | Installed via Windows Registry (enterprise) |
| `temporary-install` | Developer sideload (removed on restart) |

**`signedState` values**: -1=NOT_REQUIRED, 0=MISSING, 1=PRELIMINARY, 2=SIGNED, 3=SYSTEM, 4=PRIVILEGED.

`foreignInstall: true` + `signedState: 0` is a strong indicator of malicious sideloaded extension.

---

### 16. sessionstore.jsonlz4 / recovery.jsonlz4

**Locations**:
- `{Profile}/sessionstore.jsonlz4` — created on clean shutdown
- `{Profile}/sessionstore-backups/recovery.jsonlz4` — updated every ~15 seconds while running
- `{Profile}/sessionstore-backups/recovery.baklz4` — previous recovery backup
- `{Profile}/sessionstore-backups/previous.jsonlz4` — last clean shutdown state

**Format**: Mozilla's custom mozLz4 framing (NOT standard LZ4 frame format)

#### mozLz4 Binary Layout

```
Offset  Size   Endian  Field
0       8      --      Magic: "mozLz40\0" (0x6D 0x6F 0x7A 0x4C 0x7A 0x34 0x30 0x00)
8       4      LE      Uncompressed size of JSON data (u32)
12      N      --      LZ4 block-compressed JSON (raw LZ4 block, no frame header)
```

**Critical**: Uses LZ4 **block** format (raw), NOT LZ4 **frame** format. Use `lz4_flex::block::decompress(&data[12..], uncompressed_size)`.

Key decompressed JSON fields:

| Field | Epoch | Description |
|-------|-------|-------------|
| `tabs[].lastAccessed` | Unix ms | Last time the user switched to this tab |
| `tabs[].entries[]` | -- | Back/forward history stack per tab |
| `tabs[].userContextId` | -- | Firefox Multi-Account Container ID |
| `session.lastUpdate` | Unix ms | When session file was last written |
| `_closedTabs[]` | -- | Explicitly closed tabs (restorable via Ctrl+Shift+T) |
| `_closedWindows[]` | -- | Explicitly closed windows |

**Forensic significance**: Session files survive `Ctrl+Shift+Del`. `_closedTabs` may reveal URLs closed before history was committed to `places.sqlite`. `recovery.jsonlz4` captures near-live state.

---

### 17. Firefox Cache2 (`cache2/entries/`)

**Location**: `{Profile}/cache2/entries/`
**Format**: Proprietary binary per-entry files

Entry filename = uppercase hex SHA-1 of the URL (cache key).

**Metadata at end of file** (read last 4 bytes as BE u32 -> `metadata_offset`):

```
[metadata_offset position:]
  4 bytes  version (u32 LE)
  4 bytes  fetch_count (u32 LE)
  4 bytes  last_fetched (u32 LE, Unix seconds)
  4 bytes  last_modified (u32 LE, Unix seconds)
  4 bytes  frecency (u32 LE)
  4 bytes  expiration (u32 LE, Unix seconds)
  4 bytes  key_size (u32 LE)
  N bytes  key (URL string)
  variable HTTP response headers (text, line-delimited)
```

---

## Safari

### 18. History.db

**Location**: `~/Library/Safari/History.db`
**Format**: SQLite 3, WAL mode
**Epoch**: Core Data / Mac Absolute Time (float64 seconds since 2001-01-01 UTC)
**Conversion**: `unix_ns = (core_data_secs + 978_307_200) * 1_000_000_000`

#### `history_items` Table

```sql
CREATE TABLE history_items (
    id                                    INTEGER PRIMARY KEY AUTOINCREMENT,
    url                                   TEXT NOT NULL UNIQUE,
    domain_expansion                      TEXT,
    visit_count                           INTEGER NOT NULL,
    daily_visit_counts                    BLOB,
    weekly_visit_counts                   BLOB,
    autocomplete_triggers                 TEXT,
    should_recompute_derived_visit_counts INTEGER,
    visit_count_score                     INTEGER NOT NULL DEFAULT 0,
    status_code                           INTEGER NOT NULL DEFAULT 0
);
```

#### `history_visits` Table

```sql
CREATE TABLE history_visits (
    id                   INTEGER PRIMARY KEY AUTOINCREMENT,
    history_item         INTEGER NOT NULL REFERENCES history_items(id) ON DELETE CASCADE,
    visit_time           REAL NOT NULL,
    title                TEXT,
    load_successful      BOOLEAN NOT NULL DEFAULT 1,
    http_non_get         BOOLEAN NOT NULL DEFAULT 0,
    synthesized          BOOLEAN NOT NULL DEFAULT 0,
    redirect_source      INTEGER UNIQUE REFERENCES history_visits(id),
    redirect_destination INTEGER UNIQUE REFERENCES history_visits(id),
    origin               INTEGER NOT NULL DEFAULT 0,
    generation           INTEGER NOT NULL DEFAULT 0,
    attributes           INTEGER NOT NULL DEFAULT 0,
    score                INTEGER NOT NULL DEFAULT 0
);
```

| Column | Forensic Significance |
|--------|----------------------|
| `visit_time` | Core Data float64 seconds |
| `http_non_get` | 1 = POST/PUT/DELETE (form submission indicator) |
| `synthesized` | 1 = programmatic (redirect/prefetch, not direct user navigation) |
| `redirect_source` / `redirect_destination` | Redirect chain reconstruction |

#### `history_tombstones` Table

```sql
CREATE TABLE history_tombstones (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    start_time  REAL NOT NULL,
    end_time    REAL NOT NULL,
    url         TEXT,
    generation  INTEGER NOT NULL DEFAULT 0
);
```

**Most explicit anti-forensic detection artifact of any major browser.** When a user clears history, Safari creates tombstone records rather than simply deleting rows.

| Pattern | Interpretation |
|---------|---------------|
| `url IS NULL`, wide time range | User cleared history for a preset period |
| `url IS NOT NULL`, `start_time == end_time` | Single specific URL deleted |
| `url IS NULL`, `start_time == 0` | "Clear All History" |
| Multiple tombstones, same `generation` | Batch deletion in one operation |
| `end_time` close to now | Very recent deletion |

Tombstones persist even after subsequent history clears.

---

### 19. Cookies.binarycookies (Binary Cookie Format)

**Location**: `~/Library/Cookies/Cookies.binarycookies`
**Format**: Custom Apple binary format (NOT SQLite, NOT plist)
**Epoch**: Mac Absolute Time (float64 seconds since 2001-01-01 UTC)

**Note**: Modern macOS Safari may use `~/Library/Cookies/Cookies.db` (SQLite).

#### Complete Binary Layout

```
=== File Header ===
Offset  Size  Endian  Field
0       4     --      Magic: "cook" (0x63 0x6F 0x6F 0x6B)
4       4     BE      Number of pages (u32)
8       N*4   BE      Page sizes array (N u32 values, one per page)

=== Page N ===
0       4     LE      Page header magic: 0x00000100
4       4     LE      Number of cookies in this page (u32)
8       M*4   LE      Cookie offsets: M u32 values (byte offset from page start)

=== Cookie Record ===
Offset  Size  Endian  Field
0       4     LE      Record size in bytes (u32)
4       4     LE      Unknown/version (u32)
8       4     LE      Flags bitmask: bit 0=Secure, bit 2=HTTPOnly
12      4     LE      Unknown (u32)
16      4     LE      Byte offset to domain string
20      4     LE      Byte offset to name string
24      4     LE      Byte offset to path string
28      4     LE      Byte offset to value string
32      4     LE      Byte offset to comment string (0 if none)
36      4     --      Reserved (0x00000000)
40      8     LE      Expiry date (f64, Mac Absolute Time)
48      8     LE      Creation date (f64, Mac Absolute Time)
56+     var   --      Null-terminated strings (domain, name, path, value, comment)

=== File Footer ===
8       --      SHA-1 checksum of preceding content
```

**Endianness warning**: File header (page count, page sizes) uses **big-endian**. Page internals and cookie records use **little-endian**. This is the most common source of parser bugs.

---

### 20. Downloads.plist

**Location**: `~/Library/Safari/Downloads.plist`
**Format**: Binary plist
**Epoch**: `NSDate` = Mac Absolute Time (float64 seconds since 2001-01-01)

Key fields per entry:

| Key | Description |
|-----|-------------|
| `DownloadEntryURL` | Source URL |
| `DownloadEntryPath` | Local filesystem destination path |
| `DownloadEntryDateAddedKey` | Mac Absolute Time when download started |
| `DownloadEntryDateFinishedKey` | Mac Absolute Time when download completed |
| `DownloadEntryProgressTotalToLoad` | Total bytes (-1 if unknown) |
| `DownloadEntryProgressBytesSoFar` | Bytes downloaded at time of record |

---

### 21. Bookmarks.plist

**Location**: `~/Library/Safari/Bookmarks.plist`
**Format**: Binary plist (recursive tree)

Tree structure: Root (WebBookmarkTypeList) -> Children[] -> BookmarksBar, BookmarksMenu, com.apple.ReadingList.

URL node (`WebBookmarkTypeLeaf`): `WebBookmarkUUID`, `URLString`, `URIDictionary.title`.

Reading List entries additionally contain `ReadingList.DateAdded` (NSDate), `ReadingList.PreviewText`.

**MITRE ATT&CK**: T1217 (Browser Bookmark Discovery)

---

### 22. Safari Extensions

**Location (Legacy, Safari < 14)**: `~/Library/Safari/Extensions/Extensions.plist`
**Location (Modern, Safari 14+)**: Extensions as `.appex` bundles within `.app` packages (App Store installed)

Legacy `Extensions.plist` key fields per extension: `Archive File Name`, `Bundle Identifier`, `Display Name`, `Enabled`, `Developer Identifier`.

---

## Anti-Forensic Detection Indicators

### Chrome/Chromium

| Indicator | Detection Method |
|-----------|-----------------|
| History row deletions | Gaps in `visits.id` auto-increment sequence |
| Visit count mismatch | `urls.visit_count` > actual matching rows in `visits` |
| Full history clear | `urls` and `visits` empty but `meta` table `last_visit_time` > 0 |
| Incognito mode use | No SQLite artifacts; detect via DNS cache, prefetch files |
| WAL-resident deleted records | `History-wal` contains pre-commit versions of deleted pages |
| Chrome Sync preservation | `Sync Data/LevelDB/` may retain synced history deleted locally |
| Cookie clearing signatures | `Cookies-wal` may retain pre-deletion records |

### Firefox

| Indicator | Detection Method |
|-----------|-----------------|
| History row deletions | Gaps in `moz_historyvisits.id` sequence |
| Visit count mismatch | `moz_places.visit_count` > count of `moz_historyvisits` rows |
| Bookmark deletions | `moz_bookmarks_deleted` table — explicit deletion log |
| Private browsing leaks | `content-prefs.sqlite`, `permissions.sqlite` may record domains from private mode |
| WAL-resident data | `places.sqlite-wal`, `cookies.sqlite-wal` |
| Session file divergence | `recovery.jsonlz4` contains URLs absent from `places.sqlite` |

### Safari

| Indicator | Detection Method |
|-----------|-----------------|
| History deletion (range) | `history_tombstones` rows with `url IS NULL` |
| History deletion (specific URL) | `history_tombstones` rows with `url IS NOT NULL` |
| Full history clear | Tombstones spanning entire timeframe |
| Private browsing | No `History.db` entries; detect via DNS cache, keychain entries |
| WAL artifacts | `History.db-wal` may retain visits after deletion |
| Preferences evidence | `com.apple.Safari.plist` may contain `ClearedBrowserDataDateAndTime` |

---

## Memory-Resident Browser Artifacts

### Chrome/Chromium Process Memory

| C++ Class | Contains | Forensic Value |
|-----------|----------|----------------|
| `content::NavigationEntry` | URL, title, transition type, timestamp, page state | Full tab history including back/forward entries not in SQLite |
| `net::CookieMonster` | Decrypted in-memory cookie store | All cookies in cleartext — only way to recover without DPAPI key |
| `password_manager::PasswordStore` | Decrypted credentials | Plaintext passwords while browser runs |
| `autofill::PersonalDataManager` | Autofill profiles, credit card numbers (decrypted) | PII including full card numbers |

**Heap scanning**: Chrome V8 strings preceded by map pointer and length. Search for `https://` or `http://` byte sequences.

### Firefox Process Memory

| C++ Class | Contains |
|-----------|----------|
| `nsNavHistory` | History database LRU visit cache |
| `nsCookieService` | Active cookie list (`nsCookie` objects with plaintext values) |
| SpiderMonkey GC heap | JavaScript heap including `document.cookie` strings |

### Safari/WebKit Process Memory

| C++ Class | Contains |
|-----------|----------|
| `WebCore::HistoryItem` | URL, title, scroll position, visit time |
| `WebCore::CookieJar` | In-memory cookie store with plaintext values |
| `WebCore::ResourceRequest` | Full HTTP request: URL, method, headers |

---

## Cross-Browser Correlation Opportunities

| Correlation Vector | Chrome Fields | Firefox Fields | Safari Fields |
|-------------------|---------------|----------------|---------------|
| Same URL visited | `urls.url` | `moz_places.url` | `history_items.url` |
| Same cookie domain | `cookies.host_key` | `moz_cookies.host` | Cookie domain field |
| Same download URL | `downloads_url_chains.url` | `moz_annos` content | `Downloads.plist DownloadEntryURL` |
| Timestamp (all to Unix ns) | WebKit us -> Unix ns | Unix us -> Unix ns | Core Data s -> Unix ns |
| Same credential site | `logins.origin_url` + `username_value` | `logins[].hostname` | Keychain (external) |
| Search terms | `keyword_search_terms.term` | `moz_formhistory.value` | N/A |
| Import evidence | `visit_source.source` = 3/4/5 | N/A | N/A |

---

## References

### Official Source Code

- Chromium History schema: `components/history/core/browser/history_database.cc` at chromium.googlesource.com
- Chromium page transition types: `ui/base/page_transition_types.h`
- Chromium SimpleCache design: [Very Simple Backend](https://www.chromium.org/developers/design-documents/network-stack/disk-cache/very-simple-backend/)
- Firefox places schema: `toolkit/components/places/Database.cpp` at hg.mozilla.org
- Firefox cookie schema: `netwerk/cookie/CookiePersistentStorage.cpp`
- Firefox sessionstore: `browser/components/sessionstore/`

### Forensic Tools

- [Hindsight](https://github.com/obsidianforensics/hindsight) — Chromium browser artifact parser (Python)
- [FQLite](https://github.com/pawlaszczyk/fqlite) — SQLite deleted record recovery
- [firefox_decrypt](https://github.com/unode/firefox_decrypt) — Firefox `logins.json` credential extraction
- [BinaryCookieReader](https://github.com/as0ler/BinaryCookieReader) — Safari `Cookies.binarycookies` parser
- [ccl-ssns](https://github.com/cclgroupltd/ccl-ssns) — Chrome SNSS session file parser
- [Chrome App-Bound Encryption Research](https://github.com/xaitax/Chrome-App-Bound-Encryption-Decryption/blob/main/docs/RESEARCH.md)

### Research References

- [Chrome Values Lookup Tables](https://dfir.blog/chrome-values-lookup-tables/) — Complete enum reference
- [Chrome Transition Values](https://dfir.blog/chrome-transition-values/) — Transition bitmask analysis
- [Forensic Analysis of SQLite Databases](https://belkasoft.com/sqlite-analysis) — Belkasoft
- [Analysing Firefox Session Restore Data](https://www.foxtonforensics.com/blog/post/analysing-firefox-session-restore-data-mozlz4-jsonlz4) — Foxton Forensics
- [Forensic Analysis of iOS Binary Cookie Files](https://onlinelibrary.wiley.com/doi/abs/10.1111/1556-4029.15499) — Journal of Forensic Sciences, 2024

### MITRE ATT&CK Techniques

- **T1217** — Browser Information Discovery
- **T1539** — Steal Web Session Cookie
- **T1555.003** — Credentials from Web Browsers
- **T1005** — Data from Local System
- **T1070.004** — Indicator Removal: File Deletion
