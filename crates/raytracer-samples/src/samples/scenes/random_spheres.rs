//! The random spheres sample from the <https://raytracing.github.io/books/RayTracingInOneWeekend.html> book.

use raytracer_core::{
    primitives::Sphere, rand::Rng, Camera, Color, RngWrapper, Scene, SeedType, Vec3, World,
};

use crate::samples::materials::{
    dielectric::DielectricMaterial, lambertian::LambertianMaterial, metal::MetalMaterial,
};

fn rand_f64<R: Rng>(rng: &mut R) -> f64 {
    rng.gen_range(0.0..=1.0)
}

fn random_world(seed: SeedType) -> World {
    let mut world = World::new();
    let mut rng = RngWrapper::new(seed);

    world.add_collider(
        Sphere::builder()
            .with_center(Vec3::new(0.0, -1000.0, 0.0))
            .with_radius(1000.0)
            .with_material(LambertianMaterial::new(Color::from_f64x3(0.5, 0.5, 0.5)))
            .build(),
    );

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = rand_f64(&mut rng);
            let center = Vec3::new(
                a as f64 + 0.9 * rand_f64(&mut rng),
                0.2,
                b as f64 + 0.9 * rand_f64(&mut rng),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_material < 0.8 {
                    // Diffuse
                    world.add_collider(
                        Sphere::builder()
                            .with_center(center)
                            .with_radius(0.2)
                            .with_material(LambertianMaterial::new(
                                rng.gen::<Color>() * rng.gen::<Color>(),
                            ))
                            .build(),
                    )
                } else if choose_material < 0.95 {
                    // Metal
                    world.add_collider(
                        Sphere::builder()
                            .with_center(center)
                            .with_radius(0.2)
                            .with_material(MetalMaterial::new(
                                Color::gen_range(&mut rng, 0.5..=1.0),
                                rng.gen_range(0.0..=0.5),
                            ))
                            .build(),
                    )
                } else {
                    // Glass
                    world.add_collider(
                        Sphere::builder()
                            .with_center(center)
                            .with_radius(0.2)
                            .with_material(DielectricMaterial::new(1.5))
                            .build(),
                    )
                }
            }
        }
    }

    // Elements
    world.add_collider(
        Sphere::builder()
            .with_center(Vec3::new(0.0, 1.0, 0.0))
            .with_radius(1.0)
            .with_material(DielectricMaterial::new(1.5))
            .build(),
    );

    world.add_collider(
        Sphere::builder()
            .with_center(Vec3::new(-4.0, 1.0, 0.0))
            .with_radius(1.0)
            .with_material(LambertianMaterial::new(Color::from_f64x3(0.4, 0.2, 0.1)))
            .build(),
    );

    world.add_collider(
        Sphere::builder()
            .with_center(Vec3::new(4.0, 1.0, 0.0))
            .with_radius(1.0)
            .with_material(MetalMaterial::new(Color::from_f64x3(0.7, 0.6, 0.5), 0.0))
            .build(),
    );

    world
}

/// Generate the "random spheres" scene from the <https://raytracing.github.io/books/RayTracingInOneWeekend.html> book.
pub fn random_spheres_scene(seed: SeedType) -> Scene {
    Scene::builder((1200, 800).into())
        .with_camera(
            Camera::builder()
                .with_aspect_ratio(3.0 / 2.0)
                .with_position(Vec3::new(13.0, 2.0, 3.0))
                .with_look_at(Vec3::new(0.0, 0.0, 0.0))
                .with_field_of_view(20.0)
                .with_focus_distance(10.0)
                .with_aperture(0.1)
                .build(),
        )
        .with_antialias(500)
        .with_max_depth(50)
        .with_world(random_world(seed))
        .build()
}
