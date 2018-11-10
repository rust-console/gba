# Video Memory Intro

The GBA's Video RAM is 96k stretching from `0x0600_0000` to `0x0601_7FFF`.

The Video RAM can only be accessed totally freely during a Vertical Blank
(aka "vblank"). At other times, if the CPU tries to touch the same part of video
memory as the display controller is accessing then the CPU gets bumped by a
cycle to avoid a clash.

Annoyingly, VRAM can only be properly written to in 16 and 32 bit segments (same
with PALRAM and OAM). If you try to write just an 8 bit segment, then both parts
of the 16 bit segment get the same value written to them. In other words, if you
write the byte `5` to `0x0600_0000`, then both `0x0600_0000` and ALSO
`0x0600_0001` will have the byte `5` in them. We have to be extra careful when
trying to set an individual byte, and we also have to be careful if we use
`memcopy` or `memset` as well, because they're byte oriented by default and
don't know to follow the special rules.

## RGB15

TODO

## Mode 3

TODO

## Mode 4

TODO

## Mode 5

TODO

## In Conclusion...

TODO
