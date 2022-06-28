pub mod init;
pub mod panic;

use crate::chr::vga::VGA_BUFFER;
use core::fmt::{self, Write};

pub fn log_fmt(args: fmt::Arguments) {
    VGA_BUFFER
        .lock()
        // TODO: register uptime
        .write_fmt(format_args!("[0.0000] {}", args))
        .unwrap();
}

#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => ($crate::kernel::log_fmt(format_args!($($arg)*)));
}
