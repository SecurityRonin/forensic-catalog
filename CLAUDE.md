# CLAUDE.md

## Project Constitution

### Who this is for

**Primary:** DFIR analysts and SOC engineers who need to answer "is this binary abusable?" or "what does this artifact mean?" during an investigation — without opening a browser.

**Secondary:** Security engineers building detection pipelines, SOAR playbooks, or EDR enrichment in Rust who need a zero-dependency artifact catalog they can embed.

**Tertiary:** Threat hunters mapping TTPs to observable artifacts, pen testers checking what's available on a target system.

### What success looks like

A DFIR analyst on an active IR:
1. Runs `4n6query certutil.exe` and gets the full picture in under 50ms — what it does, what ATT&CK techniques apply, whether it downloads files or bypasses defenses
2. Runs `4n6query userassist` and gets every variant with decoded field schemas and triage priority
3. Runs `4n6query --triage` and knows exactly which artifacts to pull first in a constrained collection window

A detection engineer embeds `forensicnomicon` into their rule-generation pipeline and never hand-maintains a list of LOLBins or artifact paths again.

### What the 10-second README hook must answer

1. "Is this for me?" — DFIR analyst or security engineer, yes
2. "What does it do that MITRE ATT&CK or lolbas-project.github.io doesn't?" — Offline, enriched (decoders + triage), all 6 datasets unified
3. "Can I try it right now?" — Yes, `brew install SecurityRonin/tap/4n6query`

### What we are NOT building

- A full parsing engine — decoders live here only for compact stable formats (UserAssist, MRUListEx, FILETIME, REG_MULTI_SZ, PCA). Large evolving parsers belong in companion crates.
- A cloud-dependent product — all data compiles into the binary; zero I/O at runtime
- A GUI or web app (though GH Pages search widget is in scope as a read-only reference tool)
- A threat intelligence feed — we document known artifact properties; we don't track live IOCs

### Audience expansion to watch

Beyond DFIR analysts and SOC engineers:
- **Red teamers / pentesters** — dual-use: 4n6query tells them what's available to abuse
- **Malware analysts** — artifact decoders useful for static triage
- **DFIR instructors** — catalog is a teaching reference
- **Forensic tool developers** — Python/PyO3 bindings (future) enables SOAR integration

---

## Scope Boundary

This project is a forensic catalog first, not a full DFIR parsing engine.

- Keep `ArtifactDescriptor` for artifact location, significance, field schema, ATT&CK mapping, triage value, and authoritative citations.
- Keep `ArtifactParsingProfile` for format knowledge and analyst guidance that does not fit a small stable decoder.
- Implement in-core decoders only for compact, stable encodings where the logic is intrinsic to the artifact model, such as `UserAssist`, `MRUListEx`, `FILETIME`, `REG_MULTI_SZ`, or PCA record layouts.
- Do not keep pushing large or evolving formats such as `hiberfil.sys`, BITS job stores, or full WMI repository parsing into this crate's core decode engine.
- If execution-grade parsers are needed later, put them in a separate parsing module or companion crate rather than turning the catalog itself into a full parser framework.

### Feed review scope boundary

When reviewing blog posts, vendor announcements, and podcast content from the pending queue, apply the same boundary:

**In scope — implement into the catalog:**
- Artifact location, format, field schema, or forensic interpretation (new or corrected)
- Heuristic predicates: detection thresholds, anomaly patterns, behavioral signals derived from artifact values
- ATT&CK technique mappings based on artifact evidence
- Tool-to-artifact relationships where the *artifact* is the primary subject (e.g. "RECmd parses the MRU list at HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\ComDlg32\OpenSavePidlMRU" — the MRU path is in scope, the tool name is not)

**Out of scope — mark reviewed/0 artifacts, do not implement:**
- Vendor acquisition method announcements ("Passware now decrypts Samsung S10") — the *device image* artifact is already in scope; the vendor's capability to extract it is not
- Tool feature releases that don't name a new artifact or change its forensic interpretation
- Vendor product comparisons, pricing, or partnership announcements
- Conference schedules, job postings, award announcements
- General incident response methodology without specific artifact detail

The test: *Does this teach an analyst something new about what an artifact contains, where it lives, or what it proves?* If the answer is only "this tool can now collect X", it is out of scope.

---

## Accuracy Standards

Catalog errors like the UserAssist "Folder GUID" misnomer (the `{F4E57C4B-...}` GUID tracks shortcut-launched items, not folder navigation) damage analyst trust. Prevention:

1. **Read every URL in `sources[]` before committing** — do not add a source citation without verifying the content matches the artifact's fields
2. **For GUID-keyed artifacts**: add an inline comment on `key_path` citing which source confirms that GUID — this is the most error-prone field
3. **When a source contradicts common DFIR tooling terminology**: document the discrepancy in the doc comment (e.g. "Historically called X in RegRipper/DFIR community, but per [source] it actually does Y")
4. **Count all GUIDs a source enumerates** — for artifacts with multiple GUIDs (UserAssist, Shellbags), verify you've captured every variant documented by each cited source
5. **Source field-level attribution**: where a specific field value (name, GUID, offset) comes from a specific source distinct from the others, add a `// Source: [url]` comment on that field
6. **`// Source:` comments must be URLs** — `// Source: Magnet Forensics` is a stub, not a citation. Every `// Source:` comment must contain at least one `https://` URL so readers can independently verify without guessing which page to look at. "Confirmed by X" without a URL is rejected.
7. **`sources[]` is for documents, not MITRE URLs** — `mitre_techniques: &["T1070.004"]` already expresses the MITRE relationship. Adding `"https://attack.mitre.org/techniques/T1070/004/"` to `sources[]` is redundant (DRY violation). `sources[]` is for researcher blogs, spec documents, tool repositories, and vendor artifact profiles — external documents that informed the artifact model, not the techniques themselves.

---

## Rating Systems — Classification Rules

These rules govern how to assign `VolatilityClass`, `TriagePriority`, and `EvidenceStrength`
when writing or reviewing `ArtifactDescriptor` entries. They are enforced by tests. The
canonical definitions live in `src/volatility.rs`, `src/catalog/types.rs`, `src/evidence.rs`.

### VolatilityClass (RFC 3227 acquisition urgency, 4 = collect first)

| Class | Collect when | Archetypal examples |
|---|---|---|
| `Volatile` (4) | Before reboot — exists only in RAM or tmpfs | RAM image, process list, network connections, in-memory ShimCache, `/proc/net/*`, ESXi `/var/run/log/*` |
| `RotatingBuffer` (3) | Before buffer fills | EVTX records, Prefetch (128-entry limit), `/var/log/syslog` (logrotate), `$UsnJrnl` |
| `ActivityDriven` (2) | Before more user activity | MRU lists, browser history, shellbags, Chrome Sessions file |
| `Persistent` (1) | Standard scheduled collection | Run keys, NTDS.dit, registry values, config files, ShimCache registry value |
| `Residual` (0) | Last — always present on any live mounted volume | `$MFT` (always present on NTFS), Volume Boot Record |

**`Residual` does NOT mean "recoverable via .LOG1/.LOG2 or VSS after deletion."**
That property applies to virtually every NTFS artifact — it provides no discrimination.
A deleted registry key that *could* be recovered from a transaction log is `Persistent`
while it exists.

**tmpfs / RAM-backed filesystems are `Volatile`.** ESXi `/var/run/`, Linux `/proc/`,
Linux `/run/` — even though they look like files, they vanish on reboot. `Volatile`
overrides `RotatingBuffer` when the backing store is RAM.

#### On-disk vs in-memory split rule

When an artifact has both an on-disk/registry form AND a richer in-memory form:

1. **Create two separate descriptors** — one for the persistent form, one for the memory form.
2. The **on-disk descriptor** gets `Persistent` (or `ActivityDriven`/`RotatingBuffer` as appropriate).
3. The **memory descriptor** gets `Volatile`, `artifact_type: MemoryRegion` or `LiveResponse`, `file_path: None`.
4. Both descriptors **cross-reference each other** in `related_artifacts`.
5. The on-disk descriptor's `evidence_caveats` **mentions the memory counterpart** is richer/more recent.

Examples: `shimcache` (Persistent registry) ↔ `shimcache_memory` (Volatile RAM buffer).

Do NOT apply `Volatile` to a registry value or file artifact just because its in-memory
counterpart is volatile. The volatility class describes the artifact itself, not its counterpart.

### TriagePriority

| Priority | When to use |
|---|---|
| `Critical` | Credential access, direct evidence of compromise, must-have for any IR |
| `High` | Execution evidence, persistence mechanisms, network lateral movement artifacts |
| `Medium` | Context-building artifacts, useful but not decisive alone |
| `Low` | Supporting detail, low signal-to-noise, rarely dispositive |

### EvidenceStrength

| Strength | Meaning |
|---|---|
| `Definitive` (4) | Proves the claimed activity beyond reasonable doubt (e.g. Prefetch = execution occurred) |
| `Strong` (3) | Near-definitive; rare edge cases exist (e.g. shimcache = file on disk, probably ran) |
| `Corroborative` (2) | Supports a conclusion when combined with other artifacts; alone is insufficient |
| `Circumstantial` (1) | Weak signal; many innocent explanations exist |
| `Unreliable` (0) | Easily manipulated, clock-skewed, or structurally unreliable — use with heavy caveats |

---

## GitHub Pages Road Map

A read-only search widget for analysts who prefer a browser to a CLI is in scope:
- Static JSON generated by `examples/dump_lol.rs` at CI time
- Filter by: platform, triage priority, artifact type, MITRE technique
- Full-text search across artifact names, meanings, and key paths
- No backend; pure static site served from GH Pages
