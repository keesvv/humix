use crate::chr::serial::COM1;
use crate::interrupt::timer::TIMER;
use crate::interrupt::{self, idt, pic};
use crate::kprint;

#[inline(always)]
pub fn preload() {
    COM1.lock().initialize();
}

pub fn load() {
    TIMER.lock().initialize();
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
