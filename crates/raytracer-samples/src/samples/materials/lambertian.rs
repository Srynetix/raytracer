use raytracer_core::{Color, Context, HitRecord, Material, Ray, ScatterResult, Vec3};

#[derive(Clone, Debug)]
pub struct LambertianMaterial {
    pub albedo: Color,
}

impl LambertianMaterial {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for LambertianMaterial {
    fn scatter(&self, ctx: &mut Context, _ray: &Ray, record: &HitRecord) -> Option<ScatterResult> {
        let mut scatter_direction =
            record.normal + Vec3::gen_random_in_unit_sphere_normalized(&mut ctx.rng);

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
