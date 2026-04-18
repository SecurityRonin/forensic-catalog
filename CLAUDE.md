# CLAUDE.md

## Scope Boundary

This project is a forensic catalog first, not a full DFIR parsing engine.

- Keep `ArtifactDescriptor` for artifact location, significance, field schema, ATT&CK mapping, triage value, and authoritative citations.
- Keep `ArtifactParsingProfile` for format knowledge and analyst guidance that does not fit a small stable decoder.
- Implement in-core decoders only for compact, stable encodings where the logic is intrinsic to the artifact model, such as `UserAssist`, `MRUListEx`, `FILETIME`, `REG_MULTI_SZ`, or PCA record layouts.
- Do not keep pushing large or evolving formats such as `hiberfil.sys`, BITS job stores, or full WMI repository parsing into this crate's core decode engine.
- If execution-grade parsers are needed later, put them in a separate parsing module or companion crate rather than turning the catalog itself into a full parser framework.
