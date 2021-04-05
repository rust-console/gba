#![allow(unused)]

/// Sets up a constant new constructor for a zeroed value.
macro_rules! const_new {
  () => {
    pub const fn new() -> Self {
      Self(0)
    }
  };
}
pub(crate) use const_new;

/// Sets up a bitfield integer
macro_rules! bitfield_int {
  ($inner:ty; $low:literal ..= $high:literal : $nt:ident, $get:ident, $with:ident, $set:ident) => {
    pub const fn $get(self) -> $nt {
      const MASK: $inner = ((1 << $high) - 1) << $low;
      ((self.0 & MASK) >> $low) as $nt
    }
    pub const fn $with(self, $get: $nt) -> Self {
      const MASK: $inner = ((1 << $high) - 1) << $low;
      Self(self.0 ^ ((self.0 ^ ($get as $inner)) & MASK))
    }
    pub fn $set(&mut self, $get: $nt) {
      *self = self.$with($get);
    }
  };
}
pub(crate) use bitfield_int;

/// Sets up a bitfield int wrapped newtype
macro_rules! bitfield_newtype {
  ($inner:ty; $low:literal ..= $high:literal : $nt:ident, $get:ident, $with:ident, $set:ident) => {
    pub const fn $get(self) -> $nt {
      const MASK: $inner = ((1 << $high) - 1) << $low;
      $nt(self.0 & MASK)
    }
    pub const fn $with(self, $get: $nt) -> Self {
      const MASK: $inner = ((1 << $high) - 1) << $low;
      Self(self.0 ^ ((self.0 ^ $get.0) & MASK))
    }
    pub fn $set(&mut self, $get: $nt) {
      *self = self.$with($get);
    }
  };
}
pub(crate) use bitfield_newtype;

/// Sets up a bitfield enum (CAUTION: misuse of this can cause UB!)
macro_rules! bitfield_enum {
  ($inner:ty; $low:literal ..= $high:literal : $nt:ident, $get:ident, $with:ident, $set:ident) => {
    // TODO: make this const when we have const transmute
    pub fn $get(self) -> $nt {
      const MASK: $inner = ((1 << $high) - 1) << $low;
      unsafe { core::mem::transmute(self.0 & MASK) }
    }
    pub const fn $with(self, $get: $nt) -> Self {
      const MASK: $inner = ((1 << $high) - 1) << $low;
      Self(self.0 ^ ((self.0 ^ $get as $inner) & MASK))
    }
    pub fn $set(&mut self, $get: $nt) {
      *self = self.$with($get);
    }
  };
}
pub(crate) use bitfield_enum;

/// Sets up a bitfield bool
macro_rules! bitfield_bool {
  ($inner:ty; $bit:literal, $get:ident, $with:ident, $set:ident) => {
    pub const fn $get(self) -> bool {
      (self.0 & (1 << $bit)) != 0
    }
    pub const fn $with(self, $get: bool) -> Self {
      Self(self.0 ^ ((($get as $inner).wrapping_neg() ^ self.0) & (1 << $bit)))
    }
    pub fn $set(&mut self, $get: bool) {
      *self = self.$with($get);
    }
  };
}
pub(crate) use bitfield_bool;

mod display_control;
pub use display_control::*;

mod display_status;
pub use display_status::*;

mod background_control;
pub use background_control::*;

mod window_enable;
pub use window_enable::*;

mod mosaic_size;
pub use mosaic_size::*;

mod blend_control;
pub use blend_control::*;

mod color;
pub use color::*;

mod keys;
pub use keys::*;

mod dma_control;
pub use dma_control::*;

mod key_interrupt_control;
pub use key_interrupt_control::*;
