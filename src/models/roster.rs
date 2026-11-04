use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub number: String,
    pub name: String,
    pub position: String,
    pub experience: String,
    pub birthday: String,
    pub shoots: String,
    pub height: String,
    pub weight: String,
    pub hometown: String,
    pub team: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Roster {
    pub team_name: String,
    pub players: Vec<Player>,
}
