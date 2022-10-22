use crate::vec3::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn write(self) {
        println!(
            "{} {} {}", 
            (255.999 * self.x) as u8,
            (255.999 * self.y) as u8,
            (255.999 * self.z) as u8,
        );
    }
}

