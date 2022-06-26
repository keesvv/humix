use super::CharDevice;
use core::fmt;

pub const VGA_TEXT_W: u8 = 80;

#[allow(dead_code)]
pub const VGA_TEXT_H: u8 = 25;

pub struct Pos(u8, u8);

impl Pos {
    pub fn get_offset(&self) -> usize {
        (VGA_TEXT_W * self.1 * 2 + self.0 * 2).into()
    }
}

pub struct VgaBuffer {
    addr: *mut u8,
    pos: Pos,
}

impl VgaBuffer {
    pub fn new(addr: usize) -> Self {
        Self {
            addr: addr as *mut u8,
            pos: Pos(0, 0),
        }
    }
}

impl CharDevice for VgaBuffer {
    fn write(&mut self, b: u8) {
        if b == 0x0a {
            self.pos.0 = 0;
            self.pos.1 += 1;
            return;
        }

        let offset = self.pos.get_offset() as isize;

        unsafe {
            *self.addr.offset(offset) = b;
            *self.addr.offset(offset + 1) = 0xf;
        }

        self.pos.0 += 1;
    }

    fn read(&mut self, _: u8) {
        unimplemented!()
    }
}

impl fmt::Write for VgaBuffer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for (_, b) in s.chars().enumerate() {
            self.write(b as u8);
        }
        Ok(())
    }
}
