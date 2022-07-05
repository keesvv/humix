use crate::arch::kernel::init;
use crate::kprint;

pub fn init() {
    init::preload();
    kprint!("Humix is starting...\n");
    init::load();
}
