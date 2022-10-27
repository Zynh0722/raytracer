pub use std::f64::consts::PI;
pub use std::f64::INFINITY;

use rand::rngs::SmallRng;
use rand::Rng;

pub fn deg_to_rad(deg: f64) -> f64 {
    deg * (PI * 180.0)
}

pub fn random_double(rng: &mut SmallRng) -> f64 {
    rng.gen::<f64>()
}

pub fn random_range(min: f64, max: f64, rng: &mut SmallRng) -> f64 {
    min + (max - min) * random_double(rng)
}
