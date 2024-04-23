use {
  crate::{
    bus::{Bus, PlainRam}, cpu::Cpu, front::Front, GAME_CODE, SLEEP_TIME
  },
  sdl2::{
    image::LoadTexture,
    render::{Texture, TextureCreator},
    video::WindowContext,
  },
  std::{thread::sleep, time::Duration},
};

pub struct Emulator<B: Bus> {
  bus: B,
  cpu: Cpu,
  front: Front,
}

impl<B: Bus> Emulator<B> {
  pub fn new() -> Result<Emulator<PlainRam>, String> {
    let cpu: Cpu = Cpu::new();
    let mut front: Front = Front::new()?;
    let bus: PlainRam = PlainRam {
      memory: [0; 0x10000],
    };

    front.clear_black_and_present();

    Ok(Emulator { bus, cpu, front })
  }

  pub fn run(&mut self) -> Result<(), String> {
    let texture_creator: TextureCreator<WindowContext> = self.front.canvas.texture_creator();
    let texture: Texture<'_> = texture_creator.load_texture("nesfont.png")?;

    loop {
      if self.front.handle_events().is_break() {
        break Ok(());
      }

      let code: u8 = self.bus.mem_read(self.cpu.pc);
      // let opcode = opcodes.get(&code).ok_or(code).unwrap();
      
      self.cpu.pc += 1;

      self.front.canvas.clear();
      self.front.draw_x(&texture, self.cpu.x)?;
      self.front.draw_y(&texture, self.cpu.y)?;
      self.front.draw_a(&texture, self.cpu.a)?;
      self.front.draw_pc(&texture, self.cpu.pc)?;
      self.front.draw_code(&texture, code)?;
      self.front.draw_nesfonts(&texture)?;
      self.front.canvas.present();

      sleep(Duration::from_millis(SLEEP_TIME));
      self.cpu.x = self.cpu.x.wrapping_add(5);
      self.cpu.y = self.cpu.y.wrapping_add(3);
      self.cpu.a = self.cpu.a.wrapping_add(7);
    }
  }
  pub fn start() -> Result<(), String> {
    let program: Vec<u8> = GAME_CODE.to_vec();
    let mut emulator: Emulator<PlainRam> = Self::new()?;

    emulator.bus.load(program);
    emulator.cpu.reset(&emulator.bus);
    emulator.run()
  }
}
