use raytracer::*;

fn hit_sphere(center: &Point3, rad: f64, r: &Ray) -> bool {
    let oc: Vec3 = r.origin - *center;

    // woooooo quadratic formula
    let a = Vec3::dot(r.direction, r.direction);
    let b = 2.0 * Vec3::dot(oc, r.direction);
    let c = Vec3::dot(oc, oc) - rad * rad;

    // woooooo quadratic formula again
    let discriminant = b * b - 4.0 * a * c;

    discriminant > 0.0
}

fn ray_color(r: &Ray) -> Color {
    if hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Color::new(1.0, 0.0, 0.0);
    }

    let unit_direction = r.direction.normalized();
    let t = 0.5 * (unit_direction.y + 1.0);

    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width = 800;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // Camera

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::null();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height - 1).rev() {
        if j % 100 == 0 {
            eprintln!(
                "\rCompleted: {:.2} %",
                (image_height - j) as f32 / image_height as f32 * 100.0
            );
        }

        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;

            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );

            ray_color(&r).write();
        }
    }

    eprintln!("\nDone.");
}
