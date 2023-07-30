use crate::{Collider, Color, Context, Ray};

pub trait RayShader {
    fn ray_color(&self, ctx: &mut Context, ray: &Ray, collider: &dyn Collider, depth: u32)
        -> Color;
}
