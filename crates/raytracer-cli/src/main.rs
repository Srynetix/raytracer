use raytracer_core::{Context, Renderer, RngWrapper, SeedType};
use raytracer_samples::samples::{
    scenes::random_spheres::random_spheres_scene, shaders::simple_material::SimpleMaterialShader,
};
use raytracer_window_renderer::WindowRenderer;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env().add_directive("raytracer=info".parse().unwrap()))
        .init();

    let mut renderer = WindowRenderer::new();
    let mut scene = random_spheres_scene(SeedType::Random);
    scene.set_antialias(200);
    let mut ctx = Context {
        rng: RngWrapper::new(SeedType::Random),
    };

    let image = scene.render(&mut ctx, SimpleMaterialShader);
    renderer.render(&image).unwrap()
}
