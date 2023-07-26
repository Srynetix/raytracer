use std::path::Path;

use raytracer_core::{Color, Image, Ray, Vec3};

use crate::{ppm::PpmRenderer, renderer::Renderer};

macro_rules! assert_eq_ppm {
    ($content: expr, $target: literal) => {
        assert_eq!($content, include_str!($target))
    };
}

#[allow(unused)]
fn write_image<P: AsRef<Path>>(data: Vec<u8>, path: P) {
    std::fs::write(path, String::from_utf8(data).unwrap()).unwrap()
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

fn source_gradient_image() -> Image {
    let width = 256;
    let height = 256;
    let mut pixels = vec![];

    for y in 0..height {
        for x in 0..width {
            let r = x as f32 / (width - 1) as f32;
            let g = (height - y) as f32 / (height - 1) as f32;
            let b = 0.25;

            pixels.push(Color::from_floating_rgb(r, g, b));
        }
    }

    Image::from_pixels(width, pixels)
}

fn source_ray_color(ray: &Ray) -> Color {
    let norm_direction = ray.direction().normalized();
    let t = 0.5 * (norm_direction.y + 1.0);
    return (1.0 - t) * Color::from_rgb(255, 255, 255) + t * Color::from_floating_rgb(0.5, 0.7, 1.0);
}

fn source_ray_image() -> Image {
    let aspect_ratio = 16.0 / 9.0;
    let width = 256;
    let height = (width as f64 / aspect_ratio) as u32;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::zero();
    let horizontal = Vec3::from_xyz(viewport_width, 0.0, 0.0);
    let vertical = Vec3::from_xyz(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::from_xyz(0.0, 0.0, focal_length);

    let mut pixels = vec![];

    for y in (1..height + 1).rev() {
        for x in 0..width {
            let u = x as f64 / (width - 1) as f64;
            let v = y as f64 / (height - 1) as f64;

            let ray = Ray::from_points(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            pixels.push(source_ray_color(&ray));
        }
    }

    Image::from_pixels(width, pixels)
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

#[test]
fn test_ppm_output_gradient() {
    let mut writer: Vec<u8> = Vec::new();
    let renderer = PpmRenderer::new();
    let image = source_gradient_image();

    renderer.render(&mut writer, &image).unwrap();
    assert_eq_ppm!(String::from_utf8(writer).unwrap(), "./gradient.ppm");
}

#[test]
fn test_ppm_output_ray() {
    let mut writer: Vec<u8> = Vec::new();
    let renderer = PpmRenderer::new();
    let image = source_ray_image();

    renderer.render(&mut writer, &image).unwrap();
    assert_eq_ppm!(String::from_utf8(writer).unwrap(), "./ray.ppm");
}
