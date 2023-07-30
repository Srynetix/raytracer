use crate::{
    assert_ppm_snapshot, build_context, sample_scene_builder,
    samples::shaders::diffuse::DiffuseShader,
};
use raytracer_core::{primitives::Sphere, Renderer, Vec3, World};
use raytracer_image_renderer::ppm::PpmRenderer;

#[test]
fn run() {
    let mut renderer = PpmRenderer::new(Vec::new());
    let scene = sample_scene_builder()
        .with_antialias(16)
        .with_max_depth(8)
        .with_world(
            World::builder()
                .with_collider(
                    Sphere::builder()
                        .with_center(Vec3::new(0.0, 0.0, -1.0))
                        .with_radius(0.5)
                        .build(),
                )
                .with_collider(
                    Sphere::builder()
                        .with_center(Vec3::new(0.0, -100.5, -1.0))
                        .with_radius(100.0)
                        .build(),
                )
                .build(),
        )
        .build();

    let image = scene.render(build_context(), DiffuseShader::new());
    renderer.render(&image).unwrap();

    assert_ppm_snapshot(renderer, "ray_diffuse.ppm");
}
