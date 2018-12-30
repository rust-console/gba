# Video RAM (VRAM)

* **Address Span:** `0x600_0000` to `0x601_7FFF` (96k)

We've used this before! VRAM has a 16-bit bus and no wait. However, the same as
with PALRAM, the "you might have to wait if the display controller is looking at
it" rule applies here.

Unfortunately there's not much more exact detail that can be given about VRAM.
The use of the memory depends on the video mode that you're using.

One general detail of note is that you can't write individual bytes to any part
of VRAM. Depending on mode and location, you'll either get your bytes doubled
into both the upper and lower parts of the 16-bit location targeted, or you
won't even affect the memory. This usually isn't a big deal, except in two
situations:

* In Mode 4, if you want to change just 1 pixel, you'll have to be very careful
  to read the old `u16`, overwrite just the byte you wanted to change, and then
  write that back.
* In any display mode, avoid using `memcopy` to place things into VRAM.
  It's written to be byte oriented, and only does 32-bit transfers under select
  conditions. The rest of the time it'll copy one byte at a time and you'll get
  either garbage or nothing at all.
