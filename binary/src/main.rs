#![no_main]
#![no_std]

extern crate panic_halt;
extern crate trustflight_board;
extern crate trustflight_firmware;

use trustflight_board::cortex_m::asm;
use trustflight_board::entry;

#[entry]
fn main() -> ! {
    loop {
        asm::bkpt();
    }
}
