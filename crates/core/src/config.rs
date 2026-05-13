use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// Return the path to the user's active `starship.toml`.
/// Checks `STARSHIP_CONFIG` env, then the default OS location.
pub fn active_starship_config_path() -> PathBuf {
    if let Ok(p) = std::env::var("STARSHIP_CONFIG") {
        return PathBuf::from(p);
    }
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".config")
        .join("starship.toml")
}

/// Apply a profile by copying its content to the active starship config location.
pub fn apply_profile(profile_content: &str, target: &Path) -> Result<()> {
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("creating parent dirs for {}", target.display()))?;
    }
    fs::write(target, profile_content)
        .with_context(|| format!("writing starship config to {}", target.display()))
}

/// Validate TOML content, returning an error message if invalid.
pub fn validate_toml(content: &str) -> Result<(), String> {
    content
        .parse::<toml_edit::DocumentMut>()
        .map(|_| ())
        .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_good_toml() {
        assert!(validate_toml("[character]\nsymbol = \"λ\"").is_ok());
    }

    #[test]
    fn validate_bad_toml() {
        assert!(validate_toml("[invalid toml !!!").is_err());
    }

    #[test]
    fn apply_profile_creates_file() {
        let tmp = tempfile::TempDir::new().unwrap();
        let target = tmp.path().join("sub").join("starship.toml");
        apply_profile("# hello", &target).unwrap();
        assert_eq!(fs::read_to_string(&target).unwrap(), "# hello");
    }
}
