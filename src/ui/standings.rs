use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

use crate::state::app_state::{AppState, is_favorite_team};

pub fn render_standings(f: &mut Frame, state: &AppState, area: Rect) {
    if state.divisions.is_empty() {
        let p = Paragraph::new(vec![
            Line::raw(""),
            Line::styled(" No standings data available.", Style::default().fg(Color::DarkGray)),
            Line::styled(" Press [r] to reload data.", Style::default().fg(Color::Yellow)),
        ])
        .block(Block::default().borders(Borders::ALL).title(" Standings "));
        f.render_widget(p, area);
        return;
    }

    // Split vertically if multiple divisions, or allow navigating division
    let num_divs = state.divisions.len();
    let mut constraints = Vec::new();
    for _ in 0..num_divs {
        constraints.push(Constraint::Ratio(1, num_divs as u32));
    }

    let div_areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);

    for (div_idx, division) in state.divisions.iter().enumerate() {
        let is_selected_div = div_idx == state.selected_division_idx;

        let mut rows = Vec::new();
        for row in &division.rows {
            let is_fav = is_favorite_team(state.config.favorite_team.as_deref(), &row.team_short, &row.team_name);

            let team_style = if is_fav {
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            let pts_style = Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD);

            rows.push(Row::new(vec![
                Cell::from(row.rank.to_string()),
                Cell::from(Span::styled(format!("{} ({})", row.team_name, row.team_short), team_style)),
                Cell::from(row.gp.to_string()),
                Cell::from(row.w.to_string()),
                Cell::from(row.l.to_string()),
                Cell::from(row.otl.to_string()),
                Cell::from(Span::styled(row.pts.to_string(), pts_style)),
                Cell::from(row.win_pct.clone()),
                Cell::from(row.gf.to_string()),
                Cell::from(row.ga.to_string()),
                Cell::from(row.diff.clone()),
                Cell::from(row.pim.to_string()),
                Cell::from(row.l10.clone()),
                Cell::from(row.streak.clone()),
            ]));
        }

        let widths = [
            Constraint::Length(4),  // Rk
            Constraint::Min(24),    // Team
            Constraint::Length(5),  // GP
            Constraint::Length(5),  // W
            Constraint::Length(5),  // L
            Constraint::Length(5),  // OTL
            Constraint::Length(6),  // PTS
            Constraint::Length(7),  // W%
            Constraint::Length(5),  // GF
            Constraint::Length(5),  // GA
            Constraint::Length(7),  // DIFF
            Constraint::Length(6),  // PIM
            Constraint::Length(10), // L10
            Constraint::Length(8),  // Streak
        ];

        let header = Row::new(vec![
            "Rk", "Team", "GP", "W", "L", "OTL", "PTS", "W%", "GF", "GA", "DIFF", "PIM", "L10", "Streak",
        ])
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

        let border_style = if is_selected_div {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(if is_selected_div { BorderType::Double } else { BorderType::Plain })
            .border_style(border_style)
            .title(format!(" {} ", division.name));

        let table = Table::new(rows, widths).header(header).block(block);
        f.render_widget(table, div_areas[div_idx]);
    }
}
