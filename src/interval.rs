use core::f64;

#[allow(dead_code)]
const EMPTY: Interval = Interval {
    min: f64::INFINITY,
    max: f64::NEG_INFINITY,
};

#[allow(dead_code)]
const ALL: Interval = Interval {
    min: f64::NEG_INFINITY,
    max: f64::INFINITY,
};

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

#[allow(dead_code)]
impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, val: f64) -> bool {
        self.min <= val && val <= self.max
    }

    pub fn surrounds(&self, val: f64) -> bool {
        self.min < val && val < self.max
    }
}
