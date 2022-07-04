#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod arch;
mod chr;
mod interrupt;
mod kernel;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    kernel::init::init();
    loop {}
}
