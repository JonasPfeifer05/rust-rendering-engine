use sdl2::pixels::Color;
use sdl2::rect::Point;
use crate::rendering::shapes::hittable::{HitRecord, Hittable};
use crate::rendering::ray::Ray;
use crate::rendering::vector_math::Vec3;

pub struct Pixel {
    pub position: Point,
    pub color: Color,
}

impl Pixel {
    pub fn new(position: Point, color: Color) -> Self {
        Self { position, color }
    }
}

pub fn calculate_color(ray: &Ray, hittable: &Box<dyn Hittable>, depth: u32) -> Vec3 {
    if depth <= 0 {
        return Vec3(0.0,0.0,0.0);
    }

    let mut hit_record = HitRecord::new();
    let color_val;
    if hittable.hit(ray, 0.001, f32::INFINITY, &mut hit_record) {
        let scatter = hit_record.material.unwrap().scatter(ray, &mut hit_record);
        if scatter.0 {
            color_val = scatter.2.mul(&calculate_color(&scatter.1, hittable, depth - 1));
        } else {
            color_val = Vec3(0.0,0.0,0.0);
        }
    } else {
        let unit_vector = ray.direction().unit();
        let t = 0.5 * (unit_vector.1 + 1.0);
        color_val = Vec3(1.0, 1.0, 1.0) * (1.0 - t) + Vec3(0.5, 0.7, 1.0) * t;
    }

    return color_val;
}

pub fn float_to_u8_color(float_color: Vec3) -> (u8,u8,u8) {
    ((float_color.0 * 255.0) as u8, (float_color.1 * 255.0) as u8, (float_color.2 * 255.0) as u8)
}

pub fn gamma_correction(color: Vec3, samples: u32) -> Vec3 {
    let mut ret: Vec3 = Vec3(0.0,0.0,0.0);

    let scale = 1.0 / samples as f32;

    ret.0 = (color.0 * scale).sqrt();
    ret.1 = (color.1 * scale).sqrt();
    ret.2 = (color.2 * scale).sqrt();

    ret
}