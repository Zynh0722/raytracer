pub use std::f64::consts::PI;
pub use std::f64::INFINITY;

pub fn deg_to_rad(deg: f64) -> f64 {
    deg * (PI * 180.0)
}
