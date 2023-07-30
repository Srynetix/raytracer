use raytracer_core::{primitives::Sphere, Color, Renderer, Scene, Vec3, World};
use raytracer_image_renderer::ppm::PpmRenderer;

use crate::{
    assert_ppm_snapshot, build_context,
    samples::{
        materials::{
            dielectric::DielectricMaterial, lambertian::LambertianMaterial, metal::MetalMaterial,
        },
        shaders::simple_material::SimpleMaterialShader,
    },
};

#[test]
fn test() {
    let mut renderer = PpmRenderer::new(Vec::new());
    let scene = Scene::builder((512, 288).into())
        .with_antialias(16)
        .with_max_depth(16)
        .with_world(
            World::builder()
                .with_collider(
                    Sphere::builder()
                        .with_center(Vec3::new(0.0, -100.5, -1.0))
                        .with_radius(100.0)
                        .with_material(LambertianMaterial::new(Color::from_f64x3(0.8, 0.8, 0.0)))
                        .build(),
                )
                .with_collider(
                    Sphere::builder()
                        .with_center(Vec3::new(0.0, 0.0, -1.0))
                        .with_radius(0.5)
                        .with_material(LambertianMaterial::new(Color::from_f64x3(0.1, 0.2, 0.5)))
                        .build(),
                )
                .with_collider(
                    Sphere::builder()
                        .with_center(Vec3::new(-1.0, 0.0, -1.0))
                        .with_radius(0.5)
                        .with_material(DielectricMaterial::new(1.5))
                        .build(),
                )
                .with_collider(
                    Sphere::builder()
                        .with_center(Vec3::new(-1.0, 0.0, -1.0))
                        .with_radius(-0.4)
                        .with_material(DielectricMaterial::new(1.5))
                        .build(),
                )
                .with_collider(
                    Sphere::builder()
                        .with_center(Vec3::new(1.0, 0.0, -1.0))
                        .with_radius(0.5)
                        .with_material(MetalMaterial::new(Color::from_f64x3(0.8, 0.6, 0.2), 0.0))
                        .build(),
                )
                .build(),
        )
        .build();

    let image = scene.render(build_context(), SimpleMaterialShader);
    renderer.render(&image).unwrap();

    assert_ppm_snapshot(renderer, "hollow_glass_sphere.ppm");
}
