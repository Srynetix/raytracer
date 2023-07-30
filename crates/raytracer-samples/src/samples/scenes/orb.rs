//! Orb scene.

use std::f64::consts::PI;

use raytracer_core::{
    primitives::Sphere, rand::Rng, Camera, RngWrapper, Scene, SeedType, Vec3, World,
};

use crate::samples::materials::{
    dielectric::DielectricMaterial, lambertian::LambertianMaterial, metal::MetalMaterial,
};

fn build_world(seed: SeedType) -> World {
    let mut world = World::new();
    let mut rng = RngWrapper::new(seed);

    world.add_collider(
        Sphere::builder()
            .with_center(Vec3::new(0.0, 0.0, -1.0))
            .with_radius(0.25)
            .with_material(MetalMaterial::new(rng.gen(), rng.gen()))
            .build(),
    );

    let points_count = 32;
    for p in 0..points_count {
        let amount = ((2.0 * PI) / points_count as f64) * p as f64;
        let x_amount = amount.cos() * 0.5;
        let y_amount = amount.sin() * 0.5;

        if p % 4 == 0 {
            world.add_collider(
                Sphere::builder()
                    .with_center(Vec3::new(x_amount, y_amount, -1.0))
                    .with_radius(0.04)
                    .with_material(DielectricMaterial::new(1.5))
                    .build(),
            );
        } else {
            world.add_collider(
                Sphere::builder()
                    .with_center(Vec3::new(x_amount, y_amount, -1.0))
                    .with_radius(0.04)
                    .with_material(LambertianMaterial::new(rng.gen()))
                    .build(),
            );
        }
    }

    world
}

/// Build the orb scene.
pub fn orb_scene(seed: SeedType) -> Scene {
    Scene::builder((1280, 720).into())
        .with_camera(
            Camera::builder()
                .with_aspect_ratio(16.0 / 9.0)
                .with_position(Vec3::new(0.0, 0.0, -0.1))
                .build(),
        )
        .with_antialias(100)
        .with_max_depth(50)
        .with_world(build_world(seed))
        .build()
}
