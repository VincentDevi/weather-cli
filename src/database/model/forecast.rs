use chrono::{DateTime, Utc};

use crate::entity::Forecast;

#[derive(Debug, Clone)]
pub struct ForecastModel {
    pub city_id: i64,
    pub forecast_at: DateTime<Utc>,
    pub temperature_celsius: f64,
    pub humidity_percent: i64,
    pub precipitation_probability: f64,
    pub timezone_offset_seconds: i32,
}
impl ForecastModel {
    pub fn new(
        forecast_at: DateTime<Utc>,
        temperature_celsius: f64,
        humidity_percent: i64,
        precipitation_probability: f64,
        timezone: i32,
        city_id: i64,
    ) -> Self {
        Self {
            forecast_at,
            temperature_celsius,
            humidity_percent,
            precipitation_probability,
            timezone_offset_seconds: timezone,
            city_id,
        }
    }
}

impl From<ForecastModel> for Forecast {
    fn from(value: ForecastModel) -> Self {
        Self::new(
            value.forecast_at,
            value.temperature_celsius,
            value.humidity_percent,
            value.precipitation_probability,
        )
    }
}

#[derive(Debug, Clone)]
pub struct CreateForecastModel {
    pub city_id: i64,
    pub forecast_at: DateTime<Utc>,
    pub temperature_celsius: f64,
    pub humidity_percent: i64,
    pub precipitation_probability: f64,
    pub timezone_offset_seconds: i32,
}

impl CreateForecastModel {
    pub fn new(
        city_id: i64,
        forecast_at: DateTime<Utc>,
        temperature_celsius: f64,
        humidity_percent: i64,
        precipitation_probability: f64,
        timezone_offset_seconds: i32,
    ) -> Self {
        Self {
            city_id,
            forecast_at,
            temperature_celsius,
            humidity_percent,
            precipitation_probability,
            timezone_offset_seconds,
        }
    }
}
