use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Row, Table, Cell},
    Frame,
};

use crate::state::app_state::AppState;
use crate::models::roster::Player;

const HEADERS: &[&str] = &["#", "Name", "POS", "Exp.", "Birthday", "Shoots", "Height", "Weight", "Hometown"];

pub fn render_team_roster(f: &mut Frame, state: &AppState, area: Rect) {
    let popup_area = centered_rect(70, 75, area);
    f.render_widget(Clear, popup_area);

    let header = Line::from(vec![Span::styled(
        " TEAM ROSTER ",
        Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD),
    )]);
    let header_p = Paragraph::new(header).block(Block::default().borders(Borders::BOTTOM));
    f.render_widget(header_p, popup_area);

    let roster = state.roster();
    let title = if let Some(r) = roster {
        if r.team_name.is_empty() {
            "Roster".to_string()
        } else {
            format!("Roster — {}", r.team_name)
        }
    } else {
        "Roster".to_string()
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .title(title.clone());

    f.render_widget(block, popup_area);

    let players: &[Player] = match roster {
        Some(r) => r.players.as_slice(),
        None => &[],
    };
    if players.is_empty() {
        let p = Paragraph::new(vec![Line::styled(
            " No roster data available.",
            Style::default().fg(Color::DarkGray),
        )])
        .block(Block::default().borders(Borders::BOTTOM).title(title));
        f.render_widget(p, popup_area);
        return;
    }

    let col_widths = [
        Constraint::Length(6),
        Constraint::Min(22),
        Constraint::Length(6),
        Constraint::Length(8),
        Constraint::Length(16),
        Constraint::Length(6),
        Constraint::Length(8),
        Constraint::Length(8),
        Constraint::Min(20),
    ];

    let mut rows = Vec::new();
    for p in players {
        rows.push(Row::new(vec![
            Cell::from(p.number.clone()),
            Cell::from(p.name.clone()),
            Cell::from(p.position.clone()),
            Cell::from(p.experience.clone()),
            Cell::from(p.birthday.clone()),
            Cell::from(p.shoots.clone()),
            Cell::from(p.height.clone()),
            Cell::from(p.weight.clone()),
            Cell::from(p.hometown.clone()),
        ]));
    }

    let header_row = Row::new(
        HEADERS
            .iter()
            .map(|h| Cell::from(Span::styled(*h, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))))
            .collect::<Vec<_>>(),
    );

    let table = Table::new(rows, col_widths)
        .header(header_row)
        .block(Block::default().borders(Borders::BOTTOM).title(format!(" {} players ", players.len())));

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(10)])
        .split(popup_area);

    f.render_widget(Paragraph::new(vec![Line::raw("")]).block(Block::default().borders(Borders::BOTTOM)), chunks[0]);

    f.render_widget(table, chunks[1]);
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
