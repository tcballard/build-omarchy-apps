# Models and implementation languages

The agent host chooses the model. The portable skills do not call model APIs, require provider credentials, prescribe a model family or depend on one provider's tools. Translate ordinary file, shell and research steps to host capabilities; keep missing capabilities visible rather than assuming a particular connector.

Rust is the v0.1.0 default development track and the bundled app helper is Rust. It is not a restriction on applications. For an existing app, retain its stack unless the user requests migration or a concrete requirement justifies one. For a new app, consider text/IME/accessibility, GPU needs, deployment size, Wayland behavior and maintainability before choosing a toolkit. A toolkit's name alone does not establish Omarchy compatibility.

New tracks should add only non-obvious language/toolkit decisions, small templates and relevant acceptance cases. Share the existing Desktop, State, Package, Test and Handoff contracts. Record what was actually implemented/tested; do not advertise working scaffold generators for languages that only have general guidance.
