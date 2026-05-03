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

---

## Accuracy Standards

Catalog errors like the UserAssist "Folder GUID" misnomer (the `{F4E57C4B-...}` GUID tracks shortcut-launched items, not folder navigation) damage analyst trust. Prevention:

1. **Read every URL in `sources[]` before committing** — do not add a source citation without verifying the content matches the artifact's fields
2. **For GUID-keyed artifacts**: add an inline comment on `key_path` citing which source confirms that GUID — this is the most error-prone field
3. **When a source contradicts common DFIR tooling terminology**: document the discrepancy in the doc comment (e.g. "Historically called X in RegRipper/DFIR community, but per [source] it actually does Y")
4. **Count all GUIDs a source enumerates** — for artifacts with multiple GUIDs (UserAssist, Shellbags), verify you've captured every variant documented by each cited source
5. **Source field-level attribution**: where a specific field value (name, GUID, offset) comes from a specific source distinct from the others, add a `// Source: [url]` comment on that field
6. **`// Source:` comments must be URLs** — `// Source: Magnet Forensics` is a stub, not a citation. Every `// Source:` comment must contain at least one `https://` URL so readers can independently verify without guessing which page to look at. "Confirmed by X" without a URL is rejected.

---

## Deferred Refactors

### Merge `ArtifactProfile` into `ArtifactDescriptor`

**Decision:** Combine them — but only after the 6,193 generated entries have meaningful defaults.

**Why they should be combined:**
- `ArtifactProfile.id` is a `&'static str` FK into `CATALOG` enforced only by tests, not the type system — a design smell
- Every artifact has exactly one evidence strength and one volatility class; no many-to-one relationship justifies a separate table
- The split API is worse: `evidence_for("mft")` vs `descriptor.evidence_strength`

**Why they are separate right now:**
- Only 361/6,554 entries have profiles; the 6,193 generated entries have no evidence-strength judgment yet — they would all be `None`, making the incompleteness visible but awkward
- `ArtifactProfile` was added after the catalog was built; touching 6,554 entries was deferred

**Prerequisite before merging:** Assign at least a meaningful default to every generated entry — `EvidenceStrength::Circumstantial` and `VolatilityClass::SystemConfig` are reasonable placeholders for unanalyzed artifacts. Until then the separate array is the honest representation of "cataloged but not assessed."

**Refactor steps when ready (strict TDD):**
1. Add `evidence_strength: Option<EvidenceStrength>` and `volatility: Option<VolatilityClass>` to `ArtifactDescriptor`
2. Fill in the 361 hand-curated values inline in their descriptors
3. Delete `ArtifactProfile` struct, `ARTIFACT_PROFILES` static, and `src/profile.rs`
4. Make `evidence_for()` and `volatility_for()` thin wrappers over `CATALOG.get(id)`
5. Update all callers in `navigator.rs`, `4n6query/main.rs`, tests

---

## GitHub Pages Road Map

A read-only search widget for analysts who prefer a browser to a CLI is in scope:
- Static JSON generated by `examples/dump_lol.rs` at CI time
- Filter by: platform, triage priority, artifact type, MITRE technique
- Full-text search across artifact names, meanings, and key paths
- No backend; pure static site served from GH Pages
