#[macro_use]
mod drawing_loop;
mod Drawing_window;
mod vertex_rs;
extern crate glium;
extern crate image;
use drawing_loop::drawing_loop;
use glium::glutin;
fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    drawing_loop(event_loop, display);
}
