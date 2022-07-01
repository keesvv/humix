use super::Interrupt;
use crate::kprint;
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::{
    instructions::port::{PortReadOnly, PortWriteOnly},
    structures::idt::InterruptStackFrame,
};

#[repr(u16)]
#[allow(dead_code)]
pub enum TimerChannel {
    Channel0 = 0x40,
    Channel1,
    Channel2,
    Mode,
}

lazy_static! {
    pub static ref TIMER: Mutex<Timer> = Mutex::new(Timer::new());
}

pub struct Timer {
    ticks: u64,
    port_chan0: PortReadOnly<u16>,
    port_mode: PortWriteOnly<u16>,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            ticks: 0,
            port_chan0: PortReadOnly::new(TimerChannel::Channel0 as u16),
            port_mode: PortWriteOnly::new(TimerChannel::Mode as u16),
        }
    }

    pub fn initialize(&mut self) {
        unsafe { self.port_mode.write(0b00110000) }; // TODO:
    }
}

impl Interrupt for Timer {
    fn handle(&mut self, _: InterruptStackFrame) {
        self.ticks += 1;
        kprint!(
            "Timer IRQ! port read {}, tick {}\n",
            unsafe { self.port_chan0.read() },
            self.ticks
        );
    }
}
