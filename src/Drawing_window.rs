use crate::vertex_rs;
use glium::Surface;

pub fn draw(display: &mut glium::Display) {
    let vertex_shader = vertex_rs::vertex_shader();
    let fragment_shader = vertex_rs::fragment_shader();
    let vertex_buffer = vertex_rs::init(display);
    let indices = vertex_rs::get_indices();
    let program =
        glium::Program::from_source(display, vertex_shader, fragment_shader, None).unwrap();

    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 1.0, 1.0);
    target
        .draw(
            &vertex_buffer,
            indices,
            &program,
            &glium::uniforms::EmptyUniforms,
            &Default::default(),
        )
        .unwrap();
    target.finish().unwrap();
}
