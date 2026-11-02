use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::state::app_state::AppState;

pub fn render_games(f: &mut Frame, state: &AppState, area: Rect) {
    let games = state.games_for_current_date();

    if games.is_empty() {
        let msg = Paragraph::new(vec![
            Line::raw(""),
            Line::styled(" No games scheduled on this date.", Style::default().fg(Color::DarkGray)),
            Line::raw(""),
            Line::styled(" Use [h]/[l] or [◄]/[►] to browse dates.", Style::default().fg(Color::Yellow)),
        ])
        .block(Block::default().borders(Borders::ALL).title(" Games "));
        f.render_widget(msg, area);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // Left column: Game Matchup Cards
    let fav_team = state.config.favorite_team.as_deref().unwrap_or("");
    let card_height = 5;
    let visible_cards = (chunks[0].height as usize / card_height).max(1);

    // Compute scroll window
    let start_idx = if state.selected_game_idx >= visible_cards {
        state.selected_game_idx - visible_cards + 1
    } else {
        0
    };
    let end_idx = (start_idx + visible_cards).min(games.len());

    let mut card_constraints = Vec::new();
    for _ in start_idx..end_idx {
        card_constraints.push(Constraint::Length(card_height as u16));
    }
    card_constraints.push(Constraint::Min(0));

    let card_areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints(card_constraints)
        .split(chunks[0]);

    for (i, &game) in games[start_idx..end_idx].iter().enumerate() {
        let real_idx = start_idx + i;
        let is_selected = real_idx == state.selected_game_idx;

        let border_style = if is_selected {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let away_is_fav = !fav_team.is_empty() && (game.away_team_short.eq_ignore_ascii_case(fav_team) || game.away_team.contains(fav_team));
        let home_is_fav = !fav_team.is_empty() && (game.home_team_short.eq_ignore_ascii_case(fav_team) || game.home_team.contains(fav_team));

        let away_style = if away_is_fav {
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        let home_style = if home_is_fav {
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        let score_text = if let (Some(a), Some(h)) = (game.away_score, game.home_score) {
            format!("{} - {}", a, h)
        } else {
            game.time.clone()
        };

        let status_color = match game.status.as_str() {
            "Final" => Color::LightBlue,
            "In Progress" => Color::LightGreen,
            _ => Color::DarkGray,
        };

        let title = format!(" Game #{} — {} ", game.number, game.display_status());

        let lines = vec![
            Line::from(vec![
                Span::styled(format!(" {:<20}", game.away_team), away_style),
                Span::styled(format!("{:>8} ", game.away_score.map(|s| s.to_string()).unwrap_or_default()), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled(format!(" {:<20}", game.home_team), home_style),
                Span::styled(format!("{:>8} ", game.home_score.map(|s| s.to_string()).unwrap_or_default()), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled(format!(" Status: {}", game.display_status()), Style::default().fg(status_color)),
                Span::raw("  "),
                Span::styled(score_text, Style::default().fg(Color::Yellow)),
            ]),
        ];

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(if is_selected { BorderType::Double } else { BorderType::Plain })
            .border_style(border_style)
            .title(title);

        let p = Paragraph::new(lines).block(block);
        f.render_widget(p, card_areas[i]);
    }

    // Right column: Selected Game Preview & Summary
    if let Some(sel_game) = state.selected_game() {
        let mut detail_lines = vec![
            Line::from(vec![
                Span::styled(" MATCHUP SUMMARY ", Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]),
            Line::raw(""),
            Line::from(vec![
                Span::styled(" Away: ", Style::default().fg(Color::DarkGray)),
                Span::styled(&sel_game.away_team, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                Span::raw(" ("),
                Span::styled(&sel_game.away_team_short, Style::default().fg(Color::Yellow)),
                Span::raw(")"),
            ]),
            Line::from(vec![
                Span::styled(" Home: ", Style::default().fg(Color::DarkGray)),
                Span::styled(&sel_game.home_team, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                Span::raw(" ("),
                Span::styled(&sel_game.home_team_short, Style::default().fg(Color::Yellow)),
                Span::raw(")"),
            ]),
            Line::raw(""),
        ];

        if let (Some(a), Some(h)) = (sel_game.away_score, sel_game.home_score) {
            detail_lines.push(Line::from(vec![
                Span::styled(" Score: ", Style::default().fg(Color::DarkGray)),
                Span::styled(format!("{} - {}", a, h), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw("  Status: "),
                Span::styled(sel_game.display_status(), Style::default().fg(Color::LightCyan)),
            ]));
        }

        if let (Some(a_shots), Some(h_shots)) = (sel_game.away_shots, sel_game.home_shots) {
            detail_lines.push(Line::from(vec![
                Span::styled(" Shots: ", Style::default().fg(Color::DarkGray)),
                Span::styled(format!("{} (Away) - {} (Home)", a_shots, h_shots), Style::default().fg(Color::White)),
            ]));
        }

        if let Some(ref facility) = sel_game.facility {
            detail_lines.push(Line::from(vec![
                Span::styled(" Venue: ", Style::default().fg(Color::DarkGray)),
                Span::styled(facility, Style::default().fg(Color::White)),
            ]));
        }

        if let Some(ref rink) = sel_game.rink {
            detail_lines.push(Line::from(vec![
                Span::styled(" Rink:  ", Style::default().fg(Color::DarkGray)),
                Span::styled(rink, Style::default().fg(Color::White)),
            ]));
        }

        detail_lines.push(Line::raw(""));
        detail_lines.push(Line::styled(" ──────────────────────────────────────────", Style::default().fg(Color::DarkGray)));
        detail_lines.push(Line::raw(""));
        detail_lines.push(Line::from(vec![
            Span::styled(" Press ", Style::default().fg(Color::DarkGray)),
            Span::styled("[Enter]", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(" to open Full Boxscore & Play-by-Play feed", Style::default().fg(Color::White)),
        ]));

        let detail_p = Paragraph::new(detail_lines)
            .block(Block::default().borders(Borders::ALL).title(" Game Preview "));
        f.render_widget(detail_p, chunks[1]);
    }
}
