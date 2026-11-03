use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::state::app_state::{AppState, Tab};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    None,
    Quit,
    RefreshAll,
    FetchBoxscore(u64),
}

pub fn handle_key(key: KeyEvent, state: &mut AppState) -> Action {
    // Check for Ctrl+C
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
        return Action::Quit;
    }

    // Modal overlay handling
    if state.show_help {
        if matches!(key.code, KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('?')) {
            state.show_help = false;
        }
        return Action::None;
    }

    if state.show_boxscore {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                state.show_boxscore = false;
            }
            KeyCode::Down | KeyCode::Char('j') => {
                state.scroll_boxscore_down(1);
            }
            KeyCode::Up | KeyCode::Char('k') => {
                state.scroll_boxscore_up(1);
            }
            KeyCode::PageDown => {
                state.scroll_boxscore_down(10);
            }
            KeyCode::PageUp => {
                state.scroll_boxscore_up(10);
            }
            KeyCode::Home | KeyCode::Char('g') => {
                state.scroll_boxscore_top();
            }
            KeyCode::End | KeyCode::Char('G') => {
                state.scroll_boxscore_bottom();
            }
            _ => {}
        }
        return Action::None;
    }

    if state.show_team_picker {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                state.show_team_picker = false;
            }
            KeyCode::Down | KeyCode::Char('j') => {
                state.select_next_team();
            }
            KeyCode::Up | KeyCode::Char('k') => {
                state.select_prev_team();
            }
            KeyCode::Enter => {
                state.confirm_team_selection();
            }
            _ => {}
        }
        return Action::None;
    }

    match key.code {
        KeyCode::Char('q') => Action::Quit,
        KeyCode::Char('?') => {
            state.show_help = true;
            Action::None
        }
        KeyCode::Char('f') => {
            state.open_team_picker();
            Action::None
        }
        KeyCode::Char('r') => Action::RefreshAll,

        // Tab switching
        KeyCode::Char('1') => {
            state.active_tab = Tab::Games;
            Action::None
        }
        KeyCode::Char('2') => {
            state.active_tab = Tab::Standings;
            Action::None
        }
        KeyCode::Char('3') => {
            state.active_tab = Tab::Leaders;
            Action::None
        }

        // Date navigation
        KeyCode::Left | KeyCode::Char('h') => {
            state.prev_date();
            Action::None
        }
        KeyCode::Right | KeyCode::Char('l') => {
            state.next_date();
            Action::None
        }
        KeyCode::Char('t') => {
            state.jump_to_today();
            Action::None
        }

        // Item navigation
        KeyCode::Down | KeyCode::Char('j') => {
            state.select_next_item();
            Action::None
        }
        KeyCode::Up | KeyCode::Char('k') => {
            state.select_prev_item();
            Action::None
        }

        // Leaders subtab toggle
        KeyCode::Tab | KeyCode::Char('s') | KeyCode::Char('g') => {
            if state.active_tab == Tab::Leaders {
                state.toggle_leaders_tab();
            }
            Action::None
        }

        // Enter action
        KeyCode::Enter => {
            if state.active_tab == Tab::Games {
                let maybe_game_id = state.selected_game().map(|g| g.game_id);
                if let Some(game_id) = maybe_game_id {
                    state.show_boxscore = true;
                    state.boxscore_scroll = 0;
                    return Action::FetchBoxscore(game_id);
                }
            }
            Action::None
        }

        _ => Action::None,
    }
}
