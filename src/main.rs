use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;

fn create_window_canvas(
    sdl_context: &Sdl,
    width: u32,
    height: u32,
) -> Result<WindowCanvas, String> {
    let video = sdl_context.video()?;
    let window = video
        .window("pathfinding-demo", width, height)
        .position_centered()
        .build()
        .map_err(|e| format!("{}", e))?;
    window.into_canvas().build().map_err(|e| e.to_string())
}

fn main() {
    let sdl_context = sdl2::init().expect("Unable to create SDL context");
    let mut canvas =
        create_window_canvas(&sdl_context, 800, 600).expect("Unable to create window canvas");
    let mut event_pump = sdl_context
        .event_pump()
        .expect("Unable to create event pump");
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {}
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }
    }
}
