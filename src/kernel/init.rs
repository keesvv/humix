use crate::chr::serial::COM1;
use crate::interrupt::timer::TIMER;
use crate::interrupt::{self, idt, pic};
use crate::kprint;

pub fn init() {
    COM1.lock().initialize();
    TIMER.lock().initialize();

    kprint!("Humix is starting...\n");

    idt::initialize(unsafe { &mut idt::IDT });
    pic::initialize(&pic::PICS);
    interrupt::enable();

    // Manually cause breakpoint exception for testing.
    // Will remove soon.
    {
        x86_64::instructions::interrupts::int3();

        kprint!("Recovered from CPU exception.\n");
    }
}
