use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeriodScore {
    pub period: String,
    pub away: String,
    pub home: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayEvent {
    pub period: String,
    pub time: String,
    pub event_type: String,
    pub team: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameBoxscore {
    pub away_team: String,
    pub home_team: String,
    pub away_score: i32,
    pub home_score: i32,
    pub venue: String,
    pub date_time_str: String,
    pub periods: Vec<PeriodScore>,
    pub shots: Vec<PeriodScore>,
    pub power_plays: (String, String), // (Away, Home)
    pub penalty_minutes: (String, String),
    pub plays: Vec<PlayEvent>,
}
