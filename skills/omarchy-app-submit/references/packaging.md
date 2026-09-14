# Contributor packaging workflow

Sources checked 2026-09-14:
- [Omarchy package repository](https://github.com/omacom/omarchy-pkgs): authoritative repository commands and metadata. Re-read its instructions and implementation at a recorded commit before acting.
- [PKGBUILD manual](https://man.archlinux.org/man/PKGBUILD.5.en): package metadata and build functions.
- [Community developer guide](https://omarchyapps.com/develop): useful secondary checklist, not an official contract.
- [Arch clean-chroot guide](https://wiki.archlinux.org/title/DeveloperWiki:Building_in_a_clean_chroot): linked reference; browser retrieval was blocked during bundle creation, so no successful inspection is claimed.

In an omarchy-pkgs contributor fork, current layout is `pkgbuilds/<name>/PKGBUILD` with `.omarchy/package.json` beneath the package directory. Minimal metadata is `{"source":"aur"}` or `{"source":"local"}`. Inspect existing packages first. The current helper offers `bin/add-package <name>` for AUR or `bin/add-package <name> --local --scaffold` for a locally maintained package. Review the helper and package inputs before execution.

For recurring AUR customization, use `.omarchy/patches/` and the repository package-worktree/sync workflow to prove the patch can be reapplied. Recheck available declarative upstream providers before writing a shell hook. Avoid changing channels, release rings or quarantine policy without a concrete requested reason.

For contributor checks, inspect `bin/repo build --help` and current implementation, then select the package, architecture and unsigned/dry-run options it actually supports. Do not copy `bin/repo release`, `deploy`, `sign`, `promote`, `sync`, `advance` or host-setup examples into a local validation recipe: those may publish or change production state. No signing credentials are required to prepare a PR.

Pin version, source URL, architecture asset and cryptographic digest as one reviewed unit. Model runtime/build/test dependencies separately. Use provides/conflicts/replaces only for their actual pacman meaning; do not pretend compiled binaries are architecture-independent. Install only package-owned files under pkgdir, with correct modes and no home-directory mutation. Preserve user data on removal. Review hooks and privileges, build-machine paths, licences and large bundled assets.

After source review, build in an isolated Arch environment as an unprivileged user where practical. Run namcap on recipe and built artifact, inspect the archive file list/modes/dependencies/size, validate the desktop file, then test install, normal-user launch, upgrade and removal on a disposable target. Report each declared but untested architecture. No static check establishes that a package is safe or accepted upstream.
