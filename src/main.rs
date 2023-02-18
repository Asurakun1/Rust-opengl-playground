#[macro_use]

extern crate glium;

mod drawing_loop;
mod Drawing_window;
mod vertex_rs;
mod texutre_load;
mod teapot;

use glium::glutin;
use drawing_loop::drawing_loop;
fn main (){

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    
    drawing_loop(event_loop, display);
}