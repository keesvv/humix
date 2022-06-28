pub mod idt;
pub mod pic;

use crate::kprint;
use pic::PIC_OFFSETS;
use x86_64::instructions::interrupts;

#[repr(u8)]
pub enum Interrupts {
    Timer = PIC_OFFSETS.0,
}

impl Interrupts {
    pub fn as_usize(self) -> usize {
        usize::from(self as u8)
    }
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
