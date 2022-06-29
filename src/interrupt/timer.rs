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

static mut MODE_PORT: PortWriteOnly<u16> = PortWriteOnly::new(TimerChannel::Mode as u16);

lazy_static! {
    pub static ref TIMER: Mutex<Timer> = {
        unsafe { MODE_PORT.write(0b00110000) }; // TODO:
        Mutex::new(Timer::new())
    };
}

pub struct Timer {
    ticks: u64,
}

impl Timer {
    pub fn new() -> Self {
        Self { ticks: 0 }
    }
}

impl Interrupt for Timer {
    fn handle(&mut self, _: InterruptStackFrame) {
        let mut port: PortReadOnly<u16> = PortReadOnly::new(TimerChannel::Channel0 as u16);
        let data = unsafe { port.read() };

        self.ticks += 1;
        kprint!("Timer IRQ! port read {}, tick {}\n", data, self.ticks);
    }
}
