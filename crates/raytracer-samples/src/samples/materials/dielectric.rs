use std::cell::RefCell;

use raytracer_core::{
    rand::Rng, Color, HitRecord, Material, Ray, RngWrapper, ScatterResult, SeedType,
};

#[derive(Clone, Debug)]
pub struct DielectricMaterial {
    index_of_refraction: f64,
    rng: RefCell<RngWrapper>,
}

impl DielectricMaterial {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            index_of_refraction,
            rng: RefCell::new(RngWrapper::new(SeedType::Random)),
        }
    }

    pub fn set_random_seed(&mut self, seed_type: SeedType) {
        *self.rng.borrow_mut() = RngWrapper::new(seed_type);
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }

    fn random_f64(&self) -> f64 {
        self.rng.borrow_mut().gen_range(0.0..=1.0)
    }
}

impl Material for DielectricMaterial {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<ScatterResult> {
        let refraction_ratio = if record.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };
        let unit_direction = ray.direction().normalized();
        let cos_theta = (-unit_direction).dot(record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract
            || Self::reflectance(cos_theta, refraction_ratio) > self.random_f64()
        {
            unit_direction.reflect(record.normal)
        } else {
            unit_direction.refract(record.normal, refraction_ratio)
        };

        Some(ScatterResult {
            attenuation: Color::white(),
            scattered: Ray::from_points(record.point, direction),
        })
    }
}
