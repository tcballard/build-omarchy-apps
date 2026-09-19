# Application boundaries and ownership

Read when designing or refactoring an app. Preserve the existing language and toolkit. These are responsibility boundaries, not mandatory directories, classes or separate crates. A small app can express them in a few modules; introduce an interface when an external dependency or test needs one.

## Dependency direction

| Responsibility | Owns | Avoid |
| --- | --- | --- |
| Startup | App identity, dependency construction, window/event-loop startup | Gameplay, document rules or persistence algorithms in main |
| Application/session | Active document or feature, commands, transitions, resource lifetime | A global object containing every feature's internal state |
| Domain/core | Rules, validation, undo and deterministic state transitions | Depending on the window, portal backend or another feature's UI |
| UI | Presentation, interaction and transient selection/drag state | Running blocking tools or duplicating authoritative domain state |
| Desktop/storage adapters | Theme input, portals, persistence and external tools | Leaking toolkit/service details throughout domain code |

The application wires the core and adapters together; the UI invokes application commands and presents results. Use ordinary functions, structs and modules before introducing frameworks. A backend may coordinate several adapters without becoming a generic service registry.

Extract a common facility when independent features actually share its policy. Features should not depend on another feature just to borrow theme or file helpers. Give shared facilities narrow APIs; do not move feature rules into a miscellaneous utilities module. Where useful, keep pure core tests buildable without GUI dependencies and check that optional features preserve that boundary.

## Ownership and transitions

For each document/session, identify the owner of mutable state, save locks, workers and child processes. Specify what happens when it is replaced or closed, not just when the whole process exits. Preserve locks through the last write and resource teardown; do not add duplicate save calls merely to standardize callback names.

Background results belong to the request/session that created them. Use request generations, cancellation or disconnected callbacks so an older completion cannot update a replacement document. Cancellation needs an observable completion/cleanup contract. For an external child, distinguish requesting cancellation from terminating and reaping it. Choose bounded waits and UI responsiveness from the product's needs; do not detach work simply to hide shutdown latency.

Define focus loss, modal entry, resume and close separately. Games may pause; editors may continue analysis. Clear or cancel held controls and unfinished interactions where necessary, and prevent a queued shortcut from undoing a focus-loss transition. Use the toolkit's appropriate event boundary before it computes widget activation; do not universally impose Arcade's specific input filter on other toolkits.

## Worked shape

For an editor, startup constructs the document session and file-picker adapter. A UI Open command asks the session to load the selected file. The session validates it and starts an owned preview job. Results include the document/request identity; switching files invalidates old results. Export writes a temporary output and replaces the destination only after successful completion. Closing resolves unsaved work and shuts down owned jobs before releasing document resources.

For a game collection, the host owns selection, active-session lifetime and desktop integration; each game owns its rules and save semantics. Shared theme/storage helpers belong below the games. Neither example requires C++, Qt, multiple processes or one crate per responsibility.

## Evidence and attribution

Inspected 19 September 2026: [OmaCut at 0948c461](https://github.com/omacom/omacut/tree/0948c4615d45ac62727b8c69112178e09781b7a4). Its small `src/main.cpp`, `Backend`, injectable `FilePicker`, ffmpeg adapter, cancellable thumbnail worker and backend tests illustrate useful boundaries. This is an interpretation of source patterns, not a published DHH architecture specification or a claim that every implementation detail is ideal.

Shared-helper separation and focus/drag regression scenarios also come from Arcade's architecture work ([platform extraction #47](https://github.com/tcballard/omarchy-retro-arcade/pull/47), [focus handling #48](https://github.com/tcballard/omarchy-retro-arcade/pull/48)). Example source and PRs do not establish universal toolkit rules or live desktop acceptance.
