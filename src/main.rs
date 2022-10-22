use raytracer::Vec3;

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
            let r = i as f32 / (image_width - 1) as f32;
            let g = j as f32 / (image_height - 1) as f32;
            let b = 255.0;
         
            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;

            println!("{} {} {}", ir, ig, ib)
        }
    }

    eprintln!("\nDone.");
}
