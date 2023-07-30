use crate::Vec3;

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub const ZERO: Self = Self {
        origin: Vec3::ZERO,
        direction: Vec3::ZERO,
    };

    pub const fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }
}

impl Default for Ray {
    fn default() -> Self {
        Self::ZERO
    }
}
