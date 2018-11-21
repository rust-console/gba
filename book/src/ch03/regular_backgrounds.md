# Regular Backgrounds

So, backgrounds, they're cool. Why do we call the ones here "regular"
backgrounds? Because there's also "affine" backgrounds. However, affine math
stuff adds a complication, so for now we'll just work with regular backgrounds.
The non-affine backgrounds are sometimes called "text mode" backgrounds by other
guides.

To get your background image working you generally need to perform all of the
following steps, though I suppose the exact ordering is up to you.

## Tiled Video Modes

When you want regular tiled display, you must use video mode 0 or 1.

* Mode 0 allows for using all four BG layers (0 through 3) as regular
  backgrounds.
* Mode 1 allows for using BG0 and BG1 as regular backgrounds, BG2 as an affine
  background, and BG3 not at all.
* Mode 2 allows for BG2 and BG3 to be used as affine backgrounds, while BG0 and
  BG1 cannot be used at all.

We will not cover affine backgrounds in this chapter, so we will naturally be
using video mode 0.

## Get Your Palette Ready

Background palette starts at `0x5000000` and is 256 `u16` values long. It'd
potentially be possible declare a static array starting at a fixed address and
use a linker script to make sure that it ends up at the right spot in the final
program, but since we have to use volatile reads and writes with PALRAM anyway,
we'll just reuse our `VolatilePtr` type. Something like this:

```rust
pub const PALRAM_BG_BASE: VolatilePtr<u16> = VolatilePtr(0x500_0000 as *mut u16);

pub fn bg_palette(slot: usize) -> u16 {
  assert!(slot < 256);
  PALRAM_BG_BASE.offset(slot as isize).read()
}

pub fn set_bg_palette(slot: usize, color: u16) {
  assert!(slot < 256);
  PALRAM_BG_BASE.offset(slot as isize).write(color)
}
```

As we discussed with the tile color depths, the palette can be utilized as a
single block of palette values (`[u16; 256]`) or as 16 palbanks of 16 palette
values each (`[[u16;16]; 16]`). This setting is assigned per background layer
via IO register.

## Get Your Tiles Ready

Tile data is placed into charblocks. A charblock is always 16kb, so depending on
color depth it will have either 256 or 512 tiles within that charblock.
Charblocks 0, 1, 2, and 3 are all for background tiles. That's a maximum of 2048
tiles for backgrounds, but as you'll see in a moment a particular tilemap entry
can't even index that high. Instead, each background layer is assigned a
"character base block", and then tilemap entries index relative to the character
base block of that background layer.

Now, if you want to move in a lot of tile data you'll probably want to use a DMA
routine, or at least write a function like memcopy32 for fast `u32` copying from
ROM into VRAM. However, for now, and because we're being very explicit since
this is our first time doing it, we'll write it as functions for individual tile
reads and writes.

The math works like indexing a pointer, except that we have two sizes we need to
go by. First you take the base address for VRAM (`0x600_0000`), then add the
size of a charblock (16kb) times the charblock you want to place the tile
within, and then you add the index of the tile slot you're placing it into times
the size of that type of tile. Like this:

```rust
pub fn bg_tile_4pp(base_block: usize, tile_index: usize) -> Tile4bpp {
  assert!(base_block < 4);
  assert!(tile_index < 512);
  let address = VRAM + size_of::<Charblock4bpp>() * base_block + size_of::<Tile4bpp>() * tile_index;
  VolatilePtr(address as *mut Tile4bpp).read()
}

pub fn set_bg_tile_4pp(base_block: usize, tile_index: usize, tile: Tile4bpp) {
  assert!(base_block < 4);
  assert!(tile_index < 512);
  let address = VRAM + size_of::<Charblock4bpp>() * base_block + size_of::<Tile4bpp>() * tile_index;
  VolatilePtr(address as *mut Tile4bpp).write(tile)
}

pub fn bg_tile_8pp(base_block: usize, tile_index: usize) -> Tile8bpp {
  assert!(base_block < 4);
  assert!(tile_index < 256);
  let address = VRAM + size_of::<Charblock8bpp>() * base_block + size_of::<Tile8bpp>() * tile_index;
  VolatilePtr(address as *mut Tile8bpp).read()
}

pub fn set_bg_tile_8pp(base_block: usize, tile_index: usize, tile: Tile8bpp) {
  assert!(base_block < 4);
  assert!(tile_index < 256);
  let address = VRAM + size_of::<Charblock8bpp>() * base_block + size_of::<Tile8bpp>() * tile_index;
  VolatilePtr(address as *mut Tile8bpp).write(tile)
}
```

For bulk operations, you'd do the exact same math to get your base destination
pointer, and then you'd get the base source pointer for the tile you're copying
out of ROM, and then you'd do the bulk copy for the correct number of `u32`
values that you're trying to move (8 per tile moved for 4bpp, or 16 per tile
moved for 8bpp).

**GBA Limitation Note:** on a modern PC (eg: `x86` or `x86_64`) you're probably
used to index based loops and iterator based loops being the same speed. The CPU
has the ability to do a "fused multiply add", so the base address of the array
plus desired index * size per element is a single CPU operation to compute. It's
slightly more complicated if there's arrays within arrays like there are here,
but with normal arrays it's basically the same speed to index per loop cycle as
it is to take a base address and then add +1 offset per loop cycle. However, the
GBA's CPU _can't do any of that_. On the GBA, there's a genuine speed difference
between looping over indexes and then indexing each loop (slow) compared to
using an iterator that just stores an internal pointer and does +1 offset per
loop until it reaches the end (fast). The repeated indexing itself can by itself
be an expensive step. If you've got a slice of data to process, be sure to go
over it with `.iter()` and `.iter_mut()` if you can, instead of looping by
index. This is Rust and all, so probably you were gonna do that anyway, but just
a heads up.

## Get your Tilemap ready

I believe that at one point I alluded to a tilemap existing. Well, just as the
tiles are arranged into charblocks, the data describing what tile to show in
what location is arranged into a thing called a **screenblock**.

A screenblock is placed into VRAM the same as the tile data charblocks. Starting
at the base of VRAM (`0x600_0000`) there are 32 slots for the screenblock array.
Each screenblock is 2048 bytes (`0x800`). Naturally, if our tiles are using up
charblock space within VRAM and our tilemaps are using up screenblock space
within the same VRAM... well it would just be a _disaster_ if they ran in to
each other. Once again, it's up to you as the programmer to determine how much
space you want to devote to each thing. Each complete charblock uses up 8
screenblocks worth of space, but you don't have to fill a complete charblock
with tiles, so you can be very fiddly with how you split the memory.

Each screenblock is composed of a series of _screenblock entry_ values, which
describe what tile index to use and if the tile should be flipped and what
palbank it should use (if any). Because both regular backgrounds and affine
backgrounds are composed of screenblocks with entries, and because the affine
background has a smaller format for screenblock entries, we'll name
appropriately.

```rust
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct RegularScreenblock {
  data: [RegularScreenblockEntry; 32 * 32],
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(transparent)]
pub struct RegularScreenblockEntry(u16);
```

So, with one entry per tile, a single screenblock allows for 32x32 tiles worth of
background.

The format of a regular screenblock entry is quite simple compared to some of
the IO register stuff:

* 10 bits for tile index (base off of the character base block of the background)
* 1 bit for horizontal flip
* 1 bit for vertical flip
* 4 bits for picking which palbank to use (if 4bpp, otherwise it's ignored)

```rust
impl RegularScreenblockEntry {
  pub fn tile_id(self) -> u16 {
    self.0 & 0b11_1111_1111
  }
  pub fn set_tile_id(&mut self, id: u16) {
    self.0 &= !0b11_1111_1111;
    self.0 |= id;
  }
  pub fn horizontal_flip(self) -> bool {
    (self.0 & (1 << 0xA)) > 0
  }
  pub fn set_horizontal_flip(&mut self, bit: bool) {
    if bit {
      self.0 |= 1 << 0xA;
    } else {
      self.0 &= !(1 << 0xA);
    }
  }
  pub fn vertical_flip(self) -> bool {
    (self.0 & (1 << 0xB)) > 0
  }
  pub fn set_vertical_flip(&mut self, bit: bool) {
    if bit {
      self.0 |= 1 << 0xB;
    } else {
      self.0 &= !(1 << 0xB);
    }
  }
  pub fn palbank_index(self) -> u16 {
    self.0 >> 12
  }
  pub fn set_palbank_index(&mut self, palbank_index: u16) {
    self.0 &= 0b1111_1111_1111;
    self.0 |= palbank_index;
  }
}
```

Now, at either 256 or 512 tiles per charblock, you might be thinking that with a
10 bit index you can index past the end of one charblock and into the next.
You'd be right, mostly.

As long as you stay within the background memory region for charblocks (that is,
0 through 3), then it all works out. However, if you try to get the background
rendering to reach outside of the background charblocks you'll get an
implementation defined result. It's not the dreaded "undefined behavior" we're
often worried about in programming, but the results _are_ determined by what
you're running the game on. With real hardware, you get a bizarre result
(basically another way to put garbage on the screen). If you use an emulator it
might or might not allow for you to do this, it's up to the emulator writers.

## Set Your IO Registers

Instead of being just a single IO register to learn about this time, there's two
separate groups of related registers.

### Background Control

* BG0CNT (`0x400_0008`): BG0 Control
* BG1CNT (`0x400_000A`): BG1 Control
* BG2CNT (`0x400_000C`): BG2 Control
* BG3CNT (`0x400_000E`): BG3 Control

Each of these are a read/write `u16` location. This is where we get to all of
the important details that we've been putting off.

* 2 bits for the priority of each background (0 being highest). If two
  backgrounds are set to the same priority the the lower numbered background
  layer takes prescience.
* 2 bits for "character base block", the charblock that all of the tile indexes
  for this background are offset from.
* 1 bit for mosaic effect being enabled (we'll get to that below).
* 1 bit to enable 8bpp, otherwise 4bpp is used.
* 5 bits to pick the "screen base block", the screen block that serves as the
  _base_ value for this background.
* 1 bit that is _not_ used in regular mode, but in affine mode it can be enabled
  to cause the affine background to wrap around at the edges.
* 2 bits for the background size.

The size works a little funny. When size is 0 only the base screen block is
used. If size is 1 or 2 then the base screenblock and the following screenblock
are placed next to each other (horizontally for 1, vertically for 2). If the
size is 3 then the base screenblock and the following three screenblocks are
arranged into a 2x2 grid of screenblocks.

### Background Offset

* BG0HOFS (`0x400_0010`): BG0 X-Offset
* BG0VOFS (`0x400_0012`): BG0 Y-Offset
* BG1HOFS (`0x400_0014`): BG1 X-Offset
* BG1VOFS (`0x400_0016`): BG1 Y-Offset
* BG2HOFS (`0x400_0018`): BG2 X-Offset
* BG2VOFS (`0x400_001A`): BG2 Y-Offset
* BG3HOFS (`0x400_001C`): BG3 X-Offset
* BG3VOFS (`0x400_001E`): BG3 Y-Offset

Each of these are a _write only_ `u16` location. Bits 0 through 8 are used, so
the offsets can be 0 through 511. They also only apply in regular backgrounds.
If a background is in an affine state then you'll use different IO registers to
control it (discussed in a later chapter).

The offset that you assign determines the pixel offset of the display area
relative to the start of the background scene, as if the screen was a camera
looking at the scene. In other words, as a BG X offset value increases, you can
think of it as the camera moving to the right, or as that background moving to
the left. Like when mario walks toward the goal. Similarly, when a BG Y offset
increases the camera is moving down, or the background is moving up, like when
mario falls down from a high platform.

Depending on how much the background is scrolled and the size of the background,
it will loop.

## Mosaic

TODO: talk about mosaic.
