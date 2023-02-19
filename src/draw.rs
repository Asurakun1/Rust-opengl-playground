use crate::shape::Vertex;
use glium::{Surface, VertexBuffer};
pub fn draw(
    display: &glium::Display,
    shape: &VertexBuffer<Vertex>,
    program: &glium::Program,
    indices: &glium::index::NoIndices,
) {
    let uniforms = uniform! {
        matrix: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ]

    };

    let mut target = display.draw();

    target.clear_color(0.2, 0.1, 0.4, 1.0);

    target
        .draw(
            shape,
            indices,
            program,
            &uniforms,
            &glium::DrawParameters::default(),
        )
        .unwrap();

    target.finish().unwrap();
}
