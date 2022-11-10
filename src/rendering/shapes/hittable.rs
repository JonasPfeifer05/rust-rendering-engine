use crate::rendering::ray::Ray;
use crate::rendering::vector_math::Vec3;

pub struct HitRecord {
    pub position: Vec3,
    pub normal: Vec3,
    pub t: f32,

    pub font_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.font_face = ray.direction() * outward_normal < 0.0;
        if self.font_face {
            self.normal = outward_normal;
        } else {
            self.normal = -1.0 * outward_normal;
        }
    }
    pub fn new() -> Self {
        Self { position: Vec3(0.0,0.0,0.0), normal: Vec3(0.0,0.0,0.0), t: 0.0, font_face: false }
    }
}

impl Clone for HitRecord {
    fn clone(&self) -> Self {
        HitRecord {t: self.t, font_face: self.font_face, normal: self.normal, position: self.position}
    }
}

impl Copy for HitRecord {
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool;
}