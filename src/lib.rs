#![no_std]
#![cfg_attr(not(feature = "on_gba"), allow(unused))]
#![warn(missing_docs)]
#![warn(unsafe_op_in_unsafe_fn)]
#![cfg_attr(feature = "doc_cfg", feature(doc_cfg))]

//! A crate for 'raw' style Game Boy Advance (GBA) development, where any code
//! can access any hardware component at any time, with no special ceremony.
//!
//! * **Note:** If you want a 'managed' hardware style, more like many other
//!   "embedded-wg" experiences, where hardware access is declared though the
//!   type system by passing around zero-sized token types, try the
//!   [agb](https://docs.rs/agb) crate instead.
//!
//! # This Is Intended For The Game Boy Advance
//!
//! When the `on_gba` crate feature is used, the crate assumes that you're
//! building the crate for, and also running the code on, the Game Boy Advance.
//! The build target is expected to be `thumbv4t-none-eabi` or
//! `armv4t-none-eabi`, and any other targets might have a build error. Further,
//! the specific device you run the code on is assumed to be the GBA (or a GBA
//! emulator). These facts are used by the `unsafe` code in this crate.
//!
//! This crate feature is **on by default** because the primary purpose of this
//! crate is to assist in the building of GBA games, but you *can* disable the
//! feature and build the crate anyway. How much of this crate actually works on
//! non-GBA platforms is **not** covered by our SemVer! Building and using the
//! crate without the `on_gba` feature is intended for non-GBA code that wants
//! the data type definitions the crate provides, such as a build script running
//! on your development machine. Without the `on_gba` feature enabled, any GBA
//! specific functions that "don't make sense" outside of a GBA context (such as
//! functions using inline assembly) will just be `unimplemented!()`, and
//! calling them will trigger a panic.
//!
//! If you're not familiar with GBA programming some explanations are provided
//! on separate pages:
//! * [Per System Setup][`per_system_setup`]
//! * [Per Project Setup][`per_project_setup`]

use bitfrob::{u16_get_bit, u16_with_bit};

macro_rules! on_gba_or_unimplemented {
  ($($token_tree:tt)*) => {
    #[cfg(feature="on_gba")]
    {
      $($token_tree)*
    }
    #[cfg(not(feature="on_gba"))]
    unimplemented!()
  }
}

pub mod asm_runtime;
pub mod bios;
pub mod dma;
pub mod gba_cell;
pub mod gba_fixed;
pub mod mem;
pub mod mgba;
pub mod mmio;
pub mod obj;
pub mod panic_handlers;
pub mod per_project_setup;
pub mod per_system_setup;
pub mod random;
pub mod sample_art;
pub mod video;

#[cfg(feature = "critical-section")]
#[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "critical-section")))]
pub mod critical_section;

/// `i16` with 8 bits of fixed-point fraction.
///
/// This is used by the affine matrix entries.
///
/// * This build of the crate uses the [`fixed`] crate
#[cfg(feature = "fixed")]
#[allow(non_camel_case_types)]
pub type i16fx8 = fixed::FixedI16<fixed::types::extra::U8>;

/// `i16` with 14 bits of fixed-point fraction.
///
/// This is used by the [`ArcTan`](crate::bios::ArcTan) and
/// [`ArcTan2`](crate::bios::ArcTan2) BIOS functions.
///
/// * This build of the crate uses the [`fixed`] crate
#[cfg(feature = "fixed")]
#[allow(non_camel_case_types)]
pub type i16fx14 = fixed::FixedI16<fixed::types::extra::U14>;

/// `i32` with 8 bits of fixed-point fraction.
///
/// This is used by the background reference point entries.
///
/// * This build of the crate uses the [`fixed`] crate
#[cfg(feature = "fixed")]
#[allow(non_camel_case_types)]
pub type i32fx8 = fixed::FixedI32<fixed::types::extra::U8>;

/// `i16` with 8 bits of fixed-point fraction.
///
/// This is used by the affine matrix entries.
///
/// * This build of the crate uses the [`gba_fixed`] module
#[cfg(not(feature = "fixed"))]
#[allow(non_camel_case_types)]
pub type i16fx8 = crate::gba_fixed::Fixed<i16, 8>;

/// `i16` with 14 bits of fixed-point fraction.
///
/// This is used by the [`ArcTan`](crate::bios::ArcTan) and
/// [`ArcTan2`](crate::bios::ArcTan2) BIOS functions.
///
/// * This build of the crate uses the [`gba_fixed`] module
#[cfg(not(feature = "fixed"))]
#[allow(non_camel_case_types)]
pub type i16fx14 = crate::gba_fixed::Fixed<i16, 14>;

/// `i32` with 8 bits of fixed-point fraction.
///
/// This is used by the background reference point entries.
///
/// * This build of the crate uses the [`gba_fixed`] module
#[cfg(not(feature = "fixed"))]
#[allow(non_camel_case_types)]
pub type i32fx8 = crate::gba_fixed::Fixed<i32, 8>;

/// Keypad input state.
#[derive(Clone, Copy, Default)]
#[repr(transparent)]
pub struct KeyInput(pub u16);
impl KeyInput {
  /// If `a` is pressed (left primary button)
  #[inline]
  #[must_use]
  pub const fn a(self) -> bool {
    !bitfrob::u16_get_bit(0, self.0)
  }
  /// If `b` is pressed (right primary button)
  #[inline]
  #[must_use]
  pub const fn b(self) -> bool {
    !bitfrob::u16_get_bit(1, self.0)
  }
  /// If `select` is pressed (lower/left secondary button)
  #[inline]
  #[must_use]
  pub const fn select(self) -> bool {
    !bitfrob::u16_get_bit(2, self.0)
  }
  /// If `start` is pressed (upper/right secondary button)
  #[inline]
  #[must_use]
  pub const fn start(self) -> bool {
    !bitfrob::u16_get_bit(3, self.0)
  }
  /// If `right` is pressed (d-pad)
  #[inline]
  #[must_use]
  pub const fn right(self) -> bool {
    !bitfrob::u16_get_bit(4, self.0)
  }
  /// If `left` is pressed (d-pad)
  #[inline]
  #[must_use]
  pub const fn left(self) -> bool {
    !bitfrob::u16_get_bit(5, self.0)
  }
  /// If `up` is pressed (d-pad)
  #[inline]
  #[must_use]
  pub const fn up(self) -> bool {
    !bitfrob::u16_get_bit(6, self.0)
  }
  /// If `down` is pressed (d-pad)
  #[inline]
  #[must_use]
  pub const fn down(self) -> bool {
    !bitfrob::u16_get_bit(7, self.0)
  }
  /// If `r` is pressed (right shoulder button)
  #[inline]
  #[must_use]
  pub const fn r(self) -> bool {
    !bitfrob::u16_get_bit(8, self.0)
  }
  /// If `l` is pressed (left shoulder button)
  #[inline]
  #[must_use]
  pub const fn l(self) -> bool {
    !bitfrob::u16_get_bit(9, self.0)
  }
  /// Delta X of the d-pad. right +1, left -1.
  #[inline]
  #[must_use]
  pub const fn dx(self) -> i8 {
    if self.right() {
      1
    } else if self.left() {
      -1
    } else {
      0
    }
  }
  /// Delta Y of the d-pad. up +1, down -1.
  #[inline]
  #[must_use]
  pub const fn dy(self) -> i8 {
    if self.up() {
      1
    } else if self.down() {
      -1
    } else {
      0
    }
  }
}

/// Interrupt bit flags.
#[derive(Clone, Copy, Default)]
#[repr(transparent)]
pub struct IrqBits(u16);
impl IrqBits {
  /// The vblank bit.
  pub const VBLANK: Self = Self::new().with_vblank(true);

  /// Makes a new, empty value.
  #[inline]
  pub const fn new() -> Self {
    Self(0)
  }
  /// Vertical-blank
  #[inline]
  #[must_use]
  pub const fn vblank(self) -> bool {
    u16_get_bit(0, self.0)
  }
  /// Horizontal-blank
  #[inline]
  #[must_use]
  pub const fn hblank(self) -> bool {
    u16_get_bit(1, self.0)
  }
  /// Vertical-counter match
  #[inline]
  #[must_use]
  pub const fn vcount(self) -> bool {
    u16_get_bit(2, self.0)
  }
  /// Timer 0 overflow
  #[inline]
  #[must_use]
  pub const fn timer0(self) -> bool {
    u16_get_bit(3, self.0)
  }
  /// Timer 1 overflow
  #[inline]
  #[must_use]
  pub const fn timer1(self) -> bool {
    u16_get_bit(4, self.0)
  }
  /// Timer 2 overflow
  #[inline]
  #[must_use]
  pub const fn timer2(self) -> bool {
    u16_get_bit(5, self.0)
  }
  /// Timer 3 overflow
  #[inline]
  #[must_use]
  pub const fn timer3(self) -> bool {
    u16_get_bit(6, self.0)
  }
  /// Serial port communication
  #[inline]
  #[must_use]
  pub const fn serial(self) -> bool {
    u16_get_bit(7, self.0)
  }
  /// DMA 0 complete
  #[inline]
  #[must_use]
  pub const fn dma0(self) -> bool {
    u16_get_bit(8, self.0)
  }
  /// DMA 1 complete
  #[inline]
  #[must_use]
  pub const fn dma1(self) -> bool {
    u16_get_bit(9, self.0)
  }
  /// DMA 2 complete
  #[inline]
  #[must_use]
  pub const fn dma2(self) -> bool {
    u16_get_bit(10, self.0)
  }
  /// DMA 3 complete
  #[inline]
  #[must_use]
  pub const fn dma3(self) -> bool {
    u16_get_bit(11, self.0)
  }
  /// Keypad match
  #[inline]
  #[must_use]
  pub const fn keypad(self) -> bool {
    u16_get_bit(12, self.0)
  }
  /// Game pak
  #[inline]
  #[must_use]
  pub const fn gamepak(self) -> bool {
    u16_get_bit(13, self.0)
  }

  /// Set if vblank triggers an interrupt.
  #[inline]
  #[must_use]
  pub const fn with_vblank(self, vblank: bool) -> Self {
    Self(u16_with_bit(0, self.0, vblank))
  }
}
