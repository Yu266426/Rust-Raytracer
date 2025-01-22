use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

use nanorand::{tls_rng, Rng};

use crate::{image::color::Color, random::gen_range_f64};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub const fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub const fn up() -> Self {
        Self {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        }
    }

    pub fn random() -> Self {
        let mut rng = tls_rng();
        Self {
            x: rng.generate(),
            y: rng.generate(),
            z: rng.generate(),
        }
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        Self {
            x: gen_range_f64(min, max),
            y: gen_range_f64(min, max),
            z: gen_range_f64(min, max),
        }
    }

    pub fn random_unit() -> Self {
        // Only return vectors (normalized) within a unit sphere
        loop {
            let p = Self::random_range(-1.0, 1.0);
            let len_sq = p.length_squared();

            if 1e-160 < len_sq && len_sq <= 1.0 {
                return p / len_sq.sqrt();
            }
        }
    }

    pub fn random_unit_on_hemisphere(normal: &Self) -> Self {
        let unit_vec = Self::random_unit();

        if unit_vec.dot(&normal) > 0.0 {
            // In same hemisphere as normal
            unit_vec
        } else {
            -unit_vec
        }
    }

    pub fn random_in_unit_disk() -> Self {
        // Only return vectors (normalized) within a unit disk
        loop {
            let p = Self::new(gen_range_f64(-1.0, 1.0), gen_range_f64(-1.0, 1.0), 0.0);

            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn to_color(&self) -> Color {
        Color {
            r: self.x,
            g: self.y,
            b: self.z,
        }
    }
}

impl Vec3 {
    pub fn get(&self, n: usize) -> f64 {
        if n == 0 {
            self.x
        } else if n == 1 {
            self.y
        } else {
            self.z
        }
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, v: &Self) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn cross(&self, v: &Self) -> Self {
        Self {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        }
    }

    pub fn normalize(&self) -> Self {
        *self / self.length()
    }

    pub fn near_zero(&self) -> bool {
        let e = 1e-8;

        self.x.abs() < e && self.y.abs() < e && self.z.abs() < e
    }

    pub fn reflected(&self, normal: &Self) -> Self {
        *self - 2.0 * self.dot(normal) * *normal
    }

    pub fn refracted(&self, normal: &Self, refraction_index: f64) -> Self {
        // Todo: Better name for last param? (Is it just ior?)

        let cos = (-*self).dot(normal).min(1.0);
        let r_out_perp = refraction_index * (*self + cos * *normal);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * *normal;

        r_out_perp + r_out_parallel
    }
}

impl ToString for Vec3 {
    fn to_string(&self) -> String {
        format!("{:.2} {:.2} {:.2}", self.x, self.y, self.z)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        (1.0 / rhs) * self
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}
