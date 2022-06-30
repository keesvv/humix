pub mod init;
pub mod panic;
pub mod power;

use crate::chr::serial::COM1;
use core::fmt::{self, Write};
use x86_64::instructions::interrupts;

pub fn log_fmt(args: fmt::Arguments) {
    interrupts::without_interrupts(|| {
        COM1.lock()
            // TODO: register uptime
            .write_fmt(format_args!("[0.0000] {}", args))
            .unwrap();
    });
}

#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => ($crate::kernel::log_fmt(format_args!($($arg)*)));
}
