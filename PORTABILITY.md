# Portability

The portable Agent Plugins 1.0.0 manifest and canonical `skills/` tree contain no model API calls, provider credentials or model-family selection. The agent host supplies file, shell and research tools. A different model does not require rewriting the bundle. Ability to install files does not prove a model performs the task correctly.

| Host format | Project destination | User destination |
| --- | --- | --- |
| Shared skills / Codex | `.agents/skills` | `~/.agents/skills` |
| Cursor | `.cursor/skills` | `~/.cursor/skills` |
| Gemini CLI | `.gemini/skills` | `~/.gemini/skills` |
| Claude Code | `.claude/skills` | `~/.claude/skills` |
| OpenCode | `.opencode/skills` | `~/.config/opencode/skills` |
| Other compatible hosts | Explicit generic destination | Explicit generic destination |

These installation routes are inherited from the inspected Plugins bundle; tests exercise filesystem behavior. Native discovery/invocation and every host/model combination remain unverified in this repository.

The root `.claude-plugin/plugin.json` points Claude at the same canonical skills. `plugins/build-omarchy-apps` contains the OpenAI manifest, UI metadata and generated skill copies. `scripts/sync_openai_adapter.py --write` refreshes the copy; `--check` rejects drift. No provider metadata belongs in canonical skills. The installer uses an Apps-specific receipt so it cannot claim ownership of the companion bundles.

Rust is the first default implementation track. Core workflow guidance preserves existing languages and user choices. New tracks should add focused references, small generators and demonstrated tests without duplicating shared desktop/packaging/evidence rules.

The copied installer and package tooling use Python 3.11+ standard-library APIs. The new app helper uses dependency-free Rust and GNU sha256sum. Its Linux verification does not imply the helper works on Windows or macOS without equivalent tooling. Portable instruction installation is tested separately from helper execution.
