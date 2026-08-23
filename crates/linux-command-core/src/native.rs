use std::collections::HashMap;
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

pub fn merge_native_installed(home: &Path, mut installed: HashMap<String, String>) -> HashMap<String, String> {
    for (app_id, _) in NATIVE_LAUNCHERS {
        if installed.contains_key(*app_id) {
            continue;
        }
        if native_launcher_path(home, app_id).is_some() {
            installed.insert((*app_id).to_string(), "local".to_string());
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
        ("commander".to_string(), format!("{quoted} --with-commander")),
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
        assert_eq!(merged.get("org.mraurevox.HubDev").map(String::as_str), Some("local"));
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
        assert_eq!(merged.get("org.mraurevox.HubDev").map(String::as_str), Some("1.0.0"));
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
