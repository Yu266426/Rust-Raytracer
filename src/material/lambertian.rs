use std::rc::Rc;

use crate::{
    hittable::HitRecord,
    image::color::Color,
    ray::Ray,
    texture::{solid_color::SolidColor, Texture},
    vec3::Vec3,
};

use super::Material;

pub struct Lambertian {
    texture: Rc<dyn Texture>,
}

impl Lambertian {
    pub fn new(texture: Rc<dyn Texture>) -> Self {
        Self { texture }
    }

    pub fn from_color(albedo: Color) -> Self {
        Self {
            texture: Rc::new(SolidColor::new(albedo)),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit();

        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        Some((
            self.texture.value(hit_record.uv, &hit_record.pos),
            Ray::new(hit_record.pos, scatter_direction, ray.time),
        ))
    }
}
