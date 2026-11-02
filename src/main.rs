mod api;
mod input;
mod models;
mod state;
mod ui;

use std::{io, time::Duration};
use anyhow::Result;
use crossterm::{
    event::{Event, EventStream},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use futures::StreamExt;
use ratatui::{Terminal, backend::CrosstermBackend};
use tokio::sync::mpsc;

use crate::{
    api::client::HockeyShiftClient,
    input::{Action, handle_key},
    models::{
        boxscore::GameBoxscore,
        games::Game,
        leaders::{GoalieLeader, SkaterLeader},
        standings::Division,
    },
    state::{app_state::AppState, config::Config},
    ui::layout::render_app,
};

enum DataEvent {
    Scores(Result<Vec<Game>>),
    Standings(Result<Vec<Division>>),
    Leaders(Result<(Vec<SkaterLeader>, Vec<GoalieLeader>)>),
    Boxscore(Result<GameBoxscore>),
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "--version" | "-V" => {
                println!("tfphl 0.1.0");
                return Ok(());
            }
            "--help" | "-h" => {
                println!("tfphl - Federal Prospects Hockey League Terminal Dashboard");
                println!();
                println!("Usage: tfphl [OPTIONS]");
                println!();
                println!("Options:");
                println!("  -h, --help     Print help");
                println!("  -V, --version  Print version");
                return Ok(());
            }
            _ => {}
        }
    }

    // Set panic hook to restore terminal on crash
    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        default_panic(info);
    }));

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Initialize state
    let config = Config::load();
    let mut state = AppState::new(config);

    // Channels for asynchronous data fetching
    let (tx, mut rx) = mpsc::channel::<DataEvent>(32);

    // Initial data fetch
    trigger_full_refresh(&tx);

    let mut event_stream = EventStream::new();
    let mut ticker = tokio::time::interval(Duration::from_millis(250));

    loop {
        terminal.draw(|f| render_app(f, &state))?;

        tokio::select! {
            _ = ticker.tick() => {
                // Regular UI redraw cycle
            }

            Some(event) = rx.recv() => {
                match event {
                    DataEvent::Scores(res) => {
                        match res {
                            Ok(games) => {
                                state.set_games(games);
                                state.loading = false;
                                state.error_message = None;
                            }
                            Err(e) => {
                                state.loading = false;
                                state.error_message = Some(format!("Failed to load games: {}", e));
                            }
                        }
                    }
                    DataEvent::Standings(res) => {
                        match res {
                            Ok(divs) => {
                                state.divisions = divs;
                                state.loading = false;
                            }
                            Err(e) => {
                                state.loading = false;
                                state.error_message = Some(format!("Failed to load standings: {}", e));
                            }
                        }
                    }
                    DataEvent::Leaders(res) => {
                        match res {
                            Ok((skaters, goalies)) => {
                                state.skaters = skaters;
                                state.goalies = goalies;
                                state.loading = false;
                            }
                            Err(e) => {
                                state.loading = false;
                                state.error_message = Some(format!("Failed to load leaders: {}", e));
                            }
                        }
                    }
                    DataEvent::Boxscore(res) => {
                        state.boxscore_loading = false;
                        match res {
                            Ok(boxscore) => {
                                state.boxscore_data = Some(boxscore);
                            }
                            Err(e) => {
                                state.error_message = Some(format!("Failed to load boxscore: {}", e));
                            }
                        }
                    }
                }
            }

            Some(Ok(event)) = event_stream.next() => {
                if let Event::Key(key) = event {
                    let action = handle_key(key, &mut state);
                    match action {
                        Action::Quit => break,
                        Action::RefreshAll => {
                            state.loading = true;
                            state.error_message = None;
                            trigger_full_refresh(&tx);
                        }
                        Action::FetchBoxscore(game_id) => {
                            state.boxscore_loading = true;
                            state.boxscore_data = None;
                            let tx_clone = tx.clone();
                            tokio::spawn(async move {
                                let mut client = HockeyShiftClient::new();
                                let res = client.fetch_boxscore(game_id).await;
                                let _ = tx_clone.send(DataEvent::Boxscore(res)).await;
                            });
                        }
                        Action::None => {}
                    }
                }
            }
        }
    }

    // Cleanup terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn trigger_full_refresh(tx: &mpsc::Sender<DataEvent>) {
    // 1. Fetch scores
    let tx_scores = tx.clone();
    tokio::spawn(async move {
        let mut client = HockeyShiftClient::new();
        let res = client.fetch_scores().await;
        let _ = tx_scores.send(DataEvent::Scores(res)).await;
    });

    // 2. Fetch standings
    let tx_standings = tx.clone();
    tokio::spawn(async move {
        let mut client = HockeyShiftClient::new();
        let res = client.fetch_standings().await;
        let _ = tx_standings.send(DataEvent::Standings(res)).await;
    });

    // 3. Fetch leaders
    let tx_leaders = tx.clone();
    tokio::spawn(async move {
        let mut client = HockeyShiftClient::new();
        let skaters_res = client.fetch_skater_leaders().await;
        let goalies_res = client.fetch_goalie_leaders().await;

        let combined = match (skaters_res, goalies_res) {
            (Ok(s), Ok(g)) => Ok((s, g)),
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
        };
        let _ = tx_leaders.send(DataEvent::Leaders(combined)).await;
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use api::client::HockeyShiftClient;

    #[tokio::test]
    async fn test_auth_login() {
        let mut client = HockeyShiftClient::new();
        let ticket = client.ensure_ticket().await;
        assert!(ticket.is_ok(), "Failed to get ticket: {:?}", ticket.err());
        let hash = ticket.unwrap();
        assert!(!hash.is_empty(), "Ticket hash was empty");
    }

    #[tokio::test]
    async fn test_fetch_scores() {
        let mut client = HockeyShiftClient::new();
        let scores = client.fetch_scores().await;
        assert!(scores.is_ok(), "Failed to fetch scores: {:?}", scores.err());
        let games = scores.unwrap();
        assert!(!games.is_empty(), "Games list was empty");
    }

    #[tokio::test]
    async fn test_fetch_standings() {
        let mut client = HockeyShiftClient::new();
        let res = client.fetch_standings().await;
        assert!(res.is_ok(), "Failed to fetch standings: {:?}", res.err());
        let divisions = res.unwrap();
        assert!(!divisions.is_empty(), "Divisions list was empty");
    }

    #[tokio::test]
    async fn test_fetch_leaders() {
        let mut client = HockeyShiftClient::new();
        let skaters = client.fetch_skater_leaders().await;
        assert!(skaters.is_ok(), "Failed to fetch skaters: {:?}", skaters.err());
        let list = skaters.unwrap();
        assert!(!list.is_empty(), "Skaters list was empty");
    }
}
