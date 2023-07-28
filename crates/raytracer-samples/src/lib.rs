#[cfg(test)]
mod samples;

use std::path::Path;

use raytracer_core::{SceneBuilder, SeedType};
use raytracer_image_renderer::ppm::PpmRenderer;

pub fn sample_scene_builder() -> SceneBuilder {
    SceneBuilder::new((384, 216).into()).with_seed(SeedType::Fixed(1234567890))
}

pub fn assert_ppm_snapshot<P: AsRef<Path>>(renderer: PpmRenderer<Vec<u8>>, path: P) {
    let data = renderer.into_inner();
    let str_data = std::str::from_utf8(&data).unwrap();
    let path = path.as_ref();

    match std::fs::metadata(path) {
        Ok(_) => {
            let file_data = std::fs::read_to_string(path).unwrap();
            println!("Comparing with snapshot {:?} ...", path);
            if str_data != file_data {
                panic!("PPM files are not the same.");
            }
        }
        Err(_) => {
            // Generating snapshot
            println!("Generating snapshot {:?} ...", path);
            std::fs::write(path, str_data).unwrap()
        }
    }
}
