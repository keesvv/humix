pub mod chr {}
pub mod interrupt {
    pub enum Interrupts {}
}
pub mod kernel {
    pub fn log_fmt(_: core::fmt::Arguments) {}
    pub mod init {
        pub fn preload() {}
        pub fn load() {}
    }
    pub mod power {
        pub fn halt() -> ! {
            loop {}
        }
    }
}
