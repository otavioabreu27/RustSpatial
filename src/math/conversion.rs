//! Módulo responsável por conversões de coordenadas geográficas e projeções.
//!
//! Este módulo fornece funções para conversões comuns utilizadas em cálculos geodésicos e projeções
//! entre diferentes sistemas de referência de coordenadas (CRS). Ele é essencial para aplicações que
//! trabalham com dados espaciais em formatos diferentes ou que requerem transformações entre
//! projeções geográficas e planas.
//!
//! ## Funcionalidades
//!
//! - **Conversão de graus para radianos**: Função utilitária para converter ângulos em graus para radianos,
//!   amplamente utilizada em cálculos geodésicos.
//! - **Conversão de WGS84 para Web Mercator**: Transforma coordenadas no sistema geográfico WGS84
//!   (EPSG:4326) para o sistema de projeção Web Mercator (EPSG:3857), comumente usado em mapas web.
//!
//! ## Exemplos
//!
//! ### Conversão de Graus para Radianos
//! ```rust
//! use RustSpatial::math::conversion::degrees_to_radians;
//!
//! let radians = degrees_to_radians(45.0);
//! println!("45 graus em radianos: {:.6}", radians);
//! ```
//!
//! ### Conversão de WGS84 para Web Mercator
//! ```rust
//! use RustSpatial::math::conversion::conversion_wgs84_web_mercator;
//! use RustSpatial::geometries::vertex::Vertex;
//!
//! let wgs84_vertex = Vertex::new(10.0, 20.0);
//! let mercator_vertex = conversion_wgs84_web_mercator(&wgs84_vertex);
//!
//! println!("Coordenadas WGS84: {:?}", wgs84_vertex);
//! println!("Coordenadas Web Mercator: {:?}", mercator_vertex);
//! ```
//!
//! ## Notas
//! - As funções deste módulo assumem que as entradas estão corretamente normalizadas, especialmente no caso
//!   de sistemas geográficos como WGS84 (latitude entre -90° e 90° e longitude entre -180° e 180°).
//! - Para sistemas de referência mais complexos, considere integrar bibliotecas especializadas, como PROJ.
//!
//! ## Futuras Expansões
//! Este módulo pode ser estendido para incluir:
//! - Conversões entre outros sistemas de referência, como UTM (Universal Transverse Mercator).
//! - Suporte para sistemas tridimensionais, incluindo altitude.
//! - Conversão de coordenadas inversa (Web Mercator para WGS84).
use crate::{consts::EARTH_RADIUS_METERS, geometries::vertex::Vertex};
use std::f64::consts::PI;

/// Converte graus para radianos.
pub fn degrees_to_radians(degree: f64) -> f64 {
    degree * PI / 180.0
}

/// Converte um ponto de WGS84 (EPSG:4326) para Web Mercator (EPSG:3857).
///
/// # Parâmetros
/// - `vertex`: Ponto no sistema de coordenadas WGS84, com latitude e longitude em graus.
///
/// # Retorno
/// Um novo `Vertex` representando o ponto no sistema de coordenadas Web Mercator.
///
/// # Fórmula
/// ```text
/// x = longitude * R * π / 180
/// y = ln(tan((90 + latitude) * π / 360)) * R
/// ```
/// Onde `R` é o raio da Terra em metros (aproximadamente 6378137.0).
///
/// # Exemplo
/// ```rust
/// use RustSpatial::geometries::vertex::Vertex;
/// use RustSpatial::math::conversion::conversion_wgs84_web_mercator;
///
/// let wgs84_vertex = Vertex::new(0.0, 0.0);
/// let web_mercator_vertex = conversion_wgs84_web_mercator(&wgs84_vertex);
///
/// println!("Web Mercator: {:?}", web_mercator_vertex);
/// ```
pub fn conversion_wgs84_web_mercator(vertex: &Vertex) -> Vertex {
    let x = vertex.longitude * EARTH_RADIUS_METERS * PI / 180.0;

    let y = if vertex.latitude >= 90.0 {
        f64::INFINITY
    } else if vertex.latitude <= -90.0 {
        f64::NEG_INFINITY
    } else {
        ((90.0 + vertex.latitude) * PI / 360.0).tan().ln() * EARTH_RADIUS_METERS
    };

    Vertex::new(y, x)
}
