use raytracer_core::{Collider, Color, Context, Ray, RayShader};

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
        (1.0 - t) * Color::from_rgb(255, 255, 255) + t * Color::from_floating_rgb(0.5, 0.7, 1.0)
    }
}
