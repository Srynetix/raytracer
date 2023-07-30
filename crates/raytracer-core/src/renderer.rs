use crate::image::Image;

/// An image renderer.
pub trait Renderer {
    /// Render an image.
    fn render(&mut self, image: &Image) -> Result<(), std::io::Error>;
}
