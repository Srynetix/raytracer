use raytracer_core::{Color, HitRecord, Material, Ray, RngWrapper, ScatterResult, SeedType};

#[derive(Clone, Debug)]
pub struct MetalMaterial {
    pub albedo: Color,

    rng: RngWrapper,
}

impl MetalMaterial {
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

impl Material for MetalMaterial {
    fn scatter(&mut self, ray: &Ray, record: &HitRecord) -> Option<ScatterResult> {
        let reflected = ray.direction().normalized().reflect(record.normal);
        let scattered = Ray::from_points(record.point, reflected);
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
