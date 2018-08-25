#![no_main]
#![no_std]

extern crate panic_halt;
#[macro_use]
extern crate trustflight_board;
extern crate trustflight_firmware;

use trustflight_board::cortex_m::asm;

entry!(|| loop {
    asm::bkpt();
});
