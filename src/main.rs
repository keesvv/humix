#![no_std]
#![no_main]

use core::fmt::Write;
use panic_halt as _;
mod chr;
use chr::vga::VgaBuffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut buf = VgaBuffer::new(0xb8000);
    write!(
        &mut buf,
        "Humix is starting\nDebug information will be directed to COM1.\n"
    )
    .unwrap();

    loop {}
}
