use crate::kprint;

pub fn halt() -> ! {
    kprint!("Halted.\n");
    loop {}
}
