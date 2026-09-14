---
name: omarchy-app-test
description: "Validate standalone Omarchy apps and app packages, separating portable tests, development handoff, real desktop acceptance and release evidence."
---

# Omarchy App Test

Choose checks from the changed behavior. Use focused unit/integration tests for core invariants and real UI tests for input/activation behavior. Do not expand a documentation edit into an unnecessary full suite; invalidate only evidence whose inputs changed.

Read [development handoff](references/handoff.md) and [test matrix](references/tests.md). Record commands, exit results, exact files or commit, toolchain, upstream revision and actual platform. Keep reproduced results, historical results, failures and checks not run separate. CI green proves only the jobs that ran.

Run a fresh profile and save/reopen path, then install/upgrade/uninstall the actual package on a disposable target when available. A cross-compile is not execution evidence; a headless render is not Hyprland acceptance; a mocked input test is not proof of clipboard/IME behavior. Missing hardware or session access is an explicit gap, not a reason to invent compatibility or block a clearly scoped development PR.

When the Scaffold helper is available, verify its evidence manifest before citing it. Otherwise hash the tested files directly. Inspect local README/media references and credits against delivered files, validate desktop entries and review packaging separately. Deliver a concise evidence table with remaining live tests and any release blockers.

## Bundle boundaries

This is part of Build Omarchy Apps, targeting standalone apps on Omarchy 4 / Hyprland. Intended target is not a tested compatibility range. Preserve existing project choices and current user scope. For development PRs/archives, use the handoff guidance where linked; live acceptance, app releases and repository submission remain separate stages.
