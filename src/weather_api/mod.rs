use crate::{
    entity::{City, CityWeather, Forecast},
    errors::AppError,
};
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

    pub async fn get_forecast(&self, city: City) -> Result<CityWeather, AppError> {
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

        let forecast = dto
            .list
            .into_iter()
            .map(Forecast::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(CityWeather::new(city, forecast))
    }
}
