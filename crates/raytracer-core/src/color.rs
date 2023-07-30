use std::ops::{Add, AddAssign, Div, DivAssign, Mul, RangeInclusive};

use rand::{distributions::Standard, prelude::Distribution, Rng};

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

fn f64_color_from_u8(value: u8) -> f64 {
    value as f64 / 255.0
}

fn u8_color_from_f64(value: f64) -> u8 {
    (value.clamp(0.0, 0.999) * 255.999) as u8
}

impl Color {
    pub const BLACK: Self = Self::from_f64x3(0.0, 0.0, 0.0);
    pub const RED: Self = Self::from_f64x3(1.0, 0.0, 0.0);
    pub const GREEN: Self = Self::from_f64x3(0.0, 1.0, 0.0);
    pub const BLUE: Self = Self::from_f64x3(0.0, 0.0, 1.0);
    pub const WHITE: Self = Self::from_f64x3(1.0, 1.0, 1.0);
    pub const GRAY: Self = Self::from_f64x3(0.5, 0.5, 0.5);

    pub fn from_u8x3(r: u8, g: u8, b: u8) -> Self {
        Self::from_u8x4(r, g, b, 255)
    }

    pub fn from_u8x4(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::from_f64x4(
            f64_color_from_u8(r),
            f64_color_from_u8(g),
            f64_color_from_u8(b),
            f64_color_from_u8(a),
        )
    }

    pub const fn from_f64x3(r: f64, g: f64, b: f64) -> Self {
        Self::from_f64x4(r, g, b, 1.0)
    }

    pub const fn from_f64x4(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self { r, g, b, a }
    }

    pub fn to_u8x4(&self) -> [u8; 4] {
        [
            u8_color_from_f64(self.r),
            u8_color_from_f64(self.g),
            u8_color_from_f64(self.b),
            u8_color_from_f64(self.a),
        ]
    }

    pub fn map<F: Fn(f64) -> f64>(&self, f: F) -> Self {
        Self {
            r: f(self.r),
            g: f(self.g),
            b: f(self.b),
            a: self.a,
        }
    }

    pub fn gen_range<R: Rng>(rng: &mut R, range: RangeInclusive<f64>) -> Self {
        Self {
            r: rng.gen_range(range.clone()),
            g: rng.gen_range(range.clone()),
            b: rng.gen_range(range),
            a: 1.0,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
            a: self.a,
        }
    }
}

impl Mul<Color> for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
            a: self.a,
        }
    }
}

impl Add<Color> for Color {
    type Output = Self;

    fn add(self, rhs: Color) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
            a: self.a + rhs.a,
        }
    }
}

impl AddAssign<Color> for Color {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl DivAssign<f64> for Color {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Color {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
            a: self.a,
        }
    }
}

impl Div<Color> for f64 {
    type Output = Color;

    fn div(self, rhs: Color) -> Self::Output {
        rhs / self
    }
}

impl Distribution<Color> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Color {
        Color {
            r: rng.gen_range(0.0..=1.0),
            g: rng.gen_range(0.0..=1.0),
            b: rng.gen_range(0.0..=1.0),
            a: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Color;

    #[test]
    fn rgb() {
        assert_eq!(
            Color::from_u8x3(0, 0, 0),
            Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0
            }
        );
    }

    #[test]
    fn rgba() {
        assert_eq!(
            Color::from_u8x4(0, 0, 0, 0),
            Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0
            }
        );
    }

    #[test]
    fn floating_rgb() {
        assert_eq!(
            Color::from_f64x3(0.0, 0.0, 0.5),
            Color {
                r: 0.0,
                g: 0.0,
                b: 0.5,
                a: 1.0
            }
        );
    }
}
