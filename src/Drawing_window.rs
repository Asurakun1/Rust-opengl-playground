use crate::{teapot, vertex_rs};
use glium::Surface;

pub fn draw(display: &glium::Display, mut t: f32, texture: &glium::texture::SrgbTexture2d) -> f32 {
    let positions = glium::VertexBuffer::new(display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(
        display,
        glium::index::PrimitiveType::TrianglesList,
        &teapot::INDICES,
    )
    .unwrap();

    let program = vertex_rs::program(display);
    let vertex_buffer = vertex_rs::init(display);
    let indix = vertex_rs::get_indices();

    t += 0.0032;

    let uniforms = uniform! {
        matrix: [
        [0.01, 0.0, 0.0, 0.0],
        [0.0, 0.01, 0.0, 0.0],
        [0.0, 0.0, 0.01, 0.0],
        [0.0, 0.0, 0.0, 1.0f32]
    ],
        tex: texture
    };

    let matrix = [
        [0.01, 0.0, 0.0, 0.0],
        [0.0, 0.01, 0.0, 0.0],
        [0.0, 0.0, 0.01, 0.0],
        [0.0, 0.0, 0.0, 1.0f32],
    ];

    let light = [-1.0, 0.4, 0.9f32];

    let mut target = display.draw();
    target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        ..Default::default()
    };

    target
        .draw(
            (&positions, &normals),
            &indices,
            &program,
            &uniform! {matrix: matrix, u_light: light},
            &params,
        )
        .unwrap();

    target.finish().unwrap();
    t
}
