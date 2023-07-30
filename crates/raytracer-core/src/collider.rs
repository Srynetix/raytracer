use dyn_clone::DynClone;

use crate::{hit_record::HitRecord, Ray};

/// A collider can respond to hit detection.
pub trait Collider: DynClone + Send + Sync {
    /// Generate a collision response from a ray.
    fn hit<'a>(&'a self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'a>>;
}

dyn_clone::clone_trait_object!(Collider);
