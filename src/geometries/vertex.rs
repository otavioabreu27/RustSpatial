use std::fmt::Display;

pub struct Vertex {
    latitude: f64,
    longitude: f64
}

impl Vertex {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {latitude, longitude}
    }
}

impl Display for Vertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vertex(lat: {:.6}, lon: {:.6})", self.latitude, self.longitude)
    }
}