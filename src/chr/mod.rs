pub trait CharDevice {
    fn write(&mut self, b: u8);
    fn read(&mut self, b: u8);
}

pub mod vga;
