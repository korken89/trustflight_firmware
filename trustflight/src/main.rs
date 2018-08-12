#![no_main]
#![no_std]

extern crate panic_halt;
#[macro_use]
extern crate trustflight_hal;

use trustflight_hal::cortex_m::asm;
use trustflight_hal::cortex_m_rt::ExceptionFrame;

entry!(main);

#[inline(never)]
fn main() -> ! {
    loop {
        asm::bkpt();
    }
}

// define the hard fault handler
exception!(HardFault, hard_fault);

fn hard_fault(_ef: &ExceptionFrame) -> ! {
    panic!("hard fault");
}

// define the default exception handler
exception!(*, default_handler);

fn default_handler(_irqn: i16) {
    panic!("default handler");
}
