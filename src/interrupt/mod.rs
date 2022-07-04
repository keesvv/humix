pub use crate::arch::interrupt::*;

impl Interrupts {
    pub fn as_usize(self) -> usize {
        usize::from(self as u8)
    }
}
