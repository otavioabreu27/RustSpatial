use std::{f64::consts::PI, fmt::Display};

use crate::consts::EARTH_RADIUS_KM;

use super::vertex::Vertex;

/// Representa uma linha entre dois vértices na superfície da Terra.
pub struct Line {
    pub starting_vertex: Vertex,
    pub ending_vertex: Vertex,
}

impl Line {
    /// Cria uma nova linha entre dois vértices.
    pub fn new(starting_vertex: Vertex, ending_vertex: Vertex) -> Result<Self, String> {
        if !Self::line_is_valid(&starting_vertex, &ending_vertex) {
            Err(format!(
                "Os vértices de entrada e saída são iguais: Entrada: {}; Saída: {}",
                starting_vertex, ending_vertex
            ))
        } else {
            Ok(Self {
                starting_vertex,
                ending_vertex,
            })
        }
    }

    /// Verifica se os vértices de início e fim são diferentes.
    fn line_is_valid(starting_vertex: &Vertex, ending_vertex: &Vertex) -> bool {
        starting_vertex != ending_vertex
    }

    /// Converte graus para radianos.
    fn degrees_to_radians(degree: f64) -> f64 {
        degree * PI / 180.0
    }

    /// Calcula a distância da linha usando a fórmula de Haversine.
    pub fn calculate_earth_radius_distance(&self) -> f64 {
        let lat1_rad = Self::degrees_to_radians(self.starting_vertex.latitude);
        let lat2_rad = Self::degrees_to_radians(self.ending_vertex.latitude);
        let lon1_rad = Self::degrees_to_radians(self.starting_vertex.longitude);
        let lon2_rad = Self::degrees_to_radians(self.ending_vertex.longitude);

        let dist_lat = lat2_rad - lat1_rad;
        let dist_lon = lon2_rad - lon1_rad;

        let a = (dist_lat / 2.0).sin().powi(2)
            + lat1_rad.cos() * lat2_rad.cos() * (dist_lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        EARTH_RADIUS_KM * c
    }

    /// Calcula a distância euclidiana em 2D.
    pub fn calculate_euclidean_distance(&self) -> f64 {
        let dx = self.ending_vertex.latitude - self.starting_vertex.latitude;
        let dy = self.ending_vertex.longitude - self.starting_vertex.longitude;
        (dx * dx + dy * dy).sqrt()
    }
}

impl Display for Line {
    /// Implementação da trait `Display` para exibir a linha.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Line(starting_vertex: {}, ending_vertex: {})",
            self.starting_vertex, self.ending_vertex
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_creation_success() {
        let vertex_a = Vertex::new(10.123456, -20.654321);
        let vertex_b = Vertex::new(11.123456, -21.654321);

        let line = Line::new(vertex_a, vertex_b);

        // Verifica se a linha foi criada com sucesso
        assert!(line.is_ok());

        let line = line.unwrap();
        assert_eq!(line.starting_vertex, Vertex::new(10.123456, -20.654321));
        assert_eq!(line.ending_vertex, Vertex::new(11.123456, -21.654321));
    }

    #[test]
    fn test_line_exception_same_vertex() {
        let vertex_a = Vertex::new(10.123456, -20.654321);
        let vertex_b = Vertex::new(10.123456, -20.654321);

        let line = Line::new(vertex_a, vertex_b);

        assert!(line.is_err());

        assert_eq!(
            line.err().unwrap(),
            "Os vértices de entrada e saída são iguais: Entrada: Vertex(lat: 10.123456, lon: -20.654321); Saída: Vertex(lat: 10.123456, lon: -20.654321)"
        );
    }

    #[test]
    fn test_line_calculate_euclidean_distance() {
        let vertex_a = Vertex::new(0.0, 0.0);
        let vertex_b = Vertex::new(3.0, 4.0);

        let line = Line::new(vertex_a, vertex_b).unwrap();

        // Verifica se a distância euclidiana está correta
        assert_eq!(line.calculate_euclidean_distance(), 5.0);
    }

    #[test]
    fn test_line_calculate_earth_radius_distance() {
        let vertex_a = Vertex::new(0.0, 0.0);
        let vertex_b = Vertex::new(0.0, 1.0);

        let line = Line::new(vertex_a, vertex_b).unwrap();

        // Verifica se a distância haversine está próxima do esperado
        let expected_distance = EARTH_RADIUS_KM * PI / 180.0; // Aproximadamente 1 grau em radianos
        assert!((line.calculate_earth_radius_distance() - expected_distance).abs() < 0.001);
    }

    #[test]
    fn test_line_display() {
        let vertex_a = Vertex::new(10.123456, -20.654321);
        let vertex_b = Vertex::new(10.123456, -20.654322);

        let line = Line::new(vertex_a, vertex_b).unwrap();

        let display = format!("{}", line);
        assert_eq!(display, "Line(starting_vertex: Vertex(lat: 10.123456, lon: -20.654321), ending_vertex: Vertex(lat: 10.123456, lon: -20.654322))")
    }
}
