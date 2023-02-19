#[derive(Clone, Copy)]
pub struct Vertex {
    position: [f32; 3],
}

implement_vertex!(Vertex, position);

pub fn triangle(display: &glium::Display) -> glium::VertexBuffer<Vertex> {
    let v1 = Vertex {
        position: [-0.5, -0.5, 0.0],
    };
    let v2 = Vertex {
        position: [0.0, 0.5, 0.0],
    };
    let v3 = Vertex {
        position: [0.5, -0.5, 0.0],
    };

    let shape = vec![v1, v2, v3];

    glium::VertexBuffer::new(display, &shape).unwrap()
}
