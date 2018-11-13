# The VCount Register

There's an IO register called
[VCOUNT](http://problemkaputt.de/gbatek.htm#lcdiointerruptsandstatus) that shows
you, what else, the Vertical (row) COUNT(er). It's a `u16` at address
`0x0400_0006`, and it's how we'll be doing our very poor quality vertical sync
code to start.

* **What makes it poor?** Well, we're just going to read from the vcount value as
  often as possible every time we need to wait for a specific value to come up,
  and then proceed once it hits the point we're looking for.
* **Why is this bad?** Because we're making the CPU do a lot of useless work,
  which uses a lot more power that necessary. Even if you're not on an actual
  GBA you might be running inside an emulator on a phone or other handheld. You
  wanna try to save battery if all you're doing with that power use is waiting
  instead of making a game actually do something.
* **Can we do better?** We can, but not yet. The better way to do things is to
  use a BIOS call to put the CPU into low power mode until a VBlank interrupt
  happens. However, we don't know about interrupts yet, and we don't know about
  BIOS calls yet, so we'll do the basic thing for now and then upgrade later.

So the way that display hardware actually displays each frame is that it moves a
tiny pointer left to right across each pixel row one pixel at a time. When it's
within the actual screen width (240px) it's drawing out those pixels. Then it
goes _past_ the edge of the screen for 68px during a period known as the
"horizontal blank" (HBlank). Then it starts on the next row and does that loop
over again. This happens for the whole screen height (160px) and then once again
it goes past the last row for another 68px into a "vertical blank" (VBlank)
period.

* One pixel is 4 CPU cycles
* HDraw is 240 pixels, HBlank is 68 pixels (1,232 cycles per full scanline)
* VDraw is 150 scanlines, VBlank is 68 scanlines (280,896 cycles per full refresh)

Now you may remember some stuff from the display control register section where
it was mentioned that some parts of memory are best accessed during VBlank, and
also during hblank with a setting applied. These blanking periods are what was
being talked about. At other times if you attempt to access video or object
memory you (the CPU) might try touching the same memory that the display device
is trying to use, in which case you get bumped back a cycle so that the display
can finish what it's doing. Also, if you really insist on doing video memory
changes while the screen is being drawn then you might get some visual glitches.
If you can, just prepare all your changes ahead of time and then assign then all
quickly during the blank period.

So first we want a way to check the vcount value at all:

```rust
pub const VCOUNT: *mut u16 = 0x0400_0006 as *mut u16;

pub fn read_vcount() -> u16 {
  unsafe { VCOUNT.read_volatile() }
}
```

Then we want two little helper functions to wait until VBlank and vdraw.

```rust
pub const SCREEN_HEIGHT: isize = 160;

pub fn wait_until_vblank() {
  while read_vcount() < SCREEN_HEIGHT as u16 {}
}

pub fn wait_until_vdraw() {
  while read_vcount() >= SCREEN_HEIGHT as u16 {}
}
```

And... that's it. No special types to be made this time around, it's just a
number we read out of memory.
