[![License:Zlib](https://img.shields.io/badge/License-Zlib-green.svg)](https://opensource.org/licenses/Zlib)
[![License:Apache2](https://img.shields.io/badge/License-Apache2-green.svg)](https://www.apache.org/licenses/LICENSE-2.0)
[![License:MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)

[![ci](https://github.com/rust-console/gba/workflows/ci/badge.svg?branch=master)](https://github.com/rust-console/gba/actions?query=workflow%3Aci)
[![crates.io](https://img.shields.io/crates/v/gba.svg)](https://crates.io/crates/gba)
[![docs.rs](https://docs.rs/gba/badge.svg)](https://docs.rs/gba/latest/gba/)

* [![Built with cargo-make](https://sagiegurari.github.io/cargo-make/assets/badges/cargo-make.svg)](https://sagiegurari.github.io/cargo-make)
* ![Stability:None](https://img.shields.io/badge/Stability-None-red.svg)

# gba

This is a crate to make it easy to write a GBA game in rust.

## Build Dependencies

Install some stuff from rustup and cargo:
```sh
rustup install nightly
rustup +nightly component add rust-src
cargo install cargo-make
cargo install gbafix
```

Install arm build tools
* Ubuntu
  ```shell
  sudo apt-get install binutils-arm-none-eabi
  ```
* OSX
  ```shell
  brew install --cask gcc-arm-embedded
  ```
* Windows
  * Download the [GNU Arm Embedded Toolchain](https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/gnu-rm/downloads)
  * Install the toolchain, make sure to select "Add path to environment variable" during install

## First Time Setup

Writing a Rust program for the GBA requires a fair amount of special setup. All
of the steps are detailed for you in the [Development
Setup](https://rust-console.github.io/gba/development-setup.html) part at the
start of the book.

If you've done the described global setup once before and just want to get a new
project started quickly we got you covered:

```sh
curl https://raw.githubusercontent.com/rust-console/gba/master/init.sh -sSf | bash -s APP_NAME
```

## Building

When building, you'll have to:
* use Nightly (if you don't already use it by default)
* use the `-Zbuild-std=core` cargo flag.
* have `-Clink-arg=-Tlinker.ld` as part of your `RUSTFLAGS` environment variable.
* build with `--target thumbv4t-none-eabi`

If you copy the `.cargo/config.toml` file provided in this repository into your
own project then cargo will see it and set all the right values automatically.

# Contribution

This crate is Apache2 licensed and any contributions you submit must also be
Apache2 licensed.
