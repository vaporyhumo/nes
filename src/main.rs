#![warn(clippy::complexity)]
#![warn(clippy::expect_used)]
#![warn(clippy::nursery)]
#![warn(clippy::panic)]
#![warn(clippy::pedantic)]
#![warn(clippy::perf)]
#![warn(clippy::unwrap_used)]

use {
  sdl2::{
    event::Event, image::LoadTexture, keyboard::Keycode, pixels::Color, rect::{Point, Rect}, EventPump, Sdl
  },
  std::{ops::ControlFlow, thread::sleep, time::Duration},
};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const WINDOW_BUILD_ERROR: &str = "";
const CANVAS_BUILD_ERROR: &str = "";
const BLACK: Color = Color::RGB(0, 0, 0);

fn main() -> Result<(), String> {
  let sdl_context: Sdl = sdl2::init()?;
  let video_subsystem = sdl_context.video()?;
  let window = video_subsystem
    .window("RNES", WIDTH, HEIGHT)
    .position_centered()
    .build()
    .map_err(|_| WINDOW_BUILD_ERROR)?;
  let mut canvas = window
    .into_canvas()
    .build()
    .map_err(|_| CANVAS_BUILD_ERROR)?;
  let texture_creator = canvas.texture_creator();
  let texture = texture_creator.load_texture("nesfont.png")?;
  let mut event_pump = sdl_context.event_pump()?;

  canvas.set_draw_color(BLACK);
  canvas.clear();
  canvas.present();

  loop {
    if handle_events(&mut event_pump).is_break() {
      break Ok(());
    }

    let position = Point::new(0, 0);
    let (width, height) = canvas.output_size()?;
    let screen_position = position + Point::new(width as i32 / 2, height as i32 / 2);
    let sprite = Rect::new(0, 0, 256, 96);
    let screen_rect = Rect::from_center(screen_position, sprite.width(), sprite.height());
    canvas.clear();
    canvas.copy(&texture, sprite, screen_rect)?;
    canvas.present();

    sleep(Duration::from_millis(10));
  }
}

fn handle_events(event_pump: &mut EventPump) -> ControlFlow<(), ()> {
  for event in event_pump.poll_iter() {
    match event {
      Event::Quit { .. }
      | Event::KeyDown {
        keycode: Some(Keycode::Escape),
        ..
      } => return ControlFlow::Break(()),
      _ => {}
    }
  }
  ControlFlow::Continue(())
}
