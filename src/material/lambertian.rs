use crate::{hittable::HitRecord, image::color::Color, ray::Ray, vec3::Vec3};

use super::Material;

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<(&Color, Ray)> {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit();

        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        Some((&self.albedo, Ray::new(hit_record.point, scatter_direction)))
    }
}
