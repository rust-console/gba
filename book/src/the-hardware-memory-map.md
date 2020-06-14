# The Hardware Memory Map

So we saw `hello_magic.rs` and then we learned what `volatile` was all about,
but we've still got a few things that are a bit mysterious. You can't just cast
a number into a pointer and start writing to it! That's totally crazy! That's
writing to un-allocated memory! Against the rules!

Well, _kinda_. It's true that you're not allowed to write _anywhere at all_, but
those locations were carefully selected locations.

You see, on a modern computer if you need to check if a key is pressed you ask
the Operating System (OS) to please go check for you. If you need to play a
sound, you ask the OS to please play the sound on a default sound output. If you
need to show a picture you ask the OS to give you access to the video driver so
that you can ask the video driver to please put some pixels on the screen.
That's mostly fine, except how does the OS actually do it? It doesn't have an OS
to go ask, it has to stop somewhere.

Ultimately, every piece of hardware is mapped into somewhere in the address
space of the CPU. You can't actually tell that this is the case as a normal user
because your program runs inside a virtualized address space. That way you can't
go writing into another program's memory and crash what they're doing or steal
their data (well, hopefully, it's obviously not perfect). Outside of the
virtualization layer the OS is running directly in the "true" address space, and
it can access the hardware on behalf of a program whenever it's asked to.

How does directly accessing the hardware work, _precisely_? It's just the same
as accessing the RAM. Each address holds some bits, and the CPU picks an address
and loads in the bits. Then the program gets the bits and has to decide what
they mean. The "driver" of a hardware device is just the layer that translates
between raw bits in the outside world and more meaningful values inside of the
program.

Of course, memory mapped hardware can change its bits at any time. The user can
press and release a key and you can't stop them. This is where `volatile` comes
in. Whenever there's memory mapped hardware you want to access it with
`volatile` operations so that you can be sure that you're sending the data every
time, and that you're getting fresh data every time.

## GBA Specifics

That's enough about the general concept of memory mapped hardware, let's get to
some GBA specifics. The GBA has the following sections in its memory map.

* BIOS
* External Work RAM (EWRAM)
* Internal Work RAM (IWRAM)
* IO Registers
* Palette RAM (PALRAM)
* Video RAM (VRAM)
* Object Attribute Memory (OAM)
* Game Pak ROM (ROM)
* Save RAM (SRAM)

Each of these has a few key points of interest:

* **Bus Width:** Also just called "bus", this is how many little wires are
  _physically_ connecting a part of the address space to the CPU. If you need to
  transfer more data than fits in the bus you have to do repeated transfers
  until it all gets through.
* **Read/Write Modes:** Most parts of the address space can be read from in 8,
  16, or 32 bits at a time (there's a few exceptions we'll see). However, a
  significant portion of the address space can't accept 8 bit writes. Usually
  this isn't a big deal, but standard `memcopy` routine switches to doing a
  byte-by-byte copy in some situations, so we'll have to be careful about using
  it in combination with those regions of the memory.
* **Access Speed:** On top of the bus width issue, not all memory can be
  accessed at the same speed. The "fast" parts of memory can do a read or write
  in 1 cycle, but the slower parts of memory can take a few cycles per access.
  These are called "wait cycles". The exact timings depend on what you configure
  the system to use, which is also limited by what your cartridge physically
  supports. You'll often see timings broken down into `N` cycles (non-sequential
  memory access) and `S` cycles (sequential memory access, often faster). There
  are also `I` cycles (internal cycles) which happen whenever the CPU does an
  internal operation that's more than one cycle to complete (like a multiply).
  Don't worry, you don't have to count exact cycle timings unless you're on the
  razor's edge of the GBA's abilities. For more normal games you just have to be
  mindful of what you're doing and it'll be fine.

Let's briefly go over the major talking points of each memory region. All of
this information is also available in GBATEK, mostly in their [memory
map](http://www.akkit.org/info/gbatek.htm#gbamemorymap) section (though somewhat
spread through the rest of the document too).

Though I'm going to list the location range of each memory space below, most of
the hardware locations are actually mirrored at several points throughout the
address space.

### BIOS

* **Location:** `0x0` to `0x3FFF`
* **Bus:** 32-bit
* **Access:** Memory protected read-only (see text).
* **Wait Cycles:** None

The "basic input output system". This contains a grab bag of utilities that do
various tasks. The code is optimized for small size rather than great speed, so
you can sometimes write faster versions of these routines. Also, calling a bios
function has more overhead than a normal function call. You can think of bios
calls as being similar to system calls to the OS on a desktop computer. Useful,
but costly.

As a side note, not only is BIOS memory read only, but it's memory protected so
that you can't even read from bios memory unless the system is currently
executing a function that's in bios memory. If you try then the system just
gives back a nonsensical value that's not really what you asked for. If you
really want to know what's inside, there's actually a bug in one bios call
(`MidiKey2Freq`) that lets you read the bios section one byte at a time.

Also, there's not just one bios! Of course there's the official bios from
Nintendo that's used on actual hardware, but since that's code instead of
hardware it's protected by copyright. Since a bios is needed to run a GBA
emulator properly, people have come up with their own open source versions or
they simply make the emulator special case the bios and act _as if_ the function
call had done the right thing.

* The [TempGBA](https://github.com/Nebuleon/TempGBA) repository has an easy to
  look at version written in assembly. It's API and effects are close enough to
  the Nintendo version that most games will run just fine.
* You can also check out the [mGBA
  bios](https://github.com/mgba-emu/mgba/blob/master/src/gba/bios.c) if you want
  to see the C version of what various bios functions are doing.

### External Work RAM (EWRAM)

* **Location:** `0x200_0000` to `0x203_FFFF` (256k)
* **Bus:** 16-bit
* **Access:** Read-write, any size.
* **Wait Cycles:** 2

The external work ram is a sizable amount of space, but the 2 wait cycles per
access and 16-bit bus mean that you should probably think of it as being a
"heap" to avoid putting things in if you don't have to.

The GBA itself doesn't use this for anything, so any use is totally up to you.

At the moment, the linker script and `crt0.s` files provided with the `gba`
crate also have no defined use for the EWRAM, so it's 100% on you to decide how
you wanna use them.

(Note: There is an undocumented control register that lets you adjust the wait
cycles on EWRAM. Using it, you can turn EWRAM from the default 2 wait cycles
down to 1. However, not all GBA-like things support it. The GBA and GBA SP do,
the GBA Micro and DS do not. Emulators might or might not depending on the
particular emulator. See the [GBATEK system
control](https://problemkaputt.de/gbatek.htm#gbasystemcontrol) page for a full
description of that register, though probably only once you've read more of this
tutorial book and know how to make sense of IO registers and such.)

### Internal Work RAM (IWRAM)

* **Location:** `0x300_0000` to `0x300_7FFF` (32k)
* **Bus:** 32-bit
* **Access:** Read-write, any size.
* **Wait Cycles:** 0

This is where the "fast" memory for general purposes lives. By default the
system uses the 256 _bytes_ starting at `0x300_7F00` _and up_ for system and
interrupt purposes, while Rust's program stack starts at that same address _and
goes down_ from there.

Even though your stack exists in this space, it's totally reasonable to use the
bottom parts of this memory space for whatever quick scratch purposes, same as
EWRAM. 32k is fairly huge, and the stack going down from the top and the scratch
data going up from the bottom are unlikely to hit each other. If they do you
were probably well on your way to a stack overflow anyway.

The linker script and `crt0.s` file provided with the `gba` crate use the bottom
of IWRAM to store the `.data` and `.bss` [data
segments](https://en.wikipedia.org/wiki/Data_segment). That's where your global
variables get placed (both `static` and `static mut`). The `.data` segment holds
any variable that's initialized to non-zero, and the `.bss` section is for any
variable initialized to zero. When the GBA is powered on, some code in the
`crt0.s` file runs and copies the initial `.data` values into place within IWRAM
(all of `.bss` starts at 0, so there's no copy for those variables).

If you have no global variables at all, then you don't need to worry about those
details, but if you do have some global variables then you can use the _address
of_ the `__bss_end` symbol defined in the top of the `gba` crate as a marker for
where it's safe for you to start using IWRAM without overwriting your globals.

### IO Registers

* **Location:** `0x400_0000` to `0x400_03FE`
* **Bus:** 32-bit
* **Access:** different for each IO register
* **Wait Cycles:** 0

The IO Registers are where most of the magic happens, and it's where most of the
variety happens too. Each IO register is a specific width, usually 16-bit but
sometimes 32-bit. Most of them are fully read/write, but some of them are read
only or write only. Some of them have individual bits that are read only even
when the rest of the register is writable. Some of them can be written to, but
the write doesn't change the value you read back, it sets something else.
Really.

The IO registers are how you control every bit of hardware besides the CPU
itself. Reading the buttons, setting display modes, enabling timers, all of that
goes through different IO registers. Actually, even a few parts of the CPU's
operation can be controlled via IO register.

We'll go over IO registers more in the next section, including a few specific
registers, and then we'll constantly encounter more IO registers as we explore
each new topic through the rest of the book.

### Palette RAM (PALRAM)

* **Location:** `0x500_0000` to `0x500_03FF` (1k)
* **Bus:** 16-bit
* **Access:** Read any, single bytes mirrored (see text).
* **Wait Cycles:** Video Memory Wait (see text)

This is where the GBA stores color palette data. There's 256 slots for
Background color, and then 256 slots for Object color.

GBA colors are 15 bits each, with five bits per channel and the highest bit
being totally ignored, so we store them as `u16` values:

* `X_BBBBB_GGGGG_RRRRR`

Of note is the fact that the 256 palette slots can be viewed in two different
ways. There's two different formats for images in video memory: "8 bit per
pixel" (8bpp) and "4 bit per pixel mode" (4bpp).

* **8bpp:** Each pixel in the image is 8 bits and indexes directly into the full
  256 entry palette array. An index of 0 means that pixel should be transparent,
  so there's 255 possible colors.
* **4bpp:** Each pixel in the image is 4 bits and indexes into a "palbank" of 16
  colors within the palette data. Some exterior control selects the palbank to
  be used. An index of 0 still means that the pixel should be transparent, so
  there's 15 possible colors.

Different images can use different modes all at once, as long as you can fit all
the colors you want to use into your palette layout.

PALRAM can't be written to in individual bytes. This isn't normally a problem at
all, because you wouldn't really want to write half of a color entry anyway. If
you do try to write a single byte then it gets "mirrored" into both halves of
the `u16` that would be associated with that address. For example, if you tried
to write `0x01u8` to either `0x500_0000` or `0x500_0001` then you'd actually
_effectively_ be writing `0x0101u16` to `0x500_0000`.

PALRAM follows what we'll call the "Video Memory Wait" rule: If you try to
access the memory during a vertical blank or horizontal blank period there's 0
wait cycles, otherwise there is a chance of a 1 cycle wait _if_ the display
controller was using that precise memory location at that moment.

### Video RAM (VRAM)

* **Location:** `0x600_0000` to `0x601_7FFF` (96k or 64k+32k depending on mode)
* **Bus:** 16-bit
* **Access:** Read any, single bytes _sometimes_ mirrored (see text).
* **Wait Cycles:** Video Memory Wait (see text)

Video RAM tells the display controller what to draw on the screen. The GBA
actually has 6 different display modes (numbered 0 through 5). Depending on the
mode you're using, the way the controller uses the VRAM changes. Because there's
so much involved here, I'll leave more precise details to the following sections
which talk about how to use VRAM in each mode.

You can't write to VRAM in individual bytes. If you try to write a single byte
to background VRAM, the byte gets mirrored like with PALRAM. And if you try with
object VRAM the write gets ignored entirely. What is VRAM and what is object
VRAM depends on the video mode. If you want to change a single byte of data (and
you might) then the correct style is to read the full `u16`, mask out the old
data, mask in your new value, and then write the whole `u16`.

VRAM follows the same "Video Memory Wait" rule as PALRAM.

### Object Attribute Memory (OAM)

* **Location:** `0x700_0000` to `0x700_03FF` (1k)
* **Bus:** 32-bit
* **Access:** Read any, single bytes no effect (see text).
* **Wait Cycles:** Video Memory Wait (see text)

This part of memory controls the "Objects" (OBJ) on the screen. An object is
_similar to_ the concept of a "sprite". However, because of an object's size
limitations, a single sprite might require more than one object to be drawn
properly. In general, if you want to think in terms of sprites at all, you
should think of sprites as being a logical / programming concept, and objects as
being a hardware concept.

While VRAM has the _image_ data for each object, this part of memory has the
_control_ data for each object. An objects "attributes" describe what part of
the VRAM to use, where to place is on the screen, any special graphical effects
to use, all that stuff. Each object has 6 bytes of attribute data (arranged as
three `u16` values), and there's a total of 128 objects (indexed 0 through 127).

But 6 bytes each times 128 entries out of 1024 bytes leaves us with 256 bytes
left over. What's the other space used for? Well, it's a little weird, but after
every three `u16` object attribute fields there's one `i16` "affine parameter"
field mixed in. It takes four such fields to make a complete set of affine
parameters (a 2x2 matrix), so we get a total of 32 affine parameter entries
across all of OAM. "Affine" might sound fancy but it just means a transformation
where anything that started parallel stays parallel after the transform. The
affine parameters can be used to scale, rotate, and/or skew a background or
object as it's being displayed on the screen. It takes more computing power than
the non-affine display, so you can't display as many different things at once
when using the affine modes.

OAM can't ever be written to with individual bytes. The write just has no effect
at all.

OAM follows the same "Video Memory Wait" rule that PALRAM has, **and** you can
only freely access OAM during a horizontal blank if you set a special "HBlank
Interval Free" bit in one of the IO registers (the "Display Control" register,
which we'll talk about next lesson). The reason that you might _not_ want to set
that bit is because when it's enabled you can't draw as many objects at once.
You don't lose the use of an exact number of objects, you actually lose the use
of a number of display adapter drawing cycles. Since not all objects take the
same number of cycles to render, it depends on what you're drawing.
GBATEK [has the details](https://problemkaputt.de/gbatek.htm#lcdobjoverview) if
you want to know precisely.

### Game Pak ROM (ROM)

* **Location:** Special (max of 32MB)
* **Bus:** 16-bit
* **Access:** Special
* **Wait Cycles:** Special

This is where your actual game is located! As you might guess, since each
cartridge is different, the details here depend quite a bit on the one you are
using for your game. Even a simple statement like "you can't write to the ROM
region" isn't true for some carts if they have FlashROM.

The _most important_ thing to concern yourself with when considering the ROM
portion of memory is the 32MB limit. That's compiled code, images, sound,
everything put together. The total has to stay under 32MB.

The next most important thing to consider is that 16-bit bus. It means that we
compile our programs using "Thumb state" code instead of "ARM state" code.
Details about this can be found in the GBA Assembly section of the book, but
just be aware that there's two different types of assembly on the GBA. You can
switch between them, but the default for us is always Thumb state.

Another detail which you actually _don't_ have to think about much, but that you
might care if you're doing precise optimization, is that the ROM address space
is actually mirrored across three different locations:

* `0x800_0000` to `0x9FF_FFFF`: Wait State 0
* `0xA00_0000` to `0xBFF_FFFF`: Wait State 1
* `0xC00_0000` to `0xDFF_FFFF`: Wait State 2

These _don't_ mean 0, 1, and 2 wait cycles, they mean the wait cycles associated
with ROM mirrors 0, 1, and 2. On some carts the game will store different parts
of the data into different chips that are wired to be accessible through
different parts of the mirroring. The actual wait cycles used are even
configurable via an IO register called the
[WAITCNT](https://problemkaputt.de/gbatek.htm#gbasystemcontrol) ("Wait Control",
I don't know why C programmers have to give everything the worst names it's not
1980 any more).

### Save RAM (SRAM)

* **Location:** Special (max of 64k)
* **Bus:** 8-bit
* **Access:** Special
* **Wait Cycles:** Special

The Save RAM is also part of the cart that you've got your game on, so it also
depends on your hardware.

SRAM _starts_ at `0xE00_0000` and you can save up to however much the hardware
supports, to a maximum of 64k. However, you can only read and write SRAM one
_byte_ at a time. What's worse, while you can _write_ to SRAM using code
executing anywhere, you can only _read_ with code that's executing out of either
Internal or External Work RAM, not from with code that's executing out of ROM.
This means that you need to copy the code for doing the read into some scratch
space (either at startup or on the fly, doesn't matter) and call that function
you've carefully placed. It's a bit annoying, but soon enough a routine for it
all will be provided in the `gba` crate and we won't have to worry too much
about it.

(TODO: Provide the routine that I just claimed we would provide.)
