use raytracer::Vec3;
use raytracer::Color;

fn main() {
    let _uwu = Vec3::new(1.0, 2.0, 3.0);

    let image_width = 218;
    let image_height = 218;

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height-1).rev() {
        if j % 100 == 0 {
            eprintln!("\rCompleted: {:.2} %", (image_height - j) as f32 / image_height as f32 * 100.0);
        }

        for i in 0..image_width {
            let color = Color::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                25.0
            );

            color.write();
        }
    }

    eprintln!("\nDone.");
}
