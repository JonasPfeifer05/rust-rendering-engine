use crate::rendering::materials::material::Material;
use crate::rendering::ray::Ray;
use crate::rendering::shapes::hittable::HitRecord;
use crate::rendering::vector_math::Vec3;

pub struct Metal {
    color: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(color: Vec3, fuzz: f32) -> Self {
        Self { color, fuzz }
    }
}

impl Clone for Metal {
    fn clone(&self) -> Self {
        return Metal::new(self.color, self.fuzz);
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &mut HitRecord) -> (bool, Ray, Vec3) {
        let reflected = Vec3::reflect(&ray.direction().unit(), &hit_record.normal);
        let scatter = Ray::new(hit_record.position, reflected + self.fuzz*Vec3::random_in_unit_sphere());
        let color = self.color;
        (scatter.direction() * hit_record.normal > 0.0, scatter, color)
    }
}