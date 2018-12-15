[![License:Apache2](https://img.shields.io/badge/License-Apache2-green.svg)](https://www.apache.org/licenses/LICENSE-2.0)
[![travis.ci](https://travis-ci.org/rust-console/gba.svg?branch=master)](https://travis-ci.org/rust-console/gba)
[![crates.io](https://img.shields.io/crates/v/gba.svg)](https://crates.io/crates/gba)
[![docs.rs](https://docs.rs/gba/badge.svg)](https://docs.rs/gba/latest/gba/)

* [![Built with cargo-make](https://sagiegurari.github.io/cargo-make/assets/badges/cargo-make.svg)](https://sagiegurari.github.io/cargo-make)
* ![Stability:None](https://img.shields.io/badge/Stability-None-red.svg)

# gba

This repository is both a [Tutorial Book](https://rust-console.github.io/gba/)
that teaches you what you need to know to write Rust games for the GameBoy
Advance (GBA), and also a [crate](https://crates.io/crates/gba) that you can
use to do the same.

## First Time Setup

Writing a Rust program for the GBA requires a fair amount of special setup. All
of the steps are detailed for you [in the Introduction chapter of the
book](https://rust-console.github.io/gba/00-introduction/03-development-setup.html).

If you've done the described global setup once before and just want to get a new
project started quickly we got you covered:

```sh
curl https://raw.githubusercontent.com/rust-console/gba/master/init.sh -sSf | bash -s APP_NAME
```

# Contribution

This crate is Apache2 licensed and any contributions you submit must also be
Apache2 licensed.
