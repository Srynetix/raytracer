use raytracer_core::{Camera, Color, Hittable, Ray, Scene};
use raytracer_image_renderer::{ppm::PpmRenderer, Renderer};

use crate::assert_eq_ppm;

fn ray_color_gradient(ray: Ray, _hittable: &dyn Hittable) -> Color {
    let norm_direction = ray.direction().normalized();
    let t = 0.5 * (norm_direction.y + 1.0);
    (1.0 - t) * Color::from_rgb(255, 255, 255) + t * Color::from_floating_rgb(0.5, 0.7, 1.0)
}

#[test]
fn run() {
    let mut writer: Vec<u8> = Vec::new();
    let renderer = PpmRenderer::new();
    let scene = Scene::new(Camera::default());
    let image = scene.render(ray_color_gradient);

    renderer.render(&mut writer, &image).unwrap();
    assert_eq_ppm!(String::from_utf8(writer).unwrap(), "./ray_gradient.ppm");
}
