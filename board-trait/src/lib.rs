//! # Trustflight board trait
//!
//! This trait defines the required functionality of a Trustflight board. It is inspired by the
//! architecture of [`rosflight`](https://github.com/rosflight/firmware), while being adapted and
//! extended as necessary.

#![no_std]

extern crate trustflight_utils;
pub use trustflight_utils::time;

// TODO: Will be updated a lot
pub trait TrustflightBoard {
    // New
    fn new() -> Self;

    // System clock
    const CPU_CLK: u32;

    // Setup
    fn init_board(&self);

    // System time
    fn time() -> time::ClockTicks;

    // Communication
    // fn communication_init(&self);

    // Sensors
    // fn sensors_init(&self);
    // fn new_imu_data(&self);
    // fn new_baro_data(&self);

    // RC
    // fn rc_init(&self);
    // fn rc_read(&self);

    // Outputs
    // fn output_init(&self);
    // fn output_write(&self);

    // Non-volatile memory
    // fn memory_init(&self);
    // fn memory_read(&self);
    // fn memory_write(&self);

    // LEDs
    // fn led0_on(&self);
    // fn led0_off(&self);
    // fn led0_toggle(&self);

    // fn led1_on(&self);
    // fn led1_off(&self);
    // fn led1_toggle(&self);
}
