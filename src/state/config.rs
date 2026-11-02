use std::fs;
use std::path::PathBuf;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub favorite_team: Option<String>,
    pub timezone: String,
    pub log_level: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            favorite_team: Some("BBB".to_string()),
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
                    if let Ok(cfg) = toml::from_str::<Config>(&content) {
                        return cfg;
                    }
                }
            } else {
                // Try writing default config
                if let Some(parent) = path.parent() {
                    let _ = fs::create_dir_all(parent);
                }
                let def = Config::default();
                if let Ok(serialized) = toml::to_string_pretty(&def) {
                    let _ = fs::write(&path, serialized);
                }
                return def;
            }
        }
        Config::default()
    }
}
