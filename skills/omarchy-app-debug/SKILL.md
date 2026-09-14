---
name: omarchy-app-debug
description: "Diagnose standalone Omarchy app startup, Wayland, launcher, theme, state or package failures; diagnosis-only requests remain read-only."
---

# Omarchy App Debug

Start with the observed symptom, exact version/build, launch path, app logs and relevant session details. Inspect before modifying. Diagnosis-only requests authorize investigation and proposed fixes, not edits, installs, theme changes, service restarts or state deletion.

Separate failure layers: dependency/linker → executable startup → Wayland/toolkit → desktop-entry environment → portal/service → app state. Compare launcher and terminal environments without dumping credentials. Check application IDs and executable/icon paths before blaming Hyprland. For corrupted state, work on a copy; do not recommend deleting the profile as the first fix.

For theme issues identify the ownership boundary: global theme generation belongs to Theme skills; app consumption/fallback belongs here; shell-hosted QML belongs to Plugin skills. Do not patch all three speculatively.

Reproduce narrowly and show the smallest evidence-backed cause or ranked hypotheses. If fixes are authorized, make the smallest patch, add a regression for the demonstrated behavior, and verify the same failing path. Preserve user work, keep rollback possible and state which live observations could not be made.

## Bundle boundaries

This is part of Build Omarchy Apps, targeting standalone apps on Omarchy 4 / Hyprland. Intended target is not a tested compatibility range. Preserve existing project choices and current user scope. For development PRs/archives, use the handoff guidance where linked; live acceptance, app releases and repository submission remain separate stages.

Before an app development PR or archive, apply [development handoff](references/handoff.md). For diagnosis-only requests, report findings without creating a handoff artifact.
