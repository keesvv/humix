use crate::kprint;
use core::panic::PanicInfo;

#[panic_handler]
pub fn handle_panic(info: &PanicInfo) -> ! {
    kprint!("!! KERNEL PANIC !!\n{:#}\nHalted.\n", info);
    loop {}
}
