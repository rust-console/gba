# GBA Memory Mapping

The [GBA Memory Map](http://problemkaputt.de/gbatek.htm#gbamemorymap) has
several memory portions to it, each with their own little differences. Most of
the memory has pre-determined use according to the hardware, but there is also
space for games to use as a scratch pad in whatever way the game sees fit.

The memory ranges listed here are _inclusive_, so they end with a lot of F's
and E's.

We've talked about volatile memory before, but just as a reminder I'll say that
all of the memory we'll talk about here should be accessed using volatile with
two exceptions:

1) Work RAM (both internal and external) can be used normally, and if the
   compiler is able to totally elide some reads and writes that's okay.
2) However, if you set aside any space in Work RAM where an interrupt will
   communicate with the main program then that specific location will have to
   keep using volatile access, since the compiler never knows when an interrupt
   will actually happen.

## BIOS / System ROM

* `0x0` to `0x3FFF` (16k)

This is special memory for the BIOS. It is "read-only", but even then it's only
accessible when the program counter is pointing into the BIOS region. At all
other times you get a [garbage
value](http://problemkaputt.de/gbatek.htm#gbaunpredictablethings) back when you
try to read out of the BIOS.

## External Work RAM / EWRAM

* `0x2000000` to `0x203FFFF` (256k)

This is a big pile of space, the use of which is up to each game. However, the
external work ram has only a 16-bit bus (if you read/write a 32-bit value it
silently breaks it up into two 16-bit operations) and also 2 wait cycles (extra
CPU cycles that you have to expend _per 16-bit bus use_).

It's most helpful to think of EWRAM as slower, distant memory, similar to the
"heap" in a normal application. You can take the time to go store something
within EWRAM, or to load it out of EWRAM, but if you've got several operations
to do in a row and you're worried about time you should pull that value into
local memory, work on your local copy, and then push it back out to EWRAM.

## Internal Work RAM / IWRAM

* `0x3000000` to `0x3007FFF` (32k)

This is a smaller pile of space, but it has a 32-bit bus and no wait.

By default, `0x3007F00` to `0x3007FFF` is reserved for interrupt and BIOS use.
The rest of it is totally up to you. The user's stack space starts at
`0x3007F00` and proceeds _down_ from there. For best results you should probably
start at `0x3000000` and then go upwards. Under normal use it's unlikely that
the two memory regions will crash into each other.

## IO Registers

* `0x4000000` to `0x40003FE`

We've touched upon a few of these so far, and we'll get to more later. At the
moment it is enough to say that, as you might have guessed, all of them live in
this region. Each individual register is a `u16` or `u32` and they control all
sorts of things. We'll actually be talking about some more of them in this very
chapter, because that's how we'll control some of the background and object
stuff.

## Palette RAM / PALRAM

* `0x5000000` to `0x50003FF` (1k)

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

## Video RAM / VRAM

* `0x6000000` to `0x6017FFF` (96k)

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

## Object Attribute Memory / OAM

* `0x7000000` to `0x70003FF` (1k)

The Object Attribute Memory has a 32-bit bus and no default wait, but suffers
from the "you might have to wait if the display controller is looking at it"
rule. You cannot write individual bytes to OAM at all, but that's not really a
problem because all the fields of the data types within OAM are either `i16` or
`u16` anyway.

Object attribute memory is the wildest yet: it conceptually contains two types
of things, but they're _interlaced_ with each other all the way through.

Now, [GBATEK](http://problemkaputt.de/gbatek.htm#lcdobjoamattributes) and
[CowByte](https://www.cs.rit.edu/~tjh8300/CowBite/CowBiteSpec.htm#OAM%20(sprites))
doesn't quite give names to the two data types here.
[TONC](https://www.coranac.com/tonc/text/regobj.htm#sec-oam) calls them
`OBJ_ATTR` and `OBJ_AFFINE`, but we'll be giving them names fitting with the
Rust naming convention. Just know that if you try to talk about it with others
they might not be using the same names. In Rust terms their layout would look
like this:

```rust
#[repr(C)]
pub struct ObjectAttributes {
  attr0: u16,
  attr1: u16,
  attr2: u16,
  filler: i16,
}

#[repr(C)]
pub struct AffineMatrix {
  filler0: [u16; 3],
  pa: i16,
  filler1: [u16; 3],
  pb: i16,
  filler2: [u16; 3],
  pc: i16,
  filler3: [u16; 3],
  pd: i16,
}
```

(Note: the `#[repr(C)]` part just means that Rust must lay out the data exactly
in the order we specify, which otherwise it is not required to do).

So, we've got 1024 bytes in OAM and each `ObjectAttributes` value is 8 bytes, so
naturally we can support up to 128 objects.

_At the same time_, we've got 1024 bytes in OAM and each `AffineMatrix` is 32
bytes, so we can have 32 of them.

But, as I said, these things are all _interlaced_ with each other. See how
there's "filler" fields in each struct? If we imagine the OAM as being just an
array of one type or the other, indexes 0/1/2/3 of the `ObjectAttributes` array
would line up with index 0 of the `AffineMatrix` array. It's kinda weird, but
that's just how it works. When we setup functions to read and write these values
we'll have to be careful with how we do it. We probably _won't_ want to use
those representations above, at least not with the `AffineMatrix` type, because
they're quite wasteful if you want to store just object attributes or just
affine matrices.

## Game Pak ROM / Flash ROM

* `0x8000000` to `0x9FFFFFF` (wait 0)
* `0xA000000` to `0xBFFFFFF` (wait 1)
* `0xC000000` to `0xDFFFFFF` (wait 2)
* Max of 32Mb

These portions of the memory are less fixed, because they depend on the precise
details of the game pak you've inserted into the GBA. In general, they connect
to the game pak ROM and/or Flash memory, using a 16-bit bus. The ROM is
read-only, but the Flash memory (if any) allows writes.

The game pak ROM is listed as being in three sections, but it's actually the
same memory being effectively mirrored into three different locations. The
mirror that you choose to access the game pak through affects which wait state
setting it uses (configured via IO register of course). Unfortunately, the
details come down more to the game pak hardware that you load your game onto
than anything else, so there's not much I can say right here. We'll eventually
talk about it more later when I'm forced to do the boring thing and just cover
all the IO registers that aren't covered anywhere else.

One thing of note is the way that the 16-bit bus affects us: the instructions to
execute are coming through the same bus as the rest of the game data, so we want
them to be as compact as possible. The ARM chip in the GBA supports two
different instruction sets, "thumb" and "non-thumb". The thumb mode instructions
are 16-bit, so they can each be loaded one at a time, and the non-thumb
instructions are 32-bit, so we're at a penalty if we execute them directly out
of the game pak. However, some things will demand that we use non-thumb code, so
we'll have to deal with that eventually. It's possible to switch between modes,
but it's a pain to keep track of what mode you're in because there's not
currently support for it in Rust itself (perhaps some day). So we'll stick with
thumb code as much as we possibly can, that's why our target profile for our
builds starts with `thumbv4`.

## Game Pak SRAM

* `0xE000000` to `0xE00FFFF` (64k)

The game pak SRAM has an 8-bit bus. Why did Pokémon always take so long to save?
Saving the whole game one byte at a time is why. The SRAM also has some amount
of wait, but as with the ROM, the details depend on your game pak hardware (and
also as with ROM, you can adjust the settings with an IO register, should you
need to).

One thing to note about the SRAM is that the GBA has a Direct Memory Access
(DMA) feature that can be used for bulk memory movements in some cases, but the
DMA _cannot_ access the SRAM region. You really are stuck reading and writing
one byte at a time when you're using the SRAM.
