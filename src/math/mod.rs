//! Este módulo fornece funcionalidades matemáticas para operações relacionadas a geografia e geodésia.
//!
//! O módulo `math` está dividido em submódulos que tratam de diferentes aspectos matemáticos:
//!
//! ## Submódulos
//!
//! - [`conversion`]: Contém funções para conversões comuns, como graus para radianos e vice-versa,
//!   necessárias para cálculos geográficos.
//! - [`distance`]: Fornece algoritmos para calcular distâncias entre dois pontos na superfície da Terra,
//!   usando fórmulas como Haversine e Vincenty.
//!
//! ## Objetivo
//! Este módulo é projetado para lidar com cálculos geográficos de forma eficiente e modular,
//! permitindo fácil reutilização e extensão de suas funcionalidades.
//!
//! ## Exemplos
//!
//! ```rust
//! use RustSpatial::math::conversion::degrees_to_radians;
//! use RustSpatial::math::distance::{calculate_earth_radius_distance_haversine, calculate_earth_radius_distance_vincenty};
//! use RustSpatial::geometries::vertex::Vertex;
//!
//! // Conversão de graus para radianos
//! let radians = degrees_to_radians(45.0);
//! println!("45 graus em radianos: {:.6}", radians);
//!
//! // Cálculo de distância usando Haversine
//! let vertex1 = Vertex::new(10.0, 20.0);
//! let vertex2 = Vertex::new(15.0, 25.0);
//! let haversine_distance = calculate_earth_radius_distance_haversine(&vertex1, &vertex2);
//! println!("Distância aproximada (Haversine): {:.2} km", haversine_distance);
//!
//! // Cálculo de distância usando Vincenty
//! match calculate_earth_radius_distance_vincenty(&vertex1, &vertex2) {
//!     Ok(vincenty_distance) => println!("Distância precisa (Vincenty): {:.2} m", vincenty_distance),
//!     Err(err) => println!("Erro no cálculo de Vincenty: {}", err),
//! }
//! ```
//!
//! ## Notas
//! - Este módulo é altamente modular, permitindo o uso de conversões e cálculos de distância de forma independente.
//! - A precisão dos cálculos depende do uso correto das fórmulas e das constantes geodésicas fornecidas.

pub mod conversion;
pub mod distance;
