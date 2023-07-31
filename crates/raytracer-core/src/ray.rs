use crate::Vec3;

/// A ray: an origin and a direction.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    /// A "zeroed" ray.
    pub const ZERO: Self = Self {
        origin: Vec3::ZERO,
        direction: Vec3::ZERO,
    };

    /// Build a ray from an origin and a direction.
    pub const fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    /// Compute a ray point using a `t` value.
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }

    /// Get the ray origin.
    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    /// Get the ray direction.
    pub fn direction(&self) -> Vec3 {
        self.direction
    }
}

impl Default for Ray {
    fn default() -> Self {
        Self::ZERO
    }
}
