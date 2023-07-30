use raytracer_core::{Collider, Color, Context, Ray, RayShader, Vec3};

fn hit_sphere(center: Vec3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin() - center;
    let a = ray.direction().length_squared();
    let half_b = oc.dot(ray.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

#[derive(Default, Clone)]
pub struct NormalShader;

impl RayShader for NormalShader {
    fn ray_color(
        &self,
        _ctx: &mut Context,
        ray: &Ray,
        _collider: &dyn Collider,
        _depth: u32,
    ) -> Color {
        let t = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, ray);
        if t > 0.0 {
            let normal = (ray.at(t) - Vec3::new(0.0, 0.0, -1.0)).normalized();
            return Color::from_f64x3(
                (normal.x + 1.0) * 0.5,
                (normal.y + 1.0) * 0.5,
                (normal.z + 1.0) * 0.5,
            );
        }

        let norm_direction = ray.direction().normalized();
        let t = 0.5 * (norm_direction.y + 1.0);
        (1.0 - t) * Color::from_u8x3(255, 255, 255) + t * Color::from_f64x3(0.5, 0.7, 1.0)
    }
}
