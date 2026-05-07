use anyhow::Result;
use starship_manager_core::config;
use starship_manager_core::profile::{self, Profile};
use starship_manager_install::provider;
use starship_manager_preview::render;

/// The focus pane in the TUI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pane {
    Profiles,
    Editor,
    Preview,
}

/// Modal overlay state.
#[derive(Debug, Clone)]
pub enum Modal {
    None,
    Install { output: String },
    Help,
}

/// Application state.
pub struct App {
    /// Whether the app should quit.
    pub quit: bool,
    /// Currently focused pane.
    pub focus: Pane,
    /// Loaded profiles.
    pub profiles: Vec<Profile>,
    /// Index of the selected profile in the list.
    pub selected: usize,
    /// Content of the editor pane (editable TOML).
    pub editor_content: String,
    /// Cursor line in the editor (zero-based).
    pub editor_cursor: usize,
    /// Preview output (ANSI-stripped prompt text).
    pub preview_output: String,
    /// Status bar message.
    pub status: String,
    /// Current modal overlay.
    pub modal: Modal,
}

impl App {
    /// Create a new App, loading profiles from the default directory.
    pub fn new() -> Result<Self> {
        let profiles_dir = starship_manager_core::profiles_dir();
        let profiles = profile::list_profiles(&profiles_dir)?;

        let (editor_content, preview_output) = if let Some(p) = profiles.first() {
            let preview = Self::generate_preview(&p.content);
            (p.content.clone(), preview)
        } else {
            (
                String::from(
                    "# No profiles found.\n# Add .toml files to your profiles directory.\n",
                ),
                String::new(),
            )
        };

        Ok(Self {
            quit: false,
            focus: Pane::Profiles,
            profiles,
            selected: 0,
            editor_content,
            editor_cursor: 0,
            preview_output,
            status: String::from(
                "q:quit  Tab:switch pane  s:save  p:preview  a:apply  i:install  ?:help",
            ),
            modal: Modal::None,
        })
    }

    /// Select a profile by index and load its content.
    pub fn select_profile(&mut self, idx: usize) {
        if idx < self.profiles.len() {
            self.selected = idx;
            self.editor_content = self.profiles[idx].content.clone();
            self.editor_cursor = 0;
            self.refresh_preview();
        }
    }

    /// Refresh the preview pane by invoking starship with current editor content.
    pub fn refresh_preview(&mut self) {
        self.preview_output = Self::generate_preview(&self.editor_content);
    }

    fn generate_preview(toml_content: &str) -> String {
        match render::preview_starship(toml_content) {
            Ok(ansi) => render::strip_ansi(&ansi),
            Err(e) => format!("[preview error: {e}]"),
        }
    }

    /// Save editor content to the selected profile.
    pub fn save_profile(&mut self) -> Result<()> {
        if let Some(profile) = self.profiles.get_mut(self.selected) {
            // Validate first
            if let Err(msg) = config::validate_toml(&self.editor_content) {
                self.status = format!("TOML error: {msg}");
                return Ok(());
            }
            profile.content = self.editor_content.clone();
            profile.save()?;
            self.status = format!("Saved: {}", profile.name);
        }
        Ok(())
    }

    /// Apply current profile to the real starship config.
    pub fn apply_profile(&mut self) -> Result<()> {
        let target = config::active_starship_config_path();
        config::apply_profile(&self.editor_content, &target)?;
        if let Some(p) = self.profiles.get(self.selected) {
            self.status = format!("Applied '{}' → {}", p.name, target.display());
        }
        Ok(())
    }

    /// Run install/update via detected provider.
    pub fn install_starship(&mut self) {
        let output = match provider::detect_provider() {
            Some(prov) => {
                self.status = format!("Installing via {prov}...");
                match provider::install_or_update(prov) {
                    Ok(out) => out,
                    Err(e) => format!("Error: {e}"),
                }
            }
            None => "No install provider detected (need winget, brew, or curl).".into(),
        };
        self.modal = Modal::Install { output };
    }
}
