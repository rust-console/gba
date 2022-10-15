#![no_std]
#![feature(asm_sym)]
#![feature(asm_const)]
#![feature(isa_attribute)]
#![feature(naked_functions)]
//#![warn(missing_docs)]

//! A crate for GBA development.
//!
//! ## How To Make Your Own GBA Project Using This Crate
//!
//! This will require the use of Nightly Rust. Any recent-ish version of Nightly
//! should be fine.
//!
//! [arm-download]:
//!     https://developer.arm.com/Tools%20and%20Software/GNU%20Toolchain
//!
//! * **Get The ARM Binutils:** You'll need the ARM version of the GNU binutils
//!   in your path, specifically the linker (`arm-none-eabi-ld`). Linux folks
//!   can use the package manager. Mac and Windows folks can use the [ARM
//!   Website][arm-download].
//! * **Run `rustup component add rust-src`:** This makes rustup keep the
//!   standard library source code on hand, which is necessary for `build-std`
//!   to work.
//! * **Create A `.cargo/config.toml`:** You'll want to set up a file to provide
//!   all the right default settings so that a basic `cargo build` and `cargo
//!   run` will "just work". Something like the following is what you probably
//!   want.
//!
//! ```toml
//! [build]
//! target = "thumbv4t-none-eabi"
//!
//! [unstable]
//! build-std = ["core"]
//!
//! [target.thumbv4t-none-eabi]
//! runner = "mgba-qt"
//! rustflags = ["-Clink-arg=-Tlinker_scripts/mono_boot.ld"]
//! ```
//!
//! * **Make Your Executables:** At this point you can make a `bin` or an
//!   `example` file. Every executable will need to be `#![no_std]` and
//!   `#![no_main]`. They will also need a `#[panic_handler]` defined, as well
//!   as a `#[no_mangle] extern "C" fn main() -> ! {}` function, which is what
//!   the assembly runtime will call to start your Rust program after it fully
//!   initializes the system.
//!
//! ```rust
//! #![no_std]
//! #![no_main]
//!
//! #[panic_handler]
//! fn panic_handler(_: &core::panic::PanicInfo) -> ! {
//!   loop {}
//! }
//!
//! #[no_mangle]
//! extern "C" fn main() -> ! {
//!   loop {}
//! }
//! ```
//!
//! * **Optional: Use `objcopy` and `gbafix`:** The `cargo build` will produce
//!   ELF files, which mGBA can run directly. If you want to run your program on
//!   real hardware you'll need to first `objcopy` the raw binary out of the ELF
//!   into its own file, then Use `gbafix` to give an appropriate header to the
//!   file. `objcopy` is part of the ARM binutils you already installed, it
//!   should be named `arm-none-eabi-objcopy`. You can get `gbafix` through
//!   cargo: `cargo install gbafix`.
//!
//! ## Other GBA-related Crates
//!
//! This crate provides a largely "unmanaged" interaction with the GBA's
//! hardware. If you would like an API that use the borrow checker to guide you
//! more, the [agb](https://docs.rs/agb) crate might be what you want.
//!
//! ## Safety
//!
//! All safety considerations for the crate assume that you're building for the
//! `thumbv4t-none-eabi` or `armv4t-none-eabi` targets, using the provided
//! linker script, and then running the code on a GBA. While it's possible to
//! break any of these assumptions, if you do that some or all of the code
//! provided by this crate may become unsound.

mod macros;

pub mod asm_runtime;
pub mod bios;
pub mod builtin_art;
pub mod dma;
pub mod fixed;
pub mod gba_cell;
pub mod interrupts;
pub mod keys;
pub mod mgba;
pub mod mmio;
pub mod prelude;
pub mod sound;
pub mod timers;
pub mod video;
