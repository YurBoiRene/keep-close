keep-close
==========

Alarm system for doors that should be closed. Great to put on your freezer, front door, secret drawer or anything that shouldn't be open for more than 30 seconds.

The project is build in Rust for the _Arduino Uno_ but in the future could be ported to an _Arduino Nano_ as it seems more suited for the job.

## Build Instructions

*this project has [support](#nix-build) for Nix*.

1. Install prerequisites as described in the [`avr-hal` README] (`avr-gcc`, `avr-libc`, `avrdude`, [`ravedude`]).

2. Run `cargo build` to build the firmware.

3. Run `cargo run` to flash the firmware to a connected board.  If `ravedude`
   fails to detect your board, check its documentation at
   <https://crates.io/crates/ravedude>.

4. `ravedude` will open a console session after flashing where you can interact
   with the UART console of your board.

[`avr-hal` README]: https://github.com/Rahix/avr-hal#readme
[`ravedude`]: https://crates.io/crates/ravedude

## Nix Build

Simply `nix develop` to open a shell with dependencies. Steps 2-4 are the same as [above](#build-instructions).
