use raytracer_core::{
    primitives::Sphere, Color, Hittable, Ray, RayColor, Renderer, Scene, SeedType, Vec3, World,
};
use raytracer_gpu_renderer::GpuRenderer;
use tracing_subscriber::{layer::SubscriberExt, fmt, EnvFilter, util::SubscriberInitExt};

#[derive(Default)]
pub struct RayScene;

impl RayColor for RayScene {
    fn ray_color(&self, ray: &Ray, hittable: &dyn Hittable) -> Color {
        if let Some(record) = hittable.hit(ray, 0.0, f64::MAX) {
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

fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env().add_directive("raytracer=info".parse().unwrap()))
        .init();

    let mut renderer = GpuRenderer::new();
    let mut scene = Scene::builder((512 * 2, 288 * 2).into())
        .with_seed(SeedType::Fixed(1234567890))
        .with_antialias(32)
        .with_world(
            World::builder()
                .with_hittable(Box::new(Sphere::new(Vec3::from_xyz(0.0, 0.0, -1.0), 0.5)))
                .with_hittable(Box::new(Sphere::new(
                    Vec3::from_xyz(0.0, -100.5, -1.0),
                    100.0,
                )))
                .build(),
        )
        .build();

    let image = scene.render(RayScene);
    renderer.render(&image).unwrap()
}
