pub mod dielectric;
pub mod diffuse_light;
pub mod lambertian;
pub mod metal;

use crate::{hittable::HitRecord, image::color::Color, ray::Ray, vec3::Vec3};

#[allow(unused_variables)]
pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        None
    }

    fn emitted(&self, uv: (f64, f64), pos: &Vec3) -> Color {
        Color::black()
    }
}
