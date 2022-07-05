use super::Interrupt;
use crate::kprint;
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::{instructions::port::PortReadOnly, structures::idt::InterruptStackFrame};

lazy_static! {
    pub static ref KEYBOARD: Mutex<Keyboard> = Mutex::new(Keyboard::new());
}

pub struct Keyboard {
    port: PortReadOnly<u8>,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            port: PortReadOnly::new(0x60),
        }
    }
}

impl Interrupt for Keyboard {
    fn handle(&mut self, _: InterruptStackFrame) {
        kprint!("Keyboard scancode received: {}\n", unsafe {
            self.port.read()
        });
    }
}
