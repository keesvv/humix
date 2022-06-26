use super::CharDevice;
use core::fmt;

pub const VGA_TEXT_W: u8 = 80;

#[allow(dead_code)]
pub const VGA_TEXT_H: u8 = 25;

pub struct VgaBuffer {
    addr: *mut u8,
    pos: usize,
}

impl VgaBuffer {
    pub fn new(addr: usize) -> Self {
        Self {
            addr: addr as *mut u8,
            pos: 0,
        }
    }
}

impl CharDevice for VgaBuffer {
    fn write(&mut self, b: u8) {
        if b == 0x0a {
            self.pos += VGA_TEXT_W as usize * 2 - self.pos;
            return;
        }

        unsafe {
            *self.addr.offset(self.pos as isize) = b;
            *self.addr.offset(self.pos as isize + 1) = 0xf;
        }

        self.pos += 2;
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
