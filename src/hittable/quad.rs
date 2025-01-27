use std::rc::Rc;

use crate::{interval::Interval, material::Material, ray::Ray, vec3::Vec3};

use super::{aabb::AABB, HitRecord, Hittable};

pub struct Quad {
    corner: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    material: Rc<Material>,
    bounding_box: AABB,
    normal: Vec3,
    d: f64,
}

impl Quad {
    pub fn new(corner: Vec3, u: Vec3, v: Vec3, material: Rc<Material>) -> Self {
        let n = u.cross(&v);
        let normal = n.normalize();
        let d = normal.dot(&corner); // D part of plane: Ax + By + Cz = D

        let w = n / n.dot(&n);

        Self {
            corner,
            u,
            v,
            w,
            material,
            bounding_box: Self::find_bounding_box(corner, u, v),
            normal,
            d,
        }
    }

    fn find_bounding_box(corner: Vec3, u: Vec3, v: Vec3) -> AABB {
        let bounding_box_diagonal_1 = AABB::from_corners(corner, corner + u + v);
        let bounding_box_diagonal_2 = AABB::from_corners(corner + u, corner + v);

        AABB::from_aabbs(&bounding_box_diagonal_1, &bounding_box_diagonal_2)
    }

    fn is_interior(a: f64, b: f64, hit_record: &mut HitRecord) -> bool {
        let unit_interal = Interval::new(0.0, 1.0);

        if !unit_interal.contains(a) || !unit_interal.contains(b) {
            return false;
        }

        // Commented out is implementation for triangles
        // if a <= 0.0 || b <= 0.0 || a + b >= 1.0 {
        //     return false;
        // }

        hit_record.uv = (a, b);
        true
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let denom = self.normal.dot(&ray.direction);

        if denom.abs() < f64::EPSILON {
            return None;
        }

        let t = (self.d - self.normal.dot(&ray.origin)) / denom;
        if !ray_t.contains(t) {
            return None;
        }

        let intersection = ray.at(t);

        let mut hit_record = HitRecord::new(intersection, t, Rc::clone(&self.material), (0.0, 0.0));

        // If hit point is within the quad on the plane
        let planar_hit_vector = intersection - self.corner;
        let alpha = self.w.dot(&planar_hit_vector.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&planar_hit_vector));

        if !Self::is_interior(alpha, beta, &mut hit_record) {
            return None;
        }

        hit_record.set_face_normal(ray, self.normal);

        Some(hit_record)
    }

    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}
