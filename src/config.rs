use std::{collections::HashMap, fs, sync::OnceLock};

use serde::{Deserialize, Serialize};

use crate::{AppResult, fs::config_path};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    profile_map: HashMap<String, String>,
}

impl Config {
    pub fn load() -> AppResult<Self> {
        let path = config_path()?;
        let content = fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }

    pub fn profile_name(&self, command: impl AsRef<str>) -> Option<&str> {
        self.profile_map.get(command.as_ref()).map(String::as_str)
    }
}

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();
    INSTANCE.get_or_init(|| Config::load().unwrap_or_default())
}
