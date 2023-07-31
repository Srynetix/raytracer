use dyn_clone::DynClone;

use crate::{Color, Context, HitRecord, Ray};

/// Material scatter result.
pub struct ScatterResult {
    /// Scattered ray.
    pub scattered: Ray,
    /// Attenuation color.
    pub attenuation: Color,
}

/// A material.
pub trait Material: DynClone + Send + Sync {
    /// Scatter the input ray using the material properties.
    fn scatter(&self, ctx: &mut Context, ray: &Ray, record: &HitRecord) -> Option<ScatterResult>;
}

dyn_clone::clone_trait_object!(Material);
