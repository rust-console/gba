# IO Registers

As I said before, the IO registers are how you tell the GBA to do all the things
you want it to do. If you want a hint at what's available, they're all listed
out in the [GBA I/O Map](https://problemkaputt.de/gbatek.htm#gbaiomap) section
of GBATEK. Go have a quick look.

Each individual IO register has a particular address just like we talked about
in the Hardware Memory Map section. They also have a size (listed in bytes), and
a note on if they're read only, write only, or read-write. Finally, each
register has a name and a one line summary. Unfortunately for us, the names are
all C style names with heavy shorthand. I'm not normally a fan of shorthand
names, but the `gba` crate uses the register names from GBATEK as much as
possible, since they're the most commonly used set of names among GBA
programmers. That way, if you're reading other guides and they say to set the
`BG2CNT` register, then you know exactly what register to look for within the
`gba` docs.

## Register Bits

There's only about 100 registers, but there's a lot more than 100 details we
want to have control over on the GBA. How does that work? Well, let's use a
particular register to talk about it. The first one on the list is `DISPCNT`,
the "Display Control" register. It's one of the most important IO registers, so
this is a "two birds with one stone" situation.

Naturally there's a whole lot of things involved in the LCD that we want to
control, and it's all "one" value, but that value is actually many "fields"
packed into one value. When learning about an IO register, you have to look at
its bit pattern breakdown. For `DISPCNT` the GBATEK entry looks like this:

```txt
4000000h - DISPCNT - LCD Control (Read/Write)
  Bit   Expl.
  0-2   BG Mode                (0-5=Video Mode 0-5, 6-7=Prohibited)
  3     Reserved / CGB Mode    (0=GBA, 1=CGB; can be set only by BIOS opcodes)
  4     Display Frame Select   (0-1=Frame 0-1) (for BG Modes 4,5 only)
  5     H-Blank Interval Free  (1=Allow access to OAM during H-Blank)
  6     OBJ Character VRAM Mapping (0=Two dimensional, 1=One dimensional)
  7     Forced Blank           (1=Allow FAST access to VRAM,Palette,OAM)
  8     Screen Display BG0  (0=Off, 1=On)
  9     Screen Display BG1  (0=Off, 1=On)
  10    Screen Display BG2  (0=Off, 1=On)
  11    Screen Display BG3  (0=Off, 1=On)
  12    Screen Display OBJ  (0=Off, 1=On)
  13    Window 0 Display Flag   (0=Off, 1=On)
  14    Window 1 Display Flag   (0=Off, 1=On)
  15    OBJ Window Display Flag (0=Off, 1=On)
```

So what we're supposed to understand here is that we've got a `u16`, and then we
set the individual bits for the things that we want. In the `hello_magic`
example you might recall that we set this register to the value `0x0403`. That
was a bit of a trick on my part because hex numbers usually look far more
mysterious than decimal or binary numbers. If we converted it to binary it'd
look like this:

```rust
0b100_0000_0011
```

And then you can just go down the list of settings to see what bits are what:

* Bits 0-2 (BG Mode) are `0b011`, so that's Video Mode 3
* Bit 10 (Display BG2) is enabled
* Everything else is disabled

Naturally, trying to remember exactly what bit does what can be difficult. In
the `gba` crate we attempt as much as possible to make types that wrap over a
`u16` or `u32` and then have getters and setters _as if_ all the inner bits were
different fields.

* If it's a single bit then the getter/setter will use `bool`.
* If it's more than one bit and each pattern has some non-numeric meaning then
  it'll use an `enum`.
* If it's more than one bit and numeric in nature then it'll just use the
  wrapped integer type. Note that you generally won't get the full range of the
  inner number type, and any excess gets truncated down to fit in the bits
  available.

All the getters and setters are defined as `const` functions, so you can make
constant declarations for the exact setting combinations that you want.

## Some Important IO Registers

It's not easy to automatically see what registers will be important for getting
started and what registers can be saved to learn about later.

We'll go over three IO registers here that will help us the most to get started,
then next lesson we'll cover how that Video Mode 3 bitmap drawing works, and
then by the end of the next lesson we'll be able to put it all together into
something interactive.

### DISPCNT: Display Control

The [DISPCNT](https://problemkaputt.de/gbatek.htm#lcdiodisplaycontrol) register
lets us affect the major details of our video output. There's a lot of other
registers involved too, but it all starts here.

```rust
pub const DISPCNT: VolAddress<DisplayControlSetting> = unsafe { VolAddress::new(0x400_0000) };
```

As you can see, the display control register is, like most registers,
complicated enough that we make it a dedicated type with getters and setters for
the "phantom" fields. In this case it's mostly a bunch of `bool` values we can
set, and also the video mode is an `enum`.

We already looked at the bit listing above, let's go over what's important right
now and skip the other bits:

* BG Mode sets how the whole screen is going to work and even how the display
  adapter is going to interpret the bit layout of video memory for pixel
  processing. We'll start with Mode 3, which is the simplest to learn.
* The "Forced Blank" bit is one of the very few bits that starts _on_ at the
  start of the main program. When it's enabled it prevents the display adapter
  from displaying anything at all. You use this bit when you need to do a very
  long change to video memory and you don't want the user to see the
  intermediate states being partly drawn.
* The "Screen Display" bits let us enable different display layers. We care
  about BG2 right now because the bitmap modes (3, 4, and 5) are all treated as
  if they were drawing into BG2 (even though it's the only BG layer available in
  those modes).

There's a bunch of other stuff, but we'll get to those things later. They're not
relevent right now, and there's enough to learn already. Already we can see that
when the `hello_magic` demo says

```rust
  (0x400_0000 as *mut u16).write_volatile(0x0403);
```

We could re-write that more sensibly like this

```rust
  const SETTING: DisplayControlSetting = DisplayControlSetting::new()
    .with_mode(DisplayMode::Mode3)
    .with_bg2(true);
  DISPCNT.write(SETTING);
```

### VCOUNT: Vertical Display Counter

The [VCOUNT](https://problemkaputt.de/gbatek.htm#lcdiointerruptsandstatus)
register lets us find out what row of pixels (called a **scanline**) is
currently being processed.

```rust
pub const VCOUNT: ROVolAddress<u16> = unsafe { ROVolAddress::new(0x400_0006) };
```

You see, the display adapter is constantly running its own loop, along side the
CPU. It starts at the very first pixel of the very first scanline, takes 4
cycles to determine what color that pixel is, and then processes the next
pixel. Each scanline is 240 pixels long, followed by 68 "virtual" pixels so that
you have just a moment to setup for the next scanline to be drawn if you need
it. 272 cycles (68*4) is not a lot of time, but it's enough that you could
change some palette colors or move some objects around if you need to.

* Horizontal pixel value `0..240`: "HDraw"
* Horizontal pixel value `240..308`: "HBlank"

There's no way to check the current horizontal counter, but there is a way to
have the CPU interrupt the normal code when the HBlank period starts, which
we'll learn about later.

Once a complete scanline has been processed (including the blank period), the
display adapter keeps going with the next scanline. Similar to how the
horizontal processing works, there's 160 scanlines in the real display, and then
it's followed by 68 "virtual" scanlines to give you time for adjusting video
memory between the frames of the game.

* Vertical Count `0..160`: "VDraw"
* Vertical Count `160..228`: "VBlank"

Once every scanline has been processed (including the vblank period), the
display adapter starts the whole loop over again with scanline 0. A total of
280,896 cycles per display loop (4 * 308 * 228), and about 59.59ns per CPU
cycle, gives us a full speed display rate of 59.73fps. That's close enough to
60fps that I think we can just round up a bit whenever we're not counting it
down to the exact cycle timings.

However, there's a bit of a snag. If we change video memory during the middle of
a scanline the display will _immediately_ start processing using the new state
of video memory. The picture before the change and after the change won't look
like a single, clean picture. Instead you'll get what's called "[screen
tearing](https://en.wikipedia.org/wiki/Screen_tearing)", which is usually
considered to be the mark of a badly programmed game.

To avoid this we just need to only adjust video memory during one of the blank
periods. If you're really cool you can adjust things during HBlank, but we're
not that cool yet. Starting out our general program flow will be:

1) Gather input for the frame (next part of this lesson) and update the game
   state, getting everything ready for when VBlank actually starts.
2) Once VBlank starts we update all of the video memory as fast as we can.
3) Once we're done drawing we again wait for the VDraw period to begin and then
   do it all again.

Now, it's not the most efficient way, but to get our timings right we can just
read from `VCOUNT` over and over in a "busy loop". Once we read a value of 160
we know that we've entered VBlank. Once it goes back to 0 we know that we're
back in VDraw.

Doing a busy loop like this actually drains the batteries way more than
necessary. It keeps the CPU active constantly, which is what uses a fair amount
of the power. Normally you're supposed to put the CPU to sleep if you're just
waiting around for something to happen. However, that also requires learning
about some more concepts to get right. So to keep things easier starting out
we'll do the bad/lazy version and then upgrade our technique later.

### KEYINPUT: Key Input Reading

The [KEYINPUT](https://problemkaputt.de/gbatek.htm#gbakeypadinput) register is
the last one we've got to learn about this lesson. It lets you check the status
of all 10 buttons on the GBA.

```rust
pub const KEYINPUT: ROVolAddress<u16> = unsafe { ROVolAddress::new(0x400_0130) };
```

There's little to say here. It's a read only register, and the data just
contains one bit per button. The only thing that's a little weird about it is
that the bits follow a "low active" convention, so if the button is pressed then
the bit is 0, and if the button is released the bit is 1.

You _could_ work with that directly, but I think it's a lot easier to think
about having `true` for pressed and `false` for not pressed. So the `gba` crate
flips the bits when you read the keys:

```rust
/// Gets the current state of the keys
pub fn read_key_input() -> KeyInput {
  KeyInput(KEYINPUT.read() ^ 0b0000_0011_1111_1111)
}
```

Now we can treat the KeyInput values like a totally normal bitset.
