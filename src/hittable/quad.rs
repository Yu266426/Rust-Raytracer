use std::sync::Arc;

use crate::{interval::Interval, material::Material, ray::Ray, vec3::Vec3};

use super::{aabb::AABB, HitRecord, Hittable, HittableList};

pub struct Quad {
    corner: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    material: Arc<Material>,
    bounding_box: AABB,
    normal: Vec3,
    d: f64
}

pub fn quad_box(a: Vec3, b: Vec3, material: Arc<Material>) -> Arc<HittableList> {
    let mut sides = HittableList::new();

    let min = Vec3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
    let max = Vec3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));

    let dx = Vec3::new(max.x - min.x, 0.0, 0.0);
    let dy = Vec3::new(0.0, max.y - min.y, 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z - min.z);

    sides.add(Arc::new(Quad::new(
        Vec3::new(min.x, min.y, max.z),
        dx,
        dy,
        Arc::clone(&material),
    )));
    sides.add(Arc::new(Quad::new(
        Vec3::new(max.x, min.y, max.z),
        -dz,
        dy,
        Arc::clone(&material),
    )));
    sides.add(Arc::new(Quad::new(
        Vec3::new(max.x, min.y, min.z),
        -dx,
        dy,
        Arc::clone(&material),
    )));
    sides.add(Arc::new(Quad::new(
        Vec3::new(min.x, min.y, min.z),
        dz,
        dy,
        Arc::clone(&material),
    )));
    sides.add(Arc::new(Quad::new(
        Vec3::new(min.x, max.y, max.z),
        dx,
        -dz,
        Arc::clone(&material),
    )));
    sides.add(Arc::new(Quad::new(
        Vec3::new(min.x, min.y, min.z),
        dx,
        dz,
        Arc::clone(&material),
    )));

    Arc::new(sides)
}

impl Quad {
    pub fn new(corner: Vec3, u: Vec3, v: Vec3, material: Arc<Material>) -> Self {
        let n = u.cross(&v);
        let normal = n.normalize();
        let d = normal.dot(&corner);
        let w = n / n.dot(&n);

        Self {
            corner,
            u,
            v,
            w,
            material,
            bounding_box: Self::find_bounding_box(corner, u, v),
            normal,
            d
        }
    }

    fn find_bounding_box(corner: Vec3, u: Vec3, v: Vec3) -> AABB {
        let bounding_box_diagonal_1 = AABB::from_corners(corner, corner + u + v);
        let bounding_box_diagonal_2 = AABB::from_corners(corner + u, corner + v);

        AABB::from_aabbs(&bounding_box_diagonal_1, &bounding_box_diagonal_2)
    }

    fn is_interior(a: f64, b: f64, hit_record: &mut HitRecord) -> bool {
        // Commented out is implementation for triangles
        // if a <= 0.0 || b <= 0.0 || a + b >= 1.0 {
        //     return false;
        // }

        if a < 0.0 || a > 1.0 || b < 0.0 || b > 1.0 {
            return false;
        }

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

        let mut hit_record =
            HitRecord::new(intersection, t, Arc::clone(&self.material), (0.0, 0.0));

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
