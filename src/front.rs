use {
  crate::{
    A, BLACK, C, CANVAS_BUILD_ERROR, COLON, HEIGHT, HEIGHT_ERROR, O, P, TILE_SIZE_I32, TILE_SIZE_U32, WIDTH, WIDTH_ERROR, WINDOW_BUILD_ERROR, X, Y
  },
  sdl2::{
    event::Event,
    keyboard::Keycode,
    rect::{Point, Rect},
    render::{Canvas, Texture},
    video::Window,
    EventPump, Sdl, VideoSubsystem,
  },
  std::ops::ControlFlow,
};

macro_rules! sprite {
  ($x:expr, $y:expr) => {
    Rect::new(
      $x * TILE_SIZE_I32,
      $y * TILE_SIZE_I32,
      TILE_SIZE_U32,
      TILE_SIZE_U32,
    )
  };
}

pub struct Front {
  pub canvas: Canvas<Window>,
  pub event_pump: EventPump,
}

impl Front {
  pub fn new() -> Result<Self, String> {
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

  pub fn draw_nesfonts(&mut self, texture: &Texture<'_>) -> Result<(), String> {
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

  pub fn draw_x(&mut self, texture: &Texture<'_>, x: u8) -> Result<(), String> {
    let sprite = *X;
    let screen_rect = sprite!(0, 0);
    self.canvas.copy(texture, sprite, screen_rect)?;

    let sprite = *COLON;
    let screen_rect = sprite!(1, 0);
    self.canvas.copy(texture, sprite, screen_rect)?;

    let sprite = sprite!(i32::from(x / 100), 1);
    let screen_rect = sprite!(3, 0);
    self.canvas.copy(texture, sprite, screen_rect)?;

    let sprite = sprite!(i32::from(x % 100) / 10, 1);
    let screen_rect = sprite!(4, 0);
    self.canvas.copy(texture, sprite, screen_rect)?;

    let sprite = sprite!(i32::from(x % 10), 1);
    let screen_rect = sprite!(5, 0);
    self.canvas.copy(texture, sprite, screen_rect)?;
    Ok(())
  }

  pub fn draw_y(&mut self, texture: &Texture<'_>, y: u8) -> Result<(), String> {
    let sprite = *Y;
    let screen_rect = sprite!(0, 1);
    self.canvas.copy(texture, sprite, screen_rect)?;

    let sprite = *COLON;
    let screen_rect = sprite!(1, 1);
    self.canvas.copy(texture, sprite, screen_rect)?;

    let sprite = sprite!(i32::from(y / 100), 1);
    let screen_rect = sprite!(3, 1);
    self.canvas.copy(texture, sprite, screen_rect)?;

    let sprite = sprite!(i32::from(y % 100) / 10, 1);
    let screen_rect = sprite!(4, 1);
    self.canvas.copy(texture, sprite, screen_rect)?;

    let sprite = sprite!(i32::from(y % 10), 1);
    let screen_rect = sprite!(5, 1);
    self.canvas.copy(texture, sprite, screen_rect)?;
    Ok(())
  }

  pub fn draw_a(&mut self, texture: &Texture<'_>, a: u8) -> Result<(), String> {
    let sprite = *A;
    let screen_rect = sprite!(0, 2);
    self.canvas.copy(texture, sprite, screen_rect)?;

    let sprite = *COLON;
    let screen_rect = sprite!(1, 2);
    self.canvas.copy(texture, sprite, screen_rect)?;

    let sprite = sprite!(i32::from(a / 100), 1);
    let screen_rect = sprite!(3, 2);
    self.canvas.copy(texture, sprite, screen_rect)?;

    let sprite = sprite!(i32::from(a % 100) / 10, 1);
    let screen_rect = sprite!(4, 2);
    self.canvas.copy(texture, sprite, screen_rect)?;

    let sprite = sprite!(i32::from(a % 10), 1);
    let screen_rect = sprite!(5, 2);
    self.canvas.copy(texture, sprite, screen_rect)?;
    Ok(())
  }

  pub fn draw_pc(&mut self, texture: &Texture<'_>, pc: u16) -> Result<(), String> {
    let sprite = *P;
    let screen_rect = sprite!(0, 3);
    self.canvas.copy(texture, sprite, screen_rect)?;

    let sprite = *C;
    let screen_rect = sprite!(1, 3);
    self.canvas.copy(texture, sprite, screen_rect)?;

    let sprite = *COLON;
    let screen_rect = sprite!(2, 3);
    self.canvas.copy(texture, sprite, screen_rect)?;

    let sprite = sprite!(i32::from(pc / 10000), 1);
    let screen_rect = sprite!(4, 3);
    self.canvas.copy(texture, sprite, screen_rect)?;

    let sprite = sprite!(i32::from(pc % 10000) / 1000, 1);
    let screen_rect = sprite!(5, 3);
    self.canvas.copy(texture, sprite, screen_rect)?;

    let sprite = sprite!(i32::from(pc % 1000) / 100, 1);
    let screen_rect = sprite!(6, 3);
    self.canvas.copy(texture, sprite, screen_rect)?;

    let sprite = sprite!(i32::from(pc % 100) / 10, 1);
    let screen_rect = sprite!(7, 3);
    self.canvas.copy(texture, sprite, screen_rect)?;

    let sprite = sprite!(i32::from(pc % 10), 1);
    let screen_rect = sprite!(8, 3);
    self.canvas.copy(texture, sprite, screen_rect)?;
    Ok(())
  }

  pub fn draw_code(&mut self, texture: &Texture<'_>, code: u8) -> Result<(), String> {
    let sprite = *O;
    let screen_rect = sprite!(0, 4);
    self.canvas.copy(texture, sprite, screen_rect)?;

    let sprite = *P;
    let screen_rect = sprite!(1, 4);
    self.canvas.copy(texture, sprite, screen_rect)?;

    let sprite = *COLON;
    let screen_rect = sprite!(2, 4);
    self.canvas.copy(texture, sprite, screen_rect)?;

    let sprite = sprite!(i32::from(code / 100), 1);
    let screen_rect = sprite!(4, 4);
    self.canvas.copy(texture, sprite, screen_rect)?;

    let sprite = sprite!(i32::from(code % 100) / 10, 1);
    let screen_rect = sprite!(5, 4);
    self.canvas.copy(texture, sprite, screen_rect)?;

    let sprite = sprite!(i32::from(code % 10), 1);
    let screen_rect = sprite!(6, 4);
    self.canvas.copy(texture, sprite, screen_rect)?;
    Ok(())
  }

  pub fn handle_events(&mut self) -> ControlFlow<(), ()> {
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

  pub fn clear_black_and_present(&mut self) {
    self.canvas.set_draw_color(BLACK);
    self.canvas.clear();
    self.canvas.present();
  }
}
