use raytracer::shapes::Sphere;
use raytracer::*;

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

fn ray_color(r: &Ray, world: &HittableList, depth: i32, rng: &mut SmallRng) -> Color {
    let mut rec = HitRecord::new();

    if depth <= 0 {
        return Color::new(0.0,0.0,0.0);
    }

    if world.hit(r, 0.001, INFINITY, &mut rec) {
        let target = rec.p + rec.normal + Vec3::random_in_unit_sphere(rng).normalized();
        return 0.5 * ray_color(&Ray::new(rec.p, target - rec.p), world, depth - 1, rng);
    }
    let unit_direction = r.direction.normalized();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Initialize RNG
    let mut rng = SmallRng::from_entropy();

    // Image

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width = 1280;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 1000;
    let max_depth = 50;

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

            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;

                let r = cam.get_ray(u, v);

                color += ray_color(&r, &world, max_depth, &mut rng);
            }

            color.write(samples_per_pixel);
        }
    }

    eprintln!("\nDone.");
}
