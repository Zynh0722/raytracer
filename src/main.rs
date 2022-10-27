use raytracer::shapes::Sphere;
use raytracer::*;

use rand::{thread_rng, Rng};

fn ray_color(r: &Ray, world: &HittableList) -> Color {
    let mut rec = HitRecord::new();
    if world.hit(r, 0.0, INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }
    let unit_direction = r.direction.normalized();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;

    // World

    let mut world = HittableList::new();
    world.add(Sphere::new_boxxed(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new_boxxed(Point3::new(0.0, -100.5, 0.0), 100.0));

    // Camera

    let cam = Camera::new();

    // Render

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height - 1).rev() {
        if j % 10 == 0 {
            eprintln!(
                "\rCompleted: {:.2} %",
                (image_height - j) as f32 / image_height as f32 * 100.0
            );
        }

        for i in 0..image_width {
            let mut color = Color::new(0.0, 0.0, 0.0);
            let mut rng = thread_rng();

            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;

                let r = cam.get_ray(u, v);

                color += ray_color(&r, &world);
            }

            color.write(samples_per_pixel);
        }
    }

    eprintln!("\nDone.");
}
