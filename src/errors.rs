use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Weather API error")]
    OpenWeather,
    #[error("wrong argument")]
    Argument,
    #[error("Config error")]
    Config,
    #[error("error : `{0}`")]
    Dev(String),
}
