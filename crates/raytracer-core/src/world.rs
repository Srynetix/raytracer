use crate::{collider::Collider, hit_record::HitRecord, Ray};

/// World, contains colliders.
#[derive(Default, Clone)]
pub struct World {
    colliders: Vec<Box<dyn Collider>>,
}

impl World {
    /// Build an empty world.
    pub fn new() -> Self {
        Self { colliders: vec![] }
    }

    /// Create a world builder.
    pub fn builder() -> WorldBuilder {
        WorldBuilder::new()
    }

    /// Add a collider to the world.
    pub fn add_collider<C: Collider + 'static>(&mut self, element: C) {
        self.colliders.push(Box::new(element));
    }
}

/// World builder.
#[derive(Default)]
pub struct WorldBuilder {
    colliders: Vec<Box<dyn Collider>>,
}

impl WorldBuilder {
    /// Create a world builder.
    pub fn new() -> Self {
        Default::default()
    }

    /// Add a collider to the world.
    pub fn with_collider<C: Collider + 'static>(mut self, element: C) -> Self {
        self.colliders.push(Box::new(element));
        self
    }

    /// Build the world.
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
