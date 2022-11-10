use crate::rendering::ray::Ray;
use crate::rendering::shapes::hittable::HitRecord;
use crate::rendering::vector_math::Vec3;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &mut HitRecord) -> (bool, Ray, Vec3);
}