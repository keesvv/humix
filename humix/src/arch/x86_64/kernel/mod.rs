pub mod init;
pub mod power;

use crate::chr::serial::COM1;
use core::fmt::{self, Write};
use x86_64::instructions::interrupts;

pub fn log_fmt(args: fmt::Arguments) {
    interrupts::without_interrupts(|| {
        COM1.lock().write_fmt(args).unwrap();
    });
}
