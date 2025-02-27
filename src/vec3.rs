use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};
use std::simd::prelude::*;

use nanorand::{tls_rng, Rng};

use crate::{image::color::Color, random::gen_range_f64};

const EPSILON: f64 = 1e-8;

// #[repr(align(32))]
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    vec: Simd<f64, 4>,
}

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            vec: f64x4::from_array([x, y, z, 0.0]),
        }
    }

    /// Returns a vector with all components set to zero
    pub const fn zero() -> Self {
        Self {
            vec: f64x4::splat(0.0),
        }
    }

    pub const fn up() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }

    pub fn random() -> Self {
        let mut rng = tls_rng();
        Self::new(rng.generate(), rng.generate(), rng.generate())
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        Self::new(
            gen_range_f64(min, max),
            gen_range_f64(min, max),
            gen_range_f64(min, max),
        )
    }

    pub fn random_unit() -> Self {
        // Only return vectors (normalized) within a unit sphere
        loop {
            let p = Self::random_range(-1.0, 1.0);
            let len_sq = p.length_squared();
            if (1e-160..=1.0).contains(&len_sq) {
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
            r: self.get_x(),
            g: self.get_y(),
            b: self.get_z(),
        }
    }
}

impl Vec3 {
    #[inline]
    pub fn get_x(&self) -> f64 {
        self.vec[0]
    }

    #[inline]
    pub fn get_y(&self) -> f64 {
        self.vec[1]
    }

    #[inline]
    pub fn get_z(&self) -> f64 {
        self.vec[2]
    }

    #[inline]
    pub fn set_x(&mut self, value: f64) {
        self.vec[0] = value;
    }

    #[inline]
    pub fn set_y(&mut self, value: f64) {
        self.vec[1] = value;
    }

    #[inline]
    pub fn set_z(&mut self, value: f64) {
        self.vec[2] = value;
    }

    #[inline]
    pub fn get(&self, n: usize) -> f64 {
        assert!(n <= 2);

        self.vec[n]
    }

    #[inline]
    pub fn sum(&self) -> f64 {
        self.get_x() + self.get_y() + self.get_z()
    }

    #[inline]
    pub fn length_squared(&self) -> f64 {
        (self * self).sum()
    }

    #[inline]
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    #[inline]
    pub fn dot(&self, v: &Self) -> f64 {
        (self * v).sum()
    }

    #[inline]
    pub fn cross(&self, v: &Self) -> Self {
        let a_zxy = simd_swizzle!(self.vec, [2, 0, 1, 3]); // [z,x,y,w]
        let b_zxy = simd_swizzle!(v.vec, [2, 0, 1, 3]); // [z,x,y,w]

        Self {
            vec: simd_swizzle!(a_zxy * v.vec - self.vec * b_zxy, [2, 0, 1, 3]),
        }
    }

    #[inline]
    pub fn normalize(&self) -> Self {
        *self / self.length()
    }

    #[inline]
    pub fn near_zero(&self) -> bool {
        self.vec.abs().le(&f64x4::splat(EPSILON))
    }

    #[inline]
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
        format!(
            "{:.2} {:.2} {:.2}",
            self.get_x(),
            self.get_y(),
            self.get_z()
        )
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self::Output { vec: -self.vec }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            vec: self.vec + rhs.vec,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.vec += rhs.vec;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            vec: self.vec - rhs.vec,
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            vec: self.vec * rhs.vec,
        }
    }
}

impl Mul for &Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            vec: self.vec * rhs.vec,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            vec: f64x4::splat(self) * rhs.vec,
        }
    }
}

impl Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Self::Output {
            vec: f64x4::splat(self) * rhs.vec,
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
        self.vec *= f64x4::splat(rhs);
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
        self.vec /= f64x4::splat(rhs);
    }
}
