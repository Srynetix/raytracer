//! Simple gradient generator.

use raytracer_core::{Collider, Color, Context, Ray, RayShader};

/// Gradient shader.
#[derive(Default, Clone)]
pub struct GradientShader;

impl RayShader for GradientShader {
    fn ray_color(
        &self,
        _ctx: &mut Context,
        ray: &Ray,
        _collider: &dyn Collider,
        _depth: u32,
    ) -> Color {
        let norm_direction = ray.direction().normalized();
        let t = 0.5 * (norm_direction.y + 1.0);
        (1.0 - t) * Color::from_u8x3(255, 255, 255) + t * Color::from_f64x3(0.5, 0.7, 1.0)
    }
}
