use raytracer_core::{Camera, Color, Hittable, Ray, Scene, Vec3};
use raytracer_image_renderer::{ppm::PpmRenderer, Renderer};

use crate::assert_eq_ppm;

fn hit_sphere(center: Vec3, radius: f64, ray: Ray) -> f64 {
    let oc = ray.origin() - center;
    let a = ray.direction().length_squared();
    let half_b = oc.dot(ray.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn ray_color_sphere(ray: Ray, _hittable: &dyn Hittable) -> Color {
    let t = hit_sphere(Vec3::from_xyz(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        let normal = (ray.at(t) - Vec3::from_xyz(0.0, 0.0, -1.0)).normalized();
        return Color::from_floating_rgb(
            (normal.x + 1.0) * 0.5,
            (normal.y + 1.0) * 0.5,
            (normal.z + 1.0) * 0.5,
        );
    }

    let norm_direction = ray.direction().normalized();
    let t = 0.5 * (norm_direction.y + 1.0);
    (1.0 - t) * Color::from_rgb(255, 255, 255) + t * Color::from_floating_rgb(0.5, 0.7, 1.0)
}

#[test]
fn run() {
    let mut writer: Vec<u8> = Vec::new();
    let renderer = PpmRenderer::new();
    let mut camera = Camera::default();
    camera.set_width(384.0);

    let scene = Scene::new(camera);
    let image = scene.render(ray_color_sphere);

    renderer.render(&mut writer, &image).unwrap();
    assert_eq_ppm!(
        String::from_utf8(writer).unwrap(),
        "./ray_sphere_normals.ppm"
    );
}
