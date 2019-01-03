# The Display Control Register

The display control register is our first actual IO Register. GBATEK gives it the
shorthand [DISPCNT](http://problemkaputt.de/gbatek.htm#lcdiodisplaycontrol), so
you might see it under that name if you read other guides.

Among IO Registers, it's one of the simpler ones, but it's got enough complexity
that we can get a hint of what's to come.

Also it's the one that you basically always need to set at least once in every
GBA game, so it's a good starting one to go over for that reason too.

The display control register holds a `u16` value, and is located at `0x0400_0000`.

Many of the bits here won't mean much to you right now. **That is fine.** You do
NOT need to memorize them all or what they all do right away. We'll just skim
over all the parts of this register to start, and then we'll go into more detail
in later chapters when we need to come back and use more of the bits.

## Video Modes

The lowest three bits (0-2) let you select from among the GBA's six video modes.
You'll notice that 3 bits allows for eight modes, but the values 6 and 7 are
prohibited.

Modes 0, 1, and 2 are "tiled" modes. These are actually the modes that you
should eventually learn to use as much as possible. It lets the GBA's limited
video hardware do as much of the work as possible, leaving more of your CPU time
for gameplay computations. However, they're also complex enough to deserve their
own demos and chapters later on, so that's all we'll say about them for now.

Modes 3, 4, and 5 are "bitmap" modes. These let you write individual pixels to
locations on the screen.

* **Mode 3** is full resolution (240w x 160h) RGB15 color. You might not be used
  to RGB15, since modern computers have 24 or 32 bit colors. In RGB15, there's 5
  bits for each color channel stored within a `u16` value, and the highest bit is
  simply ignored.
* **Mode 4** is full resolution paletted color. Instead of being a `u16` color, each
  pixel value is a `u8` palette index entry, and then the display uses the
  palette memory (which we'll talk about later) to store the actual color data.
  Since each pixel is half sized, we can fit twice as many. This lets us have
  two "pages". At any given moment only one page is active, and you can draw to
  the other page without the user noticing. You set which page to show with
  another bit we'll get to in a moment.
* **Mode 5** is full color, but also with pages. This means that we must have a
  reduced resolution to compensate (video memory is only so big!). The screen is
  effectively only 160w x 128h in this mode.

## CGB Mode

Bit 3 is effectively read only. Technically it can be flipped using a BIOS call,
but when you write to the display control register normally it won't write to
this bit, so we'll call it effectively read only.

This bit is on if the CPU is in CGB mode.

## Page Flipping

Bit 4 lets you pick which page to use. This is only relevent in video modes 4 or
5, and is just ignored otherwise. It's very easy to remember: when the bit is 0
the 0th page is used, and when the bit is 1 the 1st page is used.

The second page always starts at `0x0600_A000`.

## OAM, VRAM, and Blanking

Bit 5 lets you access OAM during HBlank if enabled. This is cool, but it reduces
the maximum sprites per scanline, so it's not default.

Bit 6 lets you adjust if the GBA should treat Object Character VRAM as being 2d
(off) or 1d (on). This particular control can be kinda tricky to wrap your head
around, so we'll be sure to have some extra diagrams in the chapter that deals
with it.

Bit 7 forces the screen to stay in VBlank as long as it's set. This allows the
fastest use of the VRAM, Palette, and Object Attribute Memory. Obviously if you
leave this on for too long the player will notice a blank screen, but it might
be okay to use for a moment or two every once in a while.

## Screen Layers

Bits 8 through 11 control if Background layers 0 through 3 should be active.

Bit 12 affects the Object layer.

Note that not all background layers are available in all video modes:

* Mode 0: all
* Mode 1: 0/1/2
* Mode 2: 2/3
* Mode 3/4/5: 2

Bit 13 and 14 enable the display of Windows 0 and 1, and Bit 15 enables the
object display window. We'll get into how windows work later on, they let you do
some nifty graphical effects.

## In Conclusion...

So what did we do to the display control register in `hello1`?

```rust
    (0x04000000 as *mut u16).write_volatile(0x0403);
```

First let's [convert that to
binary](https://www.wolframalpha.com/input/?i=0x0403), and we get
`0b100_0000_0011`. So, that's setting Mode 3 with background 2 enabled and
nothing else special.
