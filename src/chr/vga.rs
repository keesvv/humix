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

    pub fn write(&mut self, b: &[u8]) {
        for &b in b {
            if b == 0x0a {
                self.pos += VGA_TEXT_W as usize * 2 - self.pos;
                continue;
            }

            unsafe {
                *self.addr.offset(self.pos as isize) = b;
                *self.addr.offset(self.pos as isize + 1) = 0xf;
            }

            self.pos += 2;
        }
    }
}
