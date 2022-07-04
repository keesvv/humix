pub mod init;
pub mod panic;

pub use crate::arch::kernel::*;

#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => ($crate::kernel::log_fmt(format_args!($($arg)*)));
}
