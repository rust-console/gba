//! Tiled background documentation (no code).
//!
//! When in tiled mode a background shows 2D tiles. This is sometimes called
//! "text" mode because it's the same style of graphics that was once used for
//! text terminal displays. The first 64k of VRAM is treated as a collection of
//! **tiles** and **screenblocks**.
//!
//! * A tile is 8x8 pixels. Tiles can be either 4bpp (32 bytes) or 8bpp (64
//!   bytes). Each background layer can use a different bit depth.
//! * A screenblock is 32x32 [`TileEntry`] values (2,048 bytes).
//! * Each background has a `size` value set by its [BackgroundControl]. This
//!   determines how many tilemaps in a row are actually used to draw the
//!   background:
//!   * 0: one tilemap
//!   * 1: two tilemaps horizontally: left then right.
//!   * 2: two tilemaps vertically: top then bottom.
//!   * 3: four tilemaps in a square: upper left, upper right, lower left, lower
//!     right.

// this makes the doclinks work right.
#[allow(unused_imports)]
use crate::prelude::*;
