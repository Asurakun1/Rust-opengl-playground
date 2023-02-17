use crate::vertex_rs::{get_indices, init, program};
use glium::{uniform, Surface};
use glium;
pub fn draw(display: &glium::Display, mut t: f32, texture: &glium::texture::SrgbTexture2d) -> f32 {

    let vertex_buffer = init(&display);
    let indices = get_indices();
    let program = program(display);

    t += 0.0032;

    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 1.0, 1.0);

    let uniforms = uniform! {
        matrix: [
            [t.cos(), t.sin(), t.cos(), 0.0],
            [t.sin(), t.cos(), t.sin(), 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ],
        tex: texture,
    };

    target
        .draw(
            &vertex_buffer,
            &indices,
            &program,
            &uniforms,
            &Default::default(),
        )
        .unwrap();

    target.finish().unwrap();

    t
}
