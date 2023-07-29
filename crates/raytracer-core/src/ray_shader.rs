use crate::{Collider, Color, Ray};

pub trait RayShader {
    fn ray_color(&mut self, ray: &Ray, collider: &dyn Collider, depth: u32) -> Color;
}
