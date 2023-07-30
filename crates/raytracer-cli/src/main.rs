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

    let seed = SeedType::Fixed(1234567890);
    let mut renderer = WindowRenderer::new();
    let mut scene = random_spheres_scene(seed);
    scene.set_antialias(50);
    scene.set_scale(0.5);

    let ctx = Context {
        rng: RngWrapper::new(seed),
    };

    let image = scene.render_parallel(ctx, &SimpleMaterialShader, 16);
    renderer.render(&image).unwrap()
}
