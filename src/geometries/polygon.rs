use super::{line::Line, vertex::Vertex};

pub struct Polygon<const N: usize> {
    pub lines: [Line; N],
}

impl<const N: usize> Polygon<N> {
    pub fn new(lines: [Line; N]) -> Result<Self, String> {
        // Valida que a linha inicial se integra ao final
    }

    fn last_vertex_is_first(last_vertex: &Vertex, first_vertex: &Vertex) -> bool {
        last_vertex == first_vertex
    }

    fn line_is_valiid(last_line: &Line, current_line: &Line) -> bool {
        last_line.
    }
}
