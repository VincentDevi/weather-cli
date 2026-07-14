use crate::entity::Forecast;
use crate::errors::AppError;
use chrono::DateTime;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ForecastResponseDto {
    pub list: Vec<ForecastPointDto>,
    pub city: CityDto,
}

#[derive(Debug, Deserialize)]
pub struct CityDto {
    pub name: String,
    pub country: String,
    pub coord: CoordinatesDto,
    pub timezone: i64,
}

#[derive(Debug, Deserialize)]
pub struct CoordinatesDto {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Deserialize)]
pub struct ForecastPointDto {
    pub dt: i64,
    pub main: MainDto,
    pub weather: Vec<WeatherConditionDto>,
    pub wind: WindDto,
    pub pop: f64,
}

#[derive(Debug, Deserialize)]
pub struct MainDto {
    pub temp: f64,
    pub feels_like: f64,
    pub humidity: i64,
}

#[derive(Debug, Deserialize)]
pub struct WindDto {
    pub speed: f64,
}

#[derive(Debug, Deserialize)]
pub struct WeatherConditionDto {
    pub id: u16,
    pub main: String,
    pub description: String,
}

impl TryFrom<ForecastPointDto> for Forecast {
    type Error = AppError;

    fn try_from(value: ForecastPointDto) -> Result<Self, Self::Error> {
        let timestamp = DateTime::from_timestamp(value.dt, 0).ok_or(AppError::OpenWeather)?;

        Ok(Forecast::new(
            timestamp,
            value.main.temp,
            value.main.feels_like,
            value.main.humidity,
            value.wind.speed,
            value.pop,
        ))
    }
}
