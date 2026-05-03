# /review-dfir-feeds

Review unreviewed DFIR blog posts from `archive/sources/pending-review.md` and
extract artifact findings for the forensicnomicon catalog.

## Steps

1. Read `archive/sources/pending-review.md` — collect all `- [ ]` items
2. If none pending: report "Nothing pending" and stop
3. For each unchecked item (batch at most 10 per session):
   a. Use `mcp__plugin_context-mode_context-mode__ctx_fetch_and_index` to fetch the URL
   b. Use `ctx_search` to extract:
      - Windows registry key paths containing GUIDs
      - New LOLBins / LOLBAS entries not in `src/lolbins.rs`
      - MITRE ATT&CK technique IDs
      - Forensic artifact names (UserAssist, Prefetch, MFT, etc.)
   c. For each finding: grep `src/catalog/descriptors/` and `src/lolbins.rs`
      to check if already covered
   d. If gap found: create a TDD task with artifact ID, key_path/file, source URL
   e. Mark item `[→]` (task created) or `[x]` (reviewed, no gaps)
4. Write updated `pending-review.md` back with checkboxes updated
5. Commit: `chore(feeds): mark N posts reviewed — M gaps found`
6. Print findings table

## Finding extraction patterns

| Look for | Example | Check against |
|---|---|---|
| Registry GUID key | `{CEBFF5CD-...}\Count` | `key_path` fields in descriptors |
| LOLBin name | `certutil.exe`, `curl` | `LOLBAS_WINDOWS` / `LOLBAS_LINUX` / `LOLBAS_MACOS` |
| ATT&CK ID | `T1547.001` | `mitre_techniques` in descriptors |
| File artifact | `$MFT`, `Prefetch`, `SRUM` | catalog by `id` / `name` |
| New GUID | not in any descriptor | new descriptor needed |

## Rules

- Never mark `[x]` for a URL you couldn't fetch — leave `[ ]` and note the error
- Every proposed task needs: artifact ID, source URL, OS scope, key_path or file_path
- All `// Source:` comments must be URLs, not vendor names
- Verify findings against actual code before creating tasks
