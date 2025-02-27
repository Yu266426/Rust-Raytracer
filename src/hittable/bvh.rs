use std::{cmp::Ordering, sync::Arc};

use crate::{interval::Interval, ray::Ray};

use super::{aabb::AABB, HitRecord, Hittable, HittableList};

pub struct BVHNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bounding_box: AABB,
}

impl BVHNode {
    pub fn new(mut objects: Vec<Arc<dyn Hittable>>, start: usize, end: usize) -> Self {
        let mut bounding_box = AABB::empty();
        for object in &objects[start..end] {
            bounding_box = AABB::from_aabbs(&bounding_box, object.bounding_box());
        }

        let axis = bounding_box.longest_axis();

        let comparator = if axis == 0 {
            Self::box_x_compare
        } else if axis == 1 {
            Self::box_y_compare
        } else {
            Self::box_z_compare
        };

        let object_span = end - start;

        let left: Arc<dyn Hittable>;
        let right: Arc<dyn Hittable>;

        if object_span == 1 {
            left = Arc::clone(&objects[start]);
            right = Arc::clone(&left);
        } else if object_span == 2 {
            left = Arc::clone(&objects[start]);
            right = Arc::clone(&objects[start + 1]);
        } else {
            objects[start..end].sort_by(comparator);

            let mid = start + object_span / 2;

            left = Arc::new(Self::new(objects.clone(), start, mid));
            right = Arc::new(Self::new(objects.clone(), mid, end));
        }

        Self {
            left,
            right,
            bounding_box,
        }
    }

    pub fn from_hittable_list(list: HittableList) -> Self {
        let num_objects = list.objects.len();
        Self::new(list.objects.clone(), 0, num_objects)
    }

    fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis_index: usize) -> Ordering {
        let a_axis_interval = a.bounding_box().axis_interval(axis_index);
        let b_axis_interval = b.bounding_box().axis_interval(axis_index);

        a_axis_interval.min.total_cmp(&b_axis_interval.min)
    }

    fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 0)
    }

    fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 1)
    }

    fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 2)
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        if !self.bounding_box.hit(ray, ray_t.clone()) {
            return None;
        }

        let hit_left = self.left.hit(ray, ray_t.clone());
        let hit_right = self.right.hit(
            ray,
            Interval::new(
                ray_t.min,
                if let Some(rec) = &hit_left {
                    rec.t
                } else {
                    ray_t.max
                },
            ),
        );

        if let Some(_) = hit_right {
            hit_right
        } else if let Some(_) = hit_left {
            hit_left
        } else {
            None
        }
    }

    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}
