use raytracer_core::{Collider, Color, Context, Ray, RayShader};

pub struct NormalColliderShader;

impl RayShader for NormalColliderShader {
    fn ray_color(
        &self,
        _ctx: &mut Context,
        ray: &Ray,
        collider: &dyn Collider,
        _depth: u32,
    ) -> Color {
        if let Some(record) = collider.hit(ray, 0.0, f64::MAX) {
            return Color::from_floating_rgb(
                (record.normal.x + 1.0) * 0.5,
                (record.normal.y + 1.0) * 0.5,
                (record.normal.z + 1.0) * 0.5,
            );
        }

        let norm_direction = ray.direction().normalized();
        let t = 0.5 * (norm_direction.y + 1.0);
        (1.0 - t) * Color::from_rgb(255, 255, 255) + t * Color::from_floating_rgb(0.5, 0.7, 1.0)
    }
}
