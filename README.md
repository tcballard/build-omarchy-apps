# Build Omarchy Apps

<p>
<img alt="Built for Omarchy: App" height="20" src="https://raw.githubusercontent.com/tcballard/omarchy-badges/75975e5b5bf75e7ede3764bcd2950046f7abfe2c/badges/v1/omarchy-app.svg">
<img alt="Licence: MIT" height="20" src="https://img.shields.io/badge/license-MIT-blue?style=flat-square">
</p>

**Ten model-agnostic skills for building standalone Omarchy applications.**
Design the app, integrate its desktop window, protect its state, prepare a package and hand over evidence that matches the delivered files.

A companion to [Build Omarchy Plugins](https://github.com/tcballard/build-omarchy-plugins) and [Build Omarchy Themes](https://github.com/tcballard/build-omarchy-themes).

## Status

**v0.2.1 development preview.** Intended target: Omarchy 4 / Hyprland. No live Omarchy acceptance or agent-directory approval is claimed. Individual apps must establish their own supported and actually tested versions. The badge is a community identity statement, not official certification.

## Models and languages

The agent host chooses the model. Portable skills contain no provider API calls, model selection or required provider credentials. OpenAI metadata lives in its distribution adapter; Claude uses the same canonical skills. See [portability](PORTABILITY.md).

**Rust is the v0.1.0 default, not a language restriction.** Keep existing C++, Python, Go, JavaScript and other app stacks when appropriate. Shared design, desktop, state, packaging and testing guidance applies across languages. The bundled deterministic helper is Rust; it scaffolds development documentation and checks file evidence, not a complete GUI application. Future toolkit-specific helpers can extend the shared workflow.

## Use it

From a reviewed source checkout or extracted portable package:

```sh
python3 scripts/install_agent_skills.py --target agents --scope user --dry-run
python3 scripts/install_agent_skills.py --target agents --scope user
```

The installer also accepts `codex`, `cursor`, `gemini`, `claude`, `opencode`, and `generic --destination PATH`. Use `--scope project` for project-local skills, or `--skill omarchy-app-design` for one skill. Installation does not modify an Omarchy desktop.

Preview updates with `--update --diff` and removal with `--uninstall --diff`; omit `--diff` to apply. Locally changed managed files are protected. Use `--force` only when replacing those named files is intentional. Avoid installing duplicate copies through both a host plugin and the skills installer.

For Claude, use `claude --plugin-dir .` on the source or Claude package. The OpenAI adapter is under `plugins/build-omarchy-apps`; its manifest and metadata are separate from the portable core. These are prepared distribution formats, not claims of marketplace acceptance.

Try: “Build a small native Omarchy app for keeping local notes.”
Or: “This existing Qt app launches in a terminal but not from the menu; diagnose it without changing files.”

## Skills

- [omarchy-app-debug](skills/omarchy-app-debug/SKILL.md)
- [omarchy-app-demo](skills/omarchy-app-demo/SKILL.md)
- [omarchy-app-design](skills/omarchy-app-design/SKILL.md)
- [omarchy-app-desktop](skills/omarchy-app-desktop/SKILL.md)
- [omarchy-app-package](skills/omarchy-app-package/SKILL.md)
- [omarchy-app-release](skills/omarchy-app-release/SKILL.md)
- [omarchy-app-scaffold](skills/omarchy-app-scaffold/SKILL.md)
- [omarchy-app-state](skills/omarchy-app-state/SKILL.md)
- [omarchy-app-submit](skills/omarchy-app-submit/SKILL.md)
- [omarchy-app-test](skills/omarchy-app-test/SKILL.md)

Read [workflow entry points](docs/WORKFLOWS.md). App windows belong here; shell-hosted bar/panel plugins and global theme authoring belong to the companion bundles.

## Validate and package the source repository

Requires Python 3.11+, Git, Rust/Cargo and `sha256sum` on Linux for the Rust evidence tests. Python is used by the inherited host installer and distribution tooling; applications are not required to use it.

```sh
python3 scripts/sync_openai_adapter.py --write
./scripts/test
python3 scripts/package_submission.py --output-dir dist --require-clean --git-tree HEAD
```

Packaging reads a committed Git tree and produces source, portable, Claude, OpenAI plugin and skills archives with digests and manifests. It does not publish a release. Source-repository validation needs the complete source archive or checkout; adapter archives are installation payloads.

See [acceptance evidence](evals/ACCEPTANCE.md) for reproduced checks and limits, and [source provenance](NOTICE.md) for reused code and guidance.

## Licence

MIT. App licences, third-party assets and trademark rights are established separately for each application.
