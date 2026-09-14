# Contributing

Use a branch and PR. Edit canonical `skills/`; keep provider UI/configuration in adapters. Run `python3 scripts/sync_openai_adapter.py --write` after skill edits and `./scripts/test` before handoff. Packaging uses a clean committed tree. Do not merge or tag just to validate changes.

Rust is preferred for new deterministic app helpers in v0.1.0. Retain the tested Python distribution tools. Additional language/toolkit tracks must preserve existing apps, share the common workflow and bring meaningful tests. Add a new reference or helper when it changes real decisions; do not create ten duplicate skills per language.

Update workflow routing and behavioral cases when skill purpose changes. Preserve source/licence notices for borrowed code and record the inspected revision. Keep portable claims separate from real host/model and Omarchy acceptance. CI results only cover jobs actually run.
