#![no_std]
#![feature(asm)]
#![warn(missing_docs)]
#![allow(clippy::cast_lossless)]
#![deny(clippy::float_arithmetic)]

//! This crate helps you write GBA ROMs.
//!
//! ## SAFETY POLICY
//!
//! Some parts of this crate are safe wrappers around unsafe operations. This is
//! good, and what you'd expect from a Rust crate.
//!
//! However, the safe wrappers all assume that you will _only_ attempt to
//! execute this crate on a GBA or in a GBA Emulator.
//!
//! **Do not** use this crate in programs that aren't running on the GBA. If you
//! do, it's a giant bag of Undefined Behavior.

/// Assists in defining a newtype wrapper over some base type.
///
/// Note that rustdoc and derives are all the "meta" stuff, so you can write all
/// of your docs and derives in front of your newtype in the same way you would
/// for a normal struct. Then the inner type to be wrapped it name.
///
/// The macro _assumes_ that you'll be using it to wrap zero safe numeric types,
/// so it automatically provides a `const fn` method for `new` that just wraps
/// `0`. If this is not desired you can add `, no frills` to the invocation.
///
/// Example:
/// ```
/// newtype! {
///   /// Records a particular key press combination.
///   #[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
///   KeyInput, u16
/// }
/// ```
#[macro_export]
macro_rules! newtype {
  ($(#[$attr:meta])* $new_name:ident, $old_name:ident) => {
    $(#[$attr])*
    #[repr(transparent)]
    pub struct $new_name($old_name);
    impl $new_name {
      /// A `const` "zero value" constructor
      pub const fn new() -> Self {
        $new_name(0)
      }
    }
  };
  ($(#[$attr:meta])* $new_name:ident, $old_name:ident, no frills) => {
    $(#[$attr])*
    #[repr(transparent)]
    pub struct $new_name($old_name);
  };
}

pub mod builtins;

pub mod fixed;

pub mod bios;

pub mod core_extras;
pub(crate) use crate::core_extras::*;

pub mod io_registers;

pub mod video_ram;
pub(crate) use crate::video_ram::*;
