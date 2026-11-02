use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub game_id: u64,
    #[serde(default)]
    pub date: String,
    #[serde(default)]
    pub time: String,
    #[serde(default)]
    pub datetime: String,
    #[serde(default)]
    pub time_zone_abbr: String,
    #[serde(default)]
    pub number: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub home_team: String,
    #[serde(default)]
    pub home_team_short: String,
    pub home_score: Option<i32>,
    pub home_shots: Option<i32>,
    #[serde(default)]
    pub away_team: String,
    #[serde(default)]
    pub away_team_short: String,
    pub away_score: Option<i32>,
    pub away_shots: Option<i32>,
    pub facility: Option<String>,
    pub rink: Option<String>,
    #[serde(default)]
    pub overtime: bool,
    #[serde(default)]
    pub shootout: bool,
}

impl Game {
    pub fn display_status(&self) -> String {
        if self.status.is_empty() {
            return "Scheduled".to_string();
        }
        if self.status == "Final" {
            if self.shootout {
                "Final/SO".to_string()
            } else if self.overtime {
                "Final/OT".to_string()
            } else {
                "Final".to_string()
            }
        } else {
            self.status.clone()
        }
    }
}
