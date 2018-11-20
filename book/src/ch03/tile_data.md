# Tile Data

When using the GBA's hardware graphics, if you want to let the hardware do most
of the work you have to use Modes 0, 1 or 2. However, to do that we first have
to learn about how tile data works inside of the GBA.

## Tiles

Fundamentally, a tile is an 8x8 image. If you want anything bigger than 8x8 you
need to arrange several tiles so that it looks like whatever you're trying to
draw.

As was already mentioned, the GBA supports two different color modes: 4 bits per
pixel and 8 bits per pixel. So already we have two types of tile that we need to
model. The pixel bits always represent an index into the PALRAM.

* With 4 bits per pixel, the PALRAM is imagined to be 16 **palbank** sections of
  16 palette entries each. The image data selects the index within the palbank,
  and an external configuration selects which palbank is used.
* With 8 bits per pixel, the PALRAM is imagined to be a single 256 entry array
  and the index just directly picks which of the 256 colors is used.

So, already we have some Rust types we can define:

```rust
#[derive(Debug, Clone, Copy, Default)]
#[repr(transparent)]
pub struct Tile4bpp {
  data: [u32; 8]
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(transparent)]
pub struct Tile8bpp {
  data: [u32; 16]
}
```

I hope this makes sense so far. At 4bpp, we have 4 bits per pixel, times 8
pixels per line, times 8 lines: 256 bits required. Similarly, at 8 bits per
pixel we'll need 512 bits. Why are we defining them as arrays of `u32` values?
Because when it comes time to do bulk copies the fastest way to it will be to go
one whole machine word at a time. If we make the data inside the type be an
array of `u32` then it'll already be aligned for fast `u32` bulk copies.

Keeping track of the current color depth is naturally the _programmer's_
problem. If you get it wrong you'll see a whole ton of garbage pixels all over
the screen, and you'll probably be able to guess why. You know, unless you did
one of the other things that can make a bunch of garbage pixels show up all over
the screen. Graphics programming is fun like that.

## Charblocks

Tiles don't just sit on their own, they get grouped into **charblocks**. They're
called that because tiles represent characters... even though they're also used
to draw the background? I don't get it exactly, but that's just what they're
called in other documents and I don't have a significantly better name for the
concept, so that's what we'll call it.

A charblock is 16kb long (`0x4000` bytes), which means that the number of tiles
that fit into a charblock depends on your color depth. With 4bpp you get 512
tiles, and with 8bpp there's 256 tiles. So they'd be something like this:

```rust
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Charblock4bpp {
  data: [Tile4bpp; 512],
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Charblock8bpp {
  data: [Tile8bpp; 256],
}
```

You'll note that we can't even derive `Debug` or `Default` any more because the
arrays are so big. Rust supports Clone and Copy for arrays of any size, but the
rest is still size 32 or less. We won't generally be making up an entire
Charblock on the fly though, so it's not a big deal. If we _absolutely_ had to,
we could call `core::mem::zeroed()`, but we really don't want to be trying to
build a whole charblock at runtime. We'll usually want to define our tile data
as `const` charblock values (or even parts of charblock values) that we load out
of the game pak ROM.

Anyway, with 16k per charblock and only 96k total in VRAM, it's easy math to see
that there's 6 different charblocks in VRAM when in a tiled mode. The first four
of these are for backgrounds, and the other two are for objects. There's rules
for how a tile ID on a background or object selects a tile within a charblock,
but since they're different between backgrounds and objects we'll cover that on
their own pages.

## Image Editing

It's very important to note that if you use a normal image editor you'll get
very bad results if you translate that directly into GBA memory.

Imagine you have part of an image that's 16 by 16 pixels, aka 2 tiles by 2
tiles. The data for that bitmap is the 1st row of the 1st tile, then the 1st row
of the 2nd tile. However, when we translate that into the GBA, the first 8
pixels will indeed be the first 8 tile pixels, but then the next 8 pixels in
memory will be used as the _2nd row of the first tile_, not the 1st row of the
2nd tile.

So, how do we fix this?

Well, the simple but annoying way is to edit your tile image as being an 8 pixel
wide image and then have the image get super tall as you add more and more
tiles. It can work, but it's really impractical if you have any multi-tile
things that you're trying to do.

Instead, there are some image conversion tools that devkitpro provides in their
gba-dev section. They let you take normal images and then repackage them and
export it in various formats that you can then compile into your project.

TODO: make Ketsuban write some tips on how to use whatever the image converter
is called.
