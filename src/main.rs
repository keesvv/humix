#![no_std]
#![no_main]

use core::fmt::Write;
use panic_halt as _;
mod chr;
use chr::vga::VGA_BUFFER;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    write!(
        &mut VGA_BUFFER.lock(),
        "Humix is starting\nDebug information will be directed to COM1.\n"
    )
    .unwrap();

    loop {}
}
