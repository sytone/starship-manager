use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Metadata embedded in a theme bundle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleMeta {
    pub name: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub version: String,
}

/// A shareable theme bundle = metadata + raw TOML config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeBundle {
    pub meta: BundleMeta,
    pub config_toml: String,
}

impl ThemeBundle {
    /// Export a profile to a JSON bundle file.
    pub fn export_to_file(&self, path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json).with_context(|| format!("writing bundle {}", path.display()))
    }

    /// Import a bundle from a JSON file.
    pub fn import_from_file(path: &Path) -> Result<Self> {
        let data = fs::read_to_string(path)
            .with_context(|| format!("reading bundle {}", path.display()))?;
        let bundle: Self = serde_json::from_str(&data)?;
        Ok(bundle)
    }
}

// We need serde_json for bundle serialization.
// Add it as a dependency below.

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn roundtrip_bundle() {
        let bundle = ThemeBundle {
            meta: BundleMeta {
                name: "neon".into(),
                author: "dev".into(),
                description: "A neon theme".into(),
                version: "1.0.0".into(),
            },
            config_toml: "[character]\nsymbol = \"➜\"".into(),
        };
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("neon.json");
        bundle.export_to_file(&path).unwrap();
        let loaded = ThemeBundle::import_from_file(&path).unwrap();
        assert_eq!(loaded.meta.name, "neon");
        assert!(loaded.config_toml.contains("➜"));
    }
}
