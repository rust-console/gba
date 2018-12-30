# Video Memory Intro

The GBA's Video RAM is 96k stretching from `0x0600_0000` to `0x0601_7FFF`.

The Video RAM can only be accessed totally freely during a Vertical Blank (aka
"VBlank", though sometimes I forget and don't capitalize it properly). At other
times, if the CPU tries to touch the same part of video memory as the display
controller is accessing then the CPU gets bumped by a cycle to avoid a clash.

Annoyingly, VRAM can only be properly written to in 16 and 32 bit segments (same
with PALRAM and OAM). If you try to write just an 8 bit segment, then both parts
of the 16 bit segment get the same value written to them. In other words, if you
write the byte `5` to `0x0600_0000`, then both `0x0600_0000` and ALSO
`0x0600_0001` will have the byte `5` in them. We have to be extra careful when
trying to set an individual byte, and we also have to be careful if we use
`memcopy` or `memset` as well, because they're byte oriented by default and
don't know to follow the special rules.

## RGB15

As I said before, RGB15 stores a color within a `u16` value using 5 bits for
each color channel.

```rust
pub const RED:   u16 = 0b0_00000_00000_11111;
pub const GREEN: u16 = 0b0_00000_11111_00000;
pub const BLUE:  u16 = 0b0_11111_00000_00000;
```

In Mode 3 and Mode 5 we write direct color values into VRAM, and in Mode 4 we
write palette index values, and then the color values go into the PALRAM.

## Mode 3

Mode 3 is pretty easy. We have a full resolution grid of rgb15 pixels. There's
160 rows of 240 pixels each, with the base address being the top left corner. A
particular pixel uses normal "2d indexing" math:

```rust
let row_five_col_seven = 5 + (7 * SCREEN_WIDTH);
```

To draw a pixel, we just write a value at the address for the row and col that
we want to draw to.

## Mode 4

Mode 4 introduces page flipping. Instead of one giant page at `0x0600_0000`,
there's Page 0 at `0x0600_0000` and then Page 1 at `0x0600_A000`. The resolution
for each page is the same as above, but instead of writing `u16` values, the
memory is treated as `u8` indexes into PALRAM. The PALRAM starts at
`0x0500_0000`, and there's enough space for 256 palette entries (each a `u16`).

To set the color of a palette entry we just do a normal `u16` write_volatile.

```rust
(0x0500_0000 as *mut u16).offset(target_index).write_volatile(new_color)
```

To draw a pixel we set the palette entry that we want the pixel to use. However,
we must remember the "minimum size" write limitation that applies to VRAM. So,
if we want to change just a single pixel at a time we must

1) Read the full `u16` it's a part of.
2) Clear the half of the `u16` we're going to replace
3) Write the half of the `u16` we're going to replace with the new value
4) Write that result back to the address.

So, the math for finding a byte offset is the same as Mode 3 (since they're both
a 2d grid). If the byte offset is EVEN it'll be the high bits of the `u16` at
half the byte offset rounded down. If the offset is ODD it'll be the low bits of
the `u16` at half the byte.

Does that make sense?

* If we want to write pixel (0,0) the byte offset is 0, so we change the high
  bits of `u16` offset 0. Then we want to write to (1,0), so the byte offset is
  1, so we change the low bits of `u16` offset 0. The pixels are next to each
  other, and the target bytes are next to each other, good so far.
* If we want to write to (5,6) that'd be byte `5 + 6 * 240 = 1445`, so we'd
  target the low bits of `u16` offset `floor(1445/2) = 722`.

As you can see, trying to write individual pixels in Mode 4 is mostly a bad
time. Fret not! We don't _have_ to write individual bytes. If our data is
arranged correctly ahead of time we can just write `u16` or `u32` values
directly. The video hardware doesn't care, it'll get along just fine.

## Mode 5

Mode 5 is also a two page mode, but instead of compressing the size of a pixel's
data to fit in two pages, we compress the resolution.

Mode 5 is full `u16` color, but only 160w x 128h per page.

## In Conclusion...

So what got written into VRAM in `hello1`?

```rust
    (0x06000000 as *mut u16).offset(120 + 80 * 240).write_volatile(0x001F);
    (0x06000000 as *mut u16).offset(136 + 80 * 240).write_volatile(0x03E0);
    (0x06000000 as *mut u16).offset(120 + 96 * 240).write_volatile(0x7C00);
```

So at pixels `(120,80)`, `(136,80)`, and `(120,96)` we write three values. Once
again we probably need to [convert them](https://www.wolframalpha.com/) into
binary to make sense of it.

* 0x001F: 0b0_00000_00000_11111
* 0x03E0: 0b0_00000_11111_00000
* 0x7C00: 0b0_11111_00000_00000

Ah, of course, a red pixel, a green pixel, and a blue pixel.
