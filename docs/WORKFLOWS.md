# Workflow entry points

| Request | Entry and handoff |
| --- | --- |
| New standalone app/game | Design → Scaffold → Desktop and State → Test → development handoff |
| Existing non-Rust app | Retain its stack; choose Desktop, State or Debug for the actual change |
| Diagnose only | Debug; inspect and report without changes |
| App theme consumption | Desktop; shared palette/theme generation stays in Theme skills |
| Shell bar/panel/service | Use Plugin bundle; do not generate an app to replace a requested plugin |
| Prepare package | Package → Test → development handoff → Submit; track CI, merge, channel publication and device acceptance separately |
| Screenshot/demo | Demo → development handoff; label fixtures and mockups |
| Versioned release | Release after development handoff and explicit evidence review |

Every development PR/archive gets the handoff checks in the selected skill. Missing live access remains visible without blocking a scoped development contribution. App publication, package contribution, package merge, channel publication and device acceptance are separate states. Preserve already-given authorization; do not invent extra confirmation loops.
