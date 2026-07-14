use crate::entity::Forecast;
use crate::errors::AppError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ForecastResponseDto {
    pub list: Vec<ForecastPointDto>,
}

#[derive(Debug, Deserialize)]
pub struct ForecastPointDto {
    pub main: MainDto,
    pub pop: f64,
}

#[derive(Debug, Deserialize)]
pub struct MainDto {
    pub temp: f64,
    pub humidity: i64,
}

impl TryFrom<ForecastPointDto> for Forecast {
    type Error = AppError;

    fn try_from(value: ForecastPointDto) -> Result<Self, Self::Error> {
        Ok(Forecast::new(
            value.main.temp,
            value.main.humidity,
            value.pop,
        ))
    }
}
