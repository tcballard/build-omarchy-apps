---
name: omarchy-app-scaffold
description: "Scaffold a standalone Omarchy app project with development documentation, desktop identity and honest validation defaults; excludes Quattro plugin manifests."
---

# Omarchy App Scaffold

Inspect the destination and choose the app slug, reverse-DNS application ID, executable name and toolkit before creating files. Do not overwrite an existing project or add a Quattro manifest. Use the bundled starter documentation; it is intentionally a development scaffold, not a completed GUI app or releasable package.

Read [the helper guide](references/helper.md). Resolve commands relative to this skill's actual directory, never the caller's working directory. The Rust helper scaffolds documentation without installing anything. Add the chosen toolkit's smallest working window, separated core logic and locked dependency versions using current official toolkit instructions. Put desktop integration under a packaging directory with the same application ID as the Wayland window and icon basename. Do not claim a CLI hello-world proves a native app works.

Complete the README purpose, build/run instructions, intended version range, actual tested versions and rollback path. Preserve the supplied community App badge at 20px height without fixing its width; it is not official certification. Record third-party licences and assets actually included. Add focused CI that builds/tests what can run there and reports missing desktop acceptance.

Before delivering a PR or archive, follow [development handoff](references/handoff.md), even for an unreleased scaffold. A useful development PR may explicitly defer the UI or live testing; its scope and limitations must be visible.

## Bundle boundaries

This is part of Build Omarchy Apps, targeting standalone apps on Omarchy 4 / Hyprland. Intended target is not a tested compatibility range. Preserve existing project choices and current user scope. For development PRs/archives, use the handoff guidance where linked; live acceptance, app releases and repository submission remain separate stages.
