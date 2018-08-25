#![no_std]

pub extern crate cortex_m;
pub extern crate cortex_m_rt;
extern crate stm32f3;
extern crate trustflight_board_trait;

pub use cortex_m_rt::*;

//
// Testing requirements
//
#[cfg(test)]
#[macro_use]
extern crate std;

#[cfg(test)]
mod tests;
