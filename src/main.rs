#[macro_use]
extern crate glium;

mod window_display;
mod draw;
mod shaders;
mod shape;

use glium::glutin;
use window_display::window_display;

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Hello window repitition")
        .with_inner_size(glutin::dpi::LogicalSize::new(800.0, 600.0));
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    window_display(event_loop, display);
}
