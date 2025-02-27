use std::rc::Rc;

use crate::{
    image::color::Color, interval::Interval, material::Material, random::gen_f64, ray::Ray,
    texture::Texture,
};

use super::{aabb::AABB, HitRecord, Hittable};

pub struct ConstantMedium {
    boundary: Rc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Rc<Material>,
}

impl ConstantMedium {
    pub fn new(boundary: Rc<dyn Hittable>, density: f64, texture: Rc<Texture>) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Rc::new(Material::isotropic(texture)),
        }
    }

    pub fn from_color(boundary: Rc<dyn Hittable>, density: f64, albedo: Color) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Rc::new(Material::isotropic_from_color(albedo)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut hit_1 = self.boundary.hit(ray, Interval::ALL)?;
        let mut hit_2 = self
            .boundary
            .hit(ray, Interval::new(hit_1.t + 0.0001, f64::INFINITY))?;

        hit_1.t = hit_1.t.max(ray_t.min);
        hit_2.t = hit_2.t.min(ray_t.max);

        if hit_1.t >= hit_2.t {
            return None;
        }

        hit_1.t = hit_1.t.max(0.0);

        let ray_length = ray.direction.length();
        let distance_inside_boundary = (hit_2.t - hit_1.t) * ray_length;
        let hit_distance = self.neg_inv_density * gen_f64().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = hit_1.t + hit_distance / ray_length;
        let hit = HitRecord::new(ray.at(t), t, Rc::clone(&self.phase_function), (0.0, 0.0));

        Some(hit)
    }

    fn bounding_box(&self) -> &AABB {
        self.boundary.bounding_box()
    }
}
