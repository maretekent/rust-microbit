# (Pure) Rust on the BBC micro:bit

This is heavily based on Simon Sapin's
[rust-on-bbc-microbit](https://github.com/SimonSapin/rust-on-bbc-microbit)
work. Most code is stolen from there. Note that this project is using pure
Rust, not a single line of C code!

## Status

Currently there is code that supports the following peripherals:

- Serial: As it's routed through USB, it is easy to send debug messages. Currently only sending is tested, not receiving.
- Display: Turn on arbitrary LEDs using x and y coordinates.
- Analog-Digital-Converter (ADC): For example, can route pad 0 to the ADC.
- Buttons: Can be accessed via GPIO. No debouncing.

*Not* supported at the moment:

- USB
- Bluetooth (and probably never will)
- Compass
- Accelerometer

## Installation

### DragonFly

We need the following packages:

    pkg ins llvm39 gcc-arm-embedded

Note that I am trying hard to remove the dependency on `gcc`. Right now it is
only required for linking.

You need a development version of Rust 1.15 to be able to compile `libcore`.
Basically on DragonFly this means that you have to compile Rust and Cargo from
source. I use the following version:

    rustc 1.15.0-dev (d14d74d5f 2016-12-04)

I am using `rustup`, so I set `rustup default /usr/local/my-version-of-rust`.

The Makefile I am using is adapted to my machine. You probably want to modify
it matching your paths.

Once everything is installed, you should be able to produce a hex file using

    make build

To flash it to the target (the usb mounted device of your micro:bit probably
has a different serial number, so you may want to change the Makefile in this regards):

    # as root
    make flash

Finally, you can connect to the serial console:

    # as root (or user tty, see below)
    make serial

On BSD, add the following line to /etc/devfs.conf:

    perm    ttyU*   root:tty        0660

And add yourself to the group ```tty```. Then you don't need to be root
or use ```sudo```.
