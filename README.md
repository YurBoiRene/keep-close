keep-close
==========

 Arduino Uno alarm system that blares when a door is open for too long. Great to put on your freezer, front door, secret drawer or anything that shouldn't be open for more than a few seconds.

The project is build in Rust for the _Arduino Uno_ using [ `avr-hal` ]. In the future it could be ported to an _Arduino Nano_ as it seems more suited for the job.

## Features

- [x] Detect when door is opened
- [x] Detect when door is opened for too long
- [x] Buzzer sound when door is opened for too long
- [ ] Option to use active buzzer
- [ ] Play melody on door open
- [ ] Physical implementation
- [ ] Disable button

## Build Instructions

*[support](#nix-build) for Nix*.

1. Install prerequisites as described in the [`avr-hal` README] (`avr-gcc`, `avr-libc`, `avrdude`, [`ravedude`]).

2. Run `cargo build` to build the firmware.

3. Run `cargo run` to flash the firmware to a connected board.  If `ravedude` fails to detect your board, check its documentation at <https://crates.io/crates/ravedude>.

4. `ravedude` will open a console session after flashing where you can interact
   with the UART console of your board.

[`avr-hal`]: https://github.com/Rahix/avr-hal
[`avr-hal` README]: https://github.com/Rahix/avr-hal#readme
[`ravedude`]: https://crates.io/crates/ravedude

### Nix Build

Simply `nix develop` to open a shell with dependencies. Steps 2-4 are the same as [above](#build-instructions).

## Hardware

TODO: add pictures

| Item             | Used for |
|------------------|-----|
| Arduino Uno      | 🧠 |
| Breadboard (pref with sticky) | Hold components and attach to door, side of freezer, etc. |
| Reed switch + magnet | To detect door open |
| Passive buzzer   | Sound/Melodies |
| Button           | Disable buzzer |


**Pin 2** is connected to the reed switch as pull-up input.

**Pin 9** is connected to the buzzer for PWM under Timer 1 (`OC1A`).
