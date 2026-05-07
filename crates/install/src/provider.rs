use anyhow::{Context, Result};
use std::process::Command;

/// Supported install/update providers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallProvider {
    /// `winget install Starship.Starship` (Windows)
    Winget,
    /// `brew install starship` (macOS / Linux)
    Brew,
    /// Official install script `curl -sS https://starship.rs/install.sh | sh`
    Script,
}

impl std::fmt::Display for InstallProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Winget => write!(f, "winget"),
            Self::Brew => write!(f, "brew"),
            Self::Script => write!(f, "install script"),
        }
    }
}

/// Detect which provider is available on this system.
pub fn detect_provider() -> Option<InstallProvider> {
    if cfg!(target_os = "windows") && which("winget") {
        return Some(InstallProvider::Winget);
    }
    if which("brew") {
        return Some(InstallProvider::Brew);
    }
    if cfg!(unix) {
        return Some(InstallProvider::Script);
    }
    None
}

/// Attempt to install or update starship using the given provider.
/// Returns the combined stdout+stderr output from the command.
pub fn install_or_update(provider: InstallProvider) -> Result<String> {
    let output = match provider {
        InstallProvider::Winget => Command::new("winget")
            .args([
                "install",
                "--id",
                "Starship.Starship",
                "--accept-source-agreements",
                "--accept-package-agreements",
            ])
            .output()
            .context("running winget")?,
        InstallProvider::Brew => Command::new("brew")
            .args(["install", "starship"])
            .output()
            .context("running brew install starship")?,
        InstallProvider::Script => Command::new("sh")
            .args([
                "-c",
                "curl -sS https://starship.rs/install.sh | sh -s -- --yes",
            ])
            .output()
            .context("running install script")?,
    };
    let mut result = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr);
    if !stderr.is_empty() {
        result.push('\n');
        result.push_str(&stderr);
    }
    Ok(result)
}

fn which(cmd: &str) -> bool {
    Command::new(if cfg!(target_os = "windows") {
        "where"
    } else {
        "which"
    })
    .arg(cmd)
    .output()
    .map(|o| o.status.success())
    .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_provider_returns_something_or_none() {
        // Just verify it doesn't panic.
        let _provider = detect_provider();
    }

    #[test]
    fn provider_display() {
        assert_eq!(format!("{}", InstallProvider::Brew), "brew");
    }
}
