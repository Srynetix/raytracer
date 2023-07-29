use raytracer_core::{Collider, Color, Ray, RayShader};

pub struct SimpleMaterialShader;

impl RayShader for SimpleMaterialShader {
    fn ray_color(&self, ray: &Ray, collider: &dyn Collider, depth: u32) -> Color {
        if depth == 0 {
            return Color::black();
        }

        if let Some(record) = collider.hit(ray, 0.001, f64::MAX) {
            if let Some(mat) = record.material {
                if let Some(result) = mat.scatter(ray, &record) {
                    return result.attenuation
                        * self.ray_color(&result.scattered, collider, depth - 1);
                } else {
                    return Color::black();
                }
            } else {
                return Color::black();
            }
        }

        let norm_direction = ray.direction().normalized();
        let t = 0.5 * (norm_direction.y + 1.0);
        (1.0 - t) * Color::white() + t * Color::from_floating_rgb(0.5, 0.7, 1.0)
    }
}
