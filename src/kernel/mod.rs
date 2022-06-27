#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => ($crate::chr::vga::print_fmt(format_args!($($arg)*)));
}
