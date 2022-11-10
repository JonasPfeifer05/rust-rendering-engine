use crate::rendering::materials::material::Material;
use crate::rendering::shapes::hittable::{HitRecord, Hittable};
use crate::rendering::ray::Ray;
use crate::rendering::vector_math::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Box<dyn Material>) -> Self {
        Self { center, radius, material }
    }
}

impl Hittable for Sphere {
    fn hit<'a, 'b>(&'a self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord<'b>) -> bool where 'a: 'b {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = oc * ray.direction();
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrt = discriminant.sqrt();
        let mut root = (-half_b - sqrt) / a;

        if root < t_min || root > t_max {
            root = (-half_b + sqrt) / a;
            if root < t_min || root > t_max {
                return false;
            }
        }

        record.t = root;
        record.position = ray.at(root);
        let outward_normal= (record.position - self.center) / self.radius;
        record.set_face_normal(ray, outward_normal);
        record.material = Some(self.material.as_ref());

        true
    }
}