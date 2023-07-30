//! Draw a sphere.

use raytracer_core::{Collider, Color, Context, Ray, RayShader, Vec3};

fn hit_sphere(center: Vec3, radius: f64, ray: &Ray) -> bool {
    let oc = ray.origin() - center;
    let a = ray.direction().dot(ray.direction());
    let b = 2.0 * oc.dot(ray.direction());
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

/// Simple sphere shader.
#[derive(Default, Clone)]
pub struct SimpleSphereShader;

impl RayShader for SimpleSphereShader {
    fn ray_color(
        &self,
        _ctx: &mut Context,
        ray: &Ray,
        _collider: &dyn Collider,
        _depth: u32,
    ) -> Color {
        if hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, ray) {
            return Color::from_u8x3(255, 0, 0);
        }

        let norm_direction = ray.direction().normalized();
        let t = 0.5 * (norm_direction.y + 1.0);
        (1.0 - t) * Color::from_u8x3(255, 255, 255) + t * Color::from_f64x3(0.5, 0.7, 1.0)
    }
}
