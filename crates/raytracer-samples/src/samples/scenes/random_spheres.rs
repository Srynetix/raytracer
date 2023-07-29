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

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = rand_f64(&mut rng);
            let center = Vec3::from_xyz(
                a as f64 + 0.9 * rand_f64(&mut rng),
                0.2,
                b as f64 + 0.9 * rand_f64(&mut rng),
            );

            if (center - Vec3::from_xyz(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_material < 0.8 {
                    // Diffuse
                    let albedo = rng.gen::<Color>() * rng.gen::<Color>();
                    let mut material = LambertianMaterial::new(albedo);
                    material.set_random_seed(seed);

                    let mut sphere = Sphere::new(center, 0.2);
                    sphere.set_material(Box::new(material));
                    world.add_collider(Box::new(sphere));
                } else if choose_material < 0.95 {
                    // Metal
                    let albedo = Color::gen_range(&mut rng, 0.5..=1.0);
                    let fuzz = rng.gen_range(0.0..=0.5);
                    let mut material = MetalMaterial::new(albedo, fuzz);
                    material.set_random_seed(seed);

                    let mut sphere = Sphere::new(center, 0.2);
                    sphere.set_material(Box::new(material));
                    world.add_collider(Box::new(sphere));
                } else {
                    // Glass
                    let mut material = DielectricMaterial::new(1.5);
                    material.set_random_seed(seed);

                    let mut sphere = Sphere::new(center, 0.2);
                    sphere.set_material(Box::new(material));
                    world.add_collider(Box::new(sphere));
                }
            }
        }
    }

    // Elements
    {
        let mut material = DielectricMaterial::new(1.5);
        material.set_random_seed(seed);
        let mut sphere = Sphere::new(Vec3::from_xyz(0.0, 1.0, 0.0), 1.0);
        sphere.set_material(Box::new(material));
        world.add_collider(Box::new(sphere));
    }

    {
        let mut material = LambertianMaterial::new(Color::from_floating_rgb(0.4, 0.2, 0.1));
        material.set_random_seed(seed);
        let mut sphere = Sphere::new(Vec3::from_xyz(-4.0, 1.0, 0.0), 1.0);
        sphere.set_material(Box::new(material));
        world.add_collider(Box::new(sphere));
    }

    {
        let mut material = MetalMaterial::new(Color::from_floating_rgb(0.7, 0.6, 0.5), 0.0);
        material.set_random_seed(seed);
        let mut sphere = Sphere::new(Vec3::from_xyz(4.0, 1.0, 0.0), 1.0);
        sphere.set_material(Box::new(material));
        world.add_collider(Box::new(sphere));
    }

    world
}

pub fn random_spheres_scene(seed: SeedType) -> Scene {
    Scene::builder((1200, 800).into())
        .with_seed(seed)
        .with_camera(
            Camera::builder()
                .with_aspect_ratio(3.0 / 2.0)
                .with_position(Vec3::from_xyz(13.0, 2.0, 3.0))
                .with_look_at(Vec3::from_xyz(0.0, 0.0, 0.0))
                .with_vertical_field_of_view(20.0)
                .with_focus_distance(10.0)
                .with_aperture(0.1)
                .build(),
        )
        .with_antialias(500)
        .with_max_depth(50)
        .with_world(random_world(seed))
        .build()
}
