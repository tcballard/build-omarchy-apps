# Scaffold helper

Requires Rust (edition 2021) and `sha256sum` on PATH for input manifests. No external Rust crates. Resolve this skill's actual absolute directory, then use:

```sh
cargo run --manifest-path /absolute/skill/scripts/app-tool/Cargo.toml -- scaffold /new/app/path
cargo run --manifest-path /absolute/skill/scripts/app-tool/Cargo.toml -- check /app/path
cargo run --manifest-path /absolute/skill/scripts/app-tool/Cargo.toml -- snapshot /app/path
cargo run --manifest-path /absolute/skill/scripts/app-tool/Cargo.toml -- verify /app/path
```

Scaffold refuses an existing destination and creates README, credits and a verification record only. It does not choose a toolkit or pretend to implement an app. Edit the generated documentation as implementation proceeds. `check` tests required documentation markers, the canonical badge and simple inline local Markdown/HTML links in README and CREDITS. It does not interpret arbitrary Markdown or verify prose claims; review those manually. Compatibility starts explicitly untested.

Snapshot records sorted SHA-256 identities of every regular file except `.git`, `target` directories and the root `EVIDENCE.sha256`. It refuses symlinks and ambiguous/control-character paths. Keep build outputs under target; review the manifest before citing it. Snapshot refuses to overwrite existing evidence. Archive historical manifests outside the input tree before deliberately recording new inputs, and retain the original test/result associations. Verify detects missing, changed and added files and does not run app code. Even a passing check/verify does not establish live acceptance.
