use crate::{hit_record::HitRecord, hittable::Hittable, Ray};

#[derive(Default)]
pub struct World {
    hittables: Vec<Box<dyn Hittable>>,
}

impl World {
    pub fn new() -> Self {
        Self { hittables: vec![] }
    }

    pub fn add_hittable(&mut self, element: Box<dyn Hittable>) {
        self.hittables.push(element)
    }
}

impl Hittable for World {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut record = HitRecord::default();
        let mut hit_anything = false;
        let mut closest = t_max;

        for hittable in &self.hittables {
            if let Some(hit_record) = hittable.hit(ray, t_min, closest) {
                hit_anything = true;
                closest = hit_record.t;
                record = hit_record;
            }
        }

        if hit_anything {
            Some(record)
        } else {
            None
        }
    }
}
