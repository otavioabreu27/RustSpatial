use geometries::{line::Line, vertex::Vertex};

mod geometries;
mod consts;

fn main() {
    let vertex1 = Vertex::new(63.7, 12.2);
    let vertex2 = Vertex::new(21.2, 66.6);

    let line = Line::new(vertex1, vertex2);

    println!("{}", line.size());
}
