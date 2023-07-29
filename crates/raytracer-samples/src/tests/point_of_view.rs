use raytracer_core::{primitives::Sphere, Camera, Color, Renderer, Scene, SeedType, Vec3, World};
use raytracer_image_renderer::ppm::PpmRenderer;

use crate::{
    assert_ppm_snapshot,
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
        .with_seed(SeedType::Fixed(1234567890))
        .with_camera(
            Camera::builder()
                .with_position(Vec3::from_xyz(-2.0, 2.0, 1.0))
                .with_vertical_field_of_view(20.0)
                .build(),
        )
        .with_antialias(16)
        .with_max_depth(16)
        .with_world(
            World::builder()
                .with_collider({
                    let mut sphere = Sphere::new(Vec3::from_xyz(0.0, -100.5, -1.0), 100.0);
                    let mut material =
                        LambertianMaterial::new(Color::from_floating_rgb(0.8, 0.8, 0.0));
                    material.set_random_seed(SeedType::Fixed(1234567890));
                    sphere.set_material(Box::new(material));
                    Box::new(sphere)
                })
                .with_collider({
                    let mut sphere = Sphere::new(Vec3::from_xyz(0.0, 0.0, -1.0), 0.5);
                    let mut material =
                        LambertianMaterial::new(Color::from_floating_rgb(0.1, 0.2, 0.5));
                    material.set_random_seed(SeedType::Fixed(1234567890));
                    sphere.set_material(Box::new(material));
                    Box::new(sphere)
                })
                .with_collider({
                    let mut sphere = Sphere::new(Vec3::from_xyz(-1.0, 0.0, -1.0), 0.5);
                    let mut material = DielectricMaterial::new(1.5);
                    material.set_random_seed(SeedType::Fixed(1234567890));
                    sphere.set_material(Box::new(material));
                    Box::new(sphere)
                })
                .with_collider({
                    let mut sphere = Sphere::new(Vec3::from_xyz(1.0, 0.0, -1.0), 0.5);
                    let mut material =
                        MetalMaterial::new(Color::from_floating_rgb(0.8, 0.6, 0.2), 0.0);
                    material.set_random_seed(SeedType::Fixed(1234567890));
                    sphere.set_material(Box::new(material));
                    Box::new(sphere)
                })
                .build(),
        )
        .build();

    let image = scene.render(SimpleMaterialShader);
    renderer.render(&image).unwrap();

    assert_ppm_snapshot(renderer, "point_of_view.ppm");
}
