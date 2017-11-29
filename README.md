MyLain-rs
=========

## The Very Useful, Personal Assistant for Smart Homes -- Rust port
MyLain-rs is a small experiment for a smart personal home assistant, written in Rust (previous versions were written in C). It is supposed to run, primarily, on Unix consoles for both Desktop and Smartphone devices (preferably Linux and Termux).

## How it works
The plans are to make it operate so that MyLain monitores your network, and detects whenever you join a known network. Once that happens, MyLain will try to integrate with the other relays on the network, in a peer-to-peer fashion.

Once MyLain is connected to other devices, it may remotely use their public modules (if you have set their permissions right), and possibly use distributed processing to remotely solve tasks.

Those are the actual plans, though. It might be quite audacious, and there are security concerns about that, but I'm currently more concentrated on having a working prototype before asking myself how to improve it. So I strongly advise you to not use this commercially -- and if you do, or if you fork it, please respect the GNU LGPL v3 license.

## Usability
MyLain is actually a small REPL, which is supposed to be extended by configuring submodules that could be loaded dynamically, like daemons. I'm still not concerned about how this will be done, but it's possible that it'll involve some sort of FFI.

The actual REPL is comprised of so little things right now, so soon you'll be able to type `help` after starting it to see the quick cheatsheet. `help [command]` will also give you more information on said command.

## Building
### Building natively
This project uses Cargo as project manager, so you can just build everything by typing `cargo build`, or run it by typing `cargo run`.

### Cross-compiling for Raspberry Pi
It is possible to build this project for an ARMv7 target -- specifically, for the Raspberry Pi 3 Model B, which was used for this test.

#### Installing GCC ARM Toolchain
First, you'll need to install the ARM toolchain for your distro. This will depend on which distro you're on; on Void Linux, you'll install the packages `cross-arm-linux-gnueabihf` and `cross-arm-linux-gnueabihf` (Also, you may need to create a symlink to folder `/usr/arm-linux-gnueabihf/usr/lib` on folder `/usr/arm-linux-gnueabihf` so the proper libraries are found.

#### Installing Cargo's ARM target
Now, you'll need to install Cargo's ARM target. You can do that by using `rustup`.

`$ rustup target add armv7-unknown-linux-gnueabihf`

Then, after installed, edit the file `~/.cargo/config` and put the following in it:

```conf
[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
```

#### Building the project
All that's left to do is to actually build the project. `cd` to the project folder and run the following command to cross-compile the project:

`$ cargo build --target=armv7-unknown-linux-gnueabihf`

After compilation, you should then be able to find the ARM binary on path `target/armv7-unknown-linux-gnueabihf/debug`. Just copy it to your Raspberry Pi and you're all set.

## License
This project is distributed under the GNU LGPLv3 license. See `LICENSE` or the header of source files for details.

Copyright (c) 2017 Lucas Vieira.
