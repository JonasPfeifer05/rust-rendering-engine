use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::{JoinHandle};
use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use crate::rendering::camera::Camera;
use crate::rendering::materials::lambertian::Lambertian;
use crate::rendering::materials::metal::Metal;
use crate::rendering::shapes::hittable::Hittable;
use crate::rendering::raytracing::{calculate_color, float_to_u8_color, gamma_correction, Pixel};
use crate::rendering::shapes::hittable_list::HittableList;
use crate::rendering::shapes::sphere::Sphere;
use crate::rendering::vector_math::Vec3;

pub struct RendererProperties {
    size: (u32,u32),
    samples: u32,
    max_bounces: u32,
    thread_percentage: f32,
}

impl Clone for RendererProperties {
    fn clone(&self) -> Self {
        RendererProperties {
            size: self.size,
            samples: self.samples,
            max_bounces: self.max_bounces,
            thread_percentage: self.thread_percentage
        }
    }
}

impl Copy for RendererProperties {

}

impl RendererProperties {
    pub fn new(size: (u32, u32), samples: u32, max_bounces: u32, thread_percentage: f32) -> Self {
        Self { size, samples, max_bounces, thread_percentage }
    }
}

pub struct Renderer {
    properties: RendererProperties,
}

impl Renderer {
    pub fn new(properties: RendererProperties) -> Self {
        Self { properties }
    }

    pub fn start_render(&mut self) -> (Arc<Mutex<Vec<Pixel>>>, Vec<JoinHandle<()>>, Arc<Mutex<bool>>) {
        let mut handles = Vec::new();

        let thread_amount = (self.properties.thread_percentage * num_cpus::get() as f32)as u32;

        let lines_remaining = Arc::new(Mutex::new(self.properties.size.1));
        let queue_mutex = Arc::new(Mutex::new(Vec::new()));
        let stop_mutex = Arc::new(Mutex::new(false));

        let size = self.properties.size;

        let camera = Camera::new(size, 2.0, 1.0);

        let samples = self.properties.samples;
        let max_depth = self.properties.max_bounces;

        for _ in 0..thread_amount {
            let lines_remaining = Arc::clone(&lines_remaining);
            let queue_mutex = Arc::clone(&queue_mutex);
            let stop_mutex = Arc::clone(&stop_mutex);

            let mat_ground = Box::new(Lambertian::new(Vec3(0.8,0.8,0.0)));
            let mat_center = Box::new(Lambertian::new(Vec3(0.7,0.3,0.3)));
            let mat_left = Box::new(Metal::new(Vec3(0.8,0.8,0.8), 0.3));
            let mat_right = Box::new(Metal::new(Vec3(0.8,0.6,0.2), 0.05));
            let mat_new = Box::new(Metal::new(Vec3(0.8,0.0,0.2), 0.01));

            handles.push(thread::spawn(move || {
                //TEMPORARY
                let mut world = HittableList::new();
                    world.add(Box::new(Sphere::new(Vec3(0.0,1.0,-2.0), 0.3, mat_new)));
                    world.add(Box::new(Sphere::new(Vec3(0.0,-100.5,-2.0), 100.0, mat_ground)));
                    world.add(Box::new(Sphere::new(Vec3(0.0,0.0,-2.0), 0.5,mat_center)));
                    world.add(Box::new(Sphere::new(Vec3(-1.0,0.0,-2.0), 0.5,mat_left)));
                    world.add(Box::new(Sphere::new(Vec3(1.0,0.0,-2.0), 0.5,mat_right)));

                let world: Box<dyn Hittable> = Box::new(world);

                let mut rng = rand::thread_rng();

                let mut temp = Vec::new();
                let parts = 10;
                let pixels_per_update = (size.0 as f32 / parts as f32).ceil() as i32;

                'render: loop {

                    if *stop_mutex.lock().unwrap() {
                        break 'render;
                    }

                    // If available, accept next job!
                    let y: u32;
                    {
                        let mut remaining = lines_remaining.lock().unwrap();
                        if *remaining <= 0 {
                            continue;
                        }
                        *remaining -= 1;

                        y = *remaining;
                    }

                    // Rendering Process
                    temp.clear();
                    for part in 0..parts {
                        for x in part*pixels_per_update..(part+1)*pixels_per_update {
                            if x as u32 >= size.0 { break; }

                            let mut color = Vec3(0.0, 0.0, 0.0);

                            for _ in 0..samples {
                                let u = (y as f32 + rng.gen::<f32>()) / (size.1 - 1) as f32;
                                let v = (x as f32 + rng.gen::<f32>()) / (size.0 - 1) as f32;

                                let ray = camera.get_ray(u,v);
                                color = color + calculate_color(&ray, &world, max_depth);
                            }

                            let real_color = gamma_correction(color, samples);
                            let real_color = float_to_u8_color(real_color);

                            temp.push(Pixel::new(Point::new(x as i32, y as i32), Color::RGB(real_color.0,real_color.1,real_color.2)));
                        }
                        {
                            let mut queue = queue_mutex.lock().unwrap();
                            queue.append(&mut temp);
                        }
                    }
                    // Outputting Process-Status

                    {
                        let remaining = lines_remaining.lock().unwrap();
                        print!("\rRendering at: {}%", (100.0 - *remaining as f32 / size.1 as f32 * 100.0) as u8);
                    }



                }
            }));
        }

        return (queue_mutex, handles, stop_mutex);
    }

    pub fn set_size(&mut self, size: (u32,u32)) {
        self.properties.size = size;
    }
}