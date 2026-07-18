use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::errors::AppError;

#[derive(Debug, Clone)]
pub struct Config {
    open_weather_key: Arc<str>,
    database_path: PathBuf,
    openai_key: Arc<str>,
}

impl Config {
    pub fn load() -> Result<Self, AppError> {
        let _ = dotenvy::dotenv().map_err(|_| AppError::Config)?;
        let api_key = std::env::var("OPEN_WEATHER_KEY").map_err(|_| AppError::Config)?;
        let openai_key = std::env::var("OPENAI_API_KEY").map_err(|_| AppError::Config)?;
        let database_path = std::env::var_os("WEATHER_DB_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("./data/weather.db"));

        Ok(Self {
            open_weather_key: Arc::from(api_key),
            database_path,
            openai_key: Arc::from(openai_key),
        })
    }

    pub fn open_weather_key(&self) -> Arc<str> {
        Arc::clone(&self.open_weather_key)
    }

    pub fn database_path(&self) -> &Path {
        &self.database_path
    }
    pub fn openai_key(&self) -> Arc<str> {
        Arc::clone(&self.openai_key)
    }
}
