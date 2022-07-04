#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub mod chr {}

pub mod interrupt {
    pub enum Interrupts {}
}

pub mod kernel {
    extern crate alloc;
    use alloc::vec::Vec;
    use spin::Mutex;

    pub static KERNEL_BUFFER: Mutex<Vec<&str>> = Mutex::new(Vec::new());

    pub fn log_fmt(args: core::fmt::Arguments) {
        KERNEL_BUFFER.lock().push(args.as_str().unwrap());
    }

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
