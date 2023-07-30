use raytracer_core::{Collider, Color, Context, Ray, RayShader, Vec3};

#[derive(Default, Clone)]
pub struct DiffuseShader;

impl DiffuseShader {
    pub fn new() -> Self {
        Default::default()
    }
}

impl RayShader for DiffuseShader {
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
            let target =
                record.point + record.normal + Vec3::gen_in_unit_sphere_normalized(&mut ctx.rng);
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
