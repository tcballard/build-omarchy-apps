# Evidence matrix

| Layer | Meaningful cases | Evidence boundary |
| --- | --- | --- |
| Core | Undo/state transitions, parsing malformed input, deterministic game simulation | No GUI compatibility claim |
| Storage | Interrupted write, migration recovery, concurrent instance, actual reopen | Record schema and files used |
| UI | Keyboard focus, IME, Ctrl+V, mouse, scaling, focus loss | Mocked event injection is not a real clipboard test |
| Desktop | Menu and terminal launch, icon, file argument, portal, theme change | Record Omarchy version, toolkit/backend and scale |
| Package | Clean build, file list, modes, dependencies, install/upgrade/uninstall | Record recipe/source/artifact digest and architecture |
| Handoff | Badge, local links/credits, status, instructions, input identity | Allows explicitly untested development PRs |

Capture only the layers relevant to the task and label unrun required layers. Use separate result rows for each command and platform. Do not record one overall 'pass' over a mix of completed and missing checks.

## Architecture and lifecycle regressions

Select cases from the changed behavior, not as a mandatory full suite for every app:

- Replace a document/request while its worker is pending; an old completion cannot overwrite current state.
- Close or switch features during I/O/audio/child-process work; verify cancellation, resource cleanup and final-save/lock ordering.
- Hold a key or begin a drag, lose focus, then refocus and release; no unintended command, move or resume occurs. Exercise modal entry and closing-frame input where relevant.
- Fail or cancel output generation over an existing destination; preserve the original and leave busy/progress state usable.
- Replace theme files or symlinks while open; malformed input and missing portal services leave a usable app.
- If a headless core is promised, build/test without UI features and inspect its dependency graph.

Use fake adapters to make races/failures deterministic, plus real widget event processing for activation semantics. Separate those results from native desktop acceptance.
