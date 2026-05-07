use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// A named Starship configuration profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    /// Display name (derived from filename without extension).
    pub name: String,
    /// Absolute path to the TOML file on disk.
    pub path: PathBuf,
    /// Raw TOML content.
    pub content: String,
}

impl Profile {
    /// Load a profile from a `.toml` file.
    pub fn load(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("reading profile {}", path.display()))?;
        let name = path
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| "untitled".into());
        Ok(Self {
            name,
            path: path.to_path_buf(),
            content,
        })
    }

    /// Persist current content back to disk.
    pub fn save(&self) -> Result<()> {
        fs::write(&self.path, &self.content)
            .with_context(|| format!("writing profile {}", self.path.display()))
    }
}

/// List all `.toml` profiles in the given directory, creating it if needed.
pub fn list_profiles(dir: &Path) -> Result<Vec<Profile>> {
    if !dir.exists() {
        fs::create_dir_all(dir)
            .with_context(|| format!("creating profiles dir {}", dir.display()))?;
    }
    let mut profiles = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "toml") {
            match Profile::load(&path) {
                Ok(p) => profiles.push(p),
                Err(e) => eprintln!("Warning: skipping {}: {e}", path.display()),
            }
        }
    }
    profiles.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(profiles)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn load_and_save_profile() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("test.toml");
        fs::write(&path, "[character]\nsymbol = \"➜\"").unwrap();

        let mut profile = Profile::load(&path).unwrap();
        assert_eq!(profile.name, "test");
        assert!(profile.content.contains("➜"));

        profile.content = "[character]\nsymbol = \"λ\"".into();
        profile.save().unwrap();
        let reloaded = Profile::load(&path).unwrap();
        assert!(reloaded.content.contains("λ"));
    }

    #[test]
    fn list_profiles_creates_dir_and_reads() {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path().join("profiles");
        // dir doesn't exist yet
        let profiles = list_profiles(&dir).unwrap();
        assert!(profiles.is_empty());
        assert!(dir.exists());

        // add a profile
        fs::write(dir.join("alpha.toml"), "# alpha").unwrap();
        fs::write(dir.join("beta.toml"), "# beta").unwrap();
        fs::write(dir.join("readme.md"), "not toml").unwrap();

        let profiles = list_profiles(&dir).unwrap();
        assert_eq!(profiles.len(), 2);
        assert_eq!(profiles[0].name, "alpha");
        assert_eq!(profiles[1].name, "beta");
    }
}
