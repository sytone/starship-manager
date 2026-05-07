use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap};

use crate::app::{App, Modal, Pane};

/// Render the full TUI layout.
pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(1)])
        .split(f.area());

    draw_main_panes(f, app, chunks[0]);
    draw_status_bar(f, app, chunks[1]);

    // Modal overlay
    match &app.modal {
        Modal::None => {}
        Modal::Install { output } => draw_modal(f, "Install / Update", output),
        Modal::Help => draw_modal(
            f,
            "Help — Keybindings",
            "q        Quit\n\
             Tab      Cycle pane focus\n\
             Shift+Tab  Reverse cycle\n\
             ↑/↓ k/j Navigate list / editor\n\
             s        Save profile\n\
             p        Refresh preview\n\
             a        Apply profile to starship config\n\
             i        Install/update starship\n\
             ?        Show this help\n\n\
             Press any key to close.",
        ),
    }
}

fn draw_main_panes(f: &mut Frame, app: &App, area: Rect) {
    let panes = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(45),
            Constraint::Percentage(35),
        ])
        .split(area);

    draw_profiles_pane(f, app, panes[0]);
    draw_editor_pane(f, app, panes[1]);
    draw_preview_pane(f, app, panes[2]);
}

fn pane_style(app: &App, pane: Pane) -> Style {
    if app.focus == pane {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::White)
    }
}

fn draw_profiles_pane(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .profiles
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let style = if i == app.selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(Span::styled(&p.name, style))
        })
        .collect();

    let block = Block::default()
        .title(" Profiles ")
        .borders(Borders::ALL)
        .border_style(pane_style(app, Pane::Profiles));

    let list = List::new(items).block(block);
    f.render_widget(list, area);
}

fn draw_editor_pane(f: &mut Frame, app: &App, area: Rect) {
    let lines: Vec<Line> = app
        .editor_content
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let style = if i == app.editor_cursor && app.focus == Pane::Editor {
                Style::default().bg(Color::DarkGray)
            } else {
                Style::default()
            };
            Line::styled(format!("{:>3} │ {}", i + 1, line), style)
        })
        .collect();

    let block = Block::default()
        .title(" Editor (TOML) ")
        .borders(Borders::ALL)
        .border_style(pane_style(app, Pane::Editor));

    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false });
    f.render_widget(paragraph, area);
}

fn draw_preview_pane(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title(" Preview ")
        .borders(Borders::ALL)
        .border_style(pane_style(app, Pane::Preview));

    let text = if app.preview_output.is_empty() {
        "No preview available.\nPress 'p' to refresh or install starship."
    } else {
        &app.preview_output
    };

    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: false });
    f.render_widget(paragraph, area);
}

fn draw_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let bar = Paragraph::new(Span::styled(
        &app.status,
        Style::default()
            .fg(Color::White)
            .bg(Color::DarkGray)
            .add_modifier(Modifier::BOLD),
    ));
    f.render_widget(bar, area);
}

fn draw_modal(f: &mut Frame, title: &str, body: &str) {
    let area = f.area();
    let width = (area.width as f32 * 0.7) as u16;
    let height = (area.height as f32 * 0.6) as u16;
    let x = (area.width.saturating_sub(width)) / 2;
    let y = (area.height.saturating_sub(height)) / 2;
    let modal_area = Rect::new(x, y, width, height);

    f.render_widget(Clear, modal_area);

    let block = Block::default()
        .title(format!(" {title} "))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Magenta));

    let paragraph = Paragraph::new(body).block(block).wrap(Wrap { trim: false });
    f.render_widget(paragraph, modal_area);
}
