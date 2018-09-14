#![no_main]
#![no_std]

extern crate panic_halt;
extern crate trustflight_board;
extern crate trustflight_firmware;

use core::ptr::read_volatile;
use core::sync::atomic::{self, Ordering};
use trustflight_board::TrustflightBoardRevA as fcu_board;
use trustflight_board::*;

pub fn black_box<T>(dummy: &T) {
    unsafe {
        read_volatile(&dummy);
    }
}

#[entry]
fn main() -> ! {
    let board = fcu_board::new();

    board.init_board();

    let mut time = fcu_board::time();

    loop {
        black_box(&time);
        time = fcu_board::time();
        atomic::compiler_fence(Ordering::SeqCst);
    }
}
