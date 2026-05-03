# /review-dfir-feeds

Review unreviewed DFIR blog posts from `archive/sources/pending-review.md` and
extract artifact findings for the forensicnomicon catalog.

## Steps

1. Read `archive/sources/pending-review.md` — collect all `- [ ]` and `- [!]` items
2. If none pending: report "Nothing pending" and stop
3. For each unchecked item (batch at most 10 per session):
   a. **`[!]` items (broken URL):** Before giving up, search the source domain for
      the article title using WebSearch (`site:<domain> "<title>"`). If a working
      URL is found, replace the URL in the line and treat it as a normal `[ ]` item.
      If no working URL found, mark `[x]` with a note "<!-- dead link, no mirror found -->"
      and skip fetching.
   b. **`[ ]` items:** Use `mcp__plugin_context-mode_context-mode__ctx_fetch_and_index`
      to fetch the URL
   c. Use `ctx_search` to extract:
      - Windows registry key paths containing GUIDs
      - New LOLBins / LOLBAS entries not in `src/lolbins.rs`
      - MITRE ATT&CK technique IDs
      - Forensic artifact names (UserAssist, Prefetch, MFT, etc.)
      - **Co-occurring artifact pairs** (e.g. post discusses ShimCache + Prefetch
        together → both are `related` candidates for each other's descriptor)
   d. For each finding: grep `src/catalog/descriptors/` and `src/lolbins.rs`
      to check if already covered
   e. For co-occurrences: check whether the `related` field in each descriptor
      already lists the co-occurring artifact. If not, note it as a `related`
      enrichment task (lower priority than new artifact gaps).
   f. If gap found: create a TDD task with artifact ID, key_path/file, source URL
   g. Mark item `[→]` (task created) or `[x]` (reviewed, no gaps)
4. Write updated `pending-review.md` back with checkboxes updated
5. Commit: `chore(feeds): mark N posts reviewed — M gaps found`
6. Print findings table

## Co-occurrence extraction (for `related` field enrichment)

After fetching each post, run `extract_related_artifacts(text)` from
`scripts/backfill_archives.py` against the fetched content. This function
uses `_ARTIFACT_PHRASES` — a 50-phrase lookup table mapping natural-language
terms to catalog IDs:

```
"shimcache"         → shimcache
"prefetch"          → prefetch_dir
"userassist"        → userassist_exe
"amcache"           → amcache_hve
"lnk file"          → lnk_file
"evtx"              → evtx_security
"srum"              → srudb
"usnjrnl"           → usnjrnl
"$mft"              → mft_file
"lsass"             → lsass_dump
"rdp"               → evtx_rdp_auth
"btm"               → fa_file_com_apple_backgroundtaskmanagement_backgrounditems_v
… (see _ARTIFACT_PHRASES in backfill_archives.py for full list)
```

For each pair of co-occurring artifact IDs returned, run
`check_related_gaps(artifact_id, co_occurring_ids)` to find which links are
missing from the descriptor's `related` array. Flag these as enrichment tasks.

For YouTube entries: `fetch_youtube_transcript(video_id)` returns the spoken
transcript (via youtube-transcript-api). Pass it to `extract_related_artifacts()`
instead of the sparse HTML page — transcripts contain far more artifact signal.

The `related` field builds an investigation graph: which artifacts
corroborate each other for the same TTP. Blog posts and video transcripts
are the primary real-world source for these relationships.

## Finding extraction patterns

| Look for | Example | Check against |
|---|---|---|
| Registry GUID key | `{CEBFF5CD-...}\Count` | `key_path` fields in descriptors |
| LOLBin name | `certutil.exe`, `curl` | `LOLBAS_WINDOWS` / `LOLBAS_LINUX` / `LOLBAS_MACOS` |
| ATT&CK ID | `T1547.001` | `mitre_techniques` in descriptors |
| File artifact | `$MFT`, `Prefetch`, `SRUM` | catalog by `id` / `name` |
| New GUID | not in any descriptor | new descriptor needed |

## URL status markers

| Marker | Meaning | Action |
|--------|---------|--------|
| `[ ]`  | Unreviewed, URL assumed valid | Fetch and review |
| `[!]`  | URL returned 404/410 at accumulation time | Search for mirror before giving up |
| `[→]`  | Reviewed — TDD task created | Skip |
| `[x]`  | Reviewed — no gaps found | Skip |

## Rules

- Never mark `[x]` for a URL you couldn't fetch — leave `[ ]` and note the error
- For `[!]` items: try `site:<domain> "<title>"` WebSearch first; only mark `[x]` dead if no mirror found
- Every proposed task needs: artifact ID, source URL, OS scope, key_path or file_path
- All `// Source:` comments must be URLs, not vendor names
- Verify findings against actual code before creating tasks
