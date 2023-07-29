pub mod samples;

#[cfg(test)]
mod tests;

use std::path::Path;

use raytracer_core::{SceneBuilder, SeedType};
use raytracer_image_renderer::ppm::PpmRenderer;

pub fn sample_scene_builder() -> SceneBuilder {
    SceneBuilder::new((384, 216).into()).with_seed(SeedType::Fixed(1234567890))
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

pub fn assert_ppm_snapshot<P: AsRef<Path>>(renderer: PpmRenderer<Vec<u8>>, path: P) {
    let data = renderer.into_inner();
    let str_data = std::str::from_utf8(&data).unwrap();
    let path = path.as_ref();
    let should_override = std::env::var("RT_PPM_OVERRIDE").unwrap_or_default() == "1";

    match std::fs::metadata(path) {
        Ok(_) => {
            if should_override {
                generate_snapshot(str_data, path);
            } else {
                compare_snapshot(str_data, path);
            }
        }
        Err(_) => {
            generate_snapshot(str_data, path);
        }
    }
}
