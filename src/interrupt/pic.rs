use pic8259::ChainedPics;
use spin::Mutex;

use crate::kprint;

pub const PIC_OFFSETS: (u8, u8) = (32, 40);

pub static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(PIC_OFFSETS.0, PIC_OFFSETS.1) });

pub fn initialize(pics: &'static Mutex<ChainedPics>) {
    unsafe { pics.lock().initialize() }
    kprint!("PICs initialized.\n");
}
