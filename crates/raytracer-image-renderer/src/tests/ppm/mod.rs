use raytracer_core::{Color, Image};

use crate::{ppm::PpmRenderer, renderer::Renderer};

macro_rules! assert_eq_ppm {
    ($content: expr, $target: literal) => {
        assert_eq!($content, include_str!($target))
    };
}

fn source_image(width: u32) -> Image {
    Image::from_pixels(
        width,
        vec![
            Color::from_rgb(0, 0, 0),
            Color::from_rgb(0, 0, 255),
            Color::from_rgb(0, 255, 0),
            Color::from_rgb(0, 255, 255),
            Color::from_rgb(255, 0, 0),
            Color::from_rgb(255, 0, 255),
            Color::from_rgb(255, 255, 0),
            Color::from_rgb(255, 255, 255),
        ],
    )
}

#[test]
fn test_ppm_output_2x4() {
    let mut writer: Vec<u8> = Vec::new();
    let renderer = PpmRenderer::new();
    let image = source_image(2);

    renderer.render(&mut writer, &image).unwrap();
    assert_eq_ppm!(String::from_utf8(writer).unwrap(), "./2x4.ppm");
}

#[test]
fn test_ppm_output_4x2() {
    let mut writer: Vec<u8> = Vec::new();
    let renderer = PpmRenderer::new();
    let image = source_image(4);

    renderer.render(&mut writer, &image).unwrap();
    assert_eq_ppm!(String::from_utf8(writer).unwrap(), "./4x2.ppm");
}
