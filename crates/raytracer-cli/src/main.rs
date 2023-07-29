use raytracer_core::{primitives::Sphere, Color, Renderer, Scene, SeedType, Vec3, World};
use raytracer_gpu_renderer::GpuRenderer;
use raytracer_samples::samples::{
    materials::{
        dielectric::DielectricMaterial, lambertian::LambertianMaterial, metal::MetalMaterial,
    },
    shaders::simple_material::SimpleMaterialShader,
};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env().add_directive("raytracer=info".parse().unwrap()))
        .init();

    let mut renderer = GpuRenderer::new();
    let mut scene = Scene::builder((512, 288).into())
        .with_seed(SeedType::Fixed(1234567890))
        .with_antialias(100)
        .with_max_depth(50)
        .with_world(
            World::builder()
                .with_collider({
                    let mut sphere = Sphere::new(Vec3::from_xyz(0.0, -100.5, -1.0), 100.0);
                    sphere.set_material(Box::new(LambertianMaterial::new(
                        Color::from_floating_rgb(0.8, 0.8, 0.0),
                    )));
                    Box::new(sphere)
                })
                .with_collider({
                    let mut sphere = Sphere::new(Vec3::from_xyz(0.0, 0.0, -1.0), 0.5);
                    sphere.set_material(Box::new(LambertianMaterial::new(
                        Color::from_floating_rgb(0.1, 0.2, 0.5),
                    )));
                    Box::new(sphere)
                })
                .with_collider({
                    let mut sphere = Sphere::new(Vec3::from_xyz(-1.0, 0.0, -1.0), 0.5);
                    // sphere.set_material(Box::new(MetalMaterial::new(
                    //     Color::from_floating_rgb(0.8, 0.8, 0.8),
                    //     0.3,
                    // )));
                    sphere.set_material(Box::new(DielectricMaterial::new(1.5)));
                    Box::new(sphere)
                })
                .with_collider({
                    let mut sphere = Sphere::new(Vec3::from_xyz(1.0, 0.0, -1.0), 0.5);
                    sphere.set_material(Box::new(MetalMaterial::new(
                        Color::from_floating_rgb(0.8, 0.6, 0.2),
                        0.0,
                    )));
                    Box::new(sphere)
                })
                .build(),
        )
        .build();

    let color = SimpleMaterialShader;
    let image = scene.render(color);
    renderer.render(&image).unwrap()
}
