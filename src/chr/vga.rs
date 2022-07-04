use super::CharDevice;
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

lazy_static! {
    pub static ref VGA_BUFFER: Mutex<VgaBuffer> = Mutex::new(VgaBuffer::new(0xb8000));
}

pub const VGA_TEXT_W: u8 = 80;
pub const VGA_TEXT_H: u8 = 25;

#[derive(Clone, Copy)]
pub struct Char(u8, u8);

impl Char {
    pub fn empty() -> Self {
        Self(0, 0)
    }
}

#[derive(Clone, Copy)]
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

    pub fn fill_line(&mut self, y: u8, c: Char) {
        for col in 0..(VGA_TEXT_W - 1) as usize {
            self.buf.data[y as usize][col].write(c);
        }
    }

    #[inline(always)]
    pub fn clear_line(&mut self, y: u8) {
        self.fill_line(y, Char::empty());
    }

    pub fn scroll(&mut self, lns: u8) {
        for (i, ln) in self.buf.data.clone().iter().skip(lns.into()).enumerate() {
            for (j, col) in ln.iter().enumerate() {
                self.buf.data[i][j].write(col.read());
            }
        }

        for ln in 0..lns {
            self.clear_line(VGA_TEXT_H - 1 - ln)
        }

        self.pos.1 = VGA_TEXT_H - lns;
    }
}

impl CharDevice for VgaBuffer {
    fn write(&mut self, b: u8) {
        if self.pos.1 >= VGA_TEXT_H {
            self.scroll(1);
        }

        match b {
            0x0a => {
                self.pos.0 = 0;
                self.pos.1 += 1;
            }
            _ => {
                self.buf.data[self.pos.1 as usize][self.pos.0 as usize].write(Char(b, 0xf));
                self.pos.0 += 1;
            }
        }
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
