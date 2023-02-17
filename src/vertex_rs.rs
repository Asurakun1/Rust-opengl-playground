use glium::implement_vertex;

#[derive(Clone, Copy)]
pub struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

pub fn program(display: &glium::Display) -> glium::Program {
    glium::Program::from_source(display, vertex_shader(), fragment_shader(), None).unwrap()
}

fn vertex_shader<'a>() -> &'a str {
    r#"
    #version 140

    in vec2 position;
    in vec2 tex_coords;
    out vec2 v_tex_coords;
    
    uniform mat4 matrix;
    
    void main() {
        v_tex_coords = tex_coords;
        gl_Position = matrix * vec4(position, 0.0, 1.0);
    }
    "#
}

fn fragment_shader<'a>() -> &'a str {
    r#"
    #version 140

    in vec2 v_tex_coords;
    out vec4 color;
    
    uniform sampler2D tex;
    
    void main() {
        color = texture(tex, v_tex_coords);
    }
    "#
}

pub fn get_indices() -> glium::index::NoIndices {
    glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList)
}

pub fn init(display: &glium::Display) -> glium::VertexBuffer<Vertex> {
    implement_vertex!(Vertex, position, tex_coords);

    let vertex1 = Vertex {
        position: [-0.5, -0.5],
        tex_coords: [0.0, 0.0],
    };
    let vertex2 = Vertex {
        position: [0.0, 0.5],
        tex_coords: [0.0, 1.0]
    };
    let vertex3 = Vertex {
        position: [0.5, -0.25],
        tex_coords: [1.0, 0.0]
    };

    let shape = vec![vertex1, vertex2, vertex3];

    glium::VertexBuffer::new(display, &shape).unwrap()
}
