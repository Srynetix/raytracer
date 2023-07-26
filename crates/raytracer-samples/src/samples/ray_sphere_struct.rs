use raytracer_core::{primitives::Sphere, Camera, Color, Hittable, Ray, Scene, Vec3};
use raytracer_image_renderer::{ppm::PpmRenderer, Renderer};

use crate::assert_eq_ppm;

fn ray_color(ray: Ray, hittable: &dyn Hittable) -> Color {
    if let Some(record) = hittable.hit(&ray, 0.0, f64::MAX) {
        return Color::from_floating_rgb(
            (record.normal.x + 1.0) * 0.5,
            (record.normal.y + 1.0) * 0.5,
            (record.normal.z + 1.0) * 0.5,
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

    let mut scene = Scene::new(camera);
    scene.add_hittable(Box::new(Sphere::new(Vec3::from_xyz(0.0, 0.0, -1.0), 0.5)));

    let image = scene.render(ray_color);

    renderer.render(&mut writer, &image).unwrap();
    assert_eq_ppm!(
        String::from_utf8(writer).unwrap(),
        "./ray_sphere_struct.ppm"
    );
}
