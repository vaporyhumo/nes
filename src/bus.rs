const LOAD_AT: u16 = 0x0600;

pub trait Bus {
  fn mem_read(&self, addr: u16) -> u8;
  fn mem_read_u16(&self, addr: u16) -> u16;
  fn mem_write(&mut self, addr: u16, data: u8);
  fn mem_write_u16(&mut self, addr: u16, data: u16);
  fn load(&mut self, program: Vec<u8>);
}

pub struct PlainRam {
  pub memory: [u8; 0x10000],
}

impl Bus for PlainRam {
  fn mem_read(&self, addr: u16) -> u8 {
    self.memory[addr as usize]
  }

  fn mem_read_u16(&self, addr: u16) -> u16 {
    let lo = self.mem_read(addr);
    let hi = self.mem_read(addr + 1);

    u16::from_be_bytes([hi, lo])
  }

  fn mem_write(&mut self, addr: u16, data: u8) {
    self.memory[addr as usize] = data;
  }

  fn mem_write_u16(&mut self, addr: u16, data: u16) {
    let hi = (data >> 8) as u8;
    let lo = (data & 0xFF) as u8;

    self.mem_write(addr, lo);
    self.mem_write(addr + 1, hi);
  }

  fn load(&mut self, program: Vec<u8>) {
    self.memory[(LOAD_AT as usize)..((LOAD_AT as usize) + program.len())].copy_from_slice(&program[..]);
    self.mem_write_u16(0xFFFC, LOAD_AT);
  }
}
