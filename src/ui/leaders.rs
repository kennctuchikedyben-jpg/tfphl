use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

use crate::state::app_state::{AppState, LeadersTab};

pub fn render_leaders(f: &mut Frame, state: &AppState, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(10)])
        .split(area);

    // Toggle Tab selector header: [Skaters] | [Goalies]
    let skater_style = if state.leaders_tab == LeadersTab::Skaters {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let goalie_style = if state.leaders_tab == LeadersTab::Goalies {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let selector_line = Line::from(vec![
        Span::raw(" View: "),
        Span::styled(" [S] Skaters ", skater_style),
        Span::raw(" | "),
        Span::styled(" [G] Goalies ", goalie_style),
        Span::styled("  (Press [Tab] or [s]/[g] to switch)", Style::default().fg(Color::DarkGray)),
    ]);
    let selector_p = Paragraph::new(selector_line).block(Block::default().borders(Borders::BOTTOM));
    f.render_widget(selector_p, chunks[0]);

    match state.leaders_tab {
        LeadersTab::Skaters => render_skater_leaders(f, state, chunks[1]),
        LeadersTab::Goalies => render_goalie_leaders(f, state, chunks[1]),
    }
}

fn render_skater_leaders(f: &mut Frame, state: &AppState, area: Rect) {
    if state.skaters.is_empty() {
        let p = Paragraph::new(vec![
            Line::raw(""),
            Line::styled(" No skater stats available.", Style::default().fg(Color::DarkGray)),
            Line::styled(" Press [r] to reload data.", Style::default().fg(Color::Yellow)),
        ])
        .block(Block::default().borders(Borders::ALL).title(" Skater Leaders "));
        f.render_widget(p, area);
        return;
    }

    let fav_team = state.config.favorite_team.as_deref().unwrap_or("");
    let mut rows = Vec::new();

    for skater in state.skaters.iter().take(30) {
        let is_fav = !fav_team.is_empty() && skater.team.contains(fav_team);
        let team_style = if is_fav {
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        rows.push(Row::new(vec![
            Cell::from(skater.rank.to_string()),
            Cell::from(skater.name.clone()),
            Cell::from(skater.jersey.clone()),
            Cell::from(Span::styled(skater.team.clone(), team_style)),
            Cell::from(skater.pos.clone()),
            Cell::from(skater.gp.to_string()),
            Cell::from(Span::styled(skater.goals.to_string(), Style::default().fg(Color::LightGreen))),
            Cell::from(Span::styled(skater.assists.to_string(), Style::default().fg(Color::LightCyan))),
            Cell::from(Span::styled(skater.points.to_string(), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
            Cell::from(skater.ppga.clone()),
            Cell::from(skater.plus_minus.clone()),
            Cell::from(skater.pim.to_string()),
            Cell::from(skater.gwg.to_string()),
            Cell::from(skater.sog.to_string()),
        ]));
    }

    let header = Row::new(vec![
        "Rk", "Name", "#", "Team", "Pos", "GP", "G", "A", "PTS", "PPGA", "+/-", "PIM", "GWG", "SOG",
    ])
    .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

    let widths = [
        Constraint::Length(4),
        Constraint::Min(20),
        Constraint::Length(4),
        Constraint::Length(22),
        Constraint::Length(5),
        Constraint::Length(5),
        Constraint::Length(5),
        Constraint::Length(5),
        Constraint::Length(6),
        Constraint::Length(7),
        Constraint::Length(6),
        Constraint::Length(6),
        Constraint::Length(6),
        Constraint::Length(6),
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title(format!(" Top Skaters ({} leaders) ", state.skaters.len())));
    f.render_widget(table, area);
}

fn render_goalie_leaders(f: &mut Frame, state: &AppState, area: Rect) {
    if state.goalies.is_empty() {
        let p = Paragraph::new(vec![
            Line::raw(""),
            Line::styled(" No goalie stats available.", Style::default().fg(Color::DarkGray)),
            Line::styled(" Press [r] to reload data.", Style::default().fg(Color::Yellow)),
        ])
        .block(Block::default().borders(Borders::ALL).title(" Goalie Leaders "));
        f.render_widget(p, area);
        return;
    }

    let fav_team = state.config.favorite_team.as_deref().unwrap_or("");
    let mut rows = Vec::new();

    for goalie in state.goalies.iter().take(25) {
        let is_fav = !fav_team.is_empty() && goalie.team.contains(fav_team);
        let team_style = if is_fav {
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        rows.push(Row::new(vec![
            Cell::from(goalie.rank.to_string()),
            Cell::from(goalie.name.clone()),
            Cell::from(Span::styled(goalie.team.clone(), team_style)),
            Cell::from(goalie.gp.to_string()),
            Cell::from(Span::styled(goalie.w.to_string(), Style::default().fg(Color::Green))),
            Cell::from(goalie.l.to_string()),
            Cell::from(goalie.otl.to_string()),
            Cell::from(Span::styled(goalie.gaa.clone(), Style::default().fg(Color::Yellow))),
            Cell::from(Span::styled(goalie.sv_pct.clone(), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))),
            Cell::from(goalie.so.to_string()),
        ]));
    }

    let header = Row::new(vec!["Rk", "Name", "Team", "GP", "W", "L", "OTL", "GAA", "SV%", "SO"])
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

    let widths = [
        Constraint::Length(4),
        Constraint::Min(22),
        Constraint::Length(24),
        Constraint::Length(5),
        Constraint::Length(5),
        Constraint::Length(5),
        Constraint::Length(5),
        Constraint::Length(7),
        Constraint::Length(8),
        Constraint::Length(5),
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title(format!(" Top Goalies ({} leaders) ", state.goalies.len())));
    f.render_widget(table, area);
}
