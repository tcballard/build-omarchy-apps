---
name: omarchy-app-design
description: "Design standalone Omarchy applications and games, choosing native architecture, workflows and acceptance criteria; excludes shell plugins and desktop theme authoring."
---

# Omarchy App Design

Start with the user's primary workflow, existing code and repository instructions. Choose an independently launched application for a substantial workspace, document editor or game; use Plugin skills for a shell-hosted bar/panel/service and Theme skills for shared desktop styling. Do not turn an app into a plugin simply because it targets Omarchy.

v0.1.0 uses Rust as the default implementation track, not an eligibility requirement. Preserve an existing stack or explicit user choice; do not rewrite a working C++, Python, Go, JavaScript or other app simply to fit this bundle. All app architecture, desktop, state, packaging and evidence guidance applies across languages. Future toolkit-specific tracks belong in focused references or helpers, not duplicated end-to-end skill sets. Choose a toolkit from actual requirements (text/IME/accessibility, rich documents, GPU rendering, deployment size), not from assumed Omarchy mandates. Verify current toolkit documentation and Linux/Wayland support before choosing versions.

Define a small playable/usable vertical slice, keyboard and mouse flows, window identity, theme adaptation, state ownership, offline/error behavior and performance budgets. Separate application logic from UI and OS integration. For games specify timing, pause/focus-loss and deterministic simulation; for editors specify undo, recovery and file conflicts. Do not prescribe numerical budgets without a product reason.

Hand off the chosen stack and reasons, first milestone, acceptance scenarios and unresolved dependencies. Continue through Scaffold → Desktop/State → Test → development handoff. Package, Release and Submit are separate later stages. For a narrow fix, use the relevant skill directly rather than replaying the entire workflow.

## Bundle boundaries

This is part of Build Omarchy Apps, targeting standalone apps on Omarchy 4 / Hyprland. Intended target is not a tested compatibility range. Preserve existing project choices and current user scope. For development PRs/archives, use the handoff guidance where linked; live acceptance, app releases and repository submission remain separate stages.

Before an app development PR or archive, apply [development handoff](references/handoff.md). For diagnosis-only requests, report findings without creating a handoff artifact.

Read [language and model policy](references/portability.md) when selecting a stack or adapting this bundle to an agent host.
