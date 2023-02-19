use std::io::Cursor;
use image::{ImageBuffer, Rgba};

pub fn diffuse_texture(display: &glium::Display) -> glium::texture::SrgbTexture2d {
    let image = get_diffuse_image();
    let image_dimensions = image.dimensions();
    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

    glium::texture::SrgbTexture2d::new(display, image).unwrap()
}

pub fn normal_texutre(display: &glium::Display) -> glium::texture::SrgbTexture2d {
    let image = get_normal_image();
    let image_dimensions = image.dimensions();
    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

    glium::texture::SrgbTexture2d::new(display, image).unwrap()
}

fn get_normal_image() -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    image::load(
        Cursor::new(&include_bytes!("../assets/tuto-14-normal.png")),
        image::ImageFormat::Png,
    )
    .unwrap()
    .to_rgba8()
}

fn get_diffuse_image() -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    image::load(
        Cursor::new(&include_bytes!("../assets/tuto-14-diffuse.jpg")),
        image::ImageFormat::Jpeg,
    )
    .unwrap()
    .to_rgba8()
}