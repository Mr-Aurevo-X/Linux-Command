use std::collections::HashMap;
use std::process::{Command, Stdio};

pub fn list_installed_apps() -> HashMap<String, String> {
    let output = Command::new("flatpak")
        .args(["list", "--app", "--columns=application,version"])
        .output();

    let Ok(output) = output else {
        return HashMap::new();
    };

    if !output.status.success() {
        return HashMap::new();
    }

    let text = String::from_utf8_lossy(&output.stdout);
    let mut map = HashMap::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("Application") {
            continue;
        }
        let mut parts = line.split_whitespace();
        let Some(id) = parts.next() else {
            continue;
        };
        let version = parts.next().unwrap_or("").to_string();
        map.insert(id.to_string(), version);
    }
    map
}

pub fn flatpak_is_installed(app_id: &str) -> bool {
    Command::new("flatpak")
        .args(["info", app_id])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

pub fn launch_flatpak_app(app_id: &str) -> Result<(), String> {
    Command::new("flatpak")
        .args(["run", app_id])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|err| err.to_string())?;
    Ok(())
}

pub fn install_flatpak_bundle(path: &str) -> Result<(), String> {
    let status = Command::new("flatpak")
        .args(["install", "--user", "-y", path])
        .status()
        .map_err(|err| err.to_string())?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("flatpak install failed with status {status}"))
    }
}
