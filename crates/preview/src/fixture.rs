use anyhow::Result;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// A temporary fixture environment used to invoke `starship prompt`.
pub struct PreviewEnv {
    /// Temp dir that serves as the fake working directory.
    pub work_dir: TempDir,
    /// Optional temp git repo inside work_dir.
    pub git_init: bool,
}

impl PreviewEnv {
    /// Create a new fixture environment, optionally initialising a git repo.
    pub fn new(git_init: bool) -> Result<Self> {
        let work_dir = TempDir::new()?;
        if git_init {
            std::process::Command::new("git")
                .args(["init", "--quiet"])
                .current_dir(work_dir.path())
                .status()
                .ok();
        }
        Ok(Self { work_dir, git_init })
    }

    /// Path to the fixture working directory.
    pub fn path(&self) -> &Path {
        self.work_dir.path()
    }
}

/// Write the given TOML config to a temp file and return its path.
pub fn write_temp_config(dir: &Path, toml_content: &str) -> Result<PathBuf> {
    let config_path = dir.join("starship_preview.toml");
    std::fs::write(&config_path, toml_content)?;
    Ok(config_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixture_creates_dir() {
        let env = PreviewEnv::new(false).unwrap();
        assert!(env.path().exists());
    }
}
