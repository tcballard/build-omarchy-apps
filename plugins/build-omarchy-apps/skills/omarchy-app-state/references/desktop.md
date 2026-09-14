# Desktop contracts

Primary references checked 2026-09-14; recheck current versions for implementation:
- [Desktop Entry specification 1.5](https://specifications.freedesktop.org/desktop-entry/latest/): identifiers, launcher keys and Exec argument semantics.
- [XDG Base Directory specification 0.8](https://specifications.freedesktop.org/basedir/latest/): configuration, documents/state, cache and runtime files.
- [Omarchy source](https://github.com/omacom/omarchy): inspect the targeted revision and installed app integration before assuming theme paths or toolkit APIs.

Use toolkit APIs where available. For custom paths, ignore relative XDG directory values and use the specification defaults. Configuration defaults to ~/.config; data to ~/.local/share; state to ~/.local/state; cache to ~/.cache. Runtime sockets belong in a private per-session directory. Do not write preferences next to system-installed binaries.

Application launchers belong in the standard applications data directory and icons in a standard icon-theme location, normally hicolor. Use an app ID you control. Keep GUI identity consistent with the desktop filename; verify how the chosen toolkit sets it under Wayland. A desktop entry's Exec field is not shell syntax. Only advertise file/URL field codes when argument handling implements them; validate with desktop-file-validate.

Omarchy theming is not one universal native-app API. Inspect outputs for the actual target/toolkit, respect system fonts and user preferences, use semantic colors and safe defaults. Test live changes, invalid inputs and reduced contrast. Do not copy Quattro-specific QML imports into a standalone process. Any user-requested global styling change belongs to the Theme workflow.
