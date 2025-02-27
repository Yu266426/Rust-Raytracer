use std::{f64::consts::PI, rc::Rc};

use crate::{interval::Interval, material::Material, ray::Ray, vec3::Vec3};

use super::{aabb::AABB, HitRecord, Hittable};

pub struct Sphere {
    center: Ray,
    radius: f64,
    material: Rc<Material>,
    bounding_box: AABB,
}

impl Sphere {
    pub fn still(center: Vec3, radius: f64, material: Rc<Material>) -> Self {
        let r_vec = Vec3::new(radius, radius, radius);

        Self {
            center: Ray::new(center, Vec3::zero(), 0.0),
            radius: radius.max(0.0),
            material,
            bounding_box: AABB::from_corners(center - r_vec, center + r_vec),
        }
    }

    pub fn moving(start: Vec3, end: Vec3, radius: f64, material: Rc<Material>) -> Self {
        let r_vec = Vec3::new(radius, radius, radius);

        let box_1 = AABB::from_corners(start - r_vec, start + r_vec);
        let box_2 = AABB::from_corners(end - r_vec, end + r_vec);

        Self {
            center: Ray::new(start, end - start, 0.0),
            radius,
            material,
            bounding_box: AABB::from_aabbs(&box_1, &box_2),
        }
    }

    fn get_sphere_uv(normal: &Vec3) -> (f64, f64) {
        let theta = (-normal.get_y()).acos();
        let phi = (-normal.get_z()).atan2(normal.get_x()) + PI;

        (phi / (2.0 * PI), theta / PI)
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let center = self.center.at(ray.time);

        let oc = center - ray.origin;

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

        let hit_pos = ray.at(root);
        let outward_normal = (hit_pos - center) / self.radius;

        let uv = Self::get_sphere_uv(&outward_normal);
        let mut hit_record = HitRecord::new(hit_pos, root, Rc::clone(&self.material), uv);
        hit_record.set_face_normal(ray, (hit_record.pos - center) / self.radius);

        Some(hit_record)
    }

    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}
