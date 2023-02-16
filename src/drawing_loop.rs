pub mod DrawlingLoop {

    extern crate glium;
    use glium::backend::glutin::DisplayCreationError;
    use glium::glutin::{self, event_loop::EventLoop};
    use glium::{Display};
    use crate::Drawing_window::draw;
    pub fn drawing_loop(
        event_loop: EventLoop<()>,
        mut display: Result<Display, DisplayCreationError>,
    ) {
        let _next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);

        event_loop.run(move |ev, _, control_flow| {
            let next_frame_time =
                std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);

            *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
            match ev {
                glutin::event::Event::WindowEvent { event, .. } => match event {
                    glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                        return;
                    }
                    _ => return,
                },
                _ => (),
            }

            draw(display.as_mut().unwrap());
        });
    }
}
