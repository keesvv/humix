pub mod idt;
pub mod keyboard;
pub mod pic;
pub mod timer;

use crate::kprint;
use pic::PIC_OFFSETS;
use x86_64::{instructions::interrupts, structures::idt::InterruptStackFrame};

#[repr(u8)]
pub enum Interrupts {
    Timer = PIC_OFFSETS.0,
    Keyboard,
}

impl Interrupts {
    pub fn as_usize(self) -> usize {
        usize::from(self as u8)
    }
}

pub trait Interrupt {
    fn handle(&mut self, sf: InterruptStackFrame);
}

#[inline(always)]
pub fn enable() {
    interrupts::enable();
    kprint!("Interrupts enabled.\n");
}

#[inline(always)]
#[allow(dead_code)]
pub fn disable() {
    interrupts::disable();
    kprint!("Interrupts disabled.\n");
}
