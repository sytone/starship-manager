pub mod bundle;
pub mod config;
pub mod profile;

/// Returns the default configuration directory for starship-manager.
/// - Linux/macOS: `~/.config/starship-manager`
/// - Windows: `{FOLDERID_RoamingAppData}/starship-manager`
pub fn config_dir() -> std::path::PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("starship-manager")
}

/// Returns the profiles directory inside the config dir.
pub fn profiles_dir() -> std::path::PathBuf {
    config_dir().join("profiles")
}
