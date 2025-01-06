use crate::{
    interval::{Interval, EMPTY},
    ray::Ray,
    vec3::Vec3,
};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AABB {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl AABB {
    pub fn empty() -> Self {
        Self {
            x: EMPTY,
            y: EMPTY,
            z: EMPTY,
        }
    }

    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn from_corners(a: Vec3, b: Vec3) -> Self {
        let x = if a.x <= b.x {
            Interval::new(a.x, b.x)
        } else {
            Interval::new(b.x, a.x)
        };

        let y = if a.y <= b.y {
            Interval::new(a.y, b.y)
        } else {
            Interval::new(b.y, a.y)
        };

        let z = if a.z <= b.z {
            Interval::new(a.z, b.z)
        } else {
            Interval::new(b.z, a.z)
        };

        Self { x, y, z }
    }

    pub fn from_aabbs(a: &Self, b: &Self) -> Self {
        Self {
            x: Interval::from_intervals(&a.x, &b.x),
            y: Interval::from_intervals(&a.y, &b.y),
            z: Interval::from_intervals(&a.z, &b.z),
        }
    }

    pub fn axis_interval(&self, n: usize) -> &Interval {
        if n == 0 {
            &self.x
        } else if n == 1 {
            &self.y
        } else {
            &self.z
        }
    }

    pub fn longest_axis(&self) -> usize {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() {
                0
            } else {
                2
            }
        } else {
            if self.y.size() > self.z.size() {
                1
            } else {
                2
            }
        }
    }

    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> bool {
        let ray_orig = ray.origin;
        let ray_dir = ray.direction;

        let mut ray_t = ray_t;

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let ad_inv = 1.0 / ray_dir.get(axis);

            let t0 = (ax.min - ray_orig.get(axis)) * ad_inv;
            let t1 = (ax.max - ray_orig.get(axis)) * ad_inv;

            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t.min = t0
                }
                if t1 < ray_t.max {
                    ray_t.max = t1
                }
            } else {
                if t1 > ray_t.min {
                    ray_t.min = t1;
                }
                if t0 < ray_t.max {
                    ray_t.max = t0;
                }
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }

        true
    }
}
