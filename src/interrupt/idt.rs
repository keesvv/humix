use super::Interrupts;
use crate::{interrupt::pic::PICS, kprint};
use x86_64::instructions::port::PortReadOnly;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

pub static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub fn initialize(idt: &'static mut InterruptDescriptorTable) {
    idt.breakpoint.set_handler_fn(breakpoint_handler);
    idt.double_fault.set_handler_fn(double_fault_handler);
    idt[Interrupts::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
    idt[Interrupts::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);
    idt.load();
    kprint!("Interrupt descriptor table loaded.\n");
}

extern "x86-interrupt" fn breakpoint_handler(sf: InterruptStackFrame) {
    kprint!("!! cpu exception: {:#?}\n", sf);
}

extern "x86-interrupt" fn double_fault_handler(sf: InterruptStackFrame, _: u64) -> ! {
    panic!("double fault exception: {:#?}\n", sf);
}

extern "x86-interrupt" fn timer_interrupt_handler(_: InterruptStackFrame) {
    unsafe { PICS.lock().notify_end_of_interrupt(Interrupts::Timer as u8) }
    // TODO: handle/store timer data
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_: InterruptStackFrame) {
    let mut port: PortReadOnly<u8> = PortReadOnly::new(0x60);
    let code = unsafe { port.read() };

    kprint!("Keyboard scancode received: {}\n", code);
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(Interrupts::Keyboard as u8)
    }
}
