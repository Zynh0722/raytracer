use raytracer::Point3;
use raytracer::Ray;
use raytracer::Vec3;

fn make_ray() -> Ray {
    let origin = Point3::null();
    let direction = Vec3::new(0.0, 2.0, 0.0).normalized();

    Ray::new(origin, direction)
}

#[test]
fn ray_new() {
    let ray = make_ray();

    assert_eq!(ray.origin.x, 0.0);
    assert_eq!(ray.origin.y, 0.0);
    assert_eq!(ray.origin.z, 0.0);

    assert_eq!(ray.direction.x, 0.0);
    assert_eq!(ray.direction.y, 1.0);
    assert_eq!(ray.direction.z, 0.0);
}

#[test]
fn ray_at() {
    let ray = make_ray();

    let ray_at_1 = Point3::new(0.0, 1.0, 0.0);
    let ray_at_5 = Point3::new(0.0, 5.0, 0.0);

    assert_eq!(ray.at(1.0), ray_at_1);
    assert_eq!(ray.at(5.0), ray_at_5);
}
