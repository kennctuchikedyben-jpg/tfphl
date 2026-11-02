use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkaterLeader {
    pub rank: u32,
    pub name: String,
    pub jersey: String,
    pub team: String,
    pub pos: String,
    pub gp: u32,
    pub goals: u32,
    pub assists: u32,
    pub points: u32,
    pub ppga: String,
    pub pim: u32,
    pub gwg: u32,
    pub plus_minus: String,
    pub sog: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalieLeader {
    pub rank: u32,
    pub name: String,
    pub team: String,
    pub gp: u32,
    pub w: u32,
    pub l: u32,
    pub otl: u32,
    pub gaa: String,
    pub sv_pct: String,
    pub so: u32,
}
