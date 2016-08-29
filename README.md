# Rust on the BBC micro:bit

This is heavily based on Simon Sapin's
[rust-on-bbc-microbit](https://github.com/SimonSapin/rust-on-bbc-microbit)
work. Most code is stolen from there.

## Dependencies

### DragonFly

We need the following packages:

    pkg ins llvm38 gcc-arm-embedded srecord

Note that I am trying hard to remove the dependency on `gcc`. Right now it is
only required for linking.

You need a development version of Rust 1.11 to be able to compile `libcore`.
Basically on DragonFly this means that you have to compile Rust and Cargo from
source. I use the following version:

    rustc 1.11.0-dev (55e84396e 2016-08-06)

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

    # as root
    make serial
