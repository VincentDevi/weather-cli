use crate::{
    entity::{City, CityWeather, Forecast},
    errors::AppError,
    weather_api::dto::CityDto,
};
use chrono::FixedOffset;
use reqwest::Client;
use std::sync::Arc;

mod dto;
use dto::ForecastResponseDto;

pub struct WeatherClient {
    client: Client,
    api_key: Arc<str>,
}

impl WeatherClient {
    pub fn new(api_key: &str) -> Self {
        Self {
            client: Client::new(),
            api_key: Arc::from(api_key),
        }
    }

    pub async fn get_forecast(&self, city: &City) -> Result<CityWeather, AppError> {
        let forecast_url = "https://api.openweathermap.org/data/2.5/forecast";
        let response = self
            .client
            .get(forecast_url)
            .query(&[
                ("lat", city.coordinates().lat().to_string()),
                ("lon", city.coordinates().long().to_string()),
                ("appid", self.api_key.to_string()),
                ("units", "metric".to_owned()),
                ("lang", "en".to_owned()),
            ])
            .send()
            .await
            .map_err(|_| AppError::OpenWeather)?;

        let dto = response
            .json::<ForecastResponseDto>()
            .await
            .map_err(|_| AppError::OpenWeather)?;

        let timezone_offset_seconds = dto.city.timezone;
        FixedOffset::east_opt(timezone_offset_seconds).ok_or(AppError::OpenWeather)?;

        let forecast = dto
            .list
            .into_iter()
            .map(Forecast::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(CityWeather::new(
            City::new(
                city.name(),
                crate::entity::Coordinate::new(city.coordinates().lat(), city.coordinates().long()),
            ),
            forecast,
            timezone_offset_seconds,
        ))
    }

    pub async fn get_geocoding(&self, location: impl Into<String>) -> Result<City, AppError> {
        let url = "http://api.openweathermap.org/geo/1.0/direct";

        let response = self
            .client
            .get(url)
            .query(&[
                ("q", format!("{},BE", location.into())),
                ("limit", '1'.to_string()),
                ("appid", self.api_key.to_string()),
            ])
            .send()
            .await
            .map_err(|err| AppError::Dev(err.to_string()))?;

        let dto = response
            .json::<Vec<CityDto>>()
            .await
            .map_err(|_| AppError::OpenWeather)?;
        dto.into_iter()
            .next()
            .map(City::from)
            .ok_or(AppError::OpenWeather)
    }
}
