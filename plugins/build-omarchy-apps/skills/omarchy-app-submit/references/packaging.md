# Contributor packaging workflow

Sources checked 2026-09-21:
- [Omarchy package repository](https://github.com/omacom/omarchy-pkgs): authoritative repository commands and metadata. Re-read its instructions and implementation at a recorded commit before acting.
- [PKGBUILD manual](https://man.archlinux.org/man/PKGBUILD.5.en): package metadata and build functions.
- [Community developer guide](https://omarchyapps.com/develop): useful secondary checklist, not an official contract.
- [Arch clean-chroot guide](https://wiki.archlinux.org/title/DeveloperWiki:Building_in_a_clean_chroot): linked reference; browser retrieval was blocked during bundle creation, so no successful inspection is claimed.

In an omarchy-pkgs contributor fork, current layout is `pkgbuilds/<name>/PKGBUILD` with `.omarchy/package.json` beneath the package directory. Minimal maintained metadata is `{"source":"local"}`. AUR is an optional source for importing an initial recipe; Omarchy owns the checked-in recipe after import and does not continually replace it from AUR. Inspect existing packages first. The current helper offers `bin/add-package <name> --source aur` for an initial import or `bin/add-package <name> --scaffold` for a custom package. Review the helper and package inputs before execution.

Keep ordinary source patches beside the PKGBUILD, hash them as local sources and apply them in `prepare()`. Use `bin/package-worktree` only to inspect historical AUR provenance. Prefer a declarative upstream watch/provider to a shell hook; it may update release scalars and checksums but must leave owned packaging behavior intact.

Choose channel policy deliberately:

- `"channels": ["edge"]` makes a preview buildable and publishable only in edge; channel advancement must refuse to carry it into rc or stable.
- Omitting `channels` makes the package eligible for the normal forward-only edge → rc → stable pipeline.
- `"release_ring": "fast"` requests native builds for edge, rc and stable. Use it only when that immediate availability is intended.

Preserve an explicit edge-first testing decision until the requested live acceptance is complete. Removing an edge-only bound is a later packaging change, not an automatic consequence of passing CI.

For contributor checks, inspect `bin/repo build --help` and current implementation, then select the package, architecture and unsigned/dry-run options it actually supports. Do not copy `bin/repo release`, `deploy`, `sign`, `promote`, `sync`, `advance` or host-setup examples into a local validation recipe: those may publish or change production state. No signing credentials are required to prepare a PR.

Pin version, source URL, architecture asset and cryptographic digest as one reviewed unit. Model runtime/build/test dependencies separately. Use provides/conflicts/replaces only for their actual pacman meaning; do not pretend compiled binaries are architecture-independent. Install only package-owned files under pkgdir, with correct modes and no home-directory mutation. Preserve user data on removal. Review hooks and privileges, build-machine paths, licences and large bundled assets.

After source review, build in an isolated Arch environment as an unprivileged user where practical. Run namcap on recipe and built artifact, inspect the archive file list/modes/dependencies/size, validate the desktop file, then test install, normal-user launch, upgrade and removal on a disposable target. Report each declared but untested architecture. No static check establishes that a package is safe or accepted upstream.

## Official build and publication state

Treat contributor validation, upstream app CI and the official package PR build as separate evidence. For an unvouched contributor, `Awaiting build approval` is a pending gate rather than a build failure. Once approved, inspect the checks for the current PR head commit.

When the official build fails, use the first concrete failing assertion, crash or timeout from that run. Reproduce the same package source, patches, dependencies and test path where possible. Do not replace a failed official package build with a green upstream workflow. After a fix, verify the relevant regression against the pre-fix code when practical, then rerun the official build on the new head.

Report progress as distinct stages:

1. upstream source release published;
2. package PR checks passed for an exact head;
3. package PR merged;
4. signed package present in the intended channel database;
5. package installed and accepted on the target Omarchy device.

The repository host checks merged changes on a five-minute cadence, then builds, signs and publishes queued packages. Queues, locks, backoff and failures can delay publication, so never infer channel availability from merge alone. Query the intended channel database or install from that channel before claiming availability.

## Fixing a published source release

Keep published tags and archives immutable. Prefer a new upstream patch release when the app fix is ready to ship. When the package must carry a fix first, add a separately reviewed and checksummed patch from a merged commit, apply it in `prepare()` and bump `pkgrel`. Record exactly which source release and commits the artifact contains. Check how the next declarative upstream update will remove, retain or obsolete the backport rather than allowing it to drift silently.
