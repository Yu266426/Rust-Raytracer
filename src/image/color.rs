use std::ops::{Add, AddAssign, Mul};

use crate::interval::Interval;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl ToString for Color {
    fn to_string(&self) -> String {
        format!("{:.2} {:.2} {:.2}", self.r, self.g, self.b)
    }
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    pub fn black() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    pub fn white() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
    }
}

impl Color {
    fn linear_to_gamma_2(&self) -> Self {
        Self {
            r: self.r.max(0.0).sqrt(),
            g: self.g.max(0.0).sqrt(),
            b: self.b.max(0.0).sqrt(),
        }
    }

    pub fn as_u8_string(&self) -> String {
        static INTENSITY: Interval = Interval {
            min: 0.0,
            max: 0.999999,
        };

        let gamma_color = self.linear_to_gamma_2();

        let r = (256.0 * INTENSITY.clamp(gamma_color.r)) as u8;
        let g = (256.0 * INTENSITY.clamp(gamma_color.g)) as u8;
        let b = (256.0 * INTENSITY.clamp(gamma_color.b)) as u8;

        format!("{} {} {}", r, g, b)
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Self::Output {
            r: rhs.r * self,
            g: rhs.g * self,
            b: rhs.b * self,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        rhs * self
    }
}
