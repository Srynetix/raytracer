use raytracer_core::Renderer;
use raytracer_image_renderer::ppm::PpmRenderer;

use crate::{
    assert_ppm_snapshot, sample_scene_builder, samples::shaders::simple_sphere::SimpleSphereShader,
};

#[test]
fn run() {
    let mut renderer = PpmRenderer::new(Vec::new());
    let scene = sample_scene_builder().build();
    let image = scene.render(SimpleSphereShader);

    renderer.render(&image).unwrap();
    assert_ppm_snapshot(renderer, "ray_sphere.ppm");
}
