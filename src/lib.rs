#![no_std]
#![cfg_attr(feature = "aeabi_mem_fns", feature(naked_functions))]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::let_and_return)]
#![allow(clippy::result_unit_err)]
#![warn(clippy::missing_inline_in_public_items)]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, test_runner(test_harness::test_runner))]
#![cfg_attr(test, no_main)]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]

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
//! runner = "mgba-qt" # sets the emulator to run bins/examples with
//! rustflags = [
//!   "-Clinker=arm-none-eabi-ld", # uses the ARM linker
//!   "-Clink-arg=-Tlinker_scripts/mono_boot.ld", # sets the link script
//! ]
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
//! This crate provides an API to interact with the GBA that is safe, but with
//! minimal restrictions on what components can be changed when. If you'd like
//! an API where the borrow checker provides stronger control over component
//! access then the [agb](https://docs.rs/agb) crate might be what you want.
//!
//! ## Safety
//!
//! All safety considerations for the crate assume that you're building for the
//! `thumbv4t-none-eabi` or `armv4t-none-eabi` targets, using the provided
//! linker script, and then running the code on a GBA. While it's possible to
//! break any of these assumptions, if you do that some or all of the code
//! provided by this crate may become unsound.

#[cfg(feature = "on_gba")]
use prelude::{GbaCell, IrqFn};

mod macros;

#[cfg(feature = "on_gba")]
mod asm_runtime;
#[cfg(feature = "on_gba")]
pub mod bios;
pub mod builtin_art;
#[cfg(feature = "critical-section")]
mod critical_section;
#[cfg(feature = "on_gba")]
pub mod dma;
pub mod fixed;
#[cfg(feature = "on_gba")]
pub mod gba_cell;
pub mod interrupts;
pub mod keys;
pub mod mem;
#[cfg(feature = "on_gba")]
pub mod mgba;
#[cfg(feature = "on_gba")]
pub mod mmio;
pub mod prelude;
pub mod random;
pub mod sound;
pub mod timers;
pub mod video;

/// The function pointer that the assembly runtime calls when an interrupt
/// occurs.
#[cfg(feature = "on_gba")]
pub static RUST_IRQ_HANDLER: GbaCell<Option<IrqFn>> = GbaCell::new(None);

/// Wraps a value to be aligned to a minimum of 4.
///
/// If the size of the value held is already a multiple of 4 then this will be
/// the same size as the wrapped value. Otherwise the compiler will add
/// sufficient padding bytes on the end to make the size a multiple of 4.
#[derive(Debug)]
#[repr(C, align(4))]
pub struct Align4<T>(pub T);

impl<const N: usize> Align4<[u8; N]> {
  /// Views these bytes as a slice of `u32`
  /// ## Panics
  /// * If the number of bytes isn't a multiple of 4
  #[inline]
  #[must_use]
  pub const fn as_u32_slice(&self) -> &[u32] {
    self.as_slice()
  }

  /// Views these bytes as a slice of `u16`
  /// ## Panics
  /// * If the number of bytes isn't a multiple of 2
  #[inline]
  #[must_use]
  pub const fn as_u16_slice(&self) -> &[u16] {
    self.as_slice()
  }

  /// Views these bytes as a slice of `T`
  /// ## Panics
  /// * If the number of bytes isn't a multiple of T
  /// * If the alignment of T isn't 4, 2, or 1
  #[inline]
  #[must_use]
  pub const fn as_slice<T: Sized>(&self) -> &[T] {
    const {
      assert!(N % (size_of::<T>() + (size_of::<T>() % align_of::<T>())) == 0);
      assert!(
        align_of::<T>() == 4 || align_of::<T>() == 2 || align_of::<T>() == 1
      );
    }
    let data: *const u8 = self.0.as_ptr();
    let len = const { N / size_of::<T>() };
    unsafe { core::slice::from_raw_parts(data.cast::<T>(), len) }
  }
}

/// Works like [`include_bytes!`], but the value is wrapped in [`Align4`].
#[macro_export]
macro_rules! include_aligned_bytes {
  ($file:expr $(,)?) => {{
    Align4(*include_bytes!($file))
  }};
}

#[cfg(test)]
mod test_harness {
  use crate::prelude::*;
  use crate::{bios, mem, mgba};
  use core::fmt::Write;

  #[panic_handler]
  fn panic(info: &core::panic::PanicInfo) -> ! {
    DISPSTAT.write(DisplayStatus::new().with_irq_vblank(true));
    BG_PALETTE.index(0).write(Color::from_rgb(25, 10, 5));
    IE.write(IrqBits::VBLANK);
    IME.write(true);
    VBlankIntrWait();
    VBlankIntrWait();
    VBlankIntrWait();

    // the Fatal one kills emulation after one line / 256 bytes
    // so emit all the information as Error first
    if let Ok(mut log) =
      mgba::MgbaBufferedLogger::try_new(mgba::MgbaMessageLevel::Error)
    {
      writeln!(log, "[failed]").ok();
      write!(log, "{}", info).ok();
    }

    if let Ok(mut log) =
      mgba::MgbaBufferedLogger::try_new(mgba::MgbaMessageLevel::Fatal)
    {
      if let Some(loc) = info.location() {
        write!(log, "panic at {loc}! see mgba error log for details.").ok();
      } else {
        write!(log, "panic! see mgba error log for details.").ok();
      }
    }

    IE.write(IrqBits::new());
    bios::IntrWait(true, IrqBits::new());
    loop {}
  }

  pub(crate) trait UnitTest {
    fn run(&self);
  }

  impl<T: Fn()> UnitTest for T {
    fn run(&self) {
      if let Ok(mut log) =
        mgba::MgbaBufferedLogger::try_new(mgba::MgbaMessageLevel::Info)
      {
        write!(log, "{}...", core::any::type_name::<T>()).ok();
      }

      self();

      if let Ok(mut log) =
        mgba::MgbaBufferedLogger::try_new(mgba::MgbaMessageLevel::Info)
      {
        writeln!(log, "[ok]").ok();
      }
    }
  }

  pub(crate) fn test_runner(tests: &[&dyn UnitTest]) {
    if let Ok(mut log) =
      mgba::MgbaBufferedLogger::try_new(mgba::MgbaMessageLevel::Info)
    {
      write!(log, "Running {} tests", tests.len()).ok();
    }

    for test in tests {
      test.run();
    }
    if let Ok(mut log) =
      mgba::MgbaBufferedLogger::try_new(mgba::MgbaMessageLevel::Info)
    {
      write!(log, "Tests finished successfully").ok();
    }
  }

  #[no_mangle]
  extern "C" fn main() {
    DISPCNT.write(DisplayControl::new().with_video_mode(VideoMode::_0));
    BG_PALETTE.index(0).write(Color::new());

    crate::test_main();

    BG_PALETTE.index(0).write(Color::from_rgb(5, 15, 25));
    BG_PALETTE.index(1).write(Color::new());
    BG0CNT
      .write(BackgroundControl::new().with_charblock(0).with_screenblock(31));
    DISPCNT.write(
      DisplayControl::new().with_video_mode(VideoMode::_0).with_show_bg0(true),
    );

    // some niceties for people without mgba-test-runner
    let tsb = TEXT_SCREENBLOCKS.get_frame(31).unwrap();
    unsafe {
      mem::set_u32x80_unchecked(
        tsb.into_block::<1024>().as_mut_ptr().cast(),
        0,
        12,
      );
    }
    Cga8x8Thick.bitunpack_4bpp(CHARBLOCK0_4BPP.as_region(), 0);

    let row = tsb.get_row(9).unwrap().iter().skip(6);
    for (addr, ch) in row.zip(b"all tests passed!") {
      addr.write(TextEntry::new().with_tile(*ch as u16));
    }

    DISPSTAT.write(DisplayStatus::new());
    bios::IntrWait(true, IrqBits::new());
  }
}

#[cfg(test)]
mod test {
  use super::Align4;

  #[test_case]
  fn align4_as_u16_u32_slice() {
    let a = Align4([0u8, 1u8, 2u8, 3u8]);
    assert_eq!(a.as_u16_slice(), &[0x100_u16.to_le(), 0x302_u16.to_le()]);
    assert_eq!(a.as_u32_slice(), &[0x3020100_u32.to_le()]);
  }

  #[test_case]
  fn align4_as_generic() {
    // with padding
    #[repr(C, align(4))]
    #[derive(PartialEq, Debug)]
    struct FiveByte([u8; 5]);

    assert_eq!(
      Align4(*b"hello...world...").as_slice::<FiveByte>(),
      &[FiveByte(*b"hello"), FiveByte(*b"world")]
    );

    // and without
    #[repr(C, align(2))]
    #[derive(PartialEq, Debug)]
    struct ThreeHalfWords(u16, u16, u16);

    assert_eq!(
      Align4([0x11u8, 0x11u8, 0x22u8, 0x22u8, 0x33u8, 0x33u8])
        .as_slice::<ThreeHalfWords>(),
      &[ThreeHalfWords(0x1111, 0x2222, 0x3333)]
    );
  }
}
