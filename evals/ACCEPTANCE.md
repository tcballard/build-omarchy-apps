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

## Architecture guidance update — 19 September 2026

Source under test: `e90d6335b63a2a8144da827ac8663e9af9d1046c`. Linux, Rust 1.98.1.

- `PATH=/root/.cargo/bin:$PATH ./scripts/test`: passed, including bundle/adapter validators, Python distribution tests, 13 Rust helper tests, formatting and strict Clippy.
- Generated a fresh documentation scaffold and ran its `check` command: passed. Manually inspected the architecture worksheet; it describes proposed decisions and does not claim a running app.
- `git diff --check`: passed after removing an extra trailing blank line in the evaluation document.
- OpenAI copies regenerated from canonical skills using `scripts/sync_openai_adapter.py --write`.
- New architecture behavioral cases are specified in README.md but have not been executed against independent agents. No new host/model or live Omarchy acceptance is claimed.
- This is an unreleased bundle update; version and installed personal skills are unchanged.
