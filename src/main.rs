#[macro_use]
extern crate glium;
mod wall;
mod drawing_loop;
mod Drawing_window;
mod texutre_load;
mod vertex_rs;
use drawing_loop::drawing_loop;

fn main() {
    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    drawing_loop(event_loop, display);

}
