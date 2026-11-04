use anyhow::Result;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedReceiver;

use crate::models::roster::Roster;

pub struct TeamRosterSource {
    team_id: u64,
    tx: mpsc::Sender<Result<Roster>>,
}

impl TeamRosterSource {
    pub fn new(team_id: u64, tx: mpsc::Sender<Result<Roster>>) -> Self {
        Self { team_id, tx }
    }

    pub fn team_id(&self) -> u64 {
        self.team_id
    }

    pub async fn fetch(self) -> Result<()> {
        use std::time::Duration;
        use tokio::time::timeout;

        let mut client = HockeyShiftClient::new();
        let res = timeout(
            Duration::from_secs(30),
            client.fetch_roster(self.team_id),
        )
        .await
        .map_err(|_| anyhow::anyhow!("Roster fetch timed out"))??;

        let _ = self.tx.send(res).await;
        Ok(())
    }
}
