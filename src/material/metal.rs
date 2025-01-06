use crate::{hittable::HitRecord, image::color::Color, ray::Ray, vec3::Vec3};

use super::Material;

pub struct Metal {
    albedo: Color,
    roughness: f64,
}

impl Metal {
    pub fn new(albedo: Color, roughness: f64) -> Self {
        Self { albedo, roughness }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(&Color, Ray)> {
        let mut reflected = ray.direction.reflected(&hit_record.normal);

        reflected = reflected.normalize() + (self.roughness * Vec3::random_unit());

        if reflected.dot(&hit_record.normal) > 0.0 {
            Some((&self.albedo, Ray::new(hit_record.point, reflected, ray.time)))
        } else {
            None
        }
    }
}
