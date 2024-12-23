use super::line::Line;
use rayon::prelude::*;

pub enum PathCalcFullDistanceOptions {
    Euclidean,
    EarthRadius,
}

pub struct Path<const N: usize> {
    pub lines: [Line; N],
}

impl<const N: usize> Path<N> {
    pub fn new(lines: [Line; N]) -> Result<Self, String> {
        // Valida todas as linhas no array
        for i in 1..lines.len() {
            if !Self::path_is_valid(&lines[i - 1], &lines[i]) {
                return Err(format!(
                    "Caminho inconsistente: linha {} não se conecta com a linha {}",
                    i - 1,
                    i
                ));
            }
        }

        Ok(Self { lines })
    }

    // Valida se o caminho e valido
    fn path_is_valid(last_line: &Line, current_line: &Line) -> bool {
        last_line.ending_vertex == current_line.starting_vertex
    }

    /// Calcula a distância total do caminho, com suporte a metodologia paralela.
    pub fn calculate_full_distance(&self, methodology: PathCalcFullDistanceOptions) -> f64 {
        match methodology {
            PathCalcFullDistanceOptions::Euclidean => self
                .lines
                .par_iter() // Iterador paralelo
                .map(|line| line.calculate_euclidean_distance()) // Calcula distância euclidiana para cada linha
                .sum(), // Soma os resultados em paralelo
            PathCalcFullDistanceOptions::EarthRadius => self
                .lines
                .par_iter() // Iterador paralelo
                .map(|line| line.calculate_earth_radius_distance()) // Calcula distância Haversine para cada linha
                .sum(), // Soma os resultados em paralelo
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{consts::EARTH_RADIUS_KM, geometries::vertex::Vertex};

    use super::*;

    #[test]
    fn test_path_creation_success() {
        let vertex_a = Vertex::new(10.123456, -20.654321);
        let vertex_b = Vertex::new(11.123456, -21.654321);

        let line_path = Line::new(vertex_a, vertex_b).unwrap();

        let line_check = Line::new(vertex_a, vertex_b).unwrap();

        let path = Path::new([line_path]);

        assert!(path.is_ok());

        let path = path.unwrap();

        assert_eq!(
            path.lines[0].starting_vertex.latitude,
            line_check.starting_vertex.latitude
        );
        assert_eq!(
            path.lines[0].starting_vertex.longitude,
            line_check.starting_vertex.longitude
        );
        assert_eq!(
            path.lines[0].ending_vertex.latitude,
            line_check.ending_vertex.latitude
        );
        assert_eq!(
            path.lines[0].ending_vertex.longitude,
            line_check.ending_vertex.longitude
        );
    }

    #[test]
    fn test_path_creation_error() {
        let vertex_a = Vertex::new(10.123456, -20.654321);
        let vertex_b = Vertex::new(11.123456, -21.654321);
        let vertex_c = Vertex::new(12.123456, -22.654321);

        let line_a = Line::new(vertex_a, vertex_b).unwrap();
        let line_b = Line::new(vertex_c, vertex_a).unwrap();

        let path = Path::new([line_a, line_b]);

        assert!(path.is_err());

        assert_eq!(
            path.err().unwrap(),
            "Caminho inconsistente: linha 0 não se conecta com a linha 1"
        )
    }

    #[test]
    fn test_path_creation_error_detailed() {
        let vertex_a = Vertex::new(10.123456, -20.654321);
        let vertex_b = Vertex::new(11.123456, -21.654321);
        let vertex_c = Vertex::new(12.123456, -22.654321);

        // Cria linhas desconectadas
        let line_a = Line::new(vertex_a, vertex_b).unwrap();
        let line_b = Line::new(vertex_c, vertex_a).unwrap();

        // Tenta criar o caminho inválido
        let path = Path::new([line_a, line_b]);

        // Verifica que é um erro
        assert!(path.is_err());

        // Verifica a mensagem exata do erro
        let err_message = path.err().unwrap();
        assert_eq!(
            err_message,
            "Caminho inconsistente: linha 0 não se conecta com a linha 1"
        );

        // Garanta que as variáveis `i` e `i - 1` foram acessadas
        assert!(err_message.contains("linha 0"));
        assert!(err_message.contains("linha 1"));
    }

    #[test]
    fn test_calculate_full_distance_euclidean() {
        let vertex_a = Vertex::new(0.0, 0.0);
        let vertex_b = Vertex::new(3.0, 4.0);
        let vertex_c = Vertex::new(6.0, 8.0);

        let line_a = Line::new(vertex_a, vertex_b).unwrap();
        let line_b = Line::new(vertex_b, vertex_c).unwrap();

        let path = Path::new([line_a, line_b]).unwrap();

        // Distância total: 5 (linha A -> B) + 5 (linha B -> C) = 10
        let total_distance = path.calculate_full_distance(PathCalcFullDistanceOptions::Euclidean);
        assert_eq!(total_distance, 10.0);
    }

    #[test]
    fn test_calculate_full_distance_earth_radius() {
        let vertex_a = Vertex::new(0.0, 0.0);
        let vertex_b = Vertex::new(0.0, 1.0);
        let vertex_c = Vertex::new(0.0, 2.0);

        let line_a = Line::new(vertex_a, vertex_b).unwrap();
        let line_b = Line::new(vertex_b, vertex_c).unwrap();

        let path = Path::new([line_a, line_b]).unwrap();

        // Distância Haversine esperada para cada linha (aproximadamente)
        let expected_distance_per_line = EARTH_RADIUS_KM * std::f64::consts::PI / 180.0; // 1 grau em radianos
        let total_distance = path.calculate_full_distance(PathCalcFullDistanceOptions::EarthRadius);

        // Valida que a distância total está próxima do esperado (2 vezes a distância por linha)
        assert!((total_distance - 2.0 * expected_distance_per_line).abs() < 0.001);
    }

    #[test]
    fn test_empty_path() {
        let path: Result<Path<0>, String> = Path::new([]);
        assert!(path.is_ok());
        let path = path.unwrap();
        assert_eq!(
            path.calculate_full_distance(PathCalcFullDistanceOptions::Euclidean),
            0.0
        );
    }

    #[test]
    fn test_single_line_path() {
        let vertex_a = Vertex::new(0.0, 0.0);
        let vertex_b = Vertex::new(3.0, 4.0);

        let line = Line::new(vertex_a, vertex_b).unwrap();
        let path = Path::new([line]).unwrap();

        assert_eq!(
            path.calculate_full_distance(PathCalcFullDistanceOptions::Euclidean),
            5.0
        );
    }

    #[test]
    fn test_closed_path() {
        let vertex_a = Vertex::new(0.0, 0.0);
        let vertex_b = Vertex::new(3.0, 4.0);

        let line_a = Line::new(vertex_a, vertex_b).unwrap();
        let line_b = Line::new(vertex_b, vertex_a).unwrap();

        let path = Path::new([line_a, line_b]).unwrap();

        assert_eq!(
            path.calculate_full_distance(PathCalcFullDistanceOptions::Euclidean),
            10.0
        );
    }

    #[test]
    fn test_duplicate_lines() {
        let vertex_a = Vertex::new(0.0, 0.0);
        let vertex_b = Vertex::new(3.0, 4.0);

        let line_a = Line::new(vertex_a, vertex_b).unwrap();
        let line_b = Line::new(vertex_a, vertex_b).unwrap();

        let path = Path::new([line_a, line_b]);

        assert!(path.is_err());
    }
}
