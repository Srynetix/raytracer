mod basic_gradient;
mod glass_sphere;
mod hollow_glass_sphere;
mod point_of_view;
mod random_spheres;
mod ray_diffuse;
mod ray_gradient;
mod ray_sphere;
mod ray_sphere_antialias;
mod ray_sphere_normals;

use std::path::Path;

use raytracer_core::{Context, RngWrapper, SceneBuilder, SeedType};
use raytracer_image_renderer::ppm::PpmRenderer;

pub(crate) fn sample_scene_builder() -> SceneBuilder {
    SceneBuilder::new((384, 216).into())
}

pub(crate) fn fixed_rng() -> RngWrapper {
    RngWrapper::new(SeedType::Fixed(1234567890))
}

pub(crate) fn build_context() -> Context {
    Context { rng: fixed_rng() }
}

fn compare_snapshot(data: &str, path: &Path) {
    let file_data = std::fs::read_to_string(path).unwrap();
    println!("Comparing with snapshot {:?} ...", path);

    if data != file_data {
        panic!("PPM files are not the same.");
    }
}

fn generate_snapshot(data: &str, path: &Path) {
    println!("Generating snapshot {:?} ...", path);
    std::fs::write(path, data).unwrap()
}

pub(crate) fn assert_ppm_snapshot(renderer: PpmRenderer<Vec<u8>>, name: &'static str) {
    let data = renderer.into_inner();
    let str_data = std::str::from_utf8(&data).unwrap();
    let path = Path::new("./src/tests").join(name);
    let should_override = std::env::var("RT_PPM_OVERRIDE").unwrap_or_default() == "1";

    match std::fs::metadata(&path) {
        Ok(_) => {
            if should_override {
                generate_snapshot(str_data, &path);
            } else {
                compare_snapshot(str_data, &path);
            }
        }
        Err(_) => {
            generate_snapshot(str_data, &path);
        }
    }
}
