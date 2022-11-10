use std::sync::{Arc, Mutex};
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::render::WindowCanvas;
use sdl2::Sdl;
use crate::rendering::raytracing::Pixel;

pub struct Display {
    sdl_context: Box<Sdl>,
    canvas: WindowCanvas,
    draw_array: Arc<Mutex<Vec<Pixel>>>,
}

impl<'a> Display {
    pub fn new(width: u32, height: u32) -> Result<Self, String> {
        let context = sdl2::init()?;
        let video_subsys = context.video()?;

        let window = video_subsys.
            window(
                "Rendering Engine by Jonas Pfeifer",
                width,
                height
            )
            .resizable()
            .position_centered()
            .vulkan()
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        canvas.set_draw_color(Color::RGB(255,255,255));
        canvas.clear();
        canvas.present();

        Ok(Display { sdl_context: Box::new(context), canvas, draw_array: Arc::new(Mutex::new(vec![])) })
    }

    pub fn add_pixel(&mut self, pixel: Pixel) {
        let mut draw_array = self.draw_array.lock().unwrap();

        draw_array.push(pixel);
    }

    pub fn update_pixels(&mut self) {
        let mut draw_array = self.draw_array.lock().unwrap();
        while !draw_array.is_empty() {
            let pixel = draw_array.pop().unwrap();
            self.canvas.set_draw_color(pixel.color);
            let _ = self.canvas.draw_point(pixel.position);
        }
        self.canvas.present();
    }

    pub fn get_context(&self) -> &Box<Sdl> {
        return &self.sdl_context;
    }

    pub fn get_size(&self) -> (u32, u32) {
        return self.canvas.output_size().unwrap();
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(255,255,255));
        self.canvas.clear();
        self.canvas.present();
    }

    pub fn generate_image(&mut self) -> Vec<u8> {
        self.canvas.read_pixels(self.canvas.viewport(), PixelFormatEnum::RGBA32).unwrap()
    }
}