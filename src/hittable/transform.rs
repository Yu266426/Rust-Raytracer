use std::sync::Arc;

use crate::{interval::Interval, ray::Ray, vec3::Vec3};

use super::{aabb::AABB, HitRecord, Hittable};

pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vec3,
    bounding_box: AABB,
}

impl Translate {
    pub fn new(object: Arc<dyn Hittable>, offset: Vec3) -> Self {
        let bbox = object.bounding_box() + offset;
        Self {
            object,
            offset,
            bounding_box: bbox,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let offset_ray = Ray::new(ray.origin - self.offset, ray.direction, ray.time);

        let hit = self.object.hit(&offset_ray, ray_t);

        if let Some(mut hit) = hit {
            hit.pos += self.offset;
            return Some(hit);
        }

        None
    }

    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}

pub struct RotateY {
    object: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bounding_box: AABB,
}

impl RotateY {
    pub fn new(object: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = angle.to_radians();

        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let bounding_box = object.bounding_box().clone();

        let mut min = Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Vec3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bounding_box.x.max + (1 - i) as f64 * bounding_box.x.min;
                    let y = j as f64 * bounding_box.y.max + (1 - j) as f64 * bounding_box.y.min;
                    let z = k as f64 * bounding_box.z.max + (1 - k) as f64 * bounding_box.z.min;

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    min.x = min.x.min(new_x);
                    max.x = max.x.max(new_x);

                    min.y = min.y.min(y);
                    max.y = max.y.max(y);

                    min.z = min.z.min(new_z);
                    max.z = max.z.max(new_z);
                }
            }
        }

        Self {
            object,
            sin_theta,
            cos_theta,
            bounding_box: AABB::from_corners(min, max),
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let origin = Vec3::new(
            (self.cos_theta * ray.origin.x) - (self.sin_theta * ray.origin.z),
            ray.origin.y,
            (self.sin_theta * ray.origin.x) + (self.cos_theta * ray.origin.z),
        );

        let direction = Vec3::new(
            (self.cos_theta * ray.direction.x) - (self.sin_theta * ray.direction.z),
            ray.direction.y,
            (self.sin_theta * ray.direction.x) + (self.cos_theta * ray.direction.z),
        );

        let rotated_ray = Ray::new(origin, direction, ray.time);

        let hit = self.object.hit(&rotated_ray, ray_t);

        if let Some(mut hit) = hit {
            hit.pos = Vec3::new(
                (self.cos_theta * hit.pos.x) + (self.sin_theta * hit.pos.z),
                hit.pos.y,
                (-self.sin_theta * hit.pos.x) + (self.cos_theta * hit.pos.z),
            );

            hit.normal = Vec3::new(
                (self.cos_theta * hit.normal.x) + (self.sin_theta * hit.normal.z),
                hit.normal.y,
                (-self.sin_theta * hit.normal.x) + (self.cos_theta * hit.normal.z),
            );

            return Some(hit);
        }

        None
    }

    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}
