pub mod idt;
pub mod pic;

use x86_64::instructions::interrupts;

use crate::kprint;

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
