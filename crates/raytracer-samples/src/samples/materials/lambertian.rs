use raytracer_core::{Color, HitRecord, Material, Ray, RngWrapper, ScatterResult, SeedType, Vec3};

#[derive(Clone, Debug)]
pub struct LambertianMaterial {
    pub albedo: Color,

    rng: RngWrapper,
}

impl LambertianMaterial {
    pub fn new(albedo: Color) -> Self {
        Self {
            albedo,
            rng: RngWrapper::new(SeedType::Random),
        }
    }

    pub fn set_random_seed(&mut self, seed_type: SeedType) {
        self.rng = RngWrapper::new(seed_type);
    }
}

impl Material for LambertianMaterial {
    fn scatter(&mut self, _ray: &Ray, record: &HitRecord) -> Option<ScatterResult> {
        let mut scatter_direction =
            record.normal + Vec3::gen_random_in_unit_sphere_normalized(&mut self.rng);

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
