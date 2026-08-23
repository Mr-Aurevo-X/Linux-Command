use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CatalogError {
    #[error("invalid catalog json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("unsupported catalog schema_version {0}")]
    UnsupportedVersion(u32),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AppLifecycle {
    Active,
    Interim,
    Deprecated,
}

impl Default for AppLifecycle {
    fn default() -> Self {
        Self::Active
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformMeta {
    pub id: String,
    pub name: String,
    pub commander_app: String,
    #[serde(default)]
    pub windows_pairing: Option<String>,
    #[serde(default)]
    pub updated: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Catalog {
    pub schema_version: u32,
    pub platform: PlatformMeta,
    pub hubs: Vec<HubDef>,
    pub apps: Vec<CatalogApp>,
    #[serde(default)]
    pub games: Option<GamesSection>,
    #[serde(default)]
    pub excluded: Vec<ExcludedApp>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum HubStatus {
    Active,
    Planned,
    Deprecated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HubDef {
    pub id: String,
    pub status: HubStatus,
    pub order: i32,
    pub label_fr: String,
    pub label_en: String,
    #[serde(default)]
    pub description_fr: Option<String>,
    #[serde(default)]
    pub description_en: Option<String>,
    #[serde(default)]
    pub pc_command_mirror: Option<String>,
    #[serde(default)]
    pub linux_only: bool,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default = "default_visible_in_commander")]
    pub visible_in_commander: bool,
    #[serde(default)]
    pub primary_app: Option<String>,
}

fn default_visible_in_commander() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogApp {
    pub id: String,
    pub hub: String,
    pub name: String,
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub lifecycle: AppLifecycle,
    #[serde(default)]
    pub description_fr: Option<String>,
    #[serde(default)]
    pub description_en: Option<String>,
    pub version: String,
    #[serde(default)]
    pub flatpak_url: Option<String>,
    #[serde(default)]
    pub release_repo: Option<String>,
    #[serde(default)]
    pub data_paths: Vec<String>,
    pub launch: String,
    #[serde(default = "default_show_in_grid")]
    pub show_in_grid: bool,
}

fn default_show_in_grid() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamesSection {
    pub hub: String,
    pub enabled: bool,
    #[serde(default)]
    pub visible_in_commander: Option<bool>,
    #[serde(default)]
    pub shell: Option<String>,
    #[serde(default)]
    pub base_url: Option<String>,
    #[serde(default)]
    pub entries: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExcludedApp {
    pub id: String,
    #[serde(default)]
    pub flatpak_id: Option<String>,
    #[serde(default)]
    pub reason_fr: Option<String>,
    #[serde(default)]
    pub reason_en: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimaryAppStatus {
    pub id: String,
    pub name: String,
    pub catalog_version: String,
    pub installed: bool,
    pub installed_version: Option<String>,
    pub flatpak_url: Option<String>,
    pub launch: String,
    pub install_command: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HubView {
    pub id: String,
    pub status: HubStatus,
    pub order: i32,
    pub label_fr: String,
    pub label_en: String,
    pub description_fr: Option<String>,
    pub description_en: Option<String>,
    pub pc_command_mirror: Option<String>,
    pub linux_only: bool,
    pub icon: Option<String>,
    pub visible_in_commander: bool,
    pub primary_app: Option<PrimaryAppStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformSnapshot {
    pub platform: PlatformMeta,
    pub hubs: Vec<HubView>,
    pub catalog_updated: Option<String>,
}

pub fn load_catalog_from_json(raw: &str) -> Result<Catalog, CatalogError> {
    let catalog: Catalog = serde_json::from_str(raw)?;
    if catalog.schema_version < 2 {
        return Err(CatalogError::UnsupportedVersion(catalog.schema_version));
    }
    Ok(catalog)
}

pub fn install_command(flatpak_url: &str) -> String {
    format!("flatpak install --user -y {flatpak_url}")
}

pub fn repo_site_url(release_repo: Option<&str>) -> Option<String> {
    let repo = release_repo?.trim();
    let mut parts = repo.split('/');
    let owner = parts.next()?;
    let name = parts.next()?;
    if parts.next().is_some() || owner != "Mr-Aurevo-X" || name.is_empty() {
        return None;
    }
    if !name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_'))
    {
        return None;
    }
    Some(format!("https://github.com/{owner}/{name}"))
}

fn resolve_primary_app(
    catalog: &Catalog,
    hub: &HubDef,
    installed: &HashMap<String, String>,
) -> Option<PrimaryAppStatus> {
    let app_id = hub.primary_app.as_ref()?;
    let app = catalog.apps.iter().find(|a| &a.id == app_id)?;
    if app.lifecycle == AppLifecycle::Deprecated {
        return None;
    }
    let installed_version = installed.get(app_id).cloned();
    let flatpak_url = app.flatpak_url.clone();
    let install_command = repo_site_url(app.release_repo.as_deref());
    Some(PrimaryAppStatus {
        id: app.id.clone(),
        name: app.name.clone(),
        catalog_version: app.version.clone(),
        installed: installed_version.is_some(),
        installed_version,
        flatpak_url,
        launch: app.launch.clone(),
        install_command,
    })
}

pub fn build_platform_snapshot(
    catalog: &Catalog,
    installed: &HashMap<String, String>,
) -> PlatformSnapshot {
    let mut hubs: Vec<HubView> = catalog
        .hubs
        .iter()
        .filter(|h| h.id != "commander" && h.visible_in_commander)
        .map(|hub| HubView {
            id: hub.id.clone(),
            status: hub.status.clone(),
            order: hub.order,
            label_fr: hub.label_fr.clone(),
            label_en: hub.label_en.clone(),
            description_fr: hub.description_fr.clone(),
            description_en: hub.description_en.clone(),
            pc_command_mirror: hub.pc_command_mirror.clone(),
            linux_only: hub.linux_only,
            icon: hub.icon.clone(),
            visible_in_commander: hub.visible_in_commander,
            primary_app: resolve_primary_app(catalog, hub, installed),
        })
        .collect();

    hubs.sort_by_key(|h| h.order);

    PlatformSnapshot {
        platform: catalog.platform.clone(),
        hubs,
        catalog_updated: catalog.platform.updated.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = include_str!("../resources/catalog.fixture.json");

    #[test]
    fn parses_v2_and_resolves_primary_app() {
        let catalog = load_catalog_from_json(FIXTURE).expect("catalog");
        let snapshot = build_platform_snapshot(&catalog, &HashMap::new());
        assert!(!snapshot.hubs.iter().any(|h| h.id == "jeux"));
        let systeme = snapshot
            .hubs
            .iter()
            .find(|h| h.id == "systeme")
            .expect("systeme hub");
        let primary = systeme.primary_app.as_ref().expect("primary");
        assert_eq!(primary.id, "org.mraurevox.HubSysteme");
    }

    #[test]
    fn snapshot_exposes_installed_version() {
        let catalog = load_catalog_from_json(FIXTURE).expect("catalog");
        let mut installed = HashMap::new();
        installed.insert("org.mraurevox.HubSysteme".into(), "1.2.1".into());
        let snapshot = build_platform_snapshot(&catalog, &installed);
        let primary = snapshot
            .hubs
            .iter()
            .find(|h| h.id == "systeme")
            .expect("systeme hub")
            .primary_app
            .as_ref()
            .expect("primary");
        assert!(primary.installed);
        assert_eq!(primary.installed_version.as_deref(), Some("1.2.1"));
        assert_eq!(primary.catalog_version, "1.1.3");
    }

    #[test]
    fn snapshot_install_command_is_repo_site() {
        let catalog = load_catalog_from_json(FIXTURE).expect("catalog");
        let snapshot = build_platform_snapshot(&catalog, &HashMap::new());
        let primary = snapshot
            .hubs
            .iter()
            .find(|h| h.id == "systeme")
            .expect("systeme hub")
            .primary_app
            .as_ref()
            .expect("primary");
        assert_eq!(
            primary.install_command.as_deref(),
            Some("https://github.com/Mr-Aurevo-X/Hub-Systeme-Linux")
        );
        assert!(!primary
            .install_command
            .as_deref()
            .unwrap_or("")
            .contains("flatpak"));
    }

    #[test]
    fn repo_site_url_rejects_non_allowlisted() {
        assert_eq!(
            repo_site_url(Some("Mr-Aurevo-X/Hub-Systeme-Linux")).as_deref(),
            Some("https://github.com/Mr-Aurevo-X/Hub-Systeme-Linux")
        );
        assert_eq!(repo_site_url(Some("evil/repo")), None);
        assert_eq!(repo_site_url(Some("Mr-Aurevo-X/foo/bar")), None);
        assert_eq!(repo_site_url(Some("")), None);
    }
}
