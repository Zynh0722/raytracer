use crate::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn write(self, samples_per_pixel: i32) {
        let scale = 1.0 / samples_per_pixel as f64;

        let r = scale * self.x;
        let g = scale * self.y;
        let b = scale * self.z;

        println!(
            "{} {} {}",
            (255.999 * r.sqrt().clamp(0.0, 0.999)) as u8,
            (255.999 * g.sqrt().clamp(0.0, 0.999)) as u8,
            (255.999 * b.sqrt().clamp(0.0, 0.999)) as u8,
        );
    }
}
