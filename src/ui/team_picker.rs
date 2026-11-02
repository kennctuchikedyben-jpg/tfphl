use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::state::app_state::{AppState, FPHL_TEAMS};

pub fn render_team_picker(f: &mut Frame, state: &AppState, area: Rect) {
    let popup_area = centered_rect(55, 75, area);
    f.render_widget(Clear, popup_area);

    let cur_fav = state.config.favorite_team.as_deref().unwrap_or("");

    let mut lines = vec![
        Line::from(vec![
            Span::styled(" SELECT FAVORITE TEAM ", Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)),
        ]),
        Line::styled(" Highlighted team will be accented across scoreboards & standings.", Style::default().fg(Color::DarkGray)),
        Line::raw(""),
    ];

    for (idx, &(code, name)) in FPHL_TEAMS.iter().enumerate() {
        let is_cursor = idx == state.team_picker_idx;
        let is_current = if code.is_empty() {
            cur_fav.is_empty()
        } else {
            cur_fav == code
        };

        let pointer = if is_cursor { " ► " } else { "   " };
        let check = if is_current { "[✓] " } else { "[ ] " };

        let code_str = if code.is_empty() { "---" } else { code };

        let row_style = if is_cursor {
            Style::default().fg(Color::Yellow).bg(Color::DarkGray).add_modifier(Modifier::BOLD)
        } else if is_current {
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        lines.push(Line::from(vec![
            Span::styled(pointer, if is_cursor { Style::default().fg(Color::Yellow) } else { Style::default() }),
            Span::styled(check, if is_current { Style::default().fg(Color::Green) } else { Style::default().fg(Color::DarkGray) }),
            Span::styled(format!("{:<4} ", code_str), Style::default().fg(Color::LightCyan)),
            Span::styled(name, row_style),
        ]));
    }

    lines.push(Line::raw(""));
    lines.push(Line::styled(" ─────────────────────────────────────────────", Style::default().fg(Color::DarkGray)));
    lines.push(Line::from(vec![
        Span::styled(" [Enter]", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw(" Save & Apply  "),
        Span::styled(" [Esc/q]", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw(" Cancel"),
    ]));

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .title(" Favorite Team ");

    let p = Paragraph::new(lines).block(block);
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
