use std::cell::RefCell;

use raytracer_core::{Color, HitRecord, Material, Ray, RngWrapper, ScatterResult, SeedType, Vec3};

#[derive(Clone, Debug)]
pub struct MetalMaterial {
    albedo: Color,
    fuzz: f64,
    rng: RefCell<RngWrapper>,
}

impl MetalMaterial {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.clamp(0.0, 1.0),
            rng: RefCell::new(RngWrapper::new(SeedType::Random)),
        }
    }

    pub fn set_random_seed(&mut self, seed_type: SeedType) {
        *self.rng.borrow_mut() = RngWrapper::new(seed_type);
    }
}

impl Material for MetalMaterial {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<ScatterResult> {
        let reflected = ray.direction().normalized().reflect(record.normal);
        let scattered = Ray::from_points(
            record.point,
            reflected + self.fuzz * Vec3::gen_random_in_unit_sphere(&mut *self.rng.borrow_mut()),
        );
        let amount = scattered.direction().dot(record.normal);

        if amount > 0.0 {
            Some(ScatterResult {
                scattered,
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}
