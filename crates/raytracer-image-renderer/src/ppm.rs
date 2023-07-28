use std::io::Write;

use raytracer_core::{Color, Image, Renderer};

pub struct PpmRenderer<W: Write> {
    writer: W,
}

impl<W: Write> PpmRenderer<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    pub fn into_inner(self) -> W {
        self.writer
    }

    fn render_header(&mut self, image: &Image) -> Result<(), std::io::Error> {
        writeln!(self.writer, "P3")?;
        writeln!(self.writer, "{} {}", image.width(), image.height())?;
        writeln!(self.writer, "255")?;

        Ok(())
    }

    fn render_pixel(&mut self, pixel: &Color) -> Result<(), std::io::Error> {
        let adapted_pixel = pixel.to_u8_array();
        writeln!(
            self.writer,
            "{} {} {}",
            adapted_pixel[0], adapted_pixel[1], adapted_pixel[2]
        )?;

        Ok(())
    }
}

impl<W: Write> Renderer for PpmRenderer<W> {
    fn render(&mut self, image: &Image) -> Result<(), std::io::Error> {
        self.render_header(image)?;

        for pixel in image.pixels() {
            self.render_pixel(pixel)?;
        }

        Ok(())
    }
}
