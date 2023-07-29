#[test]
fn run() {
    use raytracer_core::{Color, Image, Renderer};
    use raytracer_image_renderer::ppm::PpmRenderer;

    use crate::assert_ppm_snapshot;

    let mut renderer = PpmRenderer::new(Vec::new());

    let width = 256;
    let height = 256;
    let mut pixels = vec![];

    for y in 0..height {
        for x in 0..width {
            let r = x as f64 / (width - 1) as f64;
            let g = (height - y) as f64 / (height - 1) as f64;
            let b = 0.25;

            pixels.push(Color::from_floating_rgb(r, g, b));
        }
    }

    let image = Image::from_pixels(width, pixels);

    renderer.render(&image).unwrap();
    assert_ppm_snapshot(renderer, "./src/samples/basic_gradient.ppm");
}
