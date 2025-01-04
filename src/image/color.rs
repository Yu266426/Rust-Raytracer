use std::ops::{Add, Mul};

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
}

impl Color {
    pub fn as_u8_string(&self) -> String {
        let r = (self.r * 255.999) as u8;
        let g = (self.g * 255.999) as u8;
        let b = (self.b * 255.999) as u8;

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
