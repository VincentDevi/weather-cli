use serde::{Deserialize, Serialize};

use crate::entity::Coordinate;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct City {
    name: String,
    coordinates: Coordinate,
}

impl City {
    pub fn new(name: impl Into<String>, coordinates: Coordinate) -> Self {
        Self {
            name: name.into(),
            coordinates,
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn coordinates(&self) -> &Coordinate {
        &self.coordinates
    }
}
