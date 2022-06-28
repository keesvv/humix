use crate::interrupt::idt;
use crate::kprint;

pub fn init() {
    idt::initialize(unsafe { &mut idt::IDT });

    kprint!("Humix is starting\nDebug information will be directed to COM1.\n");

    // Manually cause breakpoint exception for testing.
    // Will remove soon.
    {
        x86_64::instructions::interrupts::int3();

        kprint!("Recovered from CPU exception.\n");
    }
}
