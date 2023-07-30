use raytracer_core::{primitives::Sphere, Camera, Color, Renderer, Scene, Vec3, World};
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
                .with_collider(Box::new(
                    Sphere::builder()
                        .with_center(Vec3::from_xyz(0.0, -100.5, -1.0))
                        .with_radius(100.0)
                        .with_material(Box::new(LambertianMaterial::new(Color::from_floating_rgb(
                            0.8, 0.8, 0.0,
                        ))))
                        .build(),
                ))
                .with_collider(Box::new(
                    Sphere::builder()
                        .with_center(Vec3::from_xyz(0.0, 0.0, -1.0))
                        .with_radius(0.5)
                        .with_material(Box::new(LambertianMaterial::new(Color::from_floating_rgb(
                            0.1, 0.2, 0.5,
                        ))))
                        .build(),
                ))
                .with_collider(Box::new(
                    Sphere::builder()
                        .with_center(Vec3::from_xyz(-1.0, 0.0, -1.0))
                        .with_radius(0.5)
                        .with_material(Box::new(DielectricMaterial::new(1.5)))
                        .build(),
                ))
                .with_collider(Box::new(
                    Sphere::builder()
                        .with_center(Vec3::from_xyz(1.0, 0.0, -1.0))
                        .with_radius(0.5)
                        .with_material(Box::new(MetalMaterial::new(
                            Color::from_floating_rgb(0.8, 0.6, 0.2),
                            0.0,
                        )))
                        .build(),
                ))
                .build(),
        )
        .build();

    let mut ctx = build_context();
    let image = scene.render(&mut ctx, SimpleMaterialShader);
    renderer.render(&image).unwrap();

    assert_ppm_snapshot(renderer, "point_of_view.ppm");
}
