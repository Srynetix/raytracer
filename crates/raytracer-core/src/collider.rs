use crate::{hit_record::HitRecord, Ray};

pub trait Collider {
    fn hit<'a>(&'a self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'a>>;
}
