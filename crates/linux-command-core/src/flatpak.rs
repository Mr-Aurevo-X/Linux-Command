use std::collections::HashMap;
use std::path::Path;
use std::process::{Command, Stdio};

fn running_in_flatpak() -> bool {
    std::env::var_os("FLATPAK_ID").is_some() || Path::new("/.flatpak-info").is_file()
}

pub(crate) fn flatpak_program_and_prefix(in_sandbox: bool) -> (String, Vec<String>) {
    if in_sandbox {
        (
            "flatpak-spawn".into(),
            vec!["--host".into(), "flatpak".into()],
        )
    } else {
        ("flatpak".into(), Vec::new())
    }
}

fn flatpak_cmd() -> Command {
    let (program, prefix) = flatpak_program_and_prefix(running_in_flatpak());
    let mut cmd = Command::new(program);
    cmd.args(prefix);
    cmd
}

pub(crate) fn parse_flatpak_list(text: &str) -> HashMap<String, String> {
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

pub fn list_installed_apps() -> HashMap<String, String> {
    let output = flatpak_cmd()
        .args(["list", "--app", "--columns=application,version"])
        .output();

    let Ok(output) = output else {
        return HashMap::new();
    };

    if !output.status.success() {
        return HashMap::new();
    }

    parse_flatpak_list(&String::from_utf8_lossy(&output.stdout))
}

pub fn flatpak_is_installed(app_id: &str) -> bool {
    flatpak_cmd()
        .args(["info", app_id])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

pub fn launch_flatpak_app(app_id: &str) -> Result<(), String> {
    flatpak_cmd()
        .args(["run", app_id])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|err| err.to_string())?;
    Ok(())
}

pub fn install_flatpak_bundle(path: &str) -> Result<(), String> {
    let status = flatpak_cmd()
        .args(["install", "--user", "-y", path])
        .status()
        .map_err(|err| err.to_string())?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("flatpak install failed with status {status}"))
    }
}

#[cfg(test)]
mod tests {
    use super::{flatpak_program_and_prefix, parse_flatpak_list};

    #[test]
    fn host_uses_flatpak_directly() {
        assert_eq!(
            flatpak_program_and_prefix(false),
            ("flatpak".into(), Vec::<String>::new())
        );
    }

    #[test]
    fn sandbox_uses_flatpak_spawn_host() {
        assert_eq!(
            flatpak_program_and_prefix(true),
            (
                "flatpak-spawn".into(),
                vec!["--host".into(), "flatpak".into()]
            )
        );
    }

    #[test]
    fn parses_host_user_hub_list() {
        let text = "\
org.mraurevox.HubSysteme\t1.1.1
org.mraurevox.HubReseau\t1.3.1
org.mraurevox.HubDev\t1.2.5
";
        let map = parse_flatpak_list(text);
        assert_eq!(
            map.get("org.mraurevox.HubSysteme").map(String::as_str),
            Some("1.1.1")
        );
        assert_eq!(
            map.get("org.mraurevox.HubReseau").map(String::as_str),
            Some("1.3.1")
        );
        assert_eq!(
            map.get("org.mraurevox.HubDev").map(String::as_str),
            Some("1.2.5")
        );
        assert!(!map.contains_key("org.mraurevox.LinuxCommand"));
    }
}
