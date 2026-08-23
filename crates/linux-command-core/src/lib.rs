pub mod catalog;
pub mod flatpak;
pub mod native;

pub use catalog::{
    build_platform_snapshot, install_command, load_catalog_from_json, repo_site_url, AppLifecycle,
    Catalog, CatalogApp, CatalogError, HubDef, HubStatus, HubView, PlatformMeta, PlatformSnapshot,
    PrimaryAppStatus,
};
pub use flatpak::{
    flatpak_is_installed, install_flatpak_bundle, launch_flatpak_app, list_installed_apps,
};
