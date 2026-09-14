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
