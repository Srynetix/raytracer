use raytracer_core::Renderer;
use raytracer_image_renderer::ppm::PpmRenderer;

use crate::{
    assert_ppm_snapshot, build_context, sample_scene_builder,
    samples::shaders::gradient::GradientShader,
};

#[test]
fn run() {
    let mut renderer = PpmRenderer::new(Vec::new());
    let scene = sample_scene_builder().build();
    let mut ctx = build_context();
    let image = scene.render(&mut ctx, GradientShader);

    renderer.render(&image).unwrap();
    assert_ppm_snapshot(renderer, "ray_gradient.ppm");
}
