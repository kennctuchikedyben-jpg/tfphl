use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::state::app_state::{AppState, Tab};

pub fn render_header(f: &mut Frame, state: &AppState, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(28),
            Constraint::Min(20),
            Constraint::Length(32),
        ])
        .split(area);

    // 1. App Title
    let title_line = Line::from(vec![
        Span::styled(" tfphl ", Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::styled(" FPHL Hockey Tracker", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
    ]);
    let title_p = Paragraph::new(title_line).block(Block::default().borders(Borders::BOTTOM));
    f.render_widget(title_p, chunks[0]);

    // 2. Navigation Tabs
    let tab_style = |t: Tab| {
        if state.active_tab == t {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        }
    };

    let tabs_line = Line::from(vec![
        Span::styled("[1] Games", tab_style(Tab::Games)),
        Span::raw("   "),
        Span::styled("[2] Standings", tab_style(Tab::Standings)),
        Span::raw("   "),
        Span::styled("[3] Leaders", tab_style(Tab::Leaders)),
        Span::raw("   "),
        Span::styled("[4] Roster", tab_style(Tab::Roster)),
        Span::raw("   "),
        Span::styled("[?] Help", Style::default().fg(Color::DarkGray)),
    ]);
    let tabs_p = Paragraph::new(tabs_line)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::BOTTOM));
    f.render_widget(tabs_p, chunks[1]);

    // 3. Date navigator (shown mainly when on Games tab)
    let date_str = state.current_date().unwrap_or("No Date");
    let fav_team = state.config.favorite_team.as_deref().unwrap_or("-");
    let date_line = Line::from(vec![
        Span::raw("◄ "),
        Span::styled(date_str, Style::default().fg(Color::LightCyan).add_modifier(Modifier::BOLD)),
        Span::raw(" ►  "),
        Span::styled(format!("Fav: {}", fav_team), Style::default().fg(Color::Green)),
    ]);
    let date_p = Paragraph::new(date_line)
        .alignment(Alignment::Right)
        .block(Block::default().borders(Borders::BOTTOM));
    f.render_widget(date_p, chunks[2]);
}
