use crate::cli::ForecastDay;
use crate::database::{
    CityFilter, CityModel, CreateCityModel, CreateForecastModel, Database, ForecastFilter,
};
use crate::entity::{City, CityWeather};
use crate::output::{WeatherTableRow, render_weather_table};
use crate::weather_api::WeatherClient;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use tokio::task::JoinSet;

use super::config::*;
use super::errors::*;

#[allow(async_fn_in_trait)]
pub trait WeatherProvider {
    async fn geocode_belgian_city(&self, name: &str) -> Result<City, AppError>;
    async fn forecast(&self, city: &City) -> Result<CityWeather, AppError>;
}

impl WeatherProvider for WeatherClient {
    async fn geocode_belgian_city(&self, name: &str) -> Result<City, AppError> {
        self.get_geocoding(name).await
    }

    async fn forecast(&self, city: &City) -> Result<CityWeather, AppError> {
        self.get_forecast(city).await
    }
}

pub async fn fav_city(
    config: Config,
    database: &mut Database,
    city: impl Into<String>,
    day: Option<ForecastDay>,
) -> Result<(), AppError> {
    let now = Utc::now();
    let weather_api = WeatherClient::new(&config.open_weather_key());
    let requested_name = city.into().trim().to_owned();
    let city = database
        .fetch_cities(&CityFilter {
            name: Some(requested_name.clone()),
            is_favorite: Some(true),
        })?
        .into_iter()
        .next()
        .ok_or(AppError::FavoriteCityNotFound(requested_name))?;
    let result = weather_for_city_and_day(database, &weather_api, city, day, now).await?;
    let row = weather_table_row(&result, day, now)?;

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
            let forecast = weather_api.get_forecast(&city).await?;
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
        .collect::<Result<Vec<_>, _>>()?;

    println!("{}", render_weather_table(&rows));
    Ok(())
}

pub async fn city(
    config: Config,
    database: &mut Database,
    city: impl Into<String>,
    day: Option<ForecastDay>,
) -> Result<(), AppError> {
    let now = Utc::now();
    let weather_api = WeatherClient::new(&config.open_weather_key());
    let result_weather = weather_for_unknown_city(
        database,
        &weather_api,
        city.into().trim().to_owned(),
        day,
        now,
    )
    .await?;
    let row = weather_table_row(&result_weather, day, now)?;

    println!("{}", render_weather_table(&[row]));

    Ok(())
}

async fn weather_for_unknown_city<P: WeatherProvider>(
    database: &mut Database,
    provider: &P,
    requested_name: String,
    day: Option<ForecastDay>,
    now: DateTime<Utc>,
) -> Result<CityWeather, AppError> {
    let stored_city = match database
        .fetch_cities(&CityFilter {
            name: Some(requested_name.clone()),
            is_favorite: None,
        })?
        .into_iter()
        .next()
    {
        Some(city) => city,
        None => {
            let geocoded = provider.geocode_belgian_city(&requested_name).await?;
            match database
                .fetch_cities(&CityFilter {
                    name: Some(geocoded.name().to_owned()),
                    is_favorite: None,
                })?
                .into_iter()
                .next()
            {
                Some(city) => city,
                None => database.create_city(&CreateCityModel::new(
                    geocoded.name(),
                    geocoded.coordinates().lat(),
                    geocoded.coordinates().long(),
                    false,
                ))?,
            }
        }
    };

    weather_for_city_and_day(database, provider, stored_city, day, now).await
}

async fn weather_for_city_and_day<P: WeatherProvider>(
    database: &mut Database,
    provider: &P,
    stored_city: CityModel,
    day: Option<ForecastDay>,
    now: DateTime<Utc>,
) -> Result<CityWeather, AppError> {
    let cached_forecasts = database.get_forecasts(&ForecastFilter {
        city_id: Some(stored_city.id),
        date: None,
    })?;
    if let Some(timezone_offset_seconds) = cached_forecasts
        .first()
        .map(|forecast| forecast.timezone_offset_seconds)
    {
        let cached_weather = CityWeather::new(
            City::from(&stored_city),
            cached_forecasts.into_iter().map(Into::into).collect(),
            timezone_offset_seconds,
        );
        if cached_weather
            .forecast_for_day(day.map_or(0, ForecastDay::days_from_now), now)
            .is_some()
        {
            return Ok(cached_weather);
        }
    }

    let weather = provider.forecast(&City::from(&stored_city)).await?;
    if weather.forecasts().is_empty() {
        return Err(AppError::EmptyForecastResponse);
    }
    if weather
        .forecast_for_day(day.map_or(0, ForecastDay::days_from_now), now)
        .is_none()
    {
        return Err(AppError::ForecastNotAvailable);
    }

    let forecasts = weather
        .forecasts()
        .iter()
        .map(|forecast| {
            CreateForecastModel::new(
                stored_city.id,
                forecast.forecast_at(),
                forecast.temperature_celsius(),
                forecast.humidity_percent(),
                forecast.precipitation_probability(),
                weather.timezone_offset_seconds(),
            )
        })
        .collect::<Vec<_>>();
    database.create_forecasts(&forecasts)?;

    Ok(weather)
}

fn weather_table_row(
    weather: &CityWeather,
    day: Option<ForecastDay>,
    now: DateTime<Utc>,
) -> Result<WeatherTableRow, AppError> {
    let forecast = weather
        .forecast_for_day(day.map_or(0, ForecastDay::days_from_now), now)
        .ok_or(AppError::ForecastNotAvailable)?;

    Ok(WeatherTableRow::new(
        weather.city().name(),
        weather.local_date(forecast),
        Some(forecast.temperature_celsius()),
        Some(forecast.humidity_percent()),
        Some(forecast.precipitation_probability()),
    ))
}
