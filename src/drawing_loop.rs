use crate::Drawing_window::draw;
use crate::texutre_load::texture;
use glium::glutin;
use glium::glutin::event_loop::EventLoop;
pub fn drawing_loop(event_loop: EventLoop<()>, display: glium::Display) {
    let mut t: f32 = -0.5;

    let texture = texture(&display);

    event_loop.run(move |ev, _, control_flow| {
        match ev {
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

        t = draw(&display, t, &texture);
    });
}
