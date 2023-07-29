use raytracer_core::{Collider, Color, Ray, RayShader, RngWrapper, SeedType, Vec3};

pub struct DiffuseHemisphereShader {
    rng: RngWrapper,
}

impl DiffuseHemisphereShader {
    pub fn new(seed_type: SeedType) -> Self {
        Self {
            rng: RngWrapper::new(seed_type),
        }
    }

    fn gen_random_in_hemisphere(&mut self, normal: Vec3) -> Vec3 {
        let value = Vec3::gen_random_in_unit_sphere(&mut self.rng);
        if value.dot(normal) > 0.0 {
            value
        } else {
            -value
        }
    }
}

impl RayShader for DiffuseHemisphereShader {
    fn ray_color(&mut self, ray: &Ray, collider: &dyn Collider, depth: u32) -> Color {
        if depth == 0 {
            return Color::black();
        }

        if let Some((record, _material)) = collider.hit(ray, 0.001, f64::MAX) {
            let target = record.point + self.gen_random_in_hemisphere(record.normal);

            return 0.5
                * self.ray_color(
                    &Ray::from_points(record.point, target - record.point),
                    collider,
                    depth - 1,
                );
        }

        let norm_direction = ray.direction().normalized();
        let t = 0.5 * (norm_direction.y + 1.0);
        (1.0 - t) * Color::white() + t * Color::from_floating_rgb(0.5, 0.7, 1.0)
    }
}
