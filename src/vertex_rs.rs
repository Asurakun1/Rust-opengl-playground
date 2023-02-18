#[derive(Clone, Copy)]
pub struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

pub fn program(display: &glium::Display) -> glium::Program {
    glium::Program::from_source(display, vertex_shader(), fragment_shader(), None).unwrap()
}

fn vertex_shader<'a>() -> &'a str {
    r#"
    #version 150

    in vec3 position;
    in vec3 normal;

    out vec3 v_normal;

    uniform mat4 matrix;

    void main(){
        v_normal = transpose(inverse(mat3(matrix))) * normal;
        gl_Position = matrix * vec4(position, 1.0);
    }

    "#
}

fn fragment_shader<'a>() -> &'a str {
    r#"
    #version 140

    in vec3 v_normal;
    out vec4 color;
    uniform vec3 u_light;
    
    void main() {
        float brightness = dot(normalize(v_normal), normalize(u_light));
        vec3 dark_color = vec3(0.6, 0.0, 0.0);
        vec3 regular_color = vec3(1.0, 0.0, 0.0);
        color = vec4(mix(dark_color, regular_color, brightness), 1.0);
    }
    "#
}

pub fn get_indices() -> glium::index::NoIndices {
    glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList)
}

pub fn init(display: &glium::Display) -> glium::VertexBuffer<Vertex> {
    let vertex1 = Vertex {
        position: [-0.5, -0.5],
        tex_coords: [0.0, 0.0],
    };
    let vertex2 = Vertex {
        position: [0.0, 0.5],
        tex_coords: [0.0, 1.0],
    };
    let vertex3 = Vertex {
        position: [0.5, -0.25],
        tex_coords: [1.0, 0.0],
    };

    let shape = vec![vertex1, vertex2, vertex3];

    glium::VertexBuffer::new(display, &shape).unwrap()
}
