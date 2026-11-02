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

pub const FPHL_TEAMS: &[(&str, &str)] = &[
    ("BRK", "Baton Rouge Kingfish"),
    ("BBB", "Binghamton Black Bears"),
    ("BRB", "Blue Ridge Bobcats"),
    ("TCT", "Twin City Thunderbirds"),
    ("CRD", "Columbus River Dragons"),
    ("DHT", "Danbury Hat Tricks"),
    ("FRF", "Fresno Falcons"),
    ("IND", "Indiana Sentinels"),
    ("MSM", "Mid-South Monarchs"),
    ("MNL", "Minnesota Northern Lights"),
    ("MMC", "Monroe Moccasins"),
    ("MCR", "Motor City Rockers"),
    ("OCS", "Oceanside Shock"),
    ("PHP", "Port Huron Prowlers"),
    ("SCK", "Stockton Thunder"),
    ("TOP", "Topeka Scarecrows"),
    ("WTW", "Watertown Wolves"),
    ("", "None (Clear favorite)"),
];

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
    pub show_team_picker: bool,
    pub team_picker_idx: usize,
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
            show_team_picker: false,
            team_picker_idx: 0,
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

    pub fn open_team_picker(&mut self) {
        self.show_team_picker = true;
        // Position on current favorite team if set
        if let Some(ref fav) = self.config.favorite_team {
            if let Some(idx) = FPHL_TEAMS.iter().position(|(code, _)| code == fav) {
                self.team_picker_idx = idx;
                return;
            }
        }
        self.team_picker_idx = 0;
    }

    pub fn select_next_team(&mut self) {
        if self.team_picker_idx + 1 < FPHL_TEAMS.len() {
            self.team_picker_idx += 1;
        }
    }

    pub fn select_prev_team(&mut self) {
        if self.team_picker_idx > 0 {
            self.team_picker_idx -= 1;
        }
    }

    pub fn confirm_team_selection(&mut self) {
        if let Some(&(code, _)) = FPHL_TEAMS.get(self.team_picker_idx) {
            if code.is_empty() {
                self.config.favorite_team = None;
            } else {
                self.config.favorite_team = Some(code.to_string());
            }
            let _ = self.config.save();
        }
        self.show_team_picker = false;
    }
}
