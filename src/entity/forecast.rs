use super::City;
use chrono::{DateTime, Days, FixedOffset, NaiveDate, NaiveTime, Utc};

#[derive(Debug)]
pub struct Forecast {
    forecast_at: DateTime<Utc>,
    temperature_celsius: f64,
    humidity_percent: i64,
    precipitation_probability: f64,
}

impl Forecast {
    pub fn new(
        forecast_at: DateTime<Utc>,
        temperature_celsius: f64,
        humidity_percent: i64,
        precipitation_probability: f64,
    ) -> Self {
        Self {
            forecast_at,
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
    timezone_offset_seconds: i32,
}

impl CityWeather {
    pub fn new(city: City, forecast: Vec<Forecast>, timezone_offset_seconds: i32) -> Self {
        Self {
            city,
            forecast,
            timezone_offset_seconds,
        }
    }

    pub fn city(&self) -> &City {
        &self.city
    }

    pub fn next_forecast(&self) -> Option<&Forecast> {
        self.forecast.first()
    }

    pub fn local_date(&self, forecast: &Forecast) -> Option<NaiveDate> {
        let timezone = FixedOffset::east_opt(self.timezone_offset_seconds)?;
        Some(forecast.forecast_at.with_timezone(&timezone).date_naive())
    }

    pub fn forecast_for_day(&self, days_from_now: u64, now: DateTime<Utc>) -> Option<&Forecast> {
        let timezone = FixedOffset::east_opt(self.timezone_offset_seconds)?;
        let target_date = now
            .with_timezone(&timezone)
            .date_naive()
            .checked_add_days(Days::new(days_from_now))?;
        let local_noon = target_date.and_time(NaiveTime::from_hms_opt(12, 0, 0)?);

        self.forecast
            .iter()
            .filter_map(|forecast| {
                let local_time = forecast.forecast_at.with_timezone(&timezone).naive_local();
                (local_time.date() == target_date).then_some((forecast, local_time))
            })
            .min_by_key(|(forecast, local_time)| {
                (
                    (*local_time - local_noon).num_seconds().unsigned_abs(),
                    forecast.forecast_at.timestamp(),
                )
            })
            .map(|(forecast, _)| forecast)
    }
}
