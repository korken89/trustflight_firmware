# TrustFlight

[![Build Status](https://travis-ci.org/korken89/trustflight_firmware.svg?branch=master)](https://travis-ci.org/korken89/trustflight_firmware)

A flight controller firmware you can trust!

## Aim

The aim of this project is twofold, 1) I am looking to learn Rust and how its `borrow checker` can be applied in embedded design to constrain and guide, while 2) I want to compare `Rust vs latest C++`, especially in the metaprogramming functionality when it comes to enforcing constraints at compile time.

From a practical perspective this is mostly a complete rewrite/redesign of the [old KFly code](https://github.com/korken89/kfly_firmware) into Rust, with better modularization and less complexity.

## Layout

In the `trustflight` folder is the main firmware, while in `trustflight-hal` is the low-level code for talking to the hardware, including abstrations, aka the HAL.

## Features

* Runs the [cortex-m-rtfm scheduler](https://github.com/japaric/cortex-m-rtfm), practically the same as [crect (C++ version)](https://github.com/korken89/crect)
* Minimal communication overhead to a high level flight computer via [COBS ZPE/ZRE](https://tools.ietf.org/html/draft-ietf-pppext-cobs-00) together with priority buffers to minimize delay and jitter
* All communication is designed to be `DMA` driven, no CPU intervention needed
* Adaptive notch filters to reduce impact of vibrations (similar to Betaflight's `dynamic filter`)
* Modern sensor suite
	* ST's ISM330DLC IMU - with heater to temperature stabilize the sensor
	* Bosch's BMP388 barometer
* Support for the latest protocols
	* `F.port` for RC input and telemetry to the pilot
	* `DShot` motor protocol with `telemetery` at about 200 Hz per motor
* Has accompanying reference hardware (OSHW) [available here](https://github.com/korken89/trustflight_hardware)
	* Has (soon) a `board trait` to allow for simple support of other hardware platforms


## Contributors

List of contributors in alphabetical order:

* Emil Fresk ([@korken89](https://github.com/korken89))

---

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

