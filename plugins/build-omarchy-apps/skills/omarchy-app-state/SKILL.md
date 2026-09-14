---
name: omarchy-app-state
description: "Build reliable local state, persistence, background work and optional IPC for standalone Omarchy apps, including migrations and crash recovery."
---

# Omarchy App State

Read [desktop contracts](references/desktop.md) for XDG storage. Separate user documents, preferences, resumable state, cache and session sockets. Preserve user-created data during uninstall. Do not store secrets in committed config or logs; use an appropriate secret store when the product needs credentials.

Version persistent formats. Use atomic replacement with a same-directory temporary file and an intentional durability policy; handle missing/corrupt/newer schemas without silently discarding data. Before migration, preserve a recoverable original. Define how multiple instances and concurrent edits behave. Test interruption and actual reopen, not only serialization round trips.

Keep blocking I/O off the UI thread. Bound requests, retries and cancellation. For periodic feeds, define start-to-start cadence, fair overdue selection, backoff and prevention of overlapping work; test slow successful requests as well as errors. Pause or stop unnecessary background work when appropriate to the product.

For optional agent/automation IPC, define a versioned request/response schema, socket ownership, input limits and error semantics. Keep destructive actions subject to the same user intent as the GUI. Do not expose unauthenticated network listeners as a default. A shell companion belongs to Plugin skills; the standalone service contract remains app-owned.

## Bundle boundaries

This is part of Build Omarchy Apps, targeting standalone apps on Omarchy 4 / Hyprland. Intended target is not a tested compatibility range. Preserve existing project choices and current user scope. For development PRs/archives, use the handoff guidance where linked; live acceptance, app releases and repository submission remain separate stages.

Before an app development PR or archive, apply [development handoff](references/handoff.md). For diagnosis-only requests, report findings without creating a handoff artifact.
