use super::CharDevice;
use core::fmt;

pub struct SerialPort {
    addr: usize,
}

impl SerialPort {
    pub fn new(addr: usize) -> Self {
        Self { addr }
    }
}

impl CharDevice for SerialPort {
    fn write(&mut self, _: u8) {
        todo!()
    }

    fn read(&mut self, _: u8) {
        todo!()
    }
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // TODO: remove duplication
        for (_, b) in s.chars().enumerate() {
            self.write(b as u8);
        }
        Ok(())
    }
}
