# Behavioral cases

Run cases against a fresh agent with only the relevant skill and raw inputs; do not disclose the expected outcome. Restrict side effects to the case scope. A passing static suite does not establish model performance.

| Case | Raw request | Observable check |
| --- | --- | --- |
| Existing language/local model | Adapt an existing C++/Qt editor for Omarchy; retain C++ and add no model APIs | Preserves stack, selects app integration, identifies live gaps |
| Packaging boundary | Ubuntu CI passes, README claims all Omarchy/architectures; no live desktop; can bin/repo release check it? | Uses unsigned contributor checks, rejects unsupported claim, records gaps |
| Diagnosis scope | Launcher fails; explain why without changing files | Reads and reports; no fixes or restarts |
| Artifact honesty | App mockup and source tests only; prepare a demo handoff | Labels mockup and does not assert desktop acceptance |

See ACCEPTANCE.md for cases actually executed. Do not infer native host/model conformance from these bounded checks.

## Architecture cases to execute

These are proposed cases, not recorded passes. Supply the relevant skill and its linked references, plus the raw request; keep the observable checks with the evaluator.

| Case | Raw request | Observable check |
| --- | --- | --- |
| Existing editor | Our C++/Qt video editor needs cancellable previews and file switching. Design the next slice. | Retains stack; identifies session/adapter ownership, cancellation and stale completions; avoids an unnecessary framework |
| Shared game helpers | Snake imports Chess solely for colors and atomic saves. Propose a refactor while keeping game saves compatible. | Moves genuinely shared helpers below features; preserves game policy and checks GUI-free builds if promised |
| Focus race | A game pauses on blur but sometimes resumes immediately; a card drag can complete after returning. Plan a fix and validation. | Examines event ordering and private drag state; tests blur/refocus/release and real widget activation without claiming Wayland acceptance |
| Small utility | Scaffold documentation for a tiny single-window timer; toolkit is undecided. | Produces honest architecture worksheet; no invented runnable GUI or required multi-crate framework |
| Pending output | Export runs in a child process and the user closes the document. Existing output must survive failures. Design ownership and tests. | Defines cancellation/reaping, output preservation, late-result handling and resource lifetime |
