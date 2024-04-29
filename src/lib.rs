#![no_std]
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
//! # Crate Features
//!
//! * `on_gba` (**Default:** enabled): When this feature is used, the crate
//!   assumes that you're building the crate for, and running the code on, the
//!   Game Boy Advance. The build target is expected to be `thumbv4t-none-eabi`
//!   or `armv4t-none-eabi`, any other targets may have a build error. Further,
//!   the specific device is assumed to be the GBA, which is used to determine
//!   the safety of all direct hardware access using MMIO. This feature is on by
//!   default because the primary purpose of this crate is to assist in the
//!   building of GBA games, but you *can* disable the feature and build the
//!   crate anyway, such as if you want to use any of the crate's data type
//!   definitions within a build script on your host machine. When this feature
//!   is disabled, GBA specific internals of functions *may* be replaced with
//!   runtime panics when necessary. How much of this crate actually works on
//!   non-GBA platforms is **not** covered by our SemVer!
//! * `critical-section` (**Default:** enabled): activates an implementation to
//!   support for the [critical-section](https://docs.rs/critical-section)
//!   crate.
//! * `track_caller` (**Default:** disabled): Causes some functions that can
//!   panic to add the [track_caller][ref-track-caller] attribute. The attribute
//!   adds a "secret" function argument to pass the `Location` of the call, so
//!   it can reduce performance when a function is not inlined (more data has to
//!   be pushed onto the stack per function call). Suggested for debugging only.
//!
//! [ref-track-caller]:
//!     https://doc.rust-lang.org/reference/attributes/codegen.html#the-track_caller-attribute
//!
//! # Additional Information
//!
//! * Development Environment Setup
//! * Project Setup
//! * Learning GBA Programming

pub mod mmio;

#[cfg(feature = "critical-section")]
#[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "critical-section")))]
pub mod critical_section;
