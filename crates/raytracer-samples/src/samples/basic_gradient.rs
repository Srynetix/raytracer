use raytracer_core::{Color, Image};
use raytracer_image_renderer::{ppm::PpmRenderer, Renderer};

use crate::assert_eq_ppm;

fn source_gradient_image() -> Image {
    let width = 256;
    let height = 256;
    let mut pixels = vec![];

    for y in 0..height {
        for x in 0..width {
            let r = x as f64 / (width - 1) as f64;
            let g = (height - y) as f64 / (height - 1) as f64;
            let b = 0.25;

            pixels.push(Color::from_floating_rgb(r, g, b));
        }
    }

    Image::from_pixels(width, pixels)
}

#[test]
fn run() {
    let mut writer: Vec<u8> = Vec::new();
    let renderer = PpmRenderer::new();
    let image = source_gradient_image();

    renderer.render(&mut writer, &image).unwrap();
    assert_eq_ppm!(String::from_utf8(writer).unwrap(), "./basic_gradient.ppm");
}
