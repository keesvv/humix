#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod chr;
mod interrupt;
mod kernel;

use panic_halt as _;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        interrupt::initialize_idt();
    }

    kprint!("Humix is starting\nDebug information will be directed to COM1.\n");

    // Manually cause breakpoint exception for testing.
    // Will remove soon.
    x86_64::instructions::interrupts::int3();

    kprint!("Recovered from CPU exception.\n");

    loop {}
}
