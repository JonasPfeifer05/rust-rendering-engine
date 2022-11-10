use sdl2::render::WindowCanvas;

pub struct Display {
    canvas: WindowCanvas,
}

impl Display {
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

        Ok(Display { canvas })
    }
}