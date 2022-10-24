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
//!   initializes the system. The C ABI must be used because Rust's own ABI is
//!   not stable.
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
pub mod random;
pub mod sound;
pub mod timers;
pub mod video;

/// Wraps a value to be aligned to a minimum of 4.
///
/// If the size of the value held is already a multiple of 4 then this will be
/// the same size as the wrapped value. Otherwise the compiler will add
/// sufficient padding bytes on the end to make the size a multiple of 4.
#[repr(C, align(4))]
pub struct Align4<T>(pub T);

impl<const N: usize> Align4<[u8; N]> {
  /// Views these bytes as a slice of `u32`
  /// ## Panics
  /// * If the number of bytes isn't a multiple of 4
  pub fn as_u32_slice(&self) -> &[u32] {
    assert!(self.0.len() % 4 == 0);
    // Safety: our struct is aligned to 4, so the pointer will already be
    // aligned, we only need to check the length
    unsafe {
      let data: *const u8 = self.0.as_ptr();
      let len: usize = self.0.len();
      core::slice::from_raw_parts(data.cast::<u32>(), len / 4)
    }
  }

  /// Views these bytes as a slice of `u16`
  /// ## Panics
  /// * If the number of bytes isn't a multiple of 2
  pub fn as_u16_slice(&self) -> &[u16] {
    assert!(self.0.len() % 2 == 0);
    // Safety: our struct is aligned to 4, so the pointer will already be
    // aligned, we only need to check the length
    unsafe {
      let data: *const u8 = self.0.as_ptr();
      let len: usize = self.0.len();
      core::slice::from_raw_parts(data.cast::<u16>(), len / 2)
    }
  }
}

/// Works like [`include_bytes!`], but the value is wrapped in [`Align4`].
#[macro_export]
macro_rules! include_aligned_bytes {
  ($file:expr $(,)?) => {{
    let LONG_NAME_THAT_DOES_NOT_CLASH = *include_bytes!($file);
    Align4(LONG_NAME_THAT_DOES_NOT_CLASH)
  }};
}
