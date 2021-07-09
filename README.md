[![License:Zlib](https://img.shields.io/badge/License-Zlib-green.svg)](https://opensource.org/licenses/Zlib)
[![License:Apache2](https://img.shields.io/badge/License-Apache2-green.svg)](https://www.apache.org/licenses/LICENSE-2.0)
[![License:MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)

[![ci](https://github.com/rust-console/gba/workflows/ci/badge.svg?branch=main)](https://github.com/rust-console/gba/actions?query=workflow%3Aci)
[![crates.io](https://img.shields.io/crates/v/gba.svg)](https://crates.io/crates/gba)
[![docs.rs](https://docs.rs/gba/badge.svg)](https://docs.rs/gba/latest/gba/)

* ![Stability:None](https://img.shields.io/badge/Stability-None-red.svg)

# gba

A crate to make GBA programming easy.

Currently we don't have as much documentation as we'd like.
If you check out the [awesome-gbadev](https://github.com/gbdev/awesome-gbadev) repository they have many resources, though most are oriented towards C.

## System Setup

There's a few extra things to install that you just need to do once per system.

Building for the GBA requires Nightly rust, and also uses the `build-std` feature, so you'll need the rust source available.

```sh
rustup install nightly
rustup +nightly component add rust-src
```

You'll also need the ARM binutils so that you can have the assembler and linker for the ARMv4T architecture.
The way to get them varies by platform:
* Ubuntu and other debian-like linux distros will usually have them in the package manager.
  ```shell
  sudo apt-get install binutils-arm-none-eabi
  ```
* With OSX you can get them via homebrew.
  ```shell
  brew install --cask gcc-arm-embedded
  ```
* On Windows you can get the installer from ARM's website and run that.
  * Download the [GNU Arm Embedded Toolchain](https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/gnu-rm/downloads)
  * When installing the toolchain, make sure to select "Add path to environment variable" during install.
  * You'll have to restart any open command prompts after you so run the installer so that they see the new PATH value.

Finally, rustc itself is only able to make ELF format files. These can be run in emulators, but aren't able to be played on actual hardware.
You'll need to convert the ELF file into a GBA rom. There's a `cargo-make` file in this repository to do this, and it relies on a tool called `gbafix`
to assign the right header data to the ROM when packing it.

```sh
cargo install cargo-make
cargo install gbafix
```

## Project Setup

To build a GBA project, you'll want to copy the `.cargo/config.toml` file from this repo into your own project.

Then use one of the examples as a guide to get started.

When you build your project, cargo will put outputs in the `target/thumbv4t-none-eabi/` directory.
This includes the `debug/` and `release/` sub-directories.
Your binary will be in there, but it'll be in ELF format.
You can run this directly in an emulator such as [mGBA](https://mgba.io/) if you'd like.

When you're ready to convert your program into a "proper" GBA rom you'll need to run an `objcopy`
to extract just the raw binary data:
```
arm-none-eabi-objcopy -O binary [RUST_BINARY_NAME] [ROM_NAME].gba
```

Then you'll need to patch the header data with `gbafix`
```
gbafix [ROM_NAME].gba
```
And you'll be all done!

# Contribution

This crate is tri-licensed under Zlib / Apache-2.0 / MIT.
Any contributions you submit must be licensed the same.
