#![no_std]
#![no_main]

mod chr;
mod kernel;

use panic_halt as _;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    kprint!("Humix is starting\nDebug information will be directed to COM1.\n");

    loop {}
}
