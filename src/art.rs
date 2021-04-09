//! This module has constants for various tile sheets in compressed form.
//!
//! Depending on the tile sheet, the optimal compression varies. The docs of
//! each constant explain how to decompress the data correctly.

mod cga_8x8_thick;
pub use cga_8x8_thick::*;
