use raytracer_core::{rand::Rng, Collider, Color, Context, Ray, RayShader, Vec3};

#[derive(Default, Clone)]
pub struct DiffuseHemisphereShader;

impl DiffuseHemisphereShader {
    pub fn new() -> Self {
        Default::default()
    }

    fn gen_in_hemisphere<R: Rng>(rng: &mut R, normal: Vec3) -> Vec3 {
        let value = Vec3::gen_in_unit_sphere(rng);
        if value.dot(normal) > 0.0 {
            value
        } else {
            -value
        }
    }
}

impl RayShader for DiffuseHemisphereShader {
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
            let target = record.point + Self::gen_in_hemisphere(&mut ctx.rng, record.normal);

            return 0.5
                * self.ray_color(
                    ctx,
                    &Ray::new(record.point, target - record.point),
                    collider,
                    depth - 1,
                );
        }

        let norm_direction = ray.direction().normalized();
        let t = 0.5 * (norm_direction.y + 1.0);
        (1.0 - t) * Color::WHITE + t * Color::from_f64x3(0.5, 0.7, 1.0)
    }
}
