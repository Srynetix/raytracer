use std::io::Write;

use raytracer_core::{Color, Image};

use crate::renderer::Renderer;

pub struct PpmRenderer;

impl PpmRenderer {
    pub fn new() -> Self {
        Self
    }

    fn render_header<W: Write>(&self, writer: &mut W, image: &Image) -> Result<(), std::io::Error> {
        writeln!(writer, "P3")?;
        writeln!(writer, "{} {}", image.width(), image.height())?;
        writeln!(writer, "255")?;

        Ok(())
    }

    fn render_pixel<W: Write>(&self, writer: &mut W, pixel: &Color) -> Result<(), std::io::Error> {
        writeln!(writer, "{} {} {}", pixel.r, pixel.g, pixel.b)?;

        Ok(())
    }
}

impl Default for PpmRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderer for PpmRenderer {
    fn render<W: Write>(&self, writer: &mut W, image: &Image) -> Result<(), std::io::Error> {
        self.render_header(writer, image)?;

        for pixel in image.pixels() {
            self.render_pixel(writer, pixel)?;
        }

        Ok(())
    }
}
