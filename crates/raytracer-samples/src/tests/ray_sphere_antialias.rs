use raytracer_core::{primitives::Sphere, Renderer, Vec3, World};
use raytracer_image_renderer::ppm::PpmRenderer;

use crate::{
    assert_ppm_snapshot, sample_scene_builder,
    samples::shaders::normal_antialias::NormalAntialiasShader,
};

#[test]
fn run() {
    let mut renderer = PpmRenderer::new(Vec::new());
    let mut scene = sample_scene_builder()
        .with_antialias(64)
        .with_world(
            World::builder()
                .with_collider(Box::new(Sphere::new(Vec3::from_xyz(0.0, 0.0, -1.0), 0.5)))
                .build(),
        )
        .build();

    let image = scene.render(NormalAntialiasShader);
    renderer.render(&image).unwrap();

    assert_ppm_snapshot(renderer, "./src/samples/ray_sphere_antialias.ppm");
}
