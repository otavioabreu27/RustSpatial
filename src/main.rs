use geometries::vertex::Vertex;

mod geometries;

fn main() {
    let vertex = Vertex::new(63.7, 12.2);

    println!("{}", vertex);
}
