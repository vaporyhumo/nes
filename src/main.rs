#![warn(clippy::complexity)]
#![warn(clippy::expect_used)]
#![warn(clippy::nursery)]
#![warn(clippy::panic)]
#![warn(clippy::pedantic)]
#![warn(clippy::perf)]
#![warn(clippy::unwrap_used)]

use {
  once_cell::sync::Lazy,
  sdl2::{
    event::Event,
    image::LoadTexture,
    keyboard::Keycode,
    pixels::Color,
    rect::{Point, Rect},
    render::{Canvas, Texture, TextureCreator},
    video::{Window, WindowContext},
    EventPump, Sdl, VideoSubsystem,
  },
  std::{ops::ControlFlow, thread::sleep, time::Duration},
};

const BLACK: Color = Color::RGB(0, 0, 0);
const CANVAS_BUILD_ERROR: &str = "";
const HEIGHT: u32 = 600;
const HEIGHT_ERROR: &str = "";
const TILE_SIZE: u8 = 16;
const TILE_SIZE_I32: i32 = TILE_SIZE as i32;
const TILE_SIZE_U32: u32 = TILE_SIZE as u32;
const WIDTH: u32 = 800;
const WIDTH_ERROR: &str = "";
const WINDOW_BUILD_ERROR: &str = "";

static COLON: Lazy<Rect> = Lazy::new(|| {
  Rect::new(
    10 * TILE_SIZE_I32,
    TILE_SIZE_I32,
    TILE_SIZE_U32,
    TILE_SIZE_U32,
  )
});
static X: Lazy<Rect> = Lazy::new(|| {
  Rect::new(
    8 * TILE_SIZE_I32,
    3 * TILE_SIZE_I32,
    TILE_SIZE_U32,
    TILE_SIZE_U32,
  )
});

struct Front {
  canvas: Canvas<Window>,
  event_pump: EventPump,
}

struct Cpu {
  x: u8,
}

impl Cpu {
  const fn new() -> Self {
    Self { x: 0 }
  }
}

impl Front {
  fn new() -> Result<Self, String> {
    let sdl: Sdl = sdl2::init()?;
    let video: VideoSubsystem = sdl.video()?;
    let window: Window = video
      .window("RNES", WIDTH, HEIGHT)
      .position_centered()
      .build()
      .map_err(|_| WINDOW_BUILD_ERROR)?;
    let canvas: Canvas<Window> = window
      .into_canvas()
      .build()
      .map_err(|_| CANVAS_BUILD_ERROR)?;
    let event_pump: EventPump = sdl.event_pump()?;

    Ok(Self { canvas, event_pump })
  }

  fn draw_nesfonts(&mut self, texture: &Texture<'_>) -> Result<(), String> {
    let position = Point::new(0, 0);
    let (width, height) = self.canvas.output_size()?;
    let screen_position = position
      + Point::new(
        i32::try_from(width).map_err(|_| WIDTH_ERROR)? / 2,
        i32::try_from(height).map_err(|_| HEIGHT_ERROR)? / 2,
      );
    let sprite = Rect::new(0, 0, 256, 96);
    let screen_rect = Rect::from_center(screen_position, sprite.width(), sprite.height());
    self.canvas.copy(texture, sprite, screen_rect)?;
    Ok(())
  }

  fn draw_x(&mut self, texture: &Texture<'_>, x: u8) -> Result<(), String> {
    let sprite = *X;
    let screen_rect = Rect::new(0, 0, TILE_SIZE_U32, TILE_SIZE_U32);
    self.canvas.copy(texture, sprite, screen_rect)?;

    let sprite = *COLON;
    let screen_rect = Rect::new(TILE_SIZE_I32, 0, TILE_SIZE_U32, TILE_SIZE_U32);
    self.canvas.copy(texture, sprite, screen_rect)?;

    let sprite = Rect::new(
      i32::from(x / 100) * TILE_SIZE_I32,
      TILE_SIZE_I32,
      TILE_SIZE_U32,
      TILE_SIZE_U32,
    );
    let screen_rect = Rect::new(3 * TILE_SIZE_I32, 0, TILE_SIZE_U32, TILE_SIZE_U32);
    self.canvas.copy(texture, sprite, screen_rect)?;

    let sprite = Rect::new(
      i32::from(x % 100) / 10 * TILE_SIZE_I32,
      TILE_SIZE_I32,
      TILE_SIZE_U32,
      TILE_SIZE_U32,
    );
    let screen_rect = Rect::new(4 * TILE_SIZE_I32, 0, TILE_SIZE_U32, TILE_SIZE_U32);
    self.canvas.copy(texture, sprite, screen_rect)?;

    let sprite = Rect::new(
      i32::from(x % 10) * TILE_SIZE_I32,
      TILE_SIZE_I32,
      TILE_SIZE_U32,
      TILE_SIZE_U32,
    );
    let screen_rect = Rect::new(5 * TILE_SIZE_I32, 0, TILE_SIZE_U32, TILE_SIZE_U32);
    self.canvas.copy(texture, sprite, screen_rect)?;
    Ok(())
  }

  fn handle_events(&mut self) -> ControlFlow<(), ()> {
    for event in self.event_pump.poll_iter() {
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

  fn clear_black_and_present(&mut self) {
    self.canvas.set_draw_color(BLACK);
    self.canvas.clear();
    self.canvas.present();
  }
}

fn main() -> Result<(), String> {
  let mut cpu: Cpu = Cpu::new();
  let mut front: Front = Front::new()?;
  let texture_creator: TextureCreator<WindowContext> = front.canvas.texture_creator();
  let texture: Texture<'_> = texture_creator.load_texture("nesfont.png")?;

  front.clear_black_and_present();

  loop {
    if front.handle_events().is_break() {
      break Ok(());
    }

    front.canvas.clear();
    front.draw_x(&texture, cpu.x)?;
    front.draw_nesfonts(&texture)?;
    front.canvas.present();

    sleep(Duration::from_millis(10));
    cpu.x = cpu.x.wrapping_add(1);
  }
}
