use crate::{assert_ppm_snapshot, sample_scene_builder, samples::shaders::diffuse::DiffuseShader};
use raytracer_core::{primitives::Sphere, Renderer, SeedType, Vec3, World};
use raytracer_image_renderer::ppm::PpmRenderer;

#[test]
fn run() {
    let mut renderer = PpmRenderer::new(Vec::new());
    let scene = sample_scene_builder()
        .with_antialias(16)
        .with_max_depth(8)
        .with_world(
            World::builder()
                .with_collider(Box::new(Sphere::new(Vec3::from_xyz(0.0, 0.0, -1.0), 0.5)))
                .with_collider(Box::new(Sphere::new(
                    Vec3::from_xyz(0.0, -100.5, -1.0),
                    100.0,
                )))
                .build(),
        )
        .build();

    let image = scene.render(DiffuseShader::new(SeedType::Fixed(1234567890)));
    renderer.render(&image).unwrap();

    assert_ppm_snapshot(renderer, "ray_diffuse.ppm");
}
