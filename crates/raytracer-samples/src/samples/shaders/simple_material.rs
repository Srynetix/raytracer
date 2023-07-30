use raytracer_core::{Collider, Color, Context, Ray, RayShader};

#[derive(Default, Clone)]
pub struct SimpleMaterialShader;

impl RayShader for SimpleMaterialShader {
    fn ray_color(
        &self,
        ctx: &mut Context,
        ray: &Ray,
        collider: &dyn Collider,
        depth: u32,
    ) -> Color {
        if depth == 0 {
            return Color::BLACK;
        }

        if let Some(record) = collider.hit(ray, 0.001, f64::MAX) {
            if let Some(mat) = record.material {
                if let Some(result) = mat.scatter(ctx, ray, &record) {
                    return result.attenuation
                        * self.ray_color(ctx, &result.scattered, collider, depth - 1);
                } else {
                    return Color::BLACK;
                }
            } else {
                return Color::BLACK;
            }
        }

        let norm_direction = ray.direction().normalized();
        let t = 0.5 * (norm_direction.y + 1.0);
        (1.0 - t) * Color::WHITE + t * Color::from_f64x3(0.5, 0.7, 1.0)
    }
}
