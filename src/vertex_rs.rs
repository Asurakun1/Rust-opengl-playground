use glium::{implement_vertex, VertexBuffer};

#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 2],
}

pub fn init(display: &glium::Display) -> VertexBuffer<Vertex> {
    let vertex1 = Vertex {
        position: [-0.5, -0.5],
    };
    let vertex2 = Vertex {
        position: [0.0, 0.5],
    };
    let vertex3 = Vertex {
        position: [0.5, -0.25],
    };
    implement_vertex!(Vertex, position);

    let shape = vec![vertex1, vertex2, vertex3];

    VertexBuffer::new(display, &shape).unwrap()
}

pub fn vertex_shader<'a>() -> &'a str {
    r#"
    #version 140

    in vec2 position;

    void main(){
        gl_Position = vec4(position, 0.0, 1.0);
    }
    "#
}

pub fn fragment_shader<'a>() -> &'a str {
    r#"
    #version 140

    out vec4 color;

    void main(){
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }

    "#
}

pub fn get_indices () -> glium::index::NoIndices{
    glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList)
}