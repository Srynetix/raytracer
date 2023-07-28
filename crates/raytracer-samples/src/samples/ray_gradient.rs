use raytracer_core::{Camera, Color, Hittable, Ray, RayColor, Renderer, Scene, SeedType};
use raytracer_image_renderer::ppm::PpmRenderer;

use crate::{assert_ppm_snapshot, sample_scene_builder};

struct RayScene;

impl RayColor for RayScene {
    fn ray_color(&self, ray: &Ray, _hittable: &dyn Hittable) -> Color {
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
    assert_ppm_snapshot(renderer, "./src/samples/ray_gradient.ppm");
}
