use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandingRow {
    pub rank: u32,
    pub team_name: String,
    pub team_short: String,
    pub gp: u32,
    pub w: u32,
    pub l: u32,
    pub otl: u32,
    pub pts: u32,
    pub win_pct: String,
    pub gf: u32,
    pub ga: u32,
    pub diff: String,
    pub pim: u32,
    pub l10: String,
    pub streak: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Division {
    pub name: String,
    pub rows: Vec<StandingRow>,
}
