use crate::cli::ForecastDay;
use crate::entity::City;
use crate::entity::CityWeather;
use crate::output::{WeatherTableRow, render_weather_table};
use crate::weather_api::WeatherClient;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use tokio::task::JoinSet;

use super::config::*;
use super::errors::*;

pub async fn fav_city(
    config: Config,
    city: impl Into<String>,
    day: Option<ForecastDay>,
) -> Result<(), AppError> {
    let now = Utc::now();
    let weather_api = WeatherClient::new(&config.open_weather_key());
    let request = city.into().to_lowercase();

    let path = "./data/cities.json";
    let cities: Vec<City> = serde_json::from_str(
        &std::fs::read_to_string(path).map_err(|err| AppError::Dev(err.to_string()))?,
    )
    .map_err(|err| AppError::Dev(err.to_string()))?;

    let city = cities
        .into_iter()
        .find(|city| city.name().to_lowercase() == request.as_str())
        .ok_or(AppError::Dev("nothing found in our json".to_string()))?;

    let result = weather_api
        .get_forecast(city)
        .await
        .map_err(|_| AppError::OpenWeather)?;
    let row = weather_table_row(&result, day, now);

    println!("{}", render_weather_table(&[row]));
    Ok(())
}

pub async fn fetch_belgian_city_forecasts(
    config: Config,
    day: Option<ForecastDay>,
) -> Result<(), AppError> {
    let now = Utc::now();
    let weather_api = Arc::new(WeatherClient::new(&config.open_weather_key()));
    let mut requests = JoinSet::new();

    let path = "./data/cities.json";
    let cities: Vec<City> = serde_json::from_str(
        &std::fs::read_to_string(path).map_err(|err| AppError::Dev(err.to_string()))?,
    )
    .map_err(|err| AppError::Dev(err.to_string()))?;
    let city_count = cities.len();

    for (index, city) in cities.into_iter().enumerate() {
        let weather_api = Arc::clone(&weather_api);
        requests.spawn(async move {
            let forecast = weather_api.get_forecast(city).await?;
            Ok::<_, AppError>((index, forecast))
        });
    }

    let mut forecasts = Vec::with_capacity(city_count);
    while let Some(result) = requests.join_next().await {
        let forecast = result.map_err(|_| AppError::OpenWeather)??;
        forecasts.push(forecast);
    }

    forecasts.sort_by_key(|(index, _)| *index);
    let rows = forecasts
        .iter()
        .map(|(_, weather)| weather_table_row(weather, day, now))
        .collect::<Vec<_>>();

    println!("{}", render_weather_table(&rows));
    Ok(())
}

pub async fn city(
    config: Config,
    city: impl Into<String>,
    day: Option<ForecastDay>,
) -> Result<(), AppError> {
    let now = Utc::now();
    let weather_api = Arc::new(WeatherClient::new(&config.open_weather_key()));
    let result_city = weather_api.get_geocoding(city).await?;

    let result_weather = weather_api
        .get_forecast(result_city)
        .await
        .map_err(|_| AppError::OpenWeather)?;
    let row = weather_table_row(&result_weather, day, now);

    println!("{}", render_weather_table(&[row]));

    Ok(())
}

fn weather_table_row(
    weather: &CityWeather,
    day: Option<ForecastDay>,
    now: DateTime<Utc>,
) -> WeatherTableRow {
    let forecast = match day {
        Some(day) => weather.forecast_for_day(day.days_from_now(), now),
        None => weather.next_forecast(),
    };

    WeatherTableRow::new(
        weather.city().name(),
        forecast.and_then(|forecast| weather.local_date(forecast)),
        forecast.map(|forecast| forecast.temperature_celsius()),
        forecast.map(|forecast| forecast.humidity_percent()),
        forecast.map(|forecast| forecast.precipitation_probability()),
    )
}
