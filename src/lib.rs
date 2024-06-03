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
use voladdress::VolAddress;

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
pub mod irq;
pub mod keys;
pub mod mem;
pub mod mgba;
pub mod panic_handlers;
pub mod per_project_setup;
pub mod per_system_setup;
pub mod random;
pub mod sample_art;
pub mod timers;
pub mod video;

/// "safe on GBA", which is either Safe or Unsafe according to the `on_gba`
/// cargo feature.
#[cfg(feature = "on_gba")]
type SOGBA = voladdress::Safe;
#[cfg(not(feature = "on_gba"))]
type SOGBA = voladdress::Unsafe;

/// Responds "normally" to read/write, just holds a setting
type PlainAddr<T> = VolAddress<T, SOGBA, SOGBA>;
/// Read-only addr
type RoAddr<T> = VolAddress<T, SOGBA, ()>;
/// Write-only addr
type WoAddr<T> = VolAddress<T, (), SOGBA>;

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
