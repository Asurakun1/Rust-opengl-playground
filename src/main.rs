#[macro_use]
mod drawing_loop;
mod vertex_rs;
mod Drawing_window;
extern crate glium;
use glium::glutin;
use drawing_loop::DrawlingLoop::drawing_loop;
fn main() {
    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop);

    drawing_loop(event_loop, display);
}
