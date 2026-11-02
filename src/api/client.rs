use anyhow::{Context, Result, anyhow};
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use scraper::{Html, Selector};
use serde::Deserialize;

use crate::models::{
    boxscore::{GameBoxscore, PeriodScore, PlayEvent},
    games::Game,
    leaders::{GoalieLeader, SkaterLeader},
    standings::{Division, StandingRow},
};

const BASE_URL: &str = "https://web.api.digitalshift.ca";
const CLIENT_SERVICE_ID: &str = "603c74d0-0ed2-4d3f-8171-b447f9398da4";
const LEAGUE_ID: u32 = 182;
const SEASON_ID: u32 = 10958;

#[derive(Deserialize)]
struct LoginResponse {
    ticket: TicketObj,
}

#[derive(Deserialize)]
struct TicketObj {
    hash: String,
}

#[derive(Deserialize)]
struct PartialResponse {
    content: Option<String>,
}

#[derive(Clone)]
pub struct HockeyShiftClient {
    client: reqwest::Client,
    ticket: Option<String>,
}

impl HockeyShiftClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .user_agent("tfphl/0.1.0")
                .build()
                .unwrap_or_default(),
            ticket: None,
        }
    }

    pub async fn ensure_ticket(&mut self) -> Result<String> {
        if let Some(ref t) = self.ticket {
            return Ok(t.clone());
        }

        let login_url = format!("{}/login", BASE_URL);
        let resp = self
            .client
            .post(&login_url)
            .json(&serde_json::json!({
                "client_service_id": CLIENT_SERVICE_ID
            }))
            .send()
            .await
            .context("Failed to send login request to HockeyShift")?;

        let data = resp
            .json::<LoginResponse>()
            .await
            .context("Failed to parse login response")?;

        let ticket = data.ticket.hash;
        self.ticket = Some(ticket.clone());
        Ok(ticket)
    }

    fn auth_headers(&self, ticket: &str) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();
        let auth_val = format!("ticket=\"{}\"", ticket);
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&auth_val)?);
        Ok(headers)
    }

    pub async fn fetch_scores(&mut self) -> Result<Vec<Game>> {
        let ticket = self.ensure_ticket().await?;
        let url = format!(
            "{}/partials/stats/scores/table?season_id={}&league_id={}",
            BASE_URL, SEASON_ID, LEAGUE_ID
        );

        let resp = self
            .client
            .get(&url)
            .headers(self.auth_headers(&ticket)?)
            .send()
            .await
            .context("Failed to fetch scores partial")?;

        let partial: PartialResponse = resp.json().await?;
        let html_content = partial
            .content
            .ok_or_else(|| anyhow!("Empty scores content"))?;

        Self::parse_scores_json(&html_content)
    }

    pub fn parse_scores_json(html: &str) -> Result<Vec<Game>> {
        let marker = "ctrl.scores=";
        let start_pos = html
            .find(marker)
            .ok_or_else(|| anyhow!("Scores marker not found in HTML"))?
            + marker.len();

        let slice = &html[start_pos..];
        // The JSON array starts with [ and ends with ]
        let json_start = slice
            .find('[')
            .ok_or_else(|| anyhow!("Opening bracket not found"))?;

        // Find corresponding closing bracket
        let mut depth = 0;
        let mut json_end = None;
        for (i, c) in slice[json_start..].char_indices() {
            if c == '[' {
                depth += 1;
            } else if c == ']' {
                depth -= 1;
                if depth == 0 {
                    json_end = Some(json_start + i + 1);
                    break;
                }
            }
        }

        let raw_json = match json_end {
            Some(end) => &slice[json_start..end],
            None => return Err(anyhow!("Could not find closing bracket for scores JSON")),
        };

        let decoded = raw_json
            .replace("&quot;", "\"")
            .replace("&amp;", "&")
            .replace("&lt;", "<")
            .replace("&gt;", ">");

        let games: Vec<Game> = serde_json::from_str(&decoded)
            .context("Failed to deserialize games from embedded JSON")?;

        Ok(games)
    }

    pub async fn fetch_standings(&mut self) -> Result<Vec<Division>> {
        let ticket = self.ensure_ticket().await?;
        let url = format!(
            "{}/partials/stats/standings/table?season_id={}&league_id={}",
            BASE_URL, SEASON_ID, LEAGUE_ID
        );

        let resp = self
            .client
            .get(&url)
            .headers(self.auth_headers(&ticket)?)
            .send()
            .await
            .context("Failed to fetch standings partial")?;

        let partial: PartialResponse = resp.json().await?;
        let html_content = partial
            .content
            .ok_or_else(|| anyhow!("Empty standings content"))?;

        Self::parse_standings_html(&html_content)
    }

    pub fn parse_standings_html(html: &str) -> Result<Vec<Division>> {
        let document = Html::parse_document(html);
        let table_selector = Selector::parse("table.standings").unwrap();
        let tr_selector = Selector::parse("tr").unwrap();
        let td_selector = Selector::parse("td").unwrap();
        let a_selector = Selector::parse("a.team-inline").unwrap();

        let mut divisions = Vec::new();
        let mut current_division_name = "League Standings".to_string();
        let mut current_rows = Vec::new();

        for table in document.select(&table_selector) {
            for tr in table.select(&tr_selector) {
                // Check if this is a division header row (class "secondary")
                if tr.value().classes().any(|c| c == "secondary") {
                    if !current_rows.is_empty() {
                        divisions.push(Division {
                            name: current_division_name.clone(),
                            rows: std::mem::take(&mut current_rows),
                        });
                    }
                    let div_text = tr.text().collect::<String>().trim().to_string();
                    if !div_text.is_empty() {
                        current_division_name = div_text;
                    }
                    continue;
                }

                let tds: Vec<_> = tr.select(&td_selector).collect();
                if tds.len() >= 13 {
                    let rank: u32 = tds[0]
                        .text()
                        .collect::<String>()
                        .trim()
                        .parse()
                        .unwrap_or(0);
                    if rank == 0 {
                        continue;
                    }

                    let team_cell = &tds[1];
                    let mut team_name = String::new();
                    let mut team_short = String::new();

                    for a in team_cell.select(&a_selector) {
                        let text = a.text().collect::<String>().trim().to_string();
                        if team_name.is_empty() {
                            team_name = text;
                        } else if team_short.is_empty() {
                            team_short = text;
                        }
                    }

                    if team_short.is_empty() {
                        team_short = team_name.chars().take(3).collect::<String>().to_uppercase();
                    }

                    let gp: u32 = tds[2].text().collect::<String>().trim().parse().unwrap_or(0);
                    let w: u32 = tds[3].text().collect::<String>().trim().parse().unwrap_or(0);
                    let l: u32 = tds[4].text().collect::<String>().trim().parse().unwrap_or(0);
                    let otl: u32 = tds[5].text().collect::<String>().trim().parse().unwrap_or(0);
                    let pts: u32 = tds[6].text().collect::<String>().trim().parse().unwrap_or(0);
                    let win_pct = tds[7].text().collect::<String>().trim().to_string();
                    let gf: u32 = tds[8].text().collect::<String>().trim().parse().unwrap_or(0);
                    let ga: u32 = tds[9].text().collect::<String>().trim().parse().unwrap_or(0);
                    let diff = tds[10].text().collect::<String>().trim().to_string();
                    let pim: u32 = tds[11].text().collect::<String>().trim().parse().unwrap_or(0);
                    let l10 = tds[12]
                        .text()
                        .collect::<String>()
                        .replace("&hyphen;", "-")
                        .replace("−", "-")
                        .replace("–", "-")
                        .trim()
                        .to_string();
                    let streak = if tds.len() > 13 {
                        tds[13].text().collect::<String>().trim().to_string()
                    } else {
                        String::new()
                    };

                    current_rows.push(StandingRow {
                        rank,
                        team_name,
                        team_short,
                        gp,
                        w,
                        l,
                        otl,
                        pts,
                        win_pct,
                        gf,
                        ga,
                        diff,
                        pim,
                        l10,
                        streak,
                    });
                }
            }
        }

        if !current_rows.is_empty() {
            divisions.push(Division {
                name: current_division_name,
                rows: current_rows,
            });
        }

        Ok(divisions)
    }

    pub async fn fetch_skater_leaders(&mut self) -> Result<Vec<SkaterLeader>> {
        let ticket = self.ensure_ticket().await?;
        let url = format!(
            "{}/partials/stats/leaders/table?season_id={}&league_id={}&player_type=players&game_type=Regular%20Season",
            BASE_URL, SEASON_ID, LEAGUE_ID
        );

        let resp = self
            .client
            .get(&url)
            .headers(self.auth_headers(&ticket)?)
            .send()
            .await
            .context("Failed to fetch skater leaders partial")?;

        let partial: PartialResponse = resp.json().await?;
        let html_content = partial
            .content
            .ok_or_else(|| anyhow!("Empty skater leaders content"))?;

        Self::parse_skater_leaders_html(&html_content)
    }

    pub fn parse_skater_leaders_html(html: &str) -> Result<Vec<SkaterLeader>> {
        let document = Html::parse_document(html);
        let tr_selector = Selector::parse("table.leaders_players tbody tr").unwrap();
        let td_selector = Selector::parse("td").unwrap();
        let a_person_selector = Selector::parse("a.person-inline").unwrap();
        let a_team_selector = Selector::parse("a.team-inline").unwrap();

        let mut leaders = Vec::new();

        for tr in document.select(&tr_selector) {
            let tds: Vec<_> = tr.select(&td_selector).collect();
            if tds.len() >= 16 {
                let rank: u32 = tds[0].text().collect::<String>().trim().parse().unwrap_or(0);
                if rank == 0 {
                    continue;
                }

                let name = tds[1]
                    .select(&a_person_selector)
                    .next()
                    .map(|a| a.text().collect::<String>().trim().to_string())
                    .unwrap_or_else(|| tds[1].text().collect::<String>().trim().to_string());

                let jersey = tds[2].text().collect::<String>().trim().to_string();

                let team = tds[3]
                    .select(&a_team_selector)
                    .next()
                    .map(|a| a.text().collect::<String>().trim().to_string())
                    .unwrap_or_else(|| tds[3].text().collect::<String>().trim().to_string());

                let pos = tds[4].text().collect::<String>().trim().to_string();
                let gp: u32 = tds[5].text().collect::<String>().trim().parse().unwrap_or(0);
                let goals: u32 = tds[6].text().collect::<String>().trim().parse().unwrap_or(0);
                let assists: u32 = tds[7].text().collect::<String>().trim().parse().unwrap_or(0);
                let points: u32 = tds[8].text().collect::<String>().trim().parse().unwrap_or(0);
                let ppga = tds[9].text().collect::<String>().trim().to_string();
                let pim: u32 = tds[10].text().collect::<String>().trim().parse().unwrap_or(0);
                let gwg: u32 = tds.get(14).and_then(|t| t.text().collect::<String>().trim().parse().ok()).unwrap_or(0);
                let plus_minus = tds.get(15).map(|t| t.text().collect::<String>().trim().to_string()).unwrap_or_default();
                let sog: u32 = tds.get(16).and_then(|t| t.text().collect::<String>().trim().parse().ok()).unwrap_or(0);

                leaders.push(SkaterLeader {
                    rank,
                    name,
                    jersey,
                    team,
                    pos,
                    gp,
                    goals,
                    assists,
                    points,
                    ppga,
                    pim,
                    gwg,
                    plus_minus,
                    sog,
                });
            }
        }

        Ok(leaders)
    }

    pub async fn fetch_goalie_leaders(&mut self) -> Result<Vec<GoalieLeader>> {
        let ticket = self.ensure_ticket().await?;
        let url = format!(
            "{}/partials/stats/leaders/table?season_id={}&league_id={}&player_type=goalies&game_type=Regular%20Season",
            BASE_URL, SEASON_ID, LEAGUE_ID
        );

        let resp = self
            .client
            .get(&url)
            .headers(self.auth_headers(&ticket)?)
            .send()
            .await
            .context("Failed to fetch goalie leaders partial")?;

        let partial: PartialResponse = resp.json().await?;
        let html_content = partial
            .content
            .ok_or_else(|| anyhow!("Empty goalie leaders content"))?;

        Self::parse_goalie_leaders_html(&html_content)
    }

    pub fn parse_goalie_leaders_html(html: &str) -> Result<Vec<GoalieLeader>> {
        let document = Html::parse_document(html);
        let tr_selector = Selector::parse("table.leaders_goalies tbody tr, table.stats-table tbody tr").unwrap();
        let td_selector = Selector::parse("td").unwrap();
        let a_person_selector = Selector::parse("a.person-inline").unwrap();
        let a_team_selector = Selector::parse("a.team-inline").unwrap();

        let mut leaders = Vec::new();

        for tr in document.select(&tr_selector) {
            let tds: Vec<_> = tr.select(&td_selector).collect();
            if tds.len() >= 15 {
                let rank: u32 = tds[0].text().collect::<String>().trim().parse().unwrap_or(0);
                if rank == 0 {
                    continue;
                }

                let name = tds[1]
                    .select(&a_person_selector)
                    .next()
                    .map(|a| a.text().collect::<String>().trim().to_string())
                    .unwrap_or_else(|| tds[1].text().collect::<String>().trim().to_string());

                let jersey = tds[2].text().collect::<String>().trim().to_string();

                let team = tds[3]
                    .select(&a_team_selector)
                    .next()
                    .map(|a| a.text().collect::<String>().trim().to_string())
                    .unwrap_or_else(|| tds[3].text().collect::<String>().trim().to_string());

                let gp: u32 = tds[4].text().collect::<String>().trim().parse().unwrap_or(0);
                let w: u32 = tds[5].text().collect::<String>().trim().parse().unwrap_or(0);
                let l: u32 = tds[6].text().collect::<String>().trim().parse().unwrap_or(0);
                let otl: u32 = tds[8].text().collect::<String>().trim().parse().unwrap_or(0);
                let gaa = tds[11].text().collect::<String>().trim().to_string();
                let sv_pct = tds[13].text().collect::<String>().trim().to_string();
                let so: u32 = tds.get(14).and_then(|td| td.text().collect::<String>().trim().parse().ok()).unwrap_or(0);

                leaders.push(GoalieLeader {
                    rank,
                    name,
                    jersey,
                    team,
                    gp,
                    w,
                    l,
                    otl,
                    gaa,
                    sv_pct,
                    so,
                });
            }
        }

        Ok(leaders)
    }

    pub async fn fetch_boxscore(&mut self, game_id: u64) -> Result<GameBoxscore> {
        let ticket = self.ensure_ticket().await?;
        let play_url = format!(
            "{}/partials/stats/game/play-by-play?game_id={}&league_id={}",
            BASE_URL, game_id, LEAGUE_ID
        );

        let resp = self
            .client
            .get(&play_url)
            .headers(self.auth_headers(&ticket)?)
            .send()
            .await
            .context("Failed to fetch game play-by-play partial")?;

        let partial: PartialResponse = resp.json().await?;
        let html_content = partial
            .content
            .ok_or_else(|| anyhow!("Empty play-by-play content"))?;

        Self::parse_boxscore_html(&html_content)
    }

    pub fn parse_boxscore_html(html: &str) -> Result<GameBoxscore> {
        let document = Html::parse_document(html);

        let table_selector = Selector::parse(".sidebar table").unwrap();
        let tr_selector = Selector::parse("tr").unwrap();
        let td_selector = Selector::parse("td").unwrap();

        let mut away_team = "Away".to_string();
        let mut home_team = "Home".to_string();
        let mut periods = Vec::new();
        let mut shots = Vec::new();
        let mut away_score = 0;
        let mut home_score = 0;
        let mut power_plays = ("-".to_string(), "-".to_string());
        let mut penalty_minutes = ("-".to_string(), "-".to_string());

        let mut tables = document.select(&table_selector);
        // Table 1: Scoring breakdown
        if let Some(scoring_table) = tables.next() {
            let rows: Vec<_> = scoring_table.select(&tr_selector).collect();
            if rows.len() >= 3 {
                let row_away_tds: Vec<_> = rows[1].select(&td_selector).collect();
                let row_home_tds: Vec<_> = rows[2].select(&td_selector).collect();

                if row_away_tds.len() >= 5 && row_home_tds.len() >= 5 {
                    away_team = row_away_tds[0].text().collect::<String>().trim().to_string();
                    home_team = row_home_tds[0].text().collect::<String>().trim().to_string();

                    periods.push(PeriodScore {
                        period: "1st".to_string(),
                        away: row_away_tds[1].text().collect::<String>().trim().to_string(),
                        home: row_home_tds[1].text().collect::<String>().trim().to_string(),
                    });
                    periods.push(PeriodScore {
                        period: "2nd".to_string(),
                        away: row_away_tds[2].text().collect::<String>().trim().to_string(),
                        home: row_home_tds[2].text().collect::<String>().trim().to_string(),
                    });
                    periods.push(PeriodScore {
                        period: "3rd".to_string(),
                        away: row_away_tds[3].text().collect::<String>().trim().to_string(),
                        home: row_home_tds[3].text().collect::<String>().trim().to_string(),
                    });

                    away_score = row_away_tds[4].text().collect::<String>().trim().parse().unwrap_or(0);
                    home_score = row_home_tds[4].text().collect::<String>().trim().parse().unwrap_or(0);
                }
            }
        }

        // Table 2: Shots breakdown
        if let Some(shots_table) = tables.next() {
            let rows: Vec<_> = shots_table.select(&tr_selector).collect();
            if rows.len() >= 3 {
                let row_away_tds: Vec<_> = rows[1].select(&td_selector).collect();
                let row_home_tds: Vec<_> = rows[2].select(&td_selector).collect();

                if row_away_tds.len() >= 5 && row_home_tds.len() >= 5 {
                    shots.push(PeriodScore {
                        period: "1st".to_string(),
                        away: row_away_tds[1].text().collect::<String>().trim().to_string(),
                        home: row_home_tds[1].text().collect::<String>().trim().to_string(),
                    });
                    shots.push(PeriodScore {
                        period: "2nd".to_string(),
                        away: row_away_tds[2].text().collect::<String>().trim().to_string(),
                        home: row_home_tds[2].text().collect::<String>().trim().to_string(),
                    });
                    shots.push(PeriodScore {
                        period: "3rd".to_string(),
                        away: row_away_tds[3].text().collect::<String>().trim().to_string(),
                        home: row_home_tds[3].text().collect::<String>().trim().to_string(),
                    });
                }
            }
        }

        // Table 3: Team Stats (Power play, PIM)
        if let Some(stats_table) = tables.next() {
            for row in stats_table.select(&tr_selector) {
                let tds: Vec<_> = row.select(&td_selector).collect();
                if tds.len() >= 3 {
                    let stat_name = tds[0].text().collect::<String>().trim().to_string();
                    let val_away = tds[1].text().collect::<String>().trim().to_string();
                    let val_home = tds[2].text().collect::<String>().trim().to_string();

                    if stat_name.contains("Power Plays") {
                        power_plays = (val_away, val_home);
                    } else if stat_name.contains("Penalty Minutes") {
                        penalty_minutes = (val_away, val_home);
                    }
                }
            }
        }

        // Play by play rows
        let play_tr_selector = Selector::parse("table.play-by-play tbody tr").unwrap();
        let mut plays = Vec::new();

        for tr in document.select(&play_tr_selector) {
            let tds: Vec<_> = tr.select(&td_selector).collect();
            if tds.len() >= 4 {
                let event_type = tds[0].text().collect::<String>().trim().to_string();
                let team = tds[1].text().collect::<String>().trim().to_string();
                let time = tds[2].text().collect::<String>().trim().to_string();
                let desc = tds[3].text().collect::<String>().trim().to_string();

                if !event_type.is_empty() && !desc.is_empty() {
                    plays.push(PlayEvent {
                        period: "Reg".to_string(),
                        time,
                        event_type,
                        team,
                        description: desc,
                    });
                }
            }
        }

        Ok(GameBoxscore {
            away_team,
            home_team,
            away_score,
            home_score,
            venue: String::new(),
            date_time_str: String::new(),
            periods,
            shots,
            power_plays,
            penalty_minutes,
            plays,
        })
    }
}
