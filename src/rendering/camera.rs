use crate::rendering::ray::Ray;
use crate::rendering::vector_math::Vec3;

pub struct Camera {
    top_left: Vec3,
    origin: Vec3,
    vertical: Vec3,
    horizontal: Vec3,

    screen_size: (u32,u32),
    viewport_height: f32,
    focal_length: f32,
}

impl Camera {
    pub fn new(screen_size: (u32, u32), viewport_height: f32, focal_length: f32) -> Self {
        let aspect_ratio = screen_size.0 as f32 / screen_size.1 as f32;

        let viewport_width = viewport_height * aspect_ratio;

        let origin = Vec3(0.0,0.0,0.0);
        let horizontal = Vec3(viewport_width, 0.0, 0.0);
        let vertical = Vec3(0.0, viewport_height, 0.0);
        let top_left = origin - horizontal/2.0 + vertical/2.0 - Vec3(0.0, 0.0, focal_length);

        Self { top_left, origin, vertical, horizontal, screen_size, viewport_height, focal_length }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin, self.top_left - self.vertical*u + self.horizontal*v - self.origin)
    }
}

impl Clone for Camera {
    fn clone(&self) -> Self {
        Camera::new(self.screen_size, self.viewport_height,self.focal_length)
    }
}

impl Copy for Camera {}