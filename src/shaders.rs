pub fn program(display: &glium::Display) -> glium::Program {
    glium::Program::from_source(display, vertex_shader(), fragment_shader(), None).unwrap()
}
fn vertex_shader<'a>() -> &'a str {
    r#"
    #version 330 core

    out vec3 my_attr;

    layout (location = 0) in vec3 position;

    void main()
    {
        my_attr = position;
        gl_Position = vec4(position.x, position.y, position.z, 1.0);
    }
    "#
}

fn fragment_shader<'a>() -> &'a str {
    r#"
    #version 330 core
    
    in vec3 my_attr;
    out vec4 color;

    void main() {
        color = vec4(my_attr*3, 1.0);
    }
    "#
}

pub fn get_indices() -> glium::index::NoIndices {
    glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList)
}
