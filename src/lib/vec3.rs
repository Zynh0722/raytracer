#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x: x, y: y, z: z }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }
}

use std::ops;

impl ops::Add for Vec3 {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Self::new(
            self.x + other.x, 
            self.y + other.y, 
            self.z + other.z,
        )
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
        );
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
        )
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
        )
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::new(
            self.x * other.x,
            self.y * other.y,
            self.z * other.z,
        )
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self::new(
            self.x * other,
            self.y * other,
            self.z * other,
        )
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Self::Output) -> Self::Output {
        other * self
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(
            -self.x,
            -self.y,
            -self.z,
        )
    }
}
