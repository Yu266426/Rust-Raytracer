pub mod lambertian;
pub mod metal;

use crate::{hittable::HitRecord, image::color::Color, ray::Ray};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(&Color, Ray)>;
}
