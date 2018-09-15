//! # time
//!
//! A helper module for creating type-safe time based entities.

/// Base type for frequency, everything is based on Hertz
#[derive(PartialOrd, PartialEq, Debug, Copy, Clone)]
pub struct Hertz(u32);

#[derive(PartialOrd, PartialEq, Debug, Clone, Copy)]
pub struct ClockTicks(u64);

impl ClockTicks {
    pub fn new(ticks: u64) -> ClockTicks {
        ClockTicks(ticks)
    }
}

/// Used to implement conversions to the Hertz struct
pub trait ToHertz {
    /// From hertz
    fn hz(self) -> Hertz;

    /// From kilohertz
    fn khz(self) -> Hertz;

    /// From megahertz
    fn mhz(self) -> Hertz;
}

impl ToHertz for u32 {
    fn hz(self) -> Hertz {
        Hertz::new(self)
    }

    fn khz(self) -> Hertz {
        Hertz::new(self * 1_000)
    }

    fn mhz(self) -> Hertz {
        Hertz::new(self * 1_000_000)
    }
}

impl Hertz {
    pub fn new(hz: u32) -> Hertz {
        Hertz(hz)
    }

    pub fn hz(self) -> u32 {
        self.0
    }
}
