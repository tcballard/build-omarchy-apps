---
name: omarchy-app-package
description: "Prepare or repair Arch and Omarchy packaging for standalone applications, including AUR/local sourcing, PKGBUILD review and reproducible updates."
---

# Omarchy App Package

Read [packaging workflow](references/packaging.md) and recheck the current target repository instructions before using its commands. Treat the linked community checklist as guidance; official package-repository implementation and Arch contracts win on conflicts.

Distinguish the app source repository from an omarchy-pkgs contributor fork. Inspect existing Omarchy/AUR packages before adding another; keep source choice and required customizations explicit. Do not treat a development checkout as a stable upstream release. Verify provenance, redistribution rights, version, architecture asset and digest together.

Review PKGBUILD and hooks as executable code before any evaluation. Never source an untrusted PKGBUILD merely to inspect metadata. Distinguish runtime, build and test dependencies, package-owned files and user data, compiled architecture and architecture-independent resources. Keep update tracking declarative when the current repository supports it; retain reproducible AUR patches.

Prepare an unsigned package build and inspect the artifact, dependencies, modes, paths and size. Use clean build environments and package/desktop validators where available; record unavailable tests. Do not use aggregate release/deploy commands for contributor validation. Before a packaging PR, perform development handoff and explain update behavior, channel policy and every untested architecture. Track source release, package CI, merge, channel publication and device acceptance as separate states.

## Bundle boundaries

This is part of Build Omarchy Apps, targeting standalone apps on Omarchy 4 / Hyprland. Intended target is not a tested compatibility range. Preserve existing project choices and current user scope. For development PRs/archives, use the handoff guidance where linked; live acceptance, app releases and repository submission remain separate stages.

Before an app development PR or archive, apply [development handoff](references/handoff.md). For diagnosis-only requests, report findings without creating a handoff artifact.
