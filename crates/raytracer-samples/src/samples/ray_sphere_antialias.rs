use raytracer_core::{
    primitives::Sphere, Camera, Color, Hittable, Ray, RayColor, Renderer, Scene, Vec3, World,
};
use raytracer_image_renderer::ppm::PpmRenderer;

use crate::{assert_ppm_snapshot, sample_scene_builder};

struct RayScene;

impl RayColor for RayScene {
    fn ray_color(&self, ray: &Ray, hittable: &dyn Hittable) -> Color {
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
}

#[test]
fn run() {
    let mut renderer = PpmRenderer::new(Vec::new());
    let mut scene = sample_scene_builder()
        .with_antialias(64)
        .with_world(
            World::builder()
                .with_hittable(Box::new(Sphere::new(Vec3::from_xyz(0.0, 0.0, -1.0), 0.5)))
                .build(),
        )
        .build();

    let image = scene.render(RayScene);
    renderer.render(&image).unwrap();

    assert_ppm_snapshot(renderer, "./src/samples/ray_sphere_antialias.ppm");
}
