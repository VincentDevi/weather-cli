use crate::database::model::{CityModel, CreateCityModel, UpdateCityModel};

use super::super::Database;
use super::super::error::DatabaseError;
use rusqlite::{Row, params, params_from_iter, types::Value};

#[derive(Debug, Clone, Default)]
pub struct CityFilter {
    pub name: Option<String>,
    pub is_favorite: Option<bool>,
}

type RawCityRow = (i64, String, f64, f64, i64);
const CITY_COLUMNS: &str = "id, name, latitude, longitude, is_favorite";

impl CityFilter {
    pub fn to_sql(&self) -> (String, Vec<Value>) {
        let mut predicates = Vec::new();
        let mut values = Vec::new();
        if let Some(name) = &self.name {
            predicates.push("name_key = ?");
            values.push(Value::Text(name.to_lowercase()));
        }
        if let Some(is_favorite) = self.is_favorite {
            predicates.push("is_favorite = ?");
            values.push(Value::Integer(i64::from(is_favorite)));
        }
        let clause = if predicates.is_empty() {
            String::new()
        } else {
            format!(" WHERE {}", predicates.join(" AND "))
        };
        (clause, values)
    }
}

impl Database {
    pub fn fetch_cities(&self, filter: &CityFilter) -> Result<Vec<CityModel>, DatabaseError> {
        let (where_clause, values) = filter.to_sql();
        let sql = format!("SELECT {CITY_COLUMNS} FROM cities{where_clause} ORDER BY id ASC");
        let mut statement = self.connection.prepare(&sql)?;
        let rows = statement.query_map(params_from_iter(values), map_city_row)?;
        rows.map(|row| city_from_row(row?)).collect()
    }

    pub fn create_city(&mut self, input: &CreateCityModel) -> Result<CityModel, DatabaseError> {
        let row = self.connection.query_row(
        &format!("INSERT INTO cities (name, name_key, latitude, longitude, is_favorite) VALUES (?1, ?2, ?3, ?4, ?5) RETURNING {CITY_COLUMNS}"),
        params![input.name, input.name.to_lowercase(), input.lat, input.lon, input.is_favorite],
        map_city_row,
    )?;
        city_from_row(row)
    }

    pub fn delete_city(&mut self, id: i64) -> Result<bool, DatabaseError> {
        Ok(self
            .connection
            .execute("DELETE FROM cities WHERE id = ?1", [id])?
            != 0)
    }

    pub fn update_city(
        &mut self,
        id: i64,
        input: &UpdateCityModel,
    ) -> Result<CityModel, DatabaseError> {
        let mut assignments = Vec::new();
        let mut values = Vec::new();
        if let Some(name) = &input.name {
            assignments.extend(["name = ?", "name_key = ?"]);
            values.extend([Value::Text(name.clone()), Value::Text(name.to_lowercase())]);
        }
        if let Some(lat) = input.lat {
            assignments.push("latitude = ?");
            values.push(Value::Real(lat));
        }
        if let Some(lon) = input.lon {
            assignments.push("longitude = ?");
            values.push(Value::Real(lon));
        }
        if let Some(is_favorite) = input.is_favorite {
            assignments.push("is_favorite = ?");
            values.push(Value::Integer(i64::from(is_favorite)));
        }
        values.push(Value::Integer(id));

        let row = if assignments.is_empty() {
            self.connection.query_row(
                &format!("SELECT {CITY_COLUMNS} FROM cities WHERE id = ?"),
                params_from_iter(values),
                map_city_row,
            )
        } else {
            self.connection.query_row(
                &format!(
                    "UPDATE cities SET {} WHERE id = ? RETURNING {CITY_COLUMNS}",
                    assignments.join(", ")
                ),
                params_from_iter(values),
                map_city_row,
            )
        };
        match row {
            Ok(row) => city_from_row(row),
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                Err(DatabaseError::DatabaseRecordNotFound {
                    resource: "city",
                    id,
                })
            }
            Err(error) => Err(error.into()),
        }
    }
}
fn map_city_row(row: &Row<'_>) -> rusqlite::Result<RawCityRow> {
    Ok((
        row.get(0)?,
        row.get(1)?,
        row.get(2)?,
        row.get(3)?,
        row.get(4)?,
    ))
}
fn city_from_row(
    (id, name, latitude, longitude, favorite): RawCityRow,
) -> Result<CityModel, DatabaseError> {
    match favorite {
        0 | 1 => Ok(CityModel {
            id,
            name,
            lat: latitude,
            lon: longitude,
            is_favorite: favorite == 1,
        }),
        value => Err(DatabaseError::InvalidDatabaseData(format!(
            "city {id} has invalid favorite flag {value}"
        ))),
    }
}
