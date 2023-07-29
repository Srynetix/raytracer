use crate::{collider::Collider, hit_record::HitRecord, Material, Ray, Vec3};

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Option<Box<dyn Material>>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self {
            center,
            radius,
            material: None,
        }
    }

    pub fn set_material(&mut self, material: Box<dyn Material>) {
        self.material = Some(material);
    }

    pub fn center(&self) -> Vec3 {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl Collider for Sphere {
    fn hit(
        &self,
        ray: &Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<(HitRecord, Option<Box<dyn Material>>)> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find nearest root
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let hit_t = root;
        let hit_point = ray.at(hit_t);
        let hit_normal = (hit_point - self.center) / self.radius;

        let mut record = HitRecord {
            point: hit_point,
            t: hit_t,
            ..Default::default()
        };
        record.set_face_normal(ray, hit_normal);

        Some((record, self.material.clone()))
    }
}
