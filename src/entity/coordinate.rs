#[derive(Debug)]
pub struct Coordinate {
    lat: f64,
    long: f64,
}

impl Coordinate {
    pub fn new(lat: f64, long: f64) -> Self {
        Self { lat, long }
    }
    pub fn lat(&self) -> f64 {
        self.lat
    }
    pub fn long(&self) -> f64 {
        self.long
    }
}
