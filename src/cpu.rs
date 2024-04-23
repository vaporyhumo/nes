use crate::bus::Bus;

pub struct Cpu {
  pub x: u8,
  pub y: u8,
  pub a: u8,
  pub pc: u16,
}

impl Cpu {
  pub const fn new() -> Self {
    Self {
      x: 0,
      y: 0,
      a: 0,
      pc: 0,
    }
  }

  pub fn reset(&mut self, bus: &impl Bus) {
    self.a = 0;
    self.x = 0;
    self.y = 0;
    // self.status = 0;

    self.pc = bus.mem_read_u16(0xFFFC);
  }
}
