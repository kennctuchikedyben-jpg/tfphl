use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::{
    state::app_state::{AppState, Tab},
    ui::{
        boxscore::render_boxscore, footer::render_footer, games::render_games, header::render_header,
        help::render_help, leaders::render_leaders, standings::render_standings,
    },
};

pub fn render_app(f: &mut Frame, state: &AppState) {
    let size = f.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // Top Header
            Constraint::Min(10),   // Main Content Area
            Constraint::Length(2), // Footer hints
        ])
        .split(size);

    // 1. Header
    render_header(f, state, chunks[0]);

    // 2. Main Tab View
    match state.active_tab {
        Tab::Games => render_games(f, state, chunks[1]),
        Tab::Standings => render_standings(f, state, chunks[1]),
        Tab::Leaders => render_leaders(f, state, chunks[1]),
    }

    // 3. Footer
    render_footer(f, state, chunks[2]);

    // 4. Overlays
    if state.show_boxscore {
        render_boxscore(f, state, size);
    }

    if state.show_help {
        render_help(f, size);
    }
}
