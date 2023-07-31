use raytracer_core::Renderer;
use raytracer_image_renderer::ppm::PpmRenderer;

use crate::samples::shaders::gradient::GradientShader;

use super::{assert_ppm_snapshot, build_context, sample_scene_builder};

#[test]
fn run() {
    let mut renderer = PpmRenderer::new(Vec::new());
    let scene = sample_scene_builder().build();
    let image = scene.render(build_context(), GradientShader);

    renderer.render(&image).unwrap();
    assert_ppm_snapshot(renderer, "ray_gradient.ppm");
}
