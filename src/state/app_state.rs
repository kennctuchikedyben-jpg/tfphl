use std::collections::BTreeSet;
use chrono::Local;

use crate::{
    models::{
        boxscore::GameBoxscore,
        games::Game,
        leaders::{GoalieLeader, SkaterLeader},
        standings::Division,
    },
    state::config::Config,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Games,
    Standings,
    Leaders,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeadersTab {
    Skaters,
    Goalies,
}

pub struct AppState {
    pub config: Config,
    pub active_tab: Tab,
    pub all_games: Vec<Game>,
    pub dates: Vec<String>,
    pub current_date_idx: usize,
    pub selected_game_idx: usize,
    pub divisions: Vec<Division>,
    pub selected_division_idx: usize,
    pub skaters: Vec<SkaterLeader>,
    pub goalies: Vec<GoalieLeader>,
    pub leaders_tab: LeadersTab,
    pub selected_leader_idx: usize,
    pub show_boxscore: bool,
    pub boxscore_data: Option<GameBoxscore>,
    pub boxscore_loading: bool,
    pub show_help: bool,
    pub loading: bool,
    pub error_message: Option<String>,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            active_tab: Tab::Games,
            all_games: Vec::new(),
            dates: Vec::new(),
            current_date_idx: 0,
            selected_game_idx: 0,
            divisions: Vec::new(),
            selected_division_idx: 0,
            skaters: Vec::new(),
            goalies: Vec::new(),
            leaders_tab: LeadersTab::Skaters,
            selected_leader_idx: 0,
            show_boxscore: false,
            boxscore_data: None,
            boxscore_loading: false,
            show_help: false,
            loading: true,
            error_message: None,
        }
    }

    pub fn set_games(&mut self, games: Vec<Game>) {
        let mut dates_set = BTreeSet::new();
        for g in &games {
            if !g.date.is_empty() {
                dates_set.insert(g.date.clone());
            }
        }

        self.dates = dates_set.into_iter().collect();
        self.all_games = games;

        // Try to select today or closest date
        let today = Local::now().format("%Y-%m-%d").to_string();
        if let Some(pos) = self.dates.iter().position(|d| d == &today) {
            self.current_date_idx = pos;
        } else if !self.dates.is_empty() {
            // Find the most recent date with played or scheduled games
            let pos = self
                .dates
                .iter()
                .rposition(|d| d <= &today)
                .unwrap_or(self.dates.len().saturating_sub(1));
            self.current_date_idx = pos;
        }
        self.selected_game_idx = 0;
    }

    pub fn current_date(&self) -> Option<&str> {
        self.dates.get(self.current_date_idx).map(|s| s.as_str())
    }

    pub fn games_for_current_date(&self) -> Vec<&Game> {
        if let Some(cur_date) = self.current_date() {
            self.all_games.iter().filter(|g| g.date == cur_date).collect()
        } else {
            Vec::new()
        }
    }

    pub fn selected_game(&self) -> Option<&Game> {
        let games = self.games_for_current_date();
        games.get(self.selected_game_idx).copied()
    }

    pub fn next_date(&mut self) {
        if !self.dates.is_empty() && self.current_date_idx + 1 < self.dates.len() {
            self.current_date_idx += 1;
            self.selected_game_idx = 0;
            self.boxscore_data = None;
            self.show_boxscore = false;
        }
    }

    pub fn prev_date(&mut self) {
        if self.current_date_idx > 0 {
            self.current_date_idx -= 1;
            self.selected_game_idx = 0;
            self.boxscore_data = None;
            self.show_boxscore = false;
        }
    }

    pub fn jump_to_today(&mut self) {
        let today = Local::now().format("%Y-%m-%d").to_string();
        if let Some(pos) = self.dates.iter().position(|d| d == &today) {
            self.current_date_idx = pos;
            self.selected_game_idx = 0;
            self.boxscore_data = None;
            self.show_boxscore = false;
        }
    }

    pub fn select_next_item(&mut self) {
        match self.active_tab {
            Tab::Games => {
                let count = self.games_for_current_date().len();
                if count > 0 && self.selected_game_idx + 1 < count {
                    self.selected_game_idx += 1;
                    self.boxscore_data = None;
                }
            }
            Tab::Standings => {
                if !self.divisions.is_empty() && self.selected_division_idx + 1 < self.divisions.len() {
                    self.selected_division_idx += 1;
                }
            }
            Tab::Leaders => {
                let count = match self.leaders_tab {
                    LeadersTab::Skaters => self.skaters.len(),
                    LeadersTab::Goalies => self.goalies.len(),
                };
                if count > 0 && self.selected_leader_idx + 1 < count {
                    self.selected_leader_idx += 1;
                }
            }
        }
    }

    pub fn select_prev_item(&mut self) {
        match self.active_tab {
            Tab::Games => {
                if self.selected_game_idx > 0 {
                    self.selected_game_idx -= 1;
                    self.boxscore_data = None;
                }
            }
            Tab::Standings => {
                if self.selected_division_idx > 0 {
                    self.selected_division_idx -= 1;
                }
            }
            Tab::Leaders => {
                if self.selected_leader_idx > 0 {
                    self.selected_leader_idx -= 1;
                }
            }
        }
    }

    pub fn toggle_leaders_tab(&mut self) {
        self.leaders_tab = match self.leaders_tab {
            LeadersTab::Skaters => LeadersTab::Goalies,
            LeadersTab::Goalies => LeadersTab::Skaters,
        };
        self.selected_leader_idx = 0;
    }
}
