use crate::vertex_rs::program;
use crate::Drawing_window::draw;
use crate::{texutre_load, vertex_rs};
use glium::glutin;
use glium::glutin::event_loop::EventLoop;

use crate::wall;
pub fn drawing_loop(event_loop: EventLoop<()>, display: glium::Display) {
    let shape = wall::shape(&display);
    let diffuse_texture = texutre_load::diffuse_texture(&display);
    let normal_map = texutre_load::normal_texutre(&display);
    let program = program(&display);
    let start = std::time::Instant::now();
    let indices = vertex_rs::get_indices();

    event_loop.run(move |event, _, control_flow| {
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        draw(&display, start, &shape, &diffuse_texture,&normal_map, &program, &indices);
    });
}
