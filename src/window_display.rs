use glium::{
    glutin::{self, event_loop::EventLoop},
    Surface,
};
use crate::draw::draw;
use crate::shaders as sh;
use crate::shape::triangle;

pub fn window_display(event_loop: EventLoop<()>, display: glium::Display) {

    let indices = sh::get_indices();
    let program = sh::program(&display);
    let triangle = triangle(&display);

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

        let frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(frame_time);

        draw(&display, &triangle, &program, &indices);
        
    });
}
