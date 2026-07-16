use thiserror::Error;

#[derive(Debug, Error)]
pub enum OpenWeatherError {
    #[error("OpenWeather request failed: {0}")]
    Request(#[source] reqwest::Error),
    #[error("OpenWeather returned an invalid response: {0}")]
    ResponseDeserialization(#[source] reqwest::Error),
    #[error("OpenWeather returned an invalid forecast timestamp: {0}")]
    InvalidForecastTimestamp(i64),
    #[error("OpenWeather returned an invalid timezone offset: {0}")]
    InvalidTimezoneOffset(i32),
    #[error("OpenWeather returned no geocoding result for `{0}`")]
    LocationNotFound(String),
}
