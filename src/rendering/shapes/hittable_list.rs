use crate::rendering::shapes::hittable::{HitRecord, Hittable};
use crate::rendering::ray::Ray;

pub struct HittableList {
    obejcts: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { obejcts: Vec::new() }
    }

    pub fn clear(&mut self) {
        self.obejcts.clear();
    }
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.obejcts.push(object);
    }
}

impl Hittable for HittableList {
    fn hit<'a, 'b>(&'a self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord<'b>) -> bool where 'a: 'b{
        let mut temp = HitRecord::new();
        let mut hit_anything = false;
        let mut closest = t_max;

        for object in &self.obejcts {
            if object.hit(ray, t_min, closest, &mut temp) {
                hit_anything = true;
                closest = temp.t.clone();
                *record = temp.clone();
            }
        }

        hit_anything
    }
}