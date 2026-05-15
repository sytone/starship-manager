use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::time::Duration;

use crate::app::{App, Modal, Pane};

/// Poll for a crossterm event with a timeout, then update app state.
/// Returns Ok(true) if the app should continue, Ok(false) if it should quit.
pub fn handle_events(app: &mut App) -> anyhow::Result<bool> {
    if !event::poll(Duration::from_millis(100))? {
        return Ok(true);
    }

    if let Event::Key(key) = event::read()? {
        // If a modal is open, any key closes it.
        if !matches!(app.modal, Modal::None) {
            app.modal = Modal::None;
            return Ok(true);
        }

        match key.code {
            // Global: quit (Ctrl+C always, q/Q when not editing)
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                app.quit = true;
                return Ok(false);
            }
            KeyCode::Char('q') | KeyCode::Char('Q') if app.focus != Pane::Editor => {
                app.quit = true;
                return Ok(false);
            }
            KeyCode::Esc => {
                if app.focus == Pane::Editor {
                    app.focus = Pane::Profiles;
                } else {
                    app.quit = true;
                    return Ok(false);
                }
            }
            // Tab: cycle pane focus
            KeyCode::Tab => {
                app.focus = match app.focus {
                    Pane::Profiles => Pane::Editor,
                    Pane::Editor => Pane::Preview,
                    Pane::Preview => Pane::Profiles,
                };
            }
            KeyCode::BackTab => {
                app.focus = match app.focus {
                    Pane::Profiles => Pane::Preview,
                    Pane::Editor => Pane::Profiles,
                    Pane::Preview => Pane::Editor,
                };
            }
            // Actions: Ctrl+key works in all panes, plain key works outside editor
            KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                app.save_profile()?;
            }
            KeyCode::Char('s') if app.focus != Pane::Editor => {
                app.save_profile()?;
            }
            KeyCode::Char('p') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                app.refresh_preview();
                app.status = "Preview refreshed.".into();
            }
            KeyCode::Char('p') if app.focus != Pane::Editor => {
                app.refresh_preview();
                app.status = "Preview refreshed.".into();
            }
            KeyCode::Char('a') if app.focus != Pane::Editor => {
                app.apply_profile()?;
            }
            KeyCode::Char('i') if app.focus != Pane::Editor => {
                app.install_starship();
            }
            KeyCode::Char('?') if app.focus != Pane::Editor => {
                app.modal = Modal::Help;
            }
            // Navigation in profiles list
            KeyCode::Up | KeyCode::Char('k')
                if app.focus == Pane::Profiles && app.selected > 0 =>
            {
                app.select_profile(app.selected - 1);
            }
            KeyCode::Down | KeyCode::Char('j')
                if app.focus == Pane::Profiles
                    && app.selected + 1 < app.profiles.len() =>
            {
                app.select_profile(app.selected + 1);
            }
            // Minimal editor key handling
            KeyCode::Up if app.focus == Pane::Editor && app.editor_cursor > 0 => {
                app.editor_cursor -= 1;
            }
            KeyCode::Down if app.focus == Pane::Editor => {
                let line_count = app.editor_content.lines().count();
                if app.editor_cursor + 1 < line_count {
                    app.editor_cursor += 1;
                }
            }
            KeyCode::Char(c) if app.focus == Pane::Editor => {
                // Very basic: append char at cursor line end
                let mut lines: Vec<String> = app.editor_content.lines().map(String::from).collect();
                if lines.is_empty() {
                    lines.push(String::new());
                }
                let idx = app.editor_cursor.min(lines.len() - 1);
                lines[idx].push(c);
                app.editor_content = lines.join("\n");
            }
            KeyCode::Enter if app.focus == Pane::Editor => {
                let mut lines: Vec<String> = app.editor_content.lines().map(String::from).collect();
                let idx = (app.editor_cursor + 1).min(lines.len());
                lines.insert(idx, String::new());
                app.editor_cursor = idx;
                app.editor_content = lines.join("\n");
            }
            KeyCode::Backspace if app.focus == Pane::Editor => {
                let mut lines: Vec<String> = app.editor_content.lines().map(String::from).collect();
                if !lines.is_empty() {
                    let idx = app.editor_cursor.min(lines.len() - 1);
                    if lines[idx].pop().is_none() && lines.len() > 1 {
                        lines.remove(idx);
                        app.editor_cursor = idx.saturating_sub(1);
                    }
                    app.editor_content = lines.join("\n");
                }
            }
            _ => {}
        }
    }

    Ok(true)
}
