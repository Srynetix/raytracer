use raytracer_core::{Collider, Color, Context, Ray, RayShader, Vec3};

#[derive(Default)]
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
            return Color::black();
        }

        if let Some(record) = collider.hit(ray, 0.001, f64::MAX) {
            let target = record.point
                + record.normal
                + Vec3::gen_random_in_unit_sphere_normalized(&mut ctx.rng);
            return 0.5
                * self.ray_color(
                    ctx,
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
