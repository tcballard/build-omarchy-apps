---
name: omarchy-app-desktop
description: "Implement or review standalone Omarchy app desktop integration: Wayland windows, launchers, icons, theme adaptation, shortcuts and portals."
---

# Omarchy App Desktop

Read [desktop contracts](references/desktop.md). Inspect the target Omarchy checkout/version and selected toolkit before naming theme files or APIs. A standalone app cannot import Quattro shell singletons as if it were a hosted plugin. Theme bundle skills own global palette/shell configuration; this skill owns the app's consumption of supported theme outputs and its fallback colors.

Keep application ID, window app_id, desktop filename and icon consistent. Use a standard launcher with valid Exec field codes, categories and Terminal behavior. Exec is not a shell command: no tilde expansion, environment assignment or unreviewed sh -c shortcut. Set MIME handlers, single-instance activation and command-line file opening only where implemented.

Use toolkit/portal APIs for file selection, opening URLs, notifications and screen access where appropriate; test the actual session backend. Do not require a tray icon, autostart, a service, a global keybind or privilege unless the feature needs it. Avoid overwriting user Hyprland configuration. App shortcuts must not silently steal desktop shortcuts.

Test launcher and terminal starts, paths with spaces, missing files, keyboard traversal, IME/paste, accessible labels, focus loss, scaling and multiple monitors as applicable. Check theme switching while open and missing/malformed theme input. Separate static desktop-entry validation from an observed launcher/icon/portal result. For visual changes, use actual captures and state their environment.

## Bundle boundaries

This is part of Build Omarchy Apps, targeting standalone apps on Omarchy 4 / Hyprland. Intended target is not a tested compatibility range. Preserve existing project choices and current user scope. For development PRs/archives, use the handoff guidance where linked; live acceptance, app releases and repository submission remain separate stages.

Before an app development PR or archive, apply [development handoff](references/handoff.md). For diagnosis-only requests, report findings without creating a handoff artifact.

## Adapter boundaries

Keep theme loading and portal requests behind small functions or adapters that can be exercised independently of the full window. Inject substitutes where they enable meaningful error/cancellation tests; retain real-session tests for actual integration. Theme watchers must recover when a theme file or symlink is replaced, using a documented fallback or last-good palette for malformed input. Portal cancellation, unavailable backends and late replies must leave the active session consistent. Resolve focus/modal input before widget activation where the toolkit requires it, and explicitly cancel game- or document-owned drags that cannot safely survive focus loss.
