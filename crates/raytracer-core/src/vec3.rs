use std::ops::{Add, Div, Mul, Neg, RangeInclusive, Sub};

use rand::Rng;

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0);

    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn normalized(&self) -> Self {
        *self / self.length()
    }

    pub fn reflect(&self, normal: Self) -> Self {
        *self - 2.0 * self.dot(normal) * normal
    }

    pub fn refract(&self, normal: Self, ratio: f64) -> Self {
        let cos_theta = (-*self).dot(normal).min(1.0);
        let perp = ratio * (*self + cos_theta * normal);
        let parallel = -(1.0 - perp.length_squared()).abs().sqrt() * normal;
        perp + parallel
    }

    pub fn is_near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    pub fn gen_range<R: Rng>(rng: &mut R, range: RangeInclusive<f64>) -> Self {
        Self {
            x: rng.gen_range(range.clone()),
            y: rng.gen_range(range.clone()),
            z: rng.gen_range(range),
        }
    }

    pub fn gen_in_unit_sphere<R: Rng>(rng: &mut R) -> Self {
        loop {
            let vec = Self::gen_range(rng, -1.0..=1.0);
            if vec.length_squared() >= 1.0 {
                continue;
            }
            return vec;
        }
    }

    pub fn gen_in_unit_disk<R: Rng>(rng: &mut R) -> Self {
        loop {
            let vec = Self {
                x: rng.gen_range(-1.0..=1.0),
                y: rng.gen_range(-1.0..=1.0),
                z: 0.0,
            };
            if vec.length_squared() >= 1.0 {
                continue;
            }
            return vec;
        }
    }

    pub fn gen_in_unit_sphere_normalized<R: Rng>(rng: &mut R) -> Self {
        Self::gen_in_unit_sphere(rng).normalized()
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        rhs / self
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
