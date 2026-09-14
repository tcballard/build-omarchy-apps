# Sources and provenance

Distribution layout, installer, adapter sync, structural validators, package tooling and installer lifecycle tests are adapted from Tom Ballard's MIT-licensed Build Omarchy Plugins at commit `26ee1e7fc57e4089daea57ff0c51e6a10659e401`, inspected 14 September 2026: https://github.com/tcballard/build-omarchy-plugins/tree/26ee1e7fc57e4089daea57ff0c51e6a10659e401 . The MIT notice is preserved in LICENSE. Names, v0.1.0 metadata, archive contents and app-specific tests differ.

The ten app skills and Rust helper originated in the Build Omarchy Apps work on 14 September 2026. The repository makes model/language boundaries explicit and separates installed-host metadata into adapters. Repository source changes do not silently update previously installed personal skills.

Guidance: https://omarchyapps.com/develop (secondary, inspected 14 September 2026); https://github.com/omacom/omarchy-pkgs (authoritative package workflow, recheck at contribution time); Arch PKGBUILD manual and freedesktop Desktop Entry/XDG specifications linked inside the relevant skills. ArchWiki clean-chroot retrieval was blocked during initial skill authoring; it is a reference, not an inspected source claim.

The README uses the community App badge at omarchy-badges commit 75975e5b5bf75e7ede3764bcd2950046f7abfe2c. Badge layout is MIT; the Omarchy name/icon retain their owners' rights. The adapter's window/code icon is original SVG artwork under this repository's MIT licence. No desktop screenshots are supplied or claimed.
