use std::fmt::Display;

use super::vertex::Vertex;

/// Representa uma linha entre dois vértices na superfície da Terra.
pub struct Line {
    pub starting_vertex: Vertex,
    pub ending_vertex: Vertex,
}

impl Line {
    /// Cria uma nova linha entre dois vértices.
    pub fn new(starting_vertex: Vertex, ending_vertex: Vertex) -> Result<Self, String> {
        if !Self::vertexes_are_different(&starting_vertex, &ending_vertex) {
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
    fn vertexes_are_different(starting_vertex: &Vertex, ending_vertex: &Vertex) -> bool {
        !starting_vertex.is_equal(ending_vertex)
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
