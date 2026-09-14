---
name: omarchy-app-release
description: "Prepare a versioned standalone Omarchy app release with reproducible source and package evidence; does not imply package-repository promotion."
---

# Omarchy App Release

First perform [development handoff](references/handoff.md). Establish whether the request is source release preparation, app release publication, package contribution or repository operation. Preserve existing authorization; completing one does not authorize the others.

Tie the version, tag target, changelog, release assets and digests to the same reviewed source. Check licences, dependency lockfiles, architecture-specific outputs, reproducible build instructions, install/upgrade/rollback and actual desktop acceptance. Carry failures and untested platforms visibly into the readiness report. Do not describe a development PR, draft release or unchecked binary as a published stable release.

Prepare concrete release notes and artifact checksums before any requested publication step. For app binaries installed by pacman, avoid a self-updater overwriting package-owned files. Evaluate schema migration rollback separately from package downgrade. Route package PRs to Package/Submit; signing, promotion and production sync of omarchy-pkgs remain repository-operator work. Missing live acceptance can still yield a development handoff, with release readiness explicitly unresolved.

## Bundle boundaries

This is part of Build Omarchy Apps, targeting standalone apps on Omarchy 4 / Hyprland. Intended target is not a tested compatibility range. Preserve existing project choices and current user scope. For development PRs/archives, use the handoff guidance where linked; live acceptance, app releases and repository submission remain separate stages.
