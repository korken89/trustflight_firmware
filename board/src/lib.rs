#![no_std]

pub extern crate cortex_m;
pub extern crate cortex_m_rt;
extern crate stm32f3;
extern crate trustflight_board_trait as tbt;

pub use cortex_m::interrupt::free as critical_section;
pub use cortex_m::peripheral as corep;
pub use cortex_m_rt::entry;
use stm32f3::stm32f302 as mcu;
pub use tbt::time::*;
pub use tbt::TrustflightBoard;

//
// Board impl
//

pub struct TrustflightBoardRevA;

fn init_accelerators(flash: &mut mcu::FLASH) {
    // Enable prefetch buffer, latency to 2 WS, no half cycle reads
    flash
        .acr
        .write(|w| unsafe { w.prftbe().set_bit().latency().bits(2) });
}

fn init_clocks(rcc: &mut mcu::RCC, mut dcb: corep::DCB, mut dwt: corep::DWT) {
    //
    // Debug clocks
    //

    dcb.enable_trace();
    dwt.enable_cycle_counter();

    //
    // Initialize PLL and Clock source
    //

    // Set HSE configuration to BYPASS (16 MHz oscillator) and divide by 2
    rcc.cr.modify(|_, w| w.hseon().set_bit().hsebyp().set_bit());
    rcc.cfgr2.modify(|_, w| unsafe { w.prediv().bits(1) });

    // Wait for HSE
    while rcc.cr.read().hserdy().bit_is_clear() {}

    //
    // PLL startup
    //

    // PLL Source = HSE, PLL Multiplier = 9 (x - 2 in register)
    rcc.cfgr
        .modify(|_, w| unsafe { w.pllsrc().bits(2).pllmul().bits(7) });

    // Enable PLL
    rcc.cr.modify(|_, w| w.pllon().set_bit());

    // Wait for PLL
    while rcc.cr.read().pllrdy().bit_is_clear() {}

    //
    // Clock tree setup
    //

    // HCLK - no division
    rcc.cfgr.modify(|_, w| unsafe { w.hpre().bits(0) });

    // SYSCLK - select PLL
    rcc.cfgr.modify(|_, w| unsafe { w.sw().bits(2) });

    // Wait for SYSCLK switch
    while rcc.cfgr.read().sws().bits() != 2 {}

    // PCLK1 - /2 from SYSCLK
    rcc.cfgr.modify(|_, w| unsafe { w.ppre1().bits(4) });

    // PCLK2 - no division
    rcc.cfgr.modify(|_, w| unsafe { w.ppre1().bits(0) });

    //
    // Peripheral clocks
    //

    // Peripheral clock selectors
    rcc.cfgr3.modify(|_, w| unsafe {
        w.usart1sw() // SYSCLK
            .bits(1)
            .usart2sw() // SYSCLK
            .bits(1)
            .usart3sw() // SYSCLK
            .bits(1)
            .i2c1sw() // SYSCLK
            .set_bit()
            .tim1sw() // PCLK2
            .clear_bit()
    });

    // Peripheral clock enables
    rcc.ahbenr.modify(|_, w| {
        w.dmaen() // DMA1
            .set_bit()
            .dma2en()
            .set_bit()
            .iopaen()
            .set_bit()
            .iopben()
            .set_bit()
            .iopcen()
            .set_bit()
            .iopfen()
            .set_bit()
    });

    rcc.apb1enr.modify(|_, w| {
        w.spi3en()
            .set_bit()
            .pwren()
            .set_bit()
            .tim2en()
            .set_bit()
            .tim4en()
            .set_bit()
            .usart2en()
            .set_bit()
            // .usart3en() - missing from the stm32f302 crate
            // .set_bit()
            .i2c1en() // Not currently connected, will be fixed in next iteration of board
            .set_bit()
    });

    // Enable USART3, temporary fix until fixed in stm32f302 crate
    rcc.apb1enr
        .modify(|r, w| unsafe { w.bits(r.bits() | 1 << 18) });

    rcc.apb2enr.modify(|_, w| {
        w.tim1en()
            .set_bit()
            .usart1en()
            .set_bit()
            .tim15en()
            .set_bit()
    });
}

impl tbt::TrustflightBoard for TrustflightBoardRevA {
    // New
    fn new() -> Self {
        TrustflightBoardRevA {}
    }

    // Setup
    fn init_board(&self) {
        let mut mcu_peripherals = mcu::Peripherals::take().unwrap();
        let cortex_peripherals = cortex_m::Peripherals::take().unwrap();

        // Core initialization
        init_accelerators(&mut mcu_peripherals.FLASH);
        init_clocks(
            &mut mcu_peripherals.RCC,
            cortex_peripherals.DCB,
            cortex_peripherals.DWT,
        );

        // Init system modules
    }

    // System time
    fn time() -> ClockTicks {
        // The internal clock is based on the Debug cycle counter running at CPU rate, however
        // since it overflows quite often it is extended to 64 bits.

        static mut DWT_OVERFLOWS: u32 = 0;
        static mut OLD_DWT: u32 = 0;

        let cyccnt = corep::DWT::get_cycle_count();

        critical_section(|_| {
            // Total clock cycles since start
            unsafe {
                if cyccnt < OLD_DWT {
                    DWT_OVERFLOWS += 1;
                }
                OLD_DWT = cyccnt;

                ClockTicks::new((DWT_OVERFLOWS as u64) << 32 | cyccnt as u64)
            }
        })
    }
}

//
// Testing requirements
//
#[cfg(test)]
#[macro_use]
extern crate std;

#[cfg(test)]
mod tests;
