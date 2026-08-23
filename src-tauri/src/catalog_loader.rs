use linux_command_core::{load_catalog_from_json, Catalog, CatalogError};
use std::fs;
use std::path::PathBuf;

const EMBEDDED_CATALOG: &str = include_str!("../../resources/catalog.json");

pub fn catalog_override_path() -> Result<PathBuf, String> {
    if let Some(data_home) = std::env::var_os("XDG_DATA_HOME") {
        return Ok(PathBuf::from(data_home)
            .join("linux-command")
            .join("catalog.override.json"));
    }
    let home = std::env::var_os("HOME").ok_or_else(|| "$HOME is not set".to_string())?;
    Ok(PathBuf::from(home)
        .join(".local")
        .join("share")
        .join("linux-command")
        .join("catalog.override.json"))
}

pub fn load_catalog() -> Result<Catalog, CatalogError> {
    let mut catalog = load_catalog_from_json(EMBEDDED_CATALOG)?;
    if let Ok(path) = catalog_override_path() {
        if path.is_file() {
            if let Ok(raw) = fs::read_to_string(&path) {
                if let Ok(override_catalog) = load_catalog_from_json(&raw) {
                    catalog = merge_catalog(catalog, override_catalog);
                }
            }
        }
    }
    Ok(catalog)
}

fn merge_catalog(base: Catalog, overlay: Catalog) -> Catalog {
    let mut merged = base;
    if overlay.platform.updated.is_some() {
        merged.platform.updated = overlay.platform.updated;
    }
    for hub in overlay.hubs {
        if let Some(existing) = merged.hubs.iter_mut().find(|h| h.id == hub.id) {
            *existing = hub;
        } else {
            merged.hubs.push(hub);
        }
    }
    for app in overlay.apps {
        if let Some(existing) = merged.apps.iter_mut().find(|a| a.id == app.id) {
            *existing = app;
        } else {
            merged.apps.push(app);
        }
    }
    if overlay.games.is_some() {
        merged.games = overlay.games;
    }
    if !overlay.excluded.is_empty() {
        merged.excluded = overlay.excluded;
    }
    merged
}
