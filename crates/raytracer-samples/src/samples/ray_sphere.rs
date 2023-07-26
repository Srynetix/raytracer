use raytracer_core::{Camera, Color, Hittable, Ray, Scene, Vec3};
use raytracer_image_renderer::{ppm::PpmRenderer, Renderer};

use crate::assert_eq_ppm;

fn hit_sphere(center: Vec3, radius: f64, ray: Ray) -> bool {
    let oc = ray.origin() - center;
    let a = ray.direction().dot(ray.direction());
    let b = 2.0 * oc.dot(ray.direction());
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

fn ray_color_sphere(ray: Ray, _hittable: &dyn Hittable) -> Color {
    if hit_sphere(Vec3::from_xyz(0.0, 0.0, -1.0), 0.5, ray) {
        return Color::from_rgb(255, 0, 0);
    }

    let norm_direction = ray.direction().normalized();
    let t = 0.5 * (norm_direction.y + 1.0);
    (1.0 - t) * Color::from_rgb(255, 255, 255) + t * Color::from_floating_rgb(0.5, 0.7, 1.0)
}

#[test]
fn run() {
    let mut writer: Vec<u8> = Vec::new();
    let renderer = PpmRenderer::new();
    let scene = Scene::new(Camera::default());
    let image = scene.render(ray_color_sphere);

    renderer.render(&mut writer, &image).unwrap();
    assert_eq_ppm!(String::from_utf8(writer).unwrap(), "./ray_sphere.ppm");
}
