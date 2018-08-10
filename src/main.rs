#![no_main]
#![no_std]

extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt as rt;
extern crate panic_halt;
extern crate stm32f3;

use core::ptr;
use cortex_m::asm;
use rt::ExceptionFrame;
use stm32f3::stm32f302;

entry!(main);

fn main() -> ! {
    let peripherals = stm32f302::Peripherals::take().unwrap();

    loop {
        unsafe {
            ptr::read_volatile(&peripherals);
        }
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
