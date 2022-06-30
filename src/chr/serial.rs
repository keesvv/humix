use super::CharDevice;
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort as UartPort;

lazy_static! {
    pub static ref COM1: Mutex<SerialPort> = Mutex::new(SerialPort::new(0x3f8));
}

pub struct SerialPort {
    port: UartPort,
}

impl SerialPort {
    pub fn new(addr: usize) -> Self {
        Self {
            port: unsafe { UartPort::new(addr as u16) },
        }
    }

    pub fn initialize(&mut self) {
        self.port.init();
    }
}

impl CharDevice for SerialPort {
    fn write(&mut self, b: u8) {
        self.port.send_raw(b)
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
