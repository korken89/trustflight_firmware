#![no_main]
#![no_std]

extern crate panic_halt;
#[macro_use]
extern crate trustflight_hal;

use trustflight_hal::cortex_m::asm;

entry!(|| loop {
    asm::bkpt();
});
