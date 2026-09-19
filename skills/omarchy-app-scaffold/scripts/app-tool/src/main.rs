use std::{
    env, fs,
    path::{Component, Path, PathBuf},
    process::Command,
};
const BADGE: &str = "https://raw.githubusercontent.com/tcballard/omarchy-badges/75975e5b5bf75e7ede3764bcd2950046f7abfe2c/badges/v1/omarchy-app.svg";
type Result<T> = std::result::Result<T, String>;
fn read(p: &Path) -> Result<String> {
    fs::read_to_string(p).map_err(|e| format!("{}: {e}", p.display()))
}
fn write(p: &Path, s: &str) -> Result<()> {
    fs::write(p, s).map_err(|e| e.to_string())
}
fn scaffold(p: &Path) -> Result<()> {
    fs::create_dir(p).map_err(|e| format!("new destination required: {e}"))?;
    write(&p.join("README.md"), &format!("# Omarchy app development scaffold\n\n<img alt=\"Built for Omarchy: App\" height=\"20\" src=\"{BADGE}\">\n\nStatus: development scaffold; no GUI implementation yet.\nIntended Omarchy target: 4 / Hyprland; establish a supported range during implementation.\nTested Omarchy versions: none. Live desktop acceptance: not run.\n\n## Build and run\n\nDocumentation scaffold only; there is no executable to build or run yet. Add verified commands with the first implementation.\n\n## Install and rollback\n\nNo package or desktop configuration is installed by this scaffold. No uninstall is required. Preserve user data when implementing package removal.\n\n## Evidence\n\nSee [verification](VERIFICATION.md) and [credits](CREDITS.md). This community badge does not imply official acceptance.\n"))?;
    write(&p.join("ARCHITECTURE.md"), "# Architecture worksheet\n\nStatus: proposed; no application implementation yet. Fill in relevant decisions as the first slice is built. Keep existing project conventions; these are responsibilities, not required directories.\n\n## Stack and boundaries\n\nRecord the language/toolkit and why it fits. Name startup, application/session, domain, UI and desktop/storage responsibilities, combining them where appropriate. State dependency direction and any genuinely shared facilities.\n\n## State and resource owners\n\nIdentify authoritative state, transient UI state, saves/locks, workers and child processes. Describe cancellation, stale-result rejection and cleanup on document or feature replacement.\n\n## Lifecycle\n\nDefine focus loss, modal entry, resume and close where relevant. Record final-save, worker teardown and lock-release ordering, including error paths.\n\n## Verification\n\nLink relevant core, adapter and UI regressions in VERIFICATION.md. Separate proposed decisions from implemented behavior and native desktop evidence.\n")?;
    write(&p.join("CREDITS.md"), "# Credits\n\nCommunity App badge: https://github.com/tcballard/omarchy-badges (MIT layout; Omarchy name/icon rights retained by their owners). Badge linked remotely; no other artwork included. Choose the application licence before distribution.\n")?;
    write(&p.join("VERIFICATION.md"), "# Verification\n\nInput identity: not recorded.\nUpstream revision: not recorded.\nToolchain/platform: not recorded.\n\n## Reproduced now\n\nNo application checks run. Record each command, exit result and input identity here.\n\n## Historical\n\nNone recorded. Preserve original input associations.\n\n## Failed\n\nNone recorded.\n\n## Not run\n\nApplication build, core tests, GUI tests, package build, installation, upgrade, removal and live Omarchy acceptance.\n")
}
fn valid_relative(s: &str) -> bool {
    !s.is_empty()
        && !s.contains('\\')
        && !s.chars().any(char::is_control)
        && Path::new(s)
            .components()
            .all(|c| matches!(c, Component::Normal(_)))
}
fn collect(root: &Path, dir: &Path, out: &mut Vec<String>) -> Result<()> {
    for e in fs::read_dir(dir).map_err(|e| e.to_string())? {
        let e = e.map_err(|e| e.to_string())?;
        let p = e.path();
        let ft = e.file_type().map_err(|e| e.to_string())?;
        if ft.is_symlink() {
            return Err(format!("symlink refused: {}", p.display()));
        }
        let name = e.file_name();
        if ft.is_dir() && (name == ".git" || name == "target") {
            continue;
        }
        if dir == root && name == "EVIDENCE.sha256" {
            continue;
        }
        if ft.is_dir() {
            collect(root, &p, out)?;
        } else if ft.is_file() {
            let s = p
                .strip_prefix(root)
                .map_err(|e| e.to_string())?
                .to_str()
                .ok_or("non-UTF8 path")?
                .to_string();
            if !valid_relative(&s) {
                return Err("unsupported path".into());
            }
            out.push(s);
        } else {
            return Err(format!("non-regular file: {}", p.display()));
        }
    }
    Ok(())
}
fn manifest(p: &Path) -> Result<String> {
    let mut paths = vec![];
    collect(p, p, &mut paths)?;
    paths.sort();
    let mut result = String::new();
    for path in paths {
        let o = Command::new("sha256sum")
            .arg("--")
            .arg(p.join(&path))
            .output()
            .map_err(|e| format!("sha256sum required: {e}"))?;
        if !o.status.success() {
            return Err(format!("hash failed: {path}"));
        }
        let text = String::from_utf8(o.stdout).map_err(|e| e.to_string())?;
        let hash = text.split_whitespace().next().ok_or("missing digest")?;
        if hash.len() != 64 || !hash.bytes().all(|c| c.is_ascii_hexdigit()) {
            return Err("invalid digest".into());
        }
        result.push_str(&format!("{hash}  {path}\n"));
    }
    Ok(result)
}
fn links(s: &str) -> Vec<&str> {
    let mut result = vec![];
    for (open, close) in [("](", ")"), ("src=\"", "\""), ("href=\"", "\"")] {
        let mut rest = s;
        while let Some(start) = rest.find(open) {
            rest = &rest[start + open.len()..];
            if let Some(end) = rest.find(close) {
                result.push(&rest[..end]);
                rest = &rest[end + close.len()..];
            } else {
                break;
            }
        }
    }
    result
}
fn check(p: &Path) -> Result<()> {
    // Enumerate without executing project contents; reject symlink escapes first.
    let mut paths = vec![];
    collect(p, p, &mut paths)?;
    let readme = read(&p.join("README.md"))?;
    if !readme.contains(BADGE) || !readme.contains("height=\"20\"") {
        return Err("missing canonical 20px App badge".into());
    }
    let badge_tag = readme
        .split('<')
        .find(|x| x.starts_with("img ") && x.contains(BADGE))
        .ok_or("badge must be an img tag")?;
    let badge_tag = badge_tag.split('>').next().unwrap_or(badge_tag);
    if !badge_tag.contains("height=\"20\"") || badge_tag.contains("width=") {
        return Err("badge must preserve proportions at height 20".into());
    }
    for marker in [
        "Status:",
        "Intended Omarchy target:",
        "Tested Omarchy versions:",
        "## Build and run",
        "## Install and rollback",
    ] {
        if !readme.contains(marker) {
            return Err(format!("missing documentation field: {marker}"));
        }
    }
    let evidence = read(&p.join("VERIFICATION.md"))?;
    for marker in [
        "Input identity:",
        "Upstream revision:",
        "Toolchain/platform:",
        "## Reproduced now",
        "## Historical",
        "## Failed",
        "## Not run",
    ] {
        if !evidence.contains(marker) {
            return Err(format!("missing evidence field: {marker}"));
        }
    }
    for file in ["README.md", "CREDITS.md"] {
        let text = read(&p.join(file))?;
        for link in links(&text) {
            if link.contains("://") || link.starts_with('#') || link.starts_with("mailto:") {
                continue;
            }
            let path = link.split('#').next().unwrap_or(link);
            if !valid_relative(path) || !p.join(path).exists() {
                return Err(format!("invalid/missing local reference in {file}: {link}"));
            }
        }
    }
    Ok(())
}
fn run(action: &str, p: &Path) -> Result<()> {
    if action == "scaffold" {
        return scaffold(p);
    }
    if fs::symlink_metadata(p)
        .map_err(|e| e.to_string())?
        .file_type()
        .is_symlink()
    {
        return Err("root symlink refused".into());
    }
    match action {
        "check" => check(p),
        "snapshot" => {
            let s = manifest(p)?;
            use std::io::Write;
            let mut f = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(p.join("EVIDENCE.sha256"))
                .map_err(|e| format!("new evidence file required: {e}"))?;
            f.write_all(s.as_bytes()).map_err(|e| e.to_string())
        }
        "verify" => {
            let actual = manifest(p)?;
            if read(&p.join("EVIDENCE.sha256"))? != actual {
                Err("input evidence differs: added, removed or changed files".into())
            } else {
                Ok(())
            }
        }
        _ => Err("use scaffold|check|snapshot|verify PATH".into()),
    }
}
fn main() {
    let args: Vec<_> = env::args().collect();
    let result = if args.len() == 3 {
        run(&args[1], &PathBuf::from(&args[2]))
    } else {
        Err("use scaffold|check|snapshot|verify PATH".into())
    };
    match result {
        Ok(()) => {
            println!("OK: requested static operation completed; no runtime acceptance established")
        }
        Err(e) => {
            eprintln!("ERROR: {e}");
            std::process::exit(1);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    struct Temp(PathBuf);
    impl Temp {
        fn new() -> Self {
            let p = env::temp_dir().join(format!(
                "app-tool-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            scaffold(&p).unwrap();
            Self(p)
        }
    }
    impl Drop for Temp {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.0).unwrap();
        }
    }
    #[test]
    fn scaffold_is_honest() {
        let t = Temp::new();
        check(&t.0).unwrap();
        assert!(read(&t.0.join("README.md"))
            .unwrap()
            .contains("Tested Omarchy versions: none"));
    }
    #[test]
    fn refuses_overwrite() {
        let t = Temp::new();
        let before = manifest(&t.0).unwrap();
        assert!(scaffold(&t.0).is_err());
        assert_eq!(before, manifest(&t.0).unwrap());
    }
    #[test]
    fn detects_missing_badge() {
        let t = Temp::new();
        let p = t.0.join("README.md");
        write(&p, &read(&p).unwrap().replace(BADGE, "wrong.svg")).unwrap();
        assert!(check(&t.0).is_err());
    }
    #[test]
    fn detects_distorted_badge() {
        let t = Temp::new();
        let p = t.0.join("README.md");
        write(
            &p,
            &read(&p)
                .unwrap()
                .replace("height=\"20\"", "height=\"20\" width=\"1\""),
        )
        .unwrap();
        assert!(check(&t.0).is_err());
    }
    #[test]
    fn stale_credit_fails() {
        let t = Temp::new();
        write(&t.0.join("CREDITS.md"), "[excluded art](missing.svg)").unwrap();
        assert!(check(&t.0).is_err());
    }
    #[test]
    fn missing_test_disclosure_fails() {
        let t = Temp::new();
        let p = t.0.join("README.md");
        write(
            &p,
            &read(&p)
                .unwrap()
                .replace("Tested Omarchy versions:", "Compatible everywhere:"),
        )
        .unwrap();
        assert!(check(&t.0).is_err());
    }
    #[test]
    fn rejects_traversal() {
        assert!(!valid_relative("../outside"));
        assert!(!valid_relative("/outside"));
        assert!(!valid_relative("a\nb"));
    }
    #[test]
    fn evidence_roundtrip_and_refuses_overwrite() {
        let t = Temp::new();
        run("snapshot", &t.0).unwrap();
        run("verify", &t.0).unwrap();
        assert!(run("snapshot", &t.0).is_err());
    }
    #[test]
    fn evidence_detects_changed_file() {
        let t = Temp::new();
        run("snapshot", &t.0).unwrap();
        write(&t.0.join("README.md"), "changed").unwrap();
        assert!(run("verify", &t.0).is_err());
    }
    #[test]
    fn evidence_detects_added_file() {
        let t = Temp::new();
        run("snapshot", &t.0).unwrap();
        write(&t.0.join("new.rs"), "new input").unwrap();
        assert!(run("verify", &t.0).is_err());
    }
    #[test]
    fn evidence_detects_removed_file() {
        let t = Temp::new();
        run("snapshot", &t.0).unwrap();
        fs::remove_file(t.0.join("CREDITS.md")).unwrap();
        assert!(run("verify", &t.0).is_err());
    }
    #[test]
    fn evidence_from_other_files_fails() {
        let t = Temp::new();
        let u = Temp::new();
        write(&u.0.join("other.rs"), "other source").unwrap();
        run("snapshot", &u.0).unwrap();
        fs::copy(u.0.join("EVIDENCE.sha256"), t.0.join("EVIDENCE.sha256")).unwrap();
        assert!(run("verify", &t.0).is_err());
    }
    #[cfg(unix)]
    #[test]
    fn rejects_symlinks() {
        let t = Temp::new();
        std::os::unix::fs::symlink("/etc/passwd", t.0.join("escape")).unwrap();
        assert!(check(&t.0).is_err());
        assert!(run("snapshot", &t.0).is_err());
    }
}
