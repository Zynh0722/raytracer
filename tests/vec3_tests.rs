use raytracer::Vec3;

#[test]
fn test_vec3_eq() {
    let vec_a = Vec3::new(1.0, 2.0, 3.0);
    let vec_b = Vec3::new(1.0, 2.0, 3.0);

    assert_eq!(vec_a, vec_b);
}

#[test]
fn test_vec3_add() {
    let vec_a = Vec3::new(1.0, 2.0, 3.0);
    let vec_b = Vec3::new(3.0, 2.0, 1.0);

    let vec_sol = Vec3::new(4.0, 4.0, 4.0);

    assert_eq!(vec_a + vec_b, vec_sol);
}

#[test]
fn test_vec3_add_assign() {
    let mut vec_a = Vec3::new(1.0, 2.0, 3.0);
    let vec_b = Vec3::new(3.0, 2.0, 1.0);

    vec_a += vec_b;

    let vec_sol = Vec3::new(4.0, 4.0, 4.0);

    assert_eq!(vec_a, vec_sol);
}

#[test]
fn test_vec3_sub() {
    let vec_a = Vec3::new(1.0, 2.0, 3.0);
    let vec_b = Vec3::new(3.0, 2.0, 1.0);

    let vec_sol = Vec3::new(-2.0, 0.0, 2.0);

    assert_eq!(vec_a - vec_b, vec_sol);
}

#[test]
fn test_vec3_sub_assign() {
    let mut vec_a = Vec3::new(1.0, 2.0, 3.0);
    let vec_b = Vec3::new(3.0, 2.0, 1.0);

    vec_a -= vec_b;

    let vec_sol = Vec3::new(-2.0, 0.0, 2.0);

    assert_eq!(vec_a, vec_sol);
}

#[test]
fn test_vec3_mul_vec3() {
    let vec_a = Vec3::new(1.0, 2.0, 3.0);
    let vec_b = Vec3::new(3.0, 2.0, 1.0);

    let vec_sol = Vec3::new(3.0, 4.0, 3.0);

    assert_eq!(vec_a * vec_b, vec_sol);
}

#[test]
fn test_vec3_mul_f64() {
    let vec_a = Vec3::new(1.0, 2.0, 3.0);
    let scalar = 3.0;

    let vec_sol = Vec3::new(3.0, 6.0, 9.0);

    assert_eq!(vec_a * scalar, vec_sol);
    // Test for communative
    assert_eq!(scalar * vec_a, vec_sol);
}

#[test]
fn test_vec3_length_squared() {
    let unit_vec = Vec3::new(1.0, 0.0, 0.0);
    let vec = Vec3::new(0.0, 4.0, 3.0);

    assert_eq!(unit_vec.length_squared(), 1.0);
    assert_eq!(vec.length_squared(), 25.0);
}

#[test]
fn test_vec3_length() {
    let unit_vec = Vec3::new(1.0, 0.0, 0.0);
    let vec = Vec3::new(0.0, 4.0, 3.0);

    assert_eq!(unit_vec.length(), 1.0);
    assert_eq!(vec.length(), 5.0);
}

#[test]
fn test_vec3_neg() {
    let vec = Vec3::new(1.0, 2.0, 3.0);
    
    let vec_sol = Vec3::new(-1.0, -2.0, -3.0);

    assert_eq!(-vec, vec_sol);
}