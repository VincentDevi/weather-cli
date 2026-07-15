use crate::entity::{City, Coordinate, Forecast};
use crate::errors::AppError;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ForecastResponseDto {
    pub list: Vec<ForecastPointDto>,
    pub city: ForecastCityDto,
}

#[derive(Debug, Deserialize)]
pub struct ForecastPointDto {
    pub dt: i64,
    pub main: MainDto,
    pub pop: f64,
}

#[derive(Debug, Deserialize)]
pub struct ForecastCityDto {
    pub timezone: i32,
}

#[derive(Debug, Deserialize)]
pub struct MainDto {
    pub temp: f64,
    pub humidity: i64,
}

impl TryFrom<ForecastPointDto> for Forecast {
    type Error = AppError;

    fn try_from(value: ForecastPointDto) -> Result<Self, Self::Error> {
        let forecast_at =
            DateTime::<Utc>::from_timestamp(value.dt, 0).ok_or(AppError::OpenWeather)?;

        Ok(Forecast::new(
            forecast_at,
            value.main.temp,
            value.main.humidity,
            value.pop,
        ))
    }
}

#[derive(Debug, Deserialize)]
pub struct CityDto {
    name: String,
    lat: f64,
    lon: f64,
}

impl From<CityDto> for City {
    fn from(value: CityDto) -> Self {
        Self::new(value.name, Coordinate::new(value.lat, value.lon))
    }
}
