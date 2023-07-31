use crate::{collider::Collider, hit_record::HitRecord, Material, Ray, Vec3};

/// A sphere.
#[derive(Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Option<Box<dyn Material>>,
}

impl Sphere {
    /// Create a new sphere builder.
    pub fn builder() -> SphereBuilder {
        SphereBuilder::new()
    }

    /// Get the sphere center.
    pub fn center(&self) -> Vec3 {
        self.center
    }

    /// Get the sphere radius.
    pub fn radius(&self) -> f64 {
        self.radius
    }
}

/// Sphere builder.
pub struct SphereBuilder {
    center: Vec3,
    radius: f64,
    material: Option<Box<dyn Material>>,
}

impl Default for SphereBuilder {
    fn default() -> Self {
        Self {
            center: Vec3::ZERO,
            radius: 1.0,
            material: None,
        }
    }
}

impl SphereBuilder {
    /// Create a new sphere builder.
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the sphere center.
    pub fn with_center(mut self, value: Vec3) -> Self {
        self.center = value;
        self
    }

    /// Set the sphere radius.
    pub fn with_radius(mut self, value: f64) -> Self {
        self.radius = value;
        self
    }

    /// Set the sphere material.
    pub fn with_material<M: Material + 'static>(mut self, material: M) -> Self {
        self.material = Some(Box::new(material));
        self
    }

    /// Build the sphere.
    pub fn build(self) -> Sphere {
        Sphere {
            center: self.center,
            radius: self.radius,
            material: self.material,
        }
    }
}

impl Collider for Sphere {
    fn hit<'a>(&'a self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'a>> {
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
            material: self.material.as_deref(),
            ..Default::default()
        };
        record.set_face_normal(ray, hit_normal);

        Some(record)
    }
}
