use crate::{image::Image, Color, Hittable, Ray};

pub trait Renderer {
    fn render(&mut self, image: &Image) -> Result<(), std::io::Error>;
}

pub trait RayColor {
    fn ray_color(&self, ray: &Ray, hittable: &dyn Hittable) -> Color;
}
