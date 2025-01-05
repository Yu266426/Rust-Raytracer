pub mod sphere;

use std::rc::Rc;

use crate::{interval::Interval, material::Material, ray::Ray, vec3::Vec3};

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    fn new(ray: &Ray, t: f64, material: Rc<dyn Material>) -> Self {
        Self {
            point: ray.at(t),
            normal: Vec3::zero(),
            material,
            t,
            front_face: false,
        }
    }

    fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
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

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;
}

pub struct HittableList<'a> {
    objects: Vec<Rc<dyn Hittable + 'a>>,
}

#[allow(dead_code)]
impl<'a> HittableList<'a> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<dyn Hittable + 'a>) {
        self.objects.push(object);
    }
}

impl<'a> Hittable for HittableList<'a> {
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
}
