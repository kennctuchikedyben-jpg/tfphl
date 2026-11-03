use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Scrollbar, ScrollbarOrientation, ScrollbarState, Table},
    Frame,
};

use crate::state::app_state::AppState;

pub fn render_boxscore(f: &mut Frame, state: &AppState, area: Rect) {
    let sel_game = match state.selected_game() {
        Some(g) => g,
        None => return,
    };

    // Render as a centered modal dialog covering 85% width & 85% height
    let popup_area = centered_rect(85, 85, area);
    f.render_widget(Clear, popup_area);

    let boxscore = state.boxscore_data.as_ref();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Length(7),
            Constraint::Length(4),
            Constraint::Min(8),
            Constraint::Length(2),
        ])
        .split(popup_area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .title(format!(" Boxscore — Game #{} ", sel_game.number));
    f.render_widget(block, popup_area);

    // 1. Header line: Teams and Score
    let away_score_str = sel_game.away_score.map(|s| s.to_string()).unwrap_or("-".to_string());
    let home_score_str = sel_game.home_score.map(|s| s.to_string()).unwrap_or("-".to_string());

    let header_lines = vec![
        Line::from(vec![
            Span::styled(format!(" {} ", sel_game.away_team), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::styled(format!(" {} ", away_score_str), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(" vs ", Style::default().fg(Color::DarkGray)),
            Span::styled(format!(" {} ", home_score_str), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(format!(" {} ", sel_game.home_team), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::styled(format!(" [{}]", sel_game.display_status()), Style::default().fg(Color::LightGreen)),
        ]),
        Line::from(vec![
            Span::styled(format!(" Venue: {}", sel_game.facility.as_deref().unwrap_or("TBD")), Style::default().fg(Color::DarkGray)),
            Span::styled(format!("  Rink: {}", sel_game.rink.as_deref().unwrap_or("TBD")), Style::default().fg(Color::DarkGray)),
        ]),
    ];
    let header_p = Paragraph::new(header_lines).block(Block::default().borders(Borders::BOTTOM));
    f.render_widget(header_p, chunks[0]);

    if state.boxscore_loading {
        let loading_p = Paragraph::new(vec![
            Line::raw(""),
            Line::styled(" Loading play-by-play and period statistics...", Style::default().fg(Color::Yellow)),
        ]);
        f.render_widget(loading_p, chunks[1]);
        return;
    }

    if let Some(box_data) = boxscore {
        // 2. Period by Period scoring & shots table
        let mut scoring_rows = Vec::new();
        let mut p1_a = "-"; let mut p1_h = "-";
        let mut p2_a = "-"; let mut p2_h = "-";
        let mut p3_a = "-"; let mut p3_h = "-";

        for p in &box_data.periods {
            if p.period == "1st" { p1_a = &p.away; p1_h = &p.home; }
            if p.period == "2nd" { p2_a = &p.away; p2_h = &p.home; }
            if p.period == "3rd" { p3_a = &p.away; p3_h = &p.home; }
        }

        scoring_rows.push(Row::new(vec![
            Cell::from(Span::styled(&sel_game.away_team_short, Style::default().fg(Color::White))),
            Cell::from(p1_a),
            Cell::from(p2_a),
            Cell::from(p3_a),
            Cell::from(Span::styled(box_data.away_score.to_string(), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))),
        ]));
        scoring_rows.push(Row::new(vec![
            Cell::from(Span::styled(&sel_game.home_team_short, Style::default().fg(Color::White))),
            Cell::from(p1_h),
            Cell::from(p2_h),
            Cell::from(p3_h),
            Cell::from(Span::styled(box_data.home_score.to_string(), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))),
        ]));

        let score_table = Table::new(
            scoring_rows,
            [
                Constraint::Length(8),
                Constraint::Length(6),
                Constraint::Length(6),
                Constraint::Length(6),
                Constraint::Length(6),
            ],
        )
        .header(Row::new(vec!["Team", "1st", "2nd", "3rd", "Total"]).style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)))
        .block(Block::default().borders(Borders::ALL).title(" Scoring by Period "));
        f.render_widget(score_table, chunks[1]);

        // 3. Team Stats Comparison: Power Plays & Penalties
        let stats_lines = vec![
            Line::from(vec![
                Span::styled(" Power Plays: ", Style::default().fg(Color::DarkGray)),
                Span::styled(format!("{} ({})", box_data.power_plays.0, sel_game.away_team_short), Style::default().fg(Color::White)),
                Span::raw(" vs "),
                Span::styled(format!("{} ({})", box_data.power_plays.1, sel_game.home_team_short), Style::default().fg(Color::White)),
                Span::raw("   |   "),
                Span::styled(" Penalty Minutes: ", Style::default().fg(Color::DarkGray)),
                Span::styled(format!("{} PIM", box_data.penalty_minutes.0), Style::default().fg(Color::White)),
                Span::raw(" vs "),
                Span::styled(format!("{} PIM", box_data.penalty_minutes.1), Style::default().fg(Color::White)),
            ]),
        ];
        let stats_p = Paragraph::new(stats_lines).block(Block::default().borders(Borders::BOTTOM));
        f.render_widget(stats_p, chunks[2]);

        // 4. Play-by-play events
        let total_plays = box_data.plays.len();
        // Visible height minus 2 (borders) and 1 (header row)
        let visible_height = (chunks[3].height.saturating_sub(3) as usize).max(1);
        let max_scroll = total_plays.saturating_sub(visible_height);
        let current_scroll = state.boxscore_scroll.min(max_scroll);
        let end_idx = (current_scroll + visible_height).min(total_plays);

        let visible_slice = if total_plays > 0 {
            &box_data.plays[current_scroll..end_idx]
        } else {
            &[]
        };

        let mut play_rows = Vec::new();
        for p in visible_slice {
            let event_style = match p.event_type.as_str() {
                "Goal" => Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                "Penalty" => Style::default().fg(Color::Red),
                "Shot" => Style::default().fg(Color::LightBlue),
                _ => Style::default().fg(Color::White),
            };

            play_rows.push(Row::new(vec![
                Cell::from(p.time.clone()),
                Cell::from(Span::styled(&p.event_type, event_style)),
                Cell::from(p.team.clone()),
                Cell::from(p.description.clone()),
            ]));
        }

        let title = if total_plays > visible_height {
            format!(
                " Play-by-Play (showing {}-{} of {} events | [j/k, ▲/▼] Scroll) ",
                current_scroll + 1,
                end_idx,
                total_plays
            )
        } else {
            format!(" Play-by-Play ({} events) ", total_plays)
        };

        let plays_table = Table::new(
            play_rows,
            [
                Constraint::Length(8),
                Constraint::Length(10),
                Constraint::Length(14),
                Constraint::Min(25),
            ],
        )
        .header(Row::new(vec!["Time", "Event", "Team", "Description"]).style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)))
        .block(Block::default().borders(Borders::ALL).title(title));
        f.render_widget(plays_table, chunks[3]);

        if total_plays > visible_height {
            let mut scrollbar_state = ScrollbarState::new(max_scroll).position(current_scroll);
            let scrollbar = Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("▲"))
                .end_symbol(Some("▼"));
            f.render_stateful_widget(scrollbar, chunks[3], &mut scrollbar_state);
        }
    } else {
        let no_data_p = Paragraph::new(vec![
            Line::raw(""),
            Line::styled(" No boxscore or play-by-play available for this game yet.", Style::default().fg(Color::DarkGray)),
        ]);
        f.render_widget(no_data_p, chunks[1]);
    }

    // 5. Modal Footer
    let footer_p = Paragraph::new(Line::from(vec![
        Span::styled(" [j/k, ▲/▼]", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw(" Scroll  "),
        Span::styled("[PgUp/PgDn]", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw(" Page  "),
        Span::styled("[g/G]", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw(" Top/End  "),
        Span::styled("[Esc/q]", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw(" Close"),
    ]));
    f.render_widget(footer_p, chunks[4]);
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
