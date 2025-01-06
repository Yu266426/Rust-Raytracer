use crate::{hittable::HitRecord, image::color::Color, ray::Ray, vec3::Vec3};

use super::Material;

pub struct Dielectric {
    albedo: Color,
    // Ratio of material's ior over ior of emclosing media
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self {
            albedo: Color::white(),
            refraction_index,
        }
    }

    fn reflectance(cos: f64, refraction_index: f64) -> f64 {
        // Use Schlick's approximation for reflectance
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;

        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(&Color, Ray)> {
        let refraction_index = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray.direction.normalize();
        let cos = (-unit_direction).dot(&hit_record.normal).min(1.0);
        let sin = (1.0 - cos * cos).sqrt();

        let cannot_refract = refraction_index * sin > 1.0;

        let direction: Vec3;
        if cannot_refract || Self::reflectance(cos, refraction_index) > rand::random() {
            direction = unit_direction.reflected(&hit_record.normal);
        } else {
            direction = unit_direction
                .normalize()
                .refracted(&hit_record.normal, refraction_index);
        }

        Some((&self.albedo, Ray::new(hit_record.point, direction, ray.time)))
    }
}
