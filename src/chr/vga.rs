use super::CharDevice;
use core::fmt;
use volatile::Volatile;

pub const VGA_TEXT_W: u8 = 80;
pub const VGA_TEXT_H: u8 = 25;

#[derive(Clone, Copy)]
pub struct Char(u8, u8);
pub struct Pos(u8, u8);

pub struct Buffer {
    data: [[Volatile<Char>; VGA_TEXT_W as usize]; VGA_TEXT_H as usize],
}

pub struct VgaBuffer {
    pos: Pos,
    buf: &'static mut Buffer,
}

impl VgaBuffer {
    pub fn new(addr: usize) -> Self {
        Self {
            pos: Pos(0, 0),
            buf: unsafe { &mut *(addr as *mut Buffer) },
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

        self.buf.data[self.pos.1 as usize][self.pos.0 as usize].write(Char(b, 0xf));
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
