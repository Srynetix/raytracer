//! Lambertian material.

use raytracer_core::{Color, Context, HitRecord, Material, Ray, ScatterResult, Vec3};

/// Lambertian material.
#[derive(Clone, Debug)]
pub struct LambertianMaterial {
    /// Albedo.
    pub albedo: Color,
}

impl LambertianMaterial {
    /// Create a new lambertian material.
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for LambertianMaterial {
    fn scatter(&self, ctx: &mut Context, _ray: &Ray, record: &HitRecord) -> Option<ScatterResult> {
        let mut scatter_direction =
            record.normal + Vec3::gen_in_unit_sphere_normalized(&mut ctx.rng);

        // Catch degenerate scatter direction
        if scatter_direction.is_near_zero() {
            scatter_direction = record.normal;
        }

        Some(ScatterResult {
            scattered: Ray::new(record.point, scatter_direction),
            attenuation: self.albedo,
        })
    }
}
