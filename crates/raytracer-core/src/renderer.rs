use crate::image::Image;

pub trait Renderer {
    fn render(&mut self, image: &Image) -> Result<(), std::io::Error>;
}
