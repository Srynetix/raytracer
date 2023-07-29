use std::cell::RefCell;

use raytracer_core::{Collider, Color, Ray, RayShader, RngWrapper, SeedType, Vec3};

pub struct DiffuseShader {
    rng: RefCell<RngWrapper>,
}

impl DiffuseShader {
    pub fn new(seed_type: SeedType) -> Self {
        Self {
            rng: RefCell::new(RngWrapper::new(seed_type)),
        }
    }
}

impl RayShader for DiffuseShader {
    fn ray_color(&self, ray: &Ray, collider: &dyn Collider, depth: u32) -> Color {
        if depth == 0 {
            return Color::black();
        }

        if let Some(record) = collider.hit(ray, 0.001, f64::MAX) {
            let target = record.point
                + record.normal
                + Vec3::gen_random_in_unit_sphere_normalized(&mut *self.rng.borrow_mut());
            return 0.5
                * self.ray_color(
                    &Ray::from_points(record.point, target - record.point),
                    collider,
                    depth - 1,
                );
        }

        let norm_direction = ray.direction().normalized();
        let t = 0.5 * (norm_direction.y + 1.0);
        (1.0 - t) * Color::white() + t * Color::from_floating_rgb(0.5, 0.7, 1.0)
    }
}
