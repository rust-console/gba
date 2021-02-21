[![License:Apache2](https://img.shields.io/badge/License-Apache2-green.svg)](https://www.apache.org/licenses/LICENSE-2.0)
[![ci](https://github.com/rust-console/gba/workflows/ci/badge.svg?branch=master)](https://github.com/rust-console/gba/actions?query=workflow%3Aci)
[![crates.io](https://img.shields.io/crates/v/gba.svg)](https://crates.io/crates/gba)
[![docs.rs](https://docs.rs/gba/badge.svg)](https://docs.rs/gba/latest/gba/)

* [![Built with cargo-make](https://sagiegurari.github.io/cargo-make/assets/badges/cargo-make.svg)](https://sagiegurari.github.io/cargo-make)
* ![Stability:None](https://img.shields.io/badge/Stability-None-red.svg)

# gba

_Eventually_ there will be a full [Tutorial
Book](https://rust-console.github.io/gba/) that goes along with this crate.
However, currently the development focus is leaning towards having minimal
coverage of all the parts of the GBA. Until that's done, unfortunately the book
will be in a rather messy state.

## What's Missing

The following major GBA features are still missing from the crate:

* Affine Graphics
* Interrupt Handling
* Serial Communication

## Build Dependencies

Install required cargo packages
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

# Contribution

This crate is Apache2 licensed and any contributions you submit must also be
Apache2 licensed.
