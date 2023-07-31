use crate::{Material, Ray, Vec3};

/// Collision hit record.
#[derive(Default)]
pub struct HitRecord<'a> {
    /// Collision point.
    pub point: Vec3,
    /// Collision normal.
    pub normal: Vec3,
    /// Ray factor.
    pub t: f64,
    /// Choose between front and back face normal.
    pub front_face: bool,
    /// Material.
    pub material: Option<&'a dyn Material>,
}

impl<'a> HitRecord<'a> {
    /// Set face normal.
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
