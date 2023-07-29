use raytracer_core::Renderer;
use raytracer_image_renderer::ppm::PpmRenderer;

use crate::{
    assert_ppm_snapshot, sample_scene_builder, samples::shaders::gradient::GradientShader,
};

#[test]
fn run() {
    let mut renderer = PpmRenderer::new(Vec::new());
    let mut scene = sample_scene_builder().build();
    let image = scene.render(GradientShader);

    renderer.render(&image).unwrap();
    assert_ppm_snapshot(renderer, "./src/samples/ray_gradient.ppm");
}
