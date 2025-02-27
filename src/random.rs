use nanorand::{tls_rng, Rng};

pub fn gen_f64() -> f64 {
    tls_rng().generate::<f64>()
}

pub fn gen_range_f64(min: f64, max: f64) -> f64 {
    tls_rng().generate::<f64>() * (max - min) + min
}
