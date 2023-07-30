use raytracer_core::{Renderer, SeedType};
use raytracer_image_renderer::ppm::PpmRenderer;

use crate::{
    assert_ppm_snapshot, build_context,
    samples::{
        scenes::random_spheres::random_spheres_scene,
        shaders::simple_material::SimpleMaterialShader,
    },
};

fn fixed_seed() -> SeedType {
    SeedType::Fixed(1234567890)
}

#[test]
fn test() {
    let mut renderer = PpmRenderer::new(Vec::new());
    let mut scene = random_spheres_scene(fixed_seed());
    scene.set_size((1200 / 4, 800 / 4).into());
    scene.set_antialias(50);
    scene.set_max_depth(50);

    let mut ctx = build_context();
    let image = scene.render(&mut ctx, SimpleMaterialShader);
    renderer.render(&image).unwrap();

    assert_ppm_snapshot(renderer, "random_spheres.ppm");
}
