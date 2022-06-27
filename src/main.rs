#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod chr;
mod interrupt;
mod kernel;

use interrupt::idt;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        idt::initialize(&mut idt::IDT);
    }

    kprint!("Humix is starting\nDebug information will be directed to COM1.\n");

    // Manually cause breakpoint exception for testing.
    // Will remove soon.
    x86_64::instructions::interrupts::int3();

    kprint!("Recovered from CPU exception.\n");

    loop {}
}
