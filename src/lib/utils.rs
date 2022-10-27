pub use std::f64::consts::PI;
pub use std::f64::INFINITY;

use rand::prelude::*;

pub fn deg_to_rad(deg: f64) -> f64 {
    deg * (PI * 180.0)
}

pub fn random_double() -> f64 {
    random::<f64>()
}

pub fn random_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}
