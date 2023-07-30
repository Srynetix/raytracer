use dyn_clone::DynClone;

use crate::{hit_record::HitRecord, Ray};

pub trait Collider: DynClone + Send + Sync {
    fn hit<'a>(&'a self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'a>>;
}

dyn_clone::clone_trait_object!(Collider);
