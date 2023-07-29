use crate::{hit_record::HitRecord, Material, Ray};

pub trait Collider {
    fn hit(
        &self,
        ray: &Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<(HitRecord, Option<Box<dyn Material>>)>;
}
