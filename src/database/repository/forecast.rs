use chrono::{DateTime, FixedOffset, NaiveDate, Utc};
use rusqlite::types::Value;
use rusqlite::{Row, TransactionBehavior, params, params_from_iter};

use crate::database::Database;
use crate::database::error::DatabaseError;
use crate::database::model::{CreateForecastModel, ForecastModel};

#[derive(Debug, Clone, Default)]
pub struct ForecastFilter {
    pub city_id: Option<i64>,
    pub date: Option<NaiveDate>,
}

impl ForecastFilter {
    pub fn to_sql(&self) -> (String, Vec<Value>) {
        let mut predicates = Vec::new();
        let mut values = Vec::new();
        if let Some(city_id) = self.city_id {
            predicates.push("city_id = ?");
            values.push(Value::Integer(city_id));
        }
        if let Some(date) = &self.date {
            predicates.push("date(forecast_at + timezone_offset_seconds, 'unixepoch') = date(?)");
            values.push(Value::Text(date.format("%Y-%m-%d").to_string()));
        }
        let clause = if predicates.is_empty() {
            String::new()
        } else {
            format!(" WHERE {}", predicates.join(" AND "))
        };
        (clause, values)
    }
}

const FORECAST_COLUMNS: &str = "city_id, forecast_at, temperature_celsius, humidity_percent, precipitation_probability, timezone_offset_seconds";
type RawForecastRow = (i64, i64, f64, i64, f64, i64);

impl Database {
    pub fn create_forecasts(
        &mut self,
        input: &[CreateForecastModel],
    ) -> Result<Vec<ForecastModel>, DatabaseError> {
        if input.is_empty() {
            return Err(DatabaseError::EmptyForecastResponse);
        }
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let mut result = Vec::with_capacity(input.len());
        for forecast in input {
            validate_forecast_input(forecast)?;
            let row = transaction.query_row(
            &format!("INSERT INTO forecasts (city_id, forecast_at, temperature_celsius, humidity_percent, precipitation_probability, timezone_offset_seconds) VALUES (?1, ?2, ?3, ?4, ?5, ?6) ON CONFLICT(city_id, forecast_at) DO UPDATE SET temperature_celsius = excluded.temperature_celsius, humidity_percent = excluded.humidity_percent, precipitation_probability = excluded.precipitation_probability, timezone_offset_seconds = excluded.timezone_offset_seconds RETURNING {FORECAST_COLUMNS}"),
            params![forecast.city_id, forecast.forecast_at.timestamp(), forecast.temperature_celsius, forecast.humidity_percent, forecast.precipitation_probability, forecast.timezone_offset_seconds],
            map_forecast_row,
        )?;
            result.push(forecast_from_row(row)?);
        }
        transaction.commit()?;
        Ok(result)
    }

    pub fn delete_forecasts(&mut self, filter: &ForecastFilter) -> Result<usize, DatabaseError> {
        if filter.city_id.is_none() && filter.date.is_none() {
            return Err(DatabaseError::EmptyForecastDeleteFilter);
        }
        let (where_clause, values) = filter.to_sql();
        Ok(self.connection.execute(
            &format!("DELETE FROM forecasts{where_clause}"),
            params_from_iter(values),
        )?)
    }

    pub fn get_forecasts(
        &self,
        filter: &ForecastFilter,
    ) -> Result<Vec<ForecastModel>, DatabaseError> {
        let (where_clause, values) = filter.to_sql();
        let sql = format!(
            "SELECT {FORECAST_COLUMNS} FROM forecasts{where_clause} ORDER BY city_id ASC, forecast_at ASC"
        );
        let mut statement = self.connection.prepare(&sql)?;
        let rows = statement.query_map(params_from_iter(values), map_forecast_row)?;
        rows.map(|row| forecast_from_row(row?)).collect()
    }
}
fn validate_forecast_input(forecast: &CreateForecastModel) -> Result<(), DatabaseError> {
    if !forecast.temperature_celsius.is_finite() {
        return Err(DatabaseError::InvalidDatabaseData(format!(
            "invalid temperature {}",
            forecast.temperature_celsius
        )));
    }
    if !(0..=100).contains(&forecast.humidity_percent) {
        return Err(DatabaseError::InvalidDatabaseData(format!(
            "invalid humidity {}",
            forecast.humidity_percent
        )));
    }
    if !forecast.precipitation_probability.is_finite()
        || !(0.0..=1.0).contains(&forecast.precipitation_probability)
    {
        return Err(DatabaseError::InvalidDatabaseData(format!(
            "invalid precipitation probability {}",
            forecast.precipitation_probability
        )));
    }
    FixedOffset::east_opt(forecast.timezone_offset_seconds).ok_or_else(|| {
        DatabaseError::InvalidDatabaseData(format!(
            "invalid timezone offset {}",
            forecast.timezone_offset_seconds
        ))
    })?;
    Ok(())
}
fn forecast_from_row(
    (city_id, forecast_at, temperature, humidity, precipitation, timezone): RawForecastRow,
) -> Result<ForecastModel, DatabaseError> {
    let forecast_at = DateTime::<Utc>::from_timestamp(forecast_at, 0).ok_or_else(|| {
        DatabaseError::InvalidDatabaseData(format!("invalid forecast timestamp {forecast_at}"))
    })?;
    if !temperature.is_finite() {
        return Err(DatabaseError::InvalidDatabaseData(format!(
            "invalid temperature {temperature}"
        )));
    }
    if !(0..=100).contains(&humidity) {
        return Err(DatabaseError::InvalidDatabaseData(format!(
            "invalid humidity {humidity}"
        )));
    }
    if !precipitation.is_finite() || !(0.0..=1.0).contains(&precipitation) {
        return Err(DatabaseError::InvalidDatabaseData(format!(
            "invalid precipitation probability {precipitation}"
        )));
    }
    let timezone = i32::try_from(timezone).map_err(|_| {
        DatabaseError::InvalidDatabaseData(format!("invalid timezone offset {timezone}"))
    })?;
    FixedOffset::east_opt(timezone).ok_or_else(|| {
        DatabaseError::InvalidDatabaseData(format!("invalid timezone offset {timezone}"))
    })?;
    Ok(ForecastModel::new(
        forecast_at,
        temperature,
        humidity,
        precipitation,
        timezone,
        city_id,
    ))
}
fn map_forecast_row(row: &Row<'_>) -> rusqlite::Result<RawForecastRow> {
    Ok((
        row.get(0)?,
        row.get(1)?,
        row.get(2)?,
        row.get(3)?,
        row.get(4)?,
        row.get(5)?,
    ))
}
