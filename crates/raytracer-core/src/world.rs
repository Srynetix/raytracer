use crate::{collider::Collider, hit_record::HitRecord, Ray};

#[derive(Default)]
pub struct World {
    colliders: Vec<Box<dyn Collider>>,
}

impl World {
    pub fn new() -> Self {
        Self { colliders: vec![] }
    }

    pub fn builder() -> WorldBuilder {
        WorldBuilder::new()
    }

    pub fn add_collider(&mut self, element: Box<dyn Collider>) {
        self.colliders.push(element)
    }
}

#[derive(Default)]
pub struct WorldBuilder {
    colliders: Vec<Box<dyn Collider>>,
}

impl WorldBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_collider(mut self, element: Box<dyn Collider>) -> Self {
        self.colliders.push(element);
        self
    }

    pub fn build(self) -> World {
        World {
            colliders: self.colliders,
        }
    }
}

impl Collider for World {
    fn hit<'a>(&'a self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'a>> {
        let mut record = HitRecord::default();
        let mut hit_anything = false;
        let mut closest = t_max;

        for collider in &self.colliders {
            if let Some(hit_record) = collider.hit(ray, t_min, closest) {
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
