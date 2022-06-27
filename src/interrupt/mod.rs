use crate::kprint;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

pub static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub unsafe fn initialize_idt() {
    IDT.breakpoint.set_handler_fn(breakpoint_handler);
    IDT.double_fault.set_handler_fn(double_fault_handler);
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(sf: InterruptStackFrame) {
    kprint!("!! cpu exception: {:#?}\n", sf);
}

extern "x86-interrupt" fn double_fault_handler(sf: InterruptStackFrame, _: u64) -> ! {
    panic!("double fault exception: {:#?}\n", sf);
}
