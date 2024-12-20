use std::{f64::consts::PI, fmt::Display};

use crate::consts::EARTH_RADIUS_KM;

use super::vertex::Vertex;

/// Representa uma linha entre dois vértices na superfície da Terra.
pub struct Line {
    starting_vertex: Vertex,
    ending_vertex: Vertex,
}

impl Line {
    /// Cria uma nova linha entre dois vértices.
    ///
    /// # Argumentos
    ///
    /// * `starting_vertex` - O vértice inicial da linha.
    /// * `ending_vertex` - O vértice final da linha.
    ///
    /// # Retorno
    ///
    /// Retorna uma instância de `Line`.
    pub fn new(starting_vertex: Vertex, ending_vertex: Vertex) -> Self {
        Self {
            starting_vertex,
            ending_vertex,
        }
    }

    /// Converte um valor em graus para radianos.
    ///
    /// # Argumentos
    ///
    /// * `degree` - O valor em graus a ser convertido.
    ///
    /// # Retorno
    ///
    /// Retorna o valor convertido em radianos.
    fn degrees_to_radians(degree: f64) -> f64 {
        degree * PI / 180.0
    }

    /// Calcula o tamanho da linha (distância entre dois vértices) usando a fórmula de Haversine.
    ///
    /// # Retorno
    ///
    /// Retorna a distância entre os vértices inicial e final da linha, em quilômetros.
    ///
    /// # Fórmula de Haversine
    ///
    /// A fórmula usada é:
    ///
    /// ```text
    /// a = sin²(Δφ/2) + cos(φ1) * cos(φ2) * sin²(Δλ/2)
    /// c = 2 * atan2(√a, √(1-a))
    /// distância = R * c
    /// ```
    /// Onde:
    /// - `Δφ` é a diferença de latitude em radianos.
    /// - `Δλ` é a diferença de longitude em radianos.
    /// - `R` é o raio médio da Terra (6371 km).
    ///
    /// Esta fórmula considera a curvatura da Terra.
    pub fn size(&self) -> f64 {
        // Converte as latitudes e longitudes de graus para radianos
        let lat1_rad = Self::degrees_to_radians(self.starting_vertex.latitude);
        let lat2_rad = Self::degrees_to_radians(self.ending_vertex.latitude);
        let lon1_rad = Self::degrees_to_radians(self.starting_vertex.longitude);
        let lon2_rad = Self::degrees_to_radians(self.ending_vertex.longitude);

        // Calcula as diferenças de latitude e longitude em radianos
        let dist_lat = lat2_rad - lat1_rad;
        let dist_lon = lon2_rad - lon1_rad;

        // Fórmula de Haversine
        let a = (dist_lat / 2.0).sin().powi(2)
            + lat1_rad.cos() * lat2_rad.cos() * (dist_lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        // Multiplica pelo raio médio da Terra para obter a distância
        EARTH_RADIUS_KM * c
    }
}

impl Display for Line {
    /// Implementação da trait `Display` para exibir uma linha no formato legível.
    ///
    /// Exemplo de saída:
    /// ```
    /// Line(starting_vertex: Vertex(lat: ..., lon: ...), ending_vertex: Vertex(lat: ..., lon: ...))
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Line(starting_vertex: {}, ending_vertex: {})",
            self.starting_vertex, self.ending_vertex
        )
    }
}
