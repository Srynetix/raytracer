use dyn_clone::DynClone;

use crate::{Color, Context, HitRecord, Ray};

pub struct ScatterResult {
    pub scattered: Ray,
    pub attenuation: Color,
}

pub trait Material: DynClone {
    fn scatter(&self, ctx: &mut Context, ray: &Ray, record: &HitRecord) -> Option<ScatterResult>;
}

dyn_clone::clone_trait_object!(Material);
