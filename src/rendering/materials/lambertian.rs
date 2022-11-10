use crate::rendering::materials::material::Material;
use crate::rendering::ray::Ray;
use crate::rendering::shapes::hittable::HitRecord;
use crate::rendering::vector_math::Vec3;

pub struct Lambertian {
    color: Vec3,
}

impl Lambertian {
    pub fn new(color: Vec3) -> Self {
        Self { color }
    }
}

impl Clone for Lambertian {
    fn clone(&self) -> Self {
        Lambertian::new(self.color)
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit_record: &mut HitRecord) -> (bool, Ray, Vec3) {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        let scattered = Ray::new(hit_record.position, scatter_direction);
        let attenuation = self.color;

        (true, scattered, attenuation)
    }
}