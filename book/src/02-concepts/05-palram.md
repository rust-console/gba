# Palette RAM (PALRAM)

* **Address Span:** `0x500_0000` to `0x500_03FF` (1k)

Palette RAM has a 16-bit bus, which isn't really a problem because it
conceptually just holds `u16` values. There's no automatic wait state, but if
you try to access the same location that the display controller is accessing you
get bumped by 1 cycle. Since the display controller can use the palette ram any
number of times per scanline it's basically impossible to predict if you'll have
to do a wait or not during VDraw. During VBlank you won't have any wait of
course.

PALRAM is among the memory where there's weirdness if you try to write just one
byte: if you try to write just 1 byte, it writes that byte into _both_ parts of
the larger 16-bit location. This doesn't really affect us much with PALRAM,
because palette values are all supposed to be `u16` anyway.

The palette memory actually contains not one, but _two_ sets of palettes. First
there's 256 entries for the background palette data (starting at `0x5000000`),
and then there's 256 entries for object palette data (starting at `0x5000200`).

The GBA also has two modes for palette access: 8-bits-per-pixel (8bpp) and
4-bits-per-pixel (4bpp).

* In 8bpp mode an 8-bit palette index value within a background or sprite
  simply indexes directly into the 256 slots for that type of thing.
* In 4bpp mode a 4-bit palette index value within a background or sprite
  specifies an index within a particular "palbank" (16 palette entries each),
  and then a _separate_ setting outside of the graphical data determines which
  palbank is to be used for that background or object (the screen entry data for
  backgrounds, and the object attributes for objects).

### Transparency

When a pixel within a background or object specifies index 0 as its palette
entry it is treated as a transparent pixel. This means that in 8bpp mode there's
only 255 actual color options (0 being transparent), and in 4bpp mode there's
only 15 actual color options available within each palbank (the 0th entry of
_each_ palbank is transparent).

Individual backgrounds, and individual objects, each determine if they're 4bpp
or 8bpp separately, so a given overall palette slot might map to a used color in
8bpp and an unused/transparent color in 4bpp. If you're a palette wizard.

Palette slot 0 of the overall background palette is used to determine the
"backdrop" color. That's the color you see if no background or object ends up
being rendered within a given pixel.

Since display mode 3 and display mode 5 don't use the palette, they cannot
benefit from transparency.
