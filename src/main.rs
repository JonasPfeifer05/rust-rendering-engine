use rendering_engine::Application;

fn main() -> Result<(), String>{
    let mut application = Application::new(1080, 920)?;

    application.start();

    Ok(())
}