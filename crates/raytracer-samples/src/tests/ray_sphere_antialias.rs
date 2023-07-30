use raytracer_core::{primitives::Sphere, Renderer, Vec3, World};
use raytracer_image_renderer::ppm::PpmRenderer;

use crate::samples::shaders::normal_collider::NormalColliderShader;

use super::{assert_ppm_snapshot, build_context, sample_scene_builder};

#[test]
fn run() {
    let mut renderer = PpmRenderer::new(Vec::new());
    let scene = sample_scene_builder()
        .with_antialias(16)
        .with_world(
            World::builder()
                .with_collider(
                    Sphere::builder()
                        .with_center(Vec3::new(0.0, 0.0, -1.0))
                        .with_radius(0.5)
                        .build(),
                )
                .build(),
        )
        .build();

    let image = scene.render(build_context(), NormalColliderShader);
    renderer.render(&image).unwrap();

    assert_ppm_snapshot(renderer, "ray_sphere_antialias.ppm");
}
