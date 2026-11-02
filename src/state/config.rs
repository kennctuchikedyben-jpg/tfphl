use std::fs;
use std::path::PathBuf;
use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Config {
    pub favorite_team: Option<String>,
    pub timezone: String,
    pub log_level: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            favorite_team: Some("BKF".to_string()),
            timezone: "US/Eastern".to_string(),
            log_level: "error".to_string(),
        }
    }
}

impl Config {
    pub fn config_path() -> Option<PathBuf> {
        ProjectDirs::from("com", "tfphl", "tfphl").map(|proj| {
            proj.config_dir().join("tfphl.toml")
        })
    }

    pub fn load() -> Self {
        if let Some(path) = Self::config_path() {
            if path.exists() {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(mut cfg) = toml::from_str::<Config>(&content) {
                        // Migrate legacy BRK code to BKF
                        if cfg.favorite_team.as_deref() == Some("BRK") {
                            cfg.favorite_team = Some("BKF".to_string());
                            let _ = cfg.save();
                        }
                        return cfg;
                    }
                }
            } else {
                let def = Config::default();
                let _ = def.save();
                return def;
            }
        }
        Config::default()
    }

    pub fn save(&self) -> Result<()> {
        if let Some(path) = Self::config_path() {
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let serialized = toml::to_string_pretty(self)
                .context("Failed to serialize config to TOML")?;
            fs::write(&path, serialized)
                .context("Failed to write config file")?;
        }
        Ok(())
    }
}
