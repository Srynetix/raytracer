use std::ops::{Add, AddAssign, Div, DivAssign, Mul};

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
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::from_rgba(r, g, b, 255)
    }

    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::from_floating_rgba(
            f64_color_from_u8(r),
            f64_color_from_u8(g),
            f64_color_from_u8(b),
            f64_color_from_u8(a),
        )
    }

    pub fn from_floating_rgb(r: f64, g: f64, b: f64) -> Self {
        Self::from_floating_rgba(r, g, b, 1.0)
    }

    pub fn from_floating_rgba(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self { r, g, b, a }
    }

    pub fn black() -> Self {
        Self::from_rgb(0, 0, 0)
    }

    pub fn white() -> Self {
        Self::from_rgb(255, 255, 255)
    }

    pub fn to_u8_array(&self) -> [u8; 4] {
        [
            u8_color_from_f64(self.r),
            u8_color_from_f64(self.g),
            u8_color_from_f64(self.b),
            u8_color_from_f64(self.a),
        ]
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

#[cfg(test)]
mod tests {
    use crate::Color;

    #[test]
    fn rgb() {
        assert_eq!(
            Color::from_rgb(0, 0, 0),
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
            Color::from_rgba(0, 0, 0, 0),
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
            Color::from_floating_rgb(0.0, 0.0, 0.5),
            Color {
                r: 0.0,
                g: 0.0,
                b: 0.5,
                a: 1.0
            }
        );
    }
}
