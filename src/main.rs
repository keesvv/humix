#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(default_alloc_error_handler)]

mod arch;
mod chr;
mod interrupt;
mod kernel;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    kernel::init::init();
    loop {}
}
