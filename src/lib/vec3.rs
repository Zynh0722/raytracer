use rand::rngs::SmallRng;
use rand::Rng;

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

    pub fn random(rng: &mut SmallRng) -> Self {
        Self {
            x: rng.gen::<f64>(),
            y: rng.gen::<f64>(),
            z: rng.gen::<f64>(),
        }
    }

    pub fn random_range(min: f64, max: f64, rng: &mut SmallRng) -> Self {
        Self {
            x: min + (max - min) * rng.gen::<f64>(),
            y: min + (max - min) * rng.gen::<f64>(),
            z: min + (max - min) * rng.gen::<f64>(),
        }
    }

    pub fn random_in_unit_sphere(rng: &mut SmallRng) -> Self {
        loop {
            let p = Self::random_range(-1.0, 1.0, rng);

            if p.length_squared() < 1.0 {
                break p;
            }
        }
    }

    pub fn null() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn cross(self, other: Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn normalize(&mut self) {
        *self = self.normalized();
    }

    pub fn normalized(&self) -> Self {
        *self / self.length()
    }
}

use std::ops;

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self::new(self.x + other.x, self.y + other.y, self.z + other.z);
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        Self::new(self.x * other, self.y * other, self.z * other)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Self::Output) -> Self::Output {
        other * self
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, other: f64) -> Self::Output {
        (1.0 / other) * self
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}
