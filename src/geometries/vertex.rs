use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub latitude: f64,
    pub longitude: f64,
}

impl Vertex {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
        }
    }

    // Compara o vertice com outro
    pub fn is_equal(&self, other_vertex: &Vertex) -> bool {
        self == other_vertex
    }
}

impl Display for Vertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Vertex(lat: {:.6}, lon: {:.6})",
            self.latitude, self.longitude
        )
    }
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.latitude == other.latitude && self.longitude == other.longitude
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_creation() {
        let vertex = Vertex::new(10.123456, -20.654321);
        assert_eq!(vertex.latitude, 10.123456);
        assert_eq!(vertex.longitude, -20.654321);
    }

    #[test]
    fn test_vertex_display() {
        let vertex = Vertex::new(10.123456, -20.654321);
        let display = format!("{}", vertex);
        assert_eq!(display, "Vertex(lat: 10.123456, lon: -20.654321)");
    }

    #[test]
    fn test_vertex_equality() {
        let vertex1 = Vertex::new(10.0, -20.0);
        let vertex2 = Vertex::new(10.0, -20.0);
        let vertex3 = Vertex::new(15.0, -25.0);

        assert_eq!(vertex1, vertex2);
        assert_ne!(vertex1, vertex3);
    }

    #[test]
    fn test_vertex_clone() {
        let vertex1 = Vertex::new(10.0, -20.0);
        let vertex2 = vertex1.clone();

        assert_eq!(vertex1, vertex2);
        assert_ne!(&vertex1 as *const _, &vertex2 as *const _);
    }
}
