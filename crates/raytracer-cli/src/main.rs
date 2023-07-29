use raytracer_core::{Renderer, SeedType};
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
    let scene = random_spheres_scene(SeedType::Random);
    let image = scene.render(SimpleMaterialShader);
    renderer.render(&image).unwrap()
}
