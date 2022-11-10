use std::path::Path;
use sdl2::event::{Event, WindowEvent};

use crate::rendering::renderer::{Renderer, RendererProperties};
use crate::visual::display::Display;

mod visual;
mod rendering;

pub struct Application {
    display: Display,
    renderer: Renderer,
}

impl Application {
    pub fn new(width: u32, height: u32) -> Result<Self, String> {
        Ok(Application {
            display: Display::new(width, height)?,
            renderer: Renderer::new(RendererProperties::new((width, height), 100, 8, 2.0)),
        })
    }

    fn render_loop(&mut self) {
        println!("Close Window to stop the Program and save the current frame!");
        let (mut queue_mutex, mut handles, mut stop) = self.renderer.start_render();

        let context = self.display.get_context();
        let mut events = context.event_pump().unwrap();

        'main: loop {
            for event in events.poll_iter() {
                match event {
                    Event::Window {win_event: WindowEvent::Close, ..} => {
                        *stop.lock().unwrap() = true;
                        break 'main;
                    }
                    Event::Window {win_event: WindowEvent::Resized(width, height), ..} => {
                        *stop.lock().unwrap() = true;
                        self.display.clear();
                        self.renderer.set_size((width as u32, height as u32));
                        (queue_mutex, handles, stop) = self.renderer.start_render();
                    }
                    _ => {}
                }
            }
            {
                let mut queue = queue_mutex.lock().unwrap();
                if !queue.is_empty() {
                    while !queue.is_empty() {
                        let pixel = queue.pop().unwrap();
                        self.display.add_pixel(pixel);
                    }

                    self.display.update_pixels();
                }
            }
        }
    }

    fn save_to_image(&mut self) {
        println!("\nSaving image...");
        let buffer = self.display.generate_image();

        let _ = image::save_buffer(&Path::new("output.png"), &buffer, self.display.get_size().0, self.display.get_size().1, image::ColorType::Rgba8).unwrap();
        println!("finished saving the image!");
    }

    pub fn start(&mut self) {
        self.render_loop();
        self.save_to_image();
    }
}