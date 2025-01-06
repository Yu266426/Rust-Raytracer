use core::f64;

pub const EMPTY: Interval = Interval {
    min: f64::INFINITY,
    max: f64::NEG_INFINITY,
};

pub const ALL: Interval = Interval {
    min: f64::NEG_INFINITY,
    max: f64::INFINITY,
};

#[derive(Debug, Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

#[allow(dead_code)]
impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn from_intervals(a: &Self, b: &Self) -> Self {
        let min = if a.min <= b.min { a.min } else { b.min };
        let max = if a.max >= b.max { a.max } else { b.max };

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

    pub fn clamp(&self, val: f64) -> f64 {
        val.clamp(self.min, self.max)
    }

    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.0;

        Self {
            min: self.min - padding,
            max: self.max + padding,
        }
    }
}
