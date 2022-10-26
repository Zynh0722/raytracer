use crate::HitRecord;
use crate::Hittable;
use crate::Ray;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: vec![] }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj);
    }
}

impl Hittable for HittableList {
    fn hit(&self, _r: &Ray, _t_min: f64, _t_max: f64, _rec: &mut HitRecord) -> bool {
        true
    }
}
