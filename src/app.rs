use crate::entity::City;
use crate::entity::Coordinate;
use crate::output::{WeatherTableRow, render_weather_table};
use crate::weather_api::WeatherClient;
use std::sync::Arc;
use tokio::task::JoinSet;

use super::config::*;
use super::errors::*;

const BELGIAN_CITY_DEFINITIONS: [(&str, f64, f64); 10] = [
    ("Brussels", 50.850346, 4.351721),
    ("Antwerp", 51.219448, 4.402464),
    ("Ghent", 51.054342, 3.717424),
    ("Charleroi", 50.410809, 4.444643),
    ("Liège", 50.632557, 5.579666),
    ("Bruges", 51.209348, 3.224700),
    ("Namur", 50.467388, 4.871985),
    ("Leuven", 50.879844, 4.700518),
    ("Mons", 50.454241, 3.956659),
    ("Hasselt", 50.930690, 5.332480),
];

pub async fn run(config: Config) -> Result<(), AppError> {
    let weather_api = WeatherClient::new(&config.open_weather_key());
    let prout = weather_api
        .get_forecast(City::new("Charleroi", Coordinate::new(50.410809, 4.444643)))
        .await
        .map_err(|_| AppError::OpenWeather)?;
    println!("{:?}", prout);
    Ok(())
}

pub async fn fetch_belgian_city_forecasts(config: Config) -> Result<(), AppError> {
    let weather_api = Arc::new(WeatherClient::new(&config.open_weather_key()));
    let mut requests = JoinSet::new();

    for (index, (name, latitude, longitude)) in BELGIAN_CITY_DEFINITIONS.into_iter().enumerate() {
        let weather_api = Arc::clone(&weather_api);
        requests.spawn(async move {
            let forecast = weather_api
                .get_forecast(City::new(name, Coordinate::new(latitude, longitude)))
                .await?;
            Ok::<_, AppError>((index, forecast))
        });
    }

    let mut forecasts = Vec::with_capacity(BELGIAN_CITY_DEFINITIONS.len());
    while let Some(result) = requests.join_next().await {
        let forecast = result.map_err(|_| AppError::OpenWeather)??;
        forecasts.push(forecast);
    }

    forecasts.sort_by_key(|(index, _)| *index);
    let rows = forecasts
        .iter()
        .map(|(_, weather)| {
            let forecast = weather.next_forecast();
            WeatherTableRow::new(
                weather.city().name(),
                forecast.map(|forecast| forecast.temperature_celsius()),
                forecast.map(|forecast| forecast.humidity_percent()),
                forecast.map(|forecast| forecast.precipitation_probability()),
            )
        })
        .collect::<Vec<_>>();

    println!("{}", render_weather_table(&rows));
    Ok(())
}
