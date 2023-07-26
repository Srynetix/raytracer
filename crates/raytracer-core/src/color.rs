use std::ops::{Add, Mul};

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::from_rgba(r, g, b, 255)
    }

    pub fn from_floating_rgb(r: f64, g: f64, b: f64) -> Self {
        Self::from_rgb(Self::f64_to_u8(r), Self::f64_to_u8(g), Self::f64_to_u8(b))
    }

    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    fn f64_to_u8(value: f64) -> u8 {
        (value * 255.0).clamp(0.0, 255.0) as u8
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            r: (self.r as f64 * rhs).clamp(0.0, 255.0) as u8,
            g: (self.g as f64 * rhs).clamp(0.0, 255.0) as u8,
            b: (self.b as f64 * rhs).clamp(0.0, 255.0) as u8,
            a: self.a,
        }
    }
}

impl Add<Color> for Color {
    type Output = Self;

    fn add(self, rhs: Color) -> Self::Output {
        Self {
            r: self.r.saturating_add(rhs.r),
            g: self.g.saturating_add(rhs.g),
            b: self.b.saturating_add(rhs.b),
            a: self.a.saturating_add(rhs.a),
        }
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        rhs * self
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
                r: 0,
                g: 0,
                b: 0,
                a: 255
            }
        );
    }

    #[test]
    fn rgba() {
        assert_eq!(
            Color::from_rgba(0, 0, 0, 127),
            Color {
                r: 0,
                g: 0,
                b: 0,
                a: 127
            }
        );
    }

    #[test]
    fn floating_rgb() {
        assert_eq!(
            Color::from_floating_rgb(0.0, 0.0, 0.5),
            Color {
                r: 0,
                g: 0,
                b: 127,
                a: 255
            }
        );
    }
}
