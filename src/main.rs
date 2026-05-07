use anyhow::Result;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;

/// Guard that restores terminal state on drop (even on panic).
struct TerminalGuard;

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
    }
}

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let _guard = TerminalGuard;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = starship_manager_tui::App::new()?;

    // Main loop
    loop {
        terminal.draw(|f| starship_manager_tui::ui::draw(f, &app))?;
        if !starship_manager_tui::event::handle_events(&mut app)? {
            break;
        }
    }

    // _guard Drop restores terminal on normal exit, error, or panic
    Ok(())
}
