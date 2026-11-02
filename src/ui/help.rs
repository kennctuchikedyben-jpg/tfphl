use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

pub fn render_help(f: &mut Frame, area: Rect) {
    let popup_area = centered_rect(65, 70, area);
    f.render_widget(Clear, popup_area);

    let key_lines = vec![
        Line::from(vec![
            Span::styled(" KEYBOARD SHORTCUTS & NAVIGATION ", Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)),
        ]),
        Line::raw(""),
        Line::from(vec![
            Span::styled("  1             ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw("Switch to Games / Scores View"),
        ]),
        Line::from(vec![
            Span::styled("  2             ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw("Switch to Standings View"),
        ]),
        Line::from(vec![
            Span::styled("  3             ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw("Switch to Leaders View (Skaters / Goalies)"),
        ]),
        Line::raw(""),
        Line::from(vec![
            Span::styled("  h / Left      ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw("Previous Game Date"),
        ]),
        Line::from(vec![
            Span::styled("  l / Right     ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw("Next Game Date"),
        ]),
        Line::from(vec![
            Span::styled("  t             ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw("Jump to Today's Date"),
        ]),
        Line::raw(""),
        Line::from(vec![
            Span::styled("  j / Down      ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw("Select Next Game / Item"),
        ]),
        Line::from(vec![
            Span::styled("  k / Up        ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw("Select Previous Game / Item"),
        ]),
        Line::from(vec![
            Span::styled("  Enter         ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw("Open Boxscore & Play-by-Play for Selected Game"),
        ]),
        Line::from(vec![
            Span::styled("  Tab / s / g   ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw("Toggle between Skaters and Goalies in Leaders view"),
        ]),
        Line::raw(""),
        Line::from(vec![
            Span::styled("  r             ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw("Force Reload Data from HockeyShift API"),
        ]),
        Line::from(vec![
            Span::styled("  ?             ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw("Toggle this Help Screen"),
        ]),
        Line::from(vec![
            Span::styled("  Esc / q       ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw("Close Popup / Exit Application"),
        ]),
        Line::raw(""),
        Line::styled(" ────────────────────────────────────────────────────────", Style::default().fg(Color::DarkGray)),
        Line::styled(" Config file: ~/.config/tfphl/tfphl.toml", Style::default().fg(Color::DarkGray)),
        Line::styled(" Press [Esc] or [?] to close", Style::default().fg(Color::LightCyan)),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .title(" Help ");

    let p = Paragraph::new(key_lines).block(block);
    f.render_widget(p, popup_area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
