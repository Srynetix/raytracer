use std::io::Write;

use raytracer_core::Image;

pub trait Renderer {
    fn render<W: Write>(&self, writer: &mut W, image: &Image) -> Result<(), std::io::Error>;
}
