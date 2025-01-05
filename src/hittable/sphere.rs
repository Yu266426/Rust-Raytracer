use std::rc::Rc;

use crate::{interval::Interval, material::Material, vec3::Vec3};

use super::{HitRecord, Hittable};

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &crate::ray::Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.center - ray.origin;

        // Quadratic formula for ray sphere intersection
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        // Discriminant >= 0 -> intersection (= 0 -> 1 intersection, > 0 -> 2 intersections)
        let discriminant_sqrt = discriminant.sqrt();

        let mut root = (h - discriminant_sqrt) / a;
        // If one root is not a hit, try the other
        if !ray_t.surrounds(root) {
            root = (h + discriminant_sqrt) / a;

            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let mut hit_record = HitRecord::new(ray, root, Rc::clone(&self.material));
        hit_record.set_face_normal(ray, (hit_record.point - self.center) / self.radius);

        Some(hit_record)
    }
}
