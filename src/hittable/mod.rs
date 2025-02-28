mod aabb;
pub mod bvh;
pub mod constant_medium;
pub mod quad;
pub mod sphere;
pub mod transform;

use std::sync::Arc;

use aabb::AABB;
use bvh::BVHNode;

use crate::{interval::Interval, material::Material, ray::Ray, vec3::Vec3};

pub struct HitRecord {
    pub pos: Vec3,
    pub normal: Vec3,
    pub material: Arc<Material>,
    pub t: f64,
    pub uv: (f64, f64),
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(pos: Vec3, t: f64, material: Arc<Material>, uv: (f64, f64)) -> Self {
        Self {
            pos,
            normal: Vec3::zero(),
            material,
            t,
            uv,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        // Sets the hit record normal
        // Outward normal is assumed to be unit length

        self.front_face = ray.direction.dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;

    fn bounding_box(&self) -> &AABB;
}

pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
    bounding_box: AABB,
}

#[allow(dead_code)]
impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bounding_box: AABB::empty(),
        }
    }

    pub fn from_object(object: Arc<dyn Hittable>) -> Self {
        let mut objects = Vec::new();
        objects.push(object);

        Self {
            objects,
            bounding_box: AABB::empty(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.bounding_box = AABB::from_aabbs(&self.bounding_box, object.bounding_box());
        self.objects.push(object);
    }

    pub fn with_bvh(self) -> Self {
        Self::from_object(Arc::new(BVHNode::from_hittable_list(self)))
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut hit_record = None;

        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if let Some(temp_record) = object.hit(ray, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = temp_record.t;
                hit_record = Some(temp_record);
            }
        }

        hit_record
    }

    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}
