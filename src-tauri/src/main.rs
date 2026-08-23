mod catalog_loader;
mod github_latest;
mod suite_settings;

use linux_command_core::{
    build_platform_snapshot, flatpak_is_installed, install_flatpak_bundle, launch_flatpak_app,
    list_installed_apps, PlatformSnapshot,
};
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

type CommandResult<T> = Result<T, String>;

#[derive(Debug, Clone, Serialize)]
struct AboutPath {
    id: String,
    path: String,
}

fn strip_host_gtk_modules() {
    std::env::remove_var("GTK_MODULES");
    std::env::remove_var("GTK3_MODULES");
}

fn platform_snapshot() -> CommandResult<PlatformSnapshot> {
    let catalog = catalog_loader::load_catalog().map_err(|err| err.to_string())?;
    let installed = list_installed_apps();
    Ok(build_platform_snapshot(&catalog, &installed))
}

#[tauri::command]
fn get_platform_snapshot() -> CommandResult<PlatformSnapshot> {
    platform_snapshot()
}

#[tauri::command]
fn launch_app(app_id: String) -> CommandResult<()> {
    if !flatpak_is_installed(&app_id) {
        return Err(format!("app not installed: {app_id}"));
    }
    launch_flatpak_app(&app_id)
}

#[tauri::command]
fn install_app(flatpak_url: String) -> CommandResult<PlatformSnapshot> {
    let url = validate_flatpak_url(&flatpak_url)?;
    let tmp = std::env::temp_dir().join(format!(
        "linux-command-{}.flatpak",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0)
    ));
    download_file(&url, &tmp)?;
    install_flatpak_bundle(tmp.to_str().ok_or("temp path invalid")?)?;
    let _ = fs::remove_file(&tmp);
    platform_snapshot()
}

#[tauri::command]
fn open_support(kind: String) -> CommandResult<()> {
    let url = match kind.trim().to_ascii_lowercase().as_str() {
        "discord" => "https://discord.com/users/406891052516114442",
        "paypal" => "https://www.paypal.com/paypalme/aurevo1",
        "revolut" => "https://revolut.me/mr_aurevo_x",
        "github" => "https://github.com/Mr-Aurevo-X",
        _ => return Err("unsupported support link".into()),
    };
    open_url(url)
}

#[tauri::command]
fn open_release(url: Option<String>) -> CommandResult<()> {
    let raw = url.unwrap_or_else(|| github_latest::RELEASES_PAGE.to_string());
    let url = github_latest::allowlisted_release_url(&raw)?;
    open_url(&url)
}

#[tauri::command]
fn open_loopback(url: String) -> CommandResult<()> {
    let url = validate_loopback_url(&url)?;
    open_url(&url)
}

#[tauri::command]
fn get_suite_settings() -> CommandResult<suite_settings::SuiteSettings> {
    suite_settings::load()
}

#[tauri::command]
fn set_suite_language(lang: String) -> CommandResult<suite_settings::SuiteSettings> {
    suite_settings::set_language(&lang)
}

#[tauri::command]
fn set_check_github_updates(enabled: bool) -> CommandResult<suite_settings::SuiteSettings> {
    suite_settings::set_check_github_updates(enabled)
}

#[tauri::command]
fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[tauri::command]
fn check_github_latest() -> CommandResult<github_latest::LatestCheck> {
    let settings = suite_settings::load()?;
    github_latest::check_latest(env!("CARGO_PKG_VERSION"), settings.check_github_updates)
}

#[tauri::command]
fn get_about_local_paths() -> CommandResult<Vec<AboutPath>> {
    let mut paths = Vec::new();
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            if !suite_settings::looks_like_clone_path(dir) {
                paths.push(AboutPath {
                    id: "app".into(),
                    path: dir.display().to_string(),
                });
            }
        }
    }
    if let Ok(path) = catalog_loader::catalog_override_path() {
        if let Some(parent) = path.parent() {
            paths.push(AboutPath {
                id: "data".into(),
                path: parent.display().to_string(),
            });
        }
        paths.push(AboutPath {
            id: "catalog_override".into(),
            path: path.display().to_string(),
        });
    }
    paths.push(AboutPath {
        id: "settings".into(),
        path: suite_settings::settings_path()?.display().to_string(),
    });
    Ok(paths)
}

fn validate_flatpak_url(raw: &str) -> CommandResult<String> {
    let url = raw.trim();
    if !url.starts_with("https://github.com/Mr-Aurevo-X/") || !url.ends_with(".flatpak") {
        return Err("flatpak url must be a Mr-Aurevo-X GitHub release bundle".into());
    }
    Ok(url.to_string())
}

fn validate_loopback_url(raw: &str) -> CommandResult<String> {
    let rest = if let Some(rest) = raw.strip_prefix("http://127.0.0.1:") {
        rest
    } else if let Some(rest) = raw.strip_prefix("http://localhost:") {
        rest
    } else {
        return Err("only http://127.0.0.1:* or http://localhost:* is allowed".into());
    };

    let port_end = rest
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(rest.len());
    if port_end == 0 {
        return Err("loopback URL must include a numeric port".into());
    }
    let port = rest[..port_end]
        .parse::<u16>()
        .map_err(|_| "loopback URL port is invalid".to_string())?;
    if port == 0 {
        return Err("loopback URL port must be greater than zero".into());
    }
    Ok(raw.to_string())
}

fn download_file(url: &str, dest: &PathBuf) -> CommandResult<()> {
    let status = Command::new("curl")
        .args(["-fsSL", "-o", dest.to_str().ok_or("dest path invalid")?, url])
        .status()
        .map_err(|err| err.to_string())?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("download failed with status {status}"))
    }
}

fn open_url(url: &str) -> CommandResult<()> {
    let status = Command::new("xdg-open")
        .arg(url)
        .status()
        .map_err(|err| err.to_string())?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("open url failed with status {status}"))
    }
}

fn main() {
    strip_host_gtk_modules();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_platform_snapshot,
            launch_app,
            install_app,
            open_support,
            open_release,
            open_loopback,
            get_suite_settings,
            set_suite_language,
            set_check_github_updates,
            get_app_version,
            check_github_latest,
            get_about_local_paths,
        ])
        .run(tauri::generate_context!())
        .expect("run Linux Command Tauri application");
}
