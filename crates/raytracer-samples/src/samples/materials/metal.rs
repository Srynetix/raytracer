use raytracer_core::{Color, Context, HitRecord, Material, Ray, ScatterResult, Vec3};

#[derive(Clone, Debug)]
pub struct MetalMaterial {
    albedo: Color,
    fuzz: f64,
}

impl MetalMaterial {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }
}

impl Material for MetalMaterial {
    fn scatter(&self, ctx: &mut Context, ray: &Ray, record: &HitRecord) -> Option<ScatterResult> {
        let reflected = ray.direction().normalized().reflect(record.normal);
        let scattered = Ray::from_points(
            record.point,
            reflected + self.fuzz * Vec3::gen_random_in_unit_sphere(&mut ctx.rng),
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
