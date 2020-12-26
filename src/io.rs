//! This module contains definitions and types for the IO Registers.
//!
//! ## Naming
//!
//! In the interest of making things easy to search for, all io register
//! constants are given the names used in the
//! [GBATEK](https://problemkaputt.de/gbatek.htm) technical description.

use super::*;

pub mod background;
pub mod color_blend;
pub mod display;
pub mod dma;
pub mod irq;
pub mod keypad;
pub mod sio;
pub mod sound;
pub mod timers;
pub mod window;
