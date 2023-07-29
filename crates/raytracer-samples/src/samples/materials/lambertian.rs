use std::cell::RefCell;

use raytracer_core::{Color, HitRecord, Material, Ray, RngWrapper, ScatterResult, SeedType, Vec3};

#[derive(Clone, Debug)]
pub struct LambertianMaterial {
    pub albedo: Color,

    rng: RefCell<RngWrapper>,
}

impl LambertianMaterial {
    pub fn new(albedo: Color) -> Self {
        Self {
            albedo,
            rng: RefCell::new(RngWrapper::new(SeedType::Random)),
        }
    }

    pub fn set_random_seed(&mut self, seed_type: SeedType) {
        *self.rng.borrow_mut() = RngWrapper::new(seed_type);
    }
}

impl Material for LambertianMaterial {
    fn scatter(&self, _ray: &Ray, record: &HitRecord) -> Option<ScatterResult> {
        let mut scatter_direction =
            record.normal + Vec3::gen_random_in_unit_sphere_normalized(&mut *self.rng.borrow_mut());

        // Catch degenerate scatter direction
        if scatter_direction.is_near_zero() {
            scatter_direction = record.normal;
        }

        Some(ScatterResult {
            scattered: Ray::from_points(record.point, scatter_direction),
            attenuation: self.albedo,
        })
    }
}
