use super::City;

#[derive(Debug)]
pub struct Forecast {
    temperature_celsius: f64,
    humidity_percent: i64,
    precipitation_probability: f64,
}

impl Forecast {
    pub fn new(
        temperature_celsius: f64,
        humidity_percent: i64,
        precipitation_probability: f64,
    ) -> Self {
        Self {
            temperature_celsius,
            humidity_percent,
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
