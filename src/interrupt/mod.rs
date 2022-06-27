use crate::kprint;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

pub static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub unsafe fn initialize_idt() {
    IDT.breakpoint.set_handler_fn(breakpoint_handler);
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(sf: InterruptStackFrame) {
    kprint!("!! cpu exception: {:#?}\n", sf);
}
