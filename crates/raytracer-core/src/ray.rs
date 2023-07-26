use crate::Vec3;

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn zero() -> Self {
        Self {
            origin: Vec3::zero(),
            direction: Vec3::zero(),
        }
    }

    pub fn from_points(origin: Vec3, direction: Vec3) -> Self {
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
        Self::zero()
    }
}
