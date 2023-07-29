use crate::{Collider, Color, Ray};

pub trait RayShader {
    fn ray_color(&self, ray: &Ray, collider: &dyn Collider, depth: u32) -> Color;
}
