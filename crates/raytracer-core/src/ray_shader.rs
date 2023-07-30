use dyn_clone::DynClone;

use crate::{Collider, Color, Context, Ray};

/// A ray shader.
pub trait RayShader: DynClone + Send + Sync {
    /// Output a color from a ray and a collider.
    fn ray_color(&self, ctx: &mut Context, ray: &Ray, collider: &dyn Collider, depth: u32)
        -> Color;
}

dyn_clone::clone_trait_object!(RayShader);
