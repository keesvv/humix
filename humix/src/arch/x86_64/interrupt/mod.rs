pub mod idt;
pub mod keyboard;
pub mod pic;
pub mod timer;

use crate::interrupt::pic::PIC_OFFSETS;
use x86_64::{instructions::interrupts, structures::idt::InterruptStackFrame};

#[inline(always)]
pub fn enable() {
    interrupts::enable();
}

#[inline(always)]
#[allow(dead_code)]
pub fn disable() {
    interrupts::disable();
}

#[repr(u8)]
pub enum Interrupts {
    Timer = PIC_OFFSETS.0,
    Keyboard,
}

pub trait Interrupt {
    fn handle(&mut self, sf: InterruptStackFrame);
}
