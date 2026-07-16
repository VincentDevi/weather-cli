use crate::entity::{City, Coordinate};

#[derive(Debug)]
pub struct CityModel {
    pub id: i64,
    pub name: String,
    pub lat: f64,
    pub lon: f64,
    pub is_favorite: bool,
}

impl From<&CityModel> for City {
    fn from(value: &CityModel) -> Self {
        Self::new(value.name.clone(), Coordinate::new(value.lat, value.lon))
    }
}

#[derive(Debug)]
pub struct CreateCityModel {
    pub name: String,
    pub lat: f64,
    pub lon: f64,
    pub is_favorite: bool,
}

impl CreateCityModel {
    pub fn new(name: impl Into<String>, lat: f64, lon: f64, is_favorite: bool) -> Self {
        Self {
            name: name.into(),
            lat,
            lon,
            is_favorite,
        }
    }
}

#[derive(Debug, Default)]
pub struct UpdateCityModel {
    pub name: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub is_favorite: Option<bool>,
}
