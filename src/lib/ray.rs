use crate::Vec3;
use crate::Point3;

#[derive(Debug)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin: origin, direction: direction }
    }

    pub fn from_direction(direction: Vec3) -> Self {
        Self { origin: Point3::null(), direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + (t * self.direction)
    }
}