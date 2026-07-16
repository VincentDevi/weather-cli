use thiserror::Error;

use crate::database::DatabaseError;
use crate::weather_api::OpenWeatherError;

#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    OpenWeather(#[from] OpenWeatherError),
    #[error("wrong argument")]
    Argument,
    #[error("favorite city `{0}` was not found")]
    FavoriteCityNotFound(String),
    #[error("no forecast is available for the requested day")]
    ForecastNotAvailable,
    #[error("the weather API returned no forecasts")]
    EmptyForecastResponse,
    #[error("Config error")]
    Config,
    #[error("error : `{0}`")]
    Dev(String),
    #[error("Database error: `{0}`")]
    Server(String),
    #[error(transparent)]
    Database(#[from] DatabaseError),
    #[error(transparent)]
    TaskJoin(#[from] tokio::task::JoinError),
}
