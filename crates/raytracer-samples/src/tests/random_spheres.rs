use raytracer_core::{Renderer, SeedType};
use raytracer_image_renderer::ppm::PpmRenderer;

use crate::samples::{
    scenes::random_spheres::random_spheres_scene, shaders::simple_material::SimpleMaterialShader,
};

use super::{assert_ppm_snapshot, build_context};

fn fixed_seed() -> SeedType {
    SeedType::Fixed(1234567890)
}

#[test]
fn test() {
    let mut renderer = PpmRenderer::new(Vec::new());
    let mut scene = random_spheres_scene(fixed_seed());
    scene.set_scale(0.25);
    scene.set_antialias(25);
    scene.set_max_depth(25);

    let image = scene.render(build_context(), SimpleMaterialShader);
    renderer.render(&image).unwrap();

    assert_ppm_snapshot(renderer, "random_spheres.ppm");
}
