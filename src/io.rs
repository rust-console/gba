//! This module contains definitions and types for the IO Registers.
//!
//! ## Naming
//!
//! In the interest of making things easy to search for, all io register
//! constants are given the names used in the
//! [GBATEK](https://problemkaputt.de/gbatek.htm) technical description.

use super::*;

pub mod display;
pub mod dma;
pub mod keypad;
pub mod background;
