use sb_core::core::config::Config;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs::File;
use std::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preference {
    pub config: Config,
}

impl Preference {
    pub fn new(config: Config) -> Self {
        Preference { config }
    }

    pub fn update(&mut self, config: Config) {
        self.config = config;
    }

    pub fn load() -> Result<Self, Box<dyn Error>> {
        let file_path = PathBuf::from("preference.json");
        let file = File::open(file_path)?;

        serde_json::from_reader(file)?
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let file_path = PathBuf::from("preference.json");
        let file = File::create(file_path)?;
        serde_json::to_writer(file, self)?;
        Ok(())
    }
}
