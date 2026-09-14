# Development handoff

Apply before delivering an app archive or opening a development PR, including scaffolds. This is distinct from real desktop acceptance, versioned release preparation and submission. Scale rechecking to changed files.

- README: purpose, development status, exact build/run or install commands appropriate to the deliverable, uninstall/rollback and data preservation. A non-runnable scaffold says so plainly; do not publish guessed installation commands as working ones.
- Include the community App badge from `https://raw.githubusercontent.com/tcballard/omarchy-badges/75975e5b5bf75e7ede3764bcd2950046f7abfe2c/badges/v1/omarchy-app.svg` at height 20, no fixed width. It is an identity label, not official approval. Any support badge describes a declared range; state separately which exact installed Omarchy versions were tested (`omarchy-version`).
- Match local README links, screenshots, icon paths and credited assets to the delivered tree. State source/licence for third-party material; remove stale credits. Label mockups and contact sheets separately from screenshots captured on Omarchy.
- Evidence: record source commit or SHA-256 file manifest, upstream revision (or retrieved content hash when revision unavailable), commands, toolchain/platform and exit results. The manifest identifies inputs; creating one does not run tests or establish compatibility.
- Separate **reproduced now**, **historical**, **failed**, **not run**. Record each check against its own input identity. Never relabel a historical result when regenerating a manifest. Documentation-only edits can retain historical runtime evidence with its original source and scope.
- Handoff: describe completed behavior, portable verification and remaining live tests. Missing live access does not block a scoped development PR. Failures affecting install/data safety or a promised feature must be explicit.

The Scaffold helper checks a narrow documentation contract and verifies byte manifests. It is not a PKGBUILD interpreter, licence auditor, Markdown parser, runtime validator or guarantee of safety. Review unsupported Markdown constructs and claims manually.
