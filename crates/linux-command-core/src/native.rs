use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

const NATIVE_LAUNCHERS: &[(&str, &str)] = &[
    ("org.mraurevox.HubSysteme", "hub-systeme"),
    ("org.mraurevox.HubReseau", "hub-reseau"),
    ("org.mraurevox.HubSecurite", "hub-securite"),
    ("org.mraurevox.HubUtilitaires", "hub-utilitaires"),
    ("org.mraurevox.HubDev", "hub-dev"),
    ("org.mraurevox.LinuxCommand", "linux-command"),
];

pub fn native_bin_name(app_id: &str) -> Option<&'static str> {
    NATIVE_LAUNCHERS
        .iter()
        .find(|(id, _)| *id == app_id)
        .map(|(_, slug)| *slug)
}

pub fn local_bin_dir(home: &Path) -> PathBuf {
    home.join(".local").join("bin")
}

pub fn native_launcher_path(home: &Path, app_id: &str) -> Option<PathBuf> {
    let slug = native_bin_name(app_id)?;
    let path = local_bin_dir(home).join(slug);
    path.is_file().then_some(path)
}

fn sanitize_version(raw: &str) -> Option<String> {
    let version = raw.trim();
    if version.is_empty() || version.len() > 32 {
        return None;
    }
    if !version
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_'))
    {
        return None;
    }
    Some(version.to_string())
}

fn unquote(value: &str) -> &str {
    let value = value.trim();
    if (value.starts_with('"') && value.ends_with('"'))
        || (value.starts_with('\'') && value.ends_with('\''))
    {
        return &value[1..value.len() - 1];
    }
    value
}

fn lancer_parent(line: &str) -> Option<PathBuf> {
    const MARKER: &str = "/LANCER.sh";
    let end = line.find(MARKER)? + MARKER.len();
    let before = &line[..end];
    let start = before.rfind('"').or_else(|| before.rfind('\''))? + 1;
    if start >= end {
        return None;
    }
    PathBuf::from(&before[start..end])
        .parent()
        .map(Path::to_path_buf)
}

fn launcher_install_root(text: &str) -> Option<PathBuf> {
    for line in text.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("INSTALL_DIR=") {
            let path = unquote(rest);
            if !path.is_empty() && !path.contains('$') {
                return Some(PathBuf::from(path));
            }
        }
    }
    text.lines().find_map(lancer_parent)
}

fn read_version_file(root: &Path) -> Option<String> {
    sanitize_version(&fs::read_to_string(root.join("VERSION")).ok()?)
}

pub fn native_app_version(home: &Path, app_id: &str) -> Option<String> {
    let launcher = native_launcher_path(home, app_id)?;
    let text = fs::read_to_string(launcher).ok()?;
    let root = launcher_install_root(&text)?;
    read_version_file(&root)
}

pub fn merge_native_installed(
    home: &Path,
    mut installed: HashMap<String, String>,
) -> HashMap<String, String> {
    for (app_id, _) in NATIVE_LAUNCHERS {
        if installed.contains_key(*app_id) {
            continue;
        }
        if native_launcher_path(home, app_id).is_some() {
            let version = native_app_version(home, app_id).unwrap_or_else(|| "local".to_string());
            installed.insert((*app_id).to_string(), version);
        }
    }
    installed
}

pub fn local_wip_script(home: &Path) -> PathBuf {
    home.join("Documents")
        .join("Dev Tree Linux")
        .join("linux-platform")
        .join("scripts")
        .join("install-local-wip.sh")
}

pub fn local_profile_commands(home: &Path) -> Vec<(String, String)> {
    let script = local_wip_script(home);
    let quoted = format!("bash \"{}\"", script.display());
    vec![
        ("hubs".to_string(), quoted.clone()),
        (
            "commander".to_string(),
            format!("{quoted} --with-commander"),
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn maps_known_hub_ids() {
        assert_eq!(native_bin_name("org.mraurevox.HubDev"), Some("hub-dev"));
        assert_eq!(native_bin_name("org.mraurevox.Unknown"), None);
    }

    #[test]
    fn merge_detects_local_launcher() {
        let tmp = tempfile_home();
        let bin = local_bin_dir(&tmp);
        fs::create_dir_all(&bin).unwrap();
        fs::write(bin.join("hub-dev"), "#!/bin/sh\n").unwrap();
        let merged = merge_native_installed(&tmp, HashMap::new());
        assert_eq!(
            merged.get("org.mraurevox.HubDev").map(String::as_str),
            Some("local")
        );
        assert!(!merged.contains_key("org.mraurevox.HubSysteme"));
    }

    #[test]
    fn merge_keeps_flatpak_version() {
        let tmp = tempfile_home();
        let bin = local_bin_dir(&tmp);
        fs::create_dir_all(&bin).unwrap();
        fs::write(bin.join("hub-dev"), "#!/bin/sh\n").unwrap();
        let mut installed = HashMap::new();
        installed.insert("org.mraurevox.HubDev".into(), "1.0.0".into());
        let merged = merge_native_installed(&tmp, installed);
        assert_eq!(
            merged.get("org.mraurevox.HubDev").map(String::as_str),
            Some("1.0.0")
        );
    }

    #[test]
    fn merge_reads_version_from_wip_lancer_path() {
        let tmp = tempfile_home();
        let dest = tmp.join("App WIP").join("Hub-Dev-1.1.4");
        fs::create_dir_all(&dest).unwrap();
        fs::write(dest.join("VERSION"), "1.1.4\n").unwrap();
        fs::write(dest.join("LANCER.sh"), "#!/bin/sh\n").unwrap();
        let bin = local_bin_dir(&tmp);
        fs::create_dir_all(&bin).unwrap();
        fs::write(
            bin.join("hub-dev"),
            format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nexec bash \"{}/LANCER.sh\" \"$@\"\n",
                dest.display()
            ),
        )
        .unwrap();
        let merged = merge_native_installed(&tmp, HashMap::new());
        assert_eq!(
            merged.get("org.mraurevox.HubDev").map(String::as_str),
            Some("1.1.4")
        );
    }

    #[test]
    fn merge_reads_version_from_install_dir() {
        let tmp = tempfile_home();
        let dest = tmp.join(".local").join("share").join("hub-reseau");
        fs::create_dir_all(&dest).unwrap();
        fs::write(dest.join("VERSION"), "1.2.0\n").unwrap();
        let bin = local_bin_dir(&tmp);
        fs::create_dir_all(&bin).unwrap();
        fs::write(
            bin.join("hub-reseau"),
            format!(
                "#!/usr/bin/env bash\nINSTALL_DIR=\"{}\"\nexec bash \"${{INSTALL_DIR}}/LANCER.sh\" \"$@\"\n",
                dest.display()
            ),
        )
        .unwrap();
        let merged = merge_native_installed(&tmp, HashMap::new());
        assert_eq!(
            merged.get("org.mraurevox.HubReseau").map(String::as_str),
            Some("1.2.0")
        );
    }

    #[test]
    fn profile_commands_point_at_wip_script() {
        let home = PathBuf::from("/home/tester");
        let cmds = local_profile_commands(&home);
        assert_eq!(cmds[0].0, "hubs");
        assert!(cmds[0].1.contains("install-local-wip.sh"));
        assert!(cmds[1].1.ends_with("--with-commander"));
        assert!(!cmds[0].1.contains("install-platform.sh"));
    }

    fn tempfile_home() -> PathBuf {
        let root = std::env::temp_dir().join(format!(
            "linux-command-native-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&root).unwrap();
        root
    }
}
