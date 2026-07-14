use std::sync::Arc;

use crate::errors::AppError;

#[derive(Debug, Clone)]
pub struct Config {
    open_weather_key: Arc<str>,
}

impl Config {
    pub fn load() -> Result<Self, AppError> {
        let _ = dotenvy::dotenv().map_err(|_| AppError::Config)?;
        let api_key = std::env::var("OPEN_WEATHER_KEY").map_err(|_| AppError::Config)?;
        Ok(Self {
            open_weather_key: Arc::from(api_key.as_str()),
        })
    }
    pub fn open_weather_key(&self) -> Arc<str> {
        self.open_weather_key.clone()
    }
}
