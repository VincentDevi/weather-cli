use super::City;
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Forecast {
    timestamp: DateTime<Utc>,
    temperature_celsius: f64,
    feels_like_celsius: f64,
    humidity_percent: i64,
    wind_speed_mps: f64,
    precipitation_probability: f64,
}

impl Forecast {
    pub fn new(
        timestamp: DateTime<Utc>,
        temperature_celsius: f64,
        feels_like_celsius: f64,
        humidity_percent: i64,
        wind_speed_mps: f64,
        precipitation_probability: f64,
    ) -> Self {
        Self {
            timestamp,
            temperature_celsius,
            feels_like_celsius,
            humidity_percent,
            wind_speed_mps,
            precipitation_probability,
        }
    }

    pub fn temperature_celsius(&self) -> f64 {
        self.temperature_celsius
    }

    pub fn humidity_percent(&self) -> i64 {
        self.humidity_percent
    }

    pub fn precipitation_probability(&self) -> f64 {
        self.precipitation_probability
    }
}

#[derive(Debug)]
pub struct CityWeather {
    city: City,
    forecast: Vec<Forecast>,
}

impl CityWeather {
    pub fn new(city: City, forecast: Vec<Forecast>) -> Self {
        Self { city, forecast }
    }

    pub fn city(&self) -> &City {
        &self.city
    }

    pub fn next_forecast(&self) -> Option<&Forecast> {
        self.forecast.first()
    }
}
