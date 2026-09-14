# v0.1.0 development acceptance

Checked 14 September 2026 in Linux, Python 3.12 and Rust 1.98.1. The exact delivered file identities are in the SOURCE-MANIFEST.json produced by `scripts/package_submission.py` from the reviewed Git tree. A package manifest identifies files; it is not a test result. This document describes the implementation/test files in the commit containing this report; later changes must retain this scope or update the record.

## Reproduced now

- `./scripts/test`: 12 Python tests and 13 Rust tests pass. Includes six host project installation paths, repeat installation, update/removal protections, provider-metadata separation, adapter drift repair, excluded build outputs, deterministic archive bytes, and installation from extracted portable/Claude archives.
- Portable manifest, ten canonical skill definitions, ten OpenAI definitions and adapter manifest validators pass; generated adapter content matches the canonical sources.
- Rust formatting and strict Clippy pass.
- Official local plugin-creator manifest validator passes.
- Independent Design-skill scenario: existing C++/Qt editor with a local model remains C++/Qt, introduces no model APIs, and keeps live Omarchy acceptance unverified. Only the skill and linked references were supplied; no files or external services were changed.

## Historical

Initial installed-skill authoring passed ten skill validators, thirteen Rust helper regressions and one independent packaging-boundary scenario. Those results are not substituted for the repository checks above.

## Failed and corrected during this pass

The initial portable archive omitted VERSION, so its installer failed after extraction. Added VERSION to both portable and Claude payloads and reran the extracted-install regression successfully. Added build-output exclusion so compiling the Rust helper does not contaminate future skill installs or adapter copies.

## Not run / limits

No live Omarchy app acceptance, no actual Cursor/Gemini/OpenCode/Claude/native OpenAI host invocation, and no provider-directory submission or acceptance. Windows/macOS portable CI is configured; its live result belongs to the PR checks and is not claimed by the local Linux run. The Rust helper's SHA-256 command dependency is verified only on Linux. Non-Rust workflow guidance is supported; non-Rust scaffold generators are not supplied in v0.1.0. The helper scaffolds documentation, not a working GUI.
