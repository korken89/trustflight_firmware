#![no_main]
#![no_std]

extern crate cortex_m;
extern crate stm32f302x;

#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;

// makes `panic!` print messages to the host stderr using semihosting
// extern crate panic_semihosting;
extern crate panic_abort;

use cortex_m::asm;
use rt::ExceptionFrame;

// the program entry point is ...
entry!(main);

// ... this never ending function
fn main() -> ! {
    loop {
        asm::bkpt();
    }
}

// define the hard fault handler
exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

// define the default exception handler
exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
