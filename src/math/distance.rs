//! Este módulo fornece funções para cálculos matemáticos relacionados a distâncias geográficas.
//!
//! As funções implementadas incluem:
//! - **Fórmula de Haversine**: Calcula distâncias aproximadas entre dois pontos na superfície da Terra,
//!   assumindo-a como uma esfera perfeita. É útil para estimativas rápidas, mas não é altamente precisa.
//! - **Fórmula de Vincenty**: Calcula distâncias geodésicas precisas entre dois pontos na superfície de um elipsoide,
//!   considerando o achatamento da Terra. Ideal para cálculos mais precisos.
//!
//! # Notas
//! - Use a fórmula de Haversine para cálculos rápidos e simples.
//! - Prefira a fórmula de Vincenty quando a precisão for fundamental, especialmente para grandes distâncias ou aplicações científicas.
//! - Este módulo depende de constantes definidas no módulo `consts`, como o raio médio da Terra (`EARTH_RADIUS_KM`) e parâmetros do elipsoide (semi-eixos maior e menor, e achatamento).
//!
//! # Exemplos
//!
//! ```rust
//! use RustSpatial::geometries::vertex::Vertex;
//! use RustSpatial::math::distance::{calculate_earth_radius_distance_haversine, calculate_earth_radius_distance_vincenty};
//!
//! let vertex1 = Vertex::new(10.0, 20.0);
//! let vertex2 = Vertex::new(15.0, 25.0);
//!
//! // Cálculo usando a fórmula de Haversine
//! let haversine_distance = calculate_earth_radius_distance_haversine(&vertex1, &vertex2);
//! println!("Distância aproximada (Haversine): {:.2} km", haversine_distance);
//!
//! // Cálculo usando a fórmula de Vincenty
//! match calculate_earth_radius_distance_vincenty(&vertex1, &vertex2) {
//!     Ok(vincenty_distance) => println!("Distância precisa (Vincenty): {:.2} m", vincenty_distance),
//!     Err(err) => println!("Erro no cálculo de Vincenty: {}", err),
//! }
//! ```
use crate::{
    consts::{EARTH_RADIUS_KM, FLATTENING, SEMI_MAJOR_AXIS_LENGTH, SEMI_MINOR_AXIS_LENGTH},
    geometries::vertex::Vertex,
};

use super::conversion::degrees_to_radians;

/// Calcula a distância entre dois vértices na superfície da Terra utilizando a fórmula de Haversine.
///
/// A fórmula de Haversine é usada para calcular a distância ao longo da superfície de uma esfera,
/// que neste caso é assumida como sendo a Terra com um raio médio fornecido pela constante `EARTH_RADIUS_KM`.
///
/// # Parâmetros
/// - `starting_vertex`: Referência para o vértice inicial, contendo latitude e longitude em graus.
/// - `ending_vertex`: Referência para o vértice final, contendo latitude e longitude em graus.
///
/// # Retorno
/// Retorna a distância entre os dois vértices em quilômetros.
///
/// # Fórmula
/// A fórmula de Haversine calcula a distância como:
/// ```text
/// a = sin²(Δlat / 2) + cos(lat1) * cos(lat2) * sin²(Δlon / 2)
/// c = 2 * atan2(√a, √(1−a))
/// d = R * c
/// ```
/// Onde:
/// - `Δlat` é a diferença de latitude em radianos.
/// - `Δlon` é a diferença de longitude em radianos.
/// - `R` é o raio da Terra (fornecido pela constante `EARTH_RADIUS_KM`).
///
/// # Exemplo
/// ```rust
/// use RustSpatial::geometries::vertex::Vertex;
/// use RustSpatial::math::distance::calculate_earth_radius_distance;
///
/// let vertex1 = Vertex { latitude: 0.0, longitude: 0.0 };
/// let vertex2 = Vertex { latitude: 0.0, longitude: 1.0 };
///
/// let distance = calculate_earth_radius_distance(&vertex1, &vertex2);
/// println!("Distância: {:.2} km", distance); // Saída: ~111.19 km
/// ```
///
/// # Notas
/// - A precisão depende da suposição de que a Terra é uma esfera perfeita.
/// - Não é o calculo mais preciso para isso
pub fn calculate_earth_radius_distance_haversine(
    starting_vertex: &Vertex,
    ending_vertex: &Vertex,
) -> f64 {
    let lat1_rad = degrees_to_radians(starting_vertex.latitude);
    let lat2_rad = degrees_to_radians(ending_vertex.latitude);
    let lon1_rad = degrees_to_radians(starting_vertex.longitude);
    let lon2_rad = degrees_to_radians(ending_vertex.longitude);

    let dist_lat = lat2_rad - lat1_rad;
    let dist_lon = lon2_rad - lon1_rad;

    let a = (dist_lat / 2.0).sin().powi(2)
        + lat1_rad.cos() * lat2_rad.cos() * (dist_lon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    EARTH_RADIUS_KM * c
}

/// Calcula a distância geodésica entre dois pontos na superfície do elipsoide usando a fórmula de Vincenty.
///
/// # Parâmetros
/// - `starting_vertex`: Referência para o vértice inicial, contendo latitude e longitude em graus.
/// - `ending_vertex`: Referência para o vértice final, contendo latitude e longitude em graus.
///
/// # Retorno
/// A distância entre os dois pontos em metros.
fn calculate_earth_radius_distance_vincenty(
    starting_vertex: &Vertex,
    ending_vertex: &Vertex,
) -> Result<f64, &'static str> {
    // Converte coordenadas para radianos
    let lat1_rad = degrees_to_radians(starting_vertex.latitude);
    let lat2_rad = degrees_to_radians(ending_vertex.latitude);
    let lon1_rad = degrees_to_radians(starting_vertex.longitude);
    let lon2_rad = degrees_to_radians(ending_vertex.longitude);

    // Diferença inicial de longitude
    let mut lambda = lon2_rad - lon1_rad;
    let mut lambda_prev;
    let mut iter_limit = 100;

    let u1 = ((1.0 - FLATTENING) * lat1_rad.tan()).atan();
    let u2 = ((1.0 - FLATTENING) * lat2_rad.tan()).atan();

    let sin_u1 = u1.sin();
    let cos_u1 = u1.cos();
    let sin_u2 = u2.sin();
    let cos_u2 = u2.cos();

    let mut cos2_sigma_m;
    let mut sin_sigma;
    let mut cos_sigma;
    let mut sigma;

    let mut sin_lambda;
    let mut cos_lambda;

    loop {
        if iter_limit == 0 {
            return Err("Convergência não atingida");
        }

        sin_lambda = lambda.sin();
        cos_lambda = lambda.cos();

        sin_sigma = ((cos_u2 * sin_lambda).powi(2)
            + (cos_u1 * sin_u2 - sin_u1 * cos_u2 * cos_lambda).powi(2))
        .sqrt();

        if sin_sigma == 0.0 {
            return Ok(0.0); // Pontos coincidentes
        }

        cos_sigma = sin_u1 * sin_u2 + cos_u1 * cos_u2 * cos_lambda;
        sigma = sin_sigma.atan2(cos_sigma);

        let sin_alpha = cos_u1 * cos_u2 * sin_lambda / sin_sigma;
        cos2_sigma_m = 1.0 - sin_alpha.powi(2);

        let c = FLATTENING / 16.0 * cos2_sigma_m * (4.0 + FLATTENING * (4.0 - 3.0 * cos2_sigma_m));

        lambda_prev = lambda;
        lambda = (lon2_rad - lon1_rad)
            + (1.0 - c)
                * FLATTENING
                * sin_alpha
                * (sigma
                    + c * sin_sigma * (cos2_sigma_m + c * cos_sigma * (-1.0 + 2.0 * cos2_sigma_m)));

        if (lambda - lambda_prev).abs() < 1e-12 {
            break;
        }

        iter_limit -= 1;
    }

    let u_squared = cos2_sigma_m
        * (SEMI_MAJOR_AXIS_LENGTH * SEMI_MAJOR_AXIS_LENGTH
            - SEMI_MINOR_AXIS_LENGTH * SEMI_MINOR_AXIS_LENGTH)
        / (SEMI_MINOR_AXIS_LENGTH * SEMI_MINOR_AXIS_LENGTH);
    let a_term = 1.0
        + u_squared / 16384.0
            * (4096.0 + u_squared * (-768.0 + u_squared * (320.0 - 175.0 * u_squared)));
    let b_term =
        u_squared / 1024.0 * (256.0 + u_squared * (-128.0 + u_squared * (74.0 - 47.0 * u_squared)));

    let delta_sigma = b_term
        * sin_sigma
        * (cos2_sigma_m
            + b_term / 4.0
                * (cos_sigma * (-1.0 + 2.0 * cos2_sigma_m.powi(2))
                    - b_term / 6.0
                        * cos2_sigma_m
                        * (-3.0 + 4.0 * sin_sigma.powi(2))
                        * (-3.0 + 4.0 * cos2_sigma_m.powi(2))));

    let distance = SEMI_MINOR_AXIS_LENGTH * a_term * (sigma - delta_sigma);

    Ok(distance)
}
