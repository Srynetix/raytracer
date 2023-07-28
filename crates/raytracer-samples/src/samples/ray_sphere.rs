use raytracer_core::{Camera, Color, Hittable, Ray, RayColor, Renderer, Scene, Vec3};
use raytracer_image_renderer::ppm::PpmRenderer;

use crate::{assert_ppm_snapshot, sample_scene_builder};

fn hit_sphere(center: Vec3, radius: f64, ray: &Ray) -> bool {
    let oc = ray.origin() - center;
    let a = ray.direction().dot(ray.direction());
    let b = 2.0 * oc.dot(ray.direction());
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

struct RayScene;

impl RayColor for RayScene {
    fn ray_color(&self, ray: &Ray, _hittable: &dyn Hittable) -> Color {
        if hit_sphere(Vec3::from_xyz(0.0, 0.0, -1.0), 0.5, ray) {
            return Color::from_rgb(255, 0, 0);
        }

        let norm_direction = ray.direction().normalized();
        let t = 0.5 * (norm_direction.y + 1.0);
        (1.0 - t) * Color::from_rgb(255, 255, 255) + t * Color::from_floating_rgb(0.5, 0.7, 1.0)
    }
}

#[test]
fn run() {
    let mut renderer = PpmRenderer::new(Vec::new());
    let mut scene = sample_scene_builder().build();
    let image = scene.render(RayScene);

    renderer.render(&image).unwrap();
    assert_ppm_snapshot(renderer, "./src/samples/ray_sphere.ppm");
}
