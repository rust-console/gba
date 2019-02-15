# Bitmap Video

Our first video modes to talk about are the bitmap video modes.

It's not because they're the best and fastest, it's because they're the
_simplest_. You can get going and practice with them really quickly. Usually
after that you end up wanting to move on to the other video modes because they
have better hardware support, so you can draw more complex things with the small
number of cycles that the GBA allows.

## The Three Bitmap Modes

As I said in the Hardware Memory Map section, the Video RAM lives in the address
space at `0x600_0000`. Depending on our video mode the display controller will
consider this memory to be in one of a few totally different formats.

### Mode 3

The screen is 160 rows, each 240 pixels long, of `u16` color values.

This is "full" resolution, and "full" color. It adds up to 76,800 bytes. VRAM is
only 96,304 bytes total though. There's enough space left over after the bitmap
for some object tile data if you want to use objects, but basically Mode3 is
using all of VRAM as one huge canvas.

### Mode 4

The screen is 160 rows, each 240 pixels long, of `u8` palette values.

This has half as much space per pixel. What's a palette value? That's an index
into the background PALRAM which says what the color of that pixel should be. We
still have the full color space available, but we can only use 256 colors at the
same time.

What did we get in exchange for this? Well, now there's a second "page". The
second page starts `0xA000` bytes into VRAM (in both Mode 4 and Mode 5). It's an
entire second set of pixel data. You determine if Page 0 or Page 1 is shown
using bit 4 of DISPCNT. When you swap which page is being displayed it's called
page flipping or flipping the page, or something like that.

Having two pages is cool, but Mode 4 has a big drawback: it's part of VRAM so
that "can't write 1 byte at a time" rule applies. This means that to set a
single byte we need to read a `u16`, adjust just one side of it, and then write
that `u16` back. We can hide the complication behind a method call, but it
simply takes longer to do all that, so editing pixels ends up being
unfortunately slow compared to the other bitmap modes.

### Mode 5

The screen is 128 rows, each 160 pixels long, of `u16` color values.

Mode 5 has two pages like Mode 4 does, but instead of keeping full resolution we
keep full color. The pixels are displayed in the top left and it's just black on
the right and bottom edges. You can use the background control registers to
shift it around, maybe center it, but there's no way to get around the fact that
not having full resolution is kinda awkward.

## Using Mode 3

Let's have a look at how this comes together. We'll call this one
`hello_world.rs`, since it's our first real program.

### Module Attributes and Imports

At the top of our file we're still `no_std` and we're still using
`feature(start)`, but now we're using the `gba` crate so we're 100% safe code!
Often enough we'll need a little `unsafe`, but for just bitmap drawing we don't
need it.

```rust
#![no_std]
#![feature(start)]
#![forbid(unsafe_code)]

use gba::{
  fatal,
  io::{
    display::{DisplayControlSetting, DisplayMode, DISPCNT, VBLANK_SCANLINE, VCOUNT},
    keypad::read_key_input,
  },
  vram::bitmap::Mode3,
  Color,
};
```

### Panic Handler

Before we had a panic handler that just looped forever. Now that we're using the
`gba` crate we can rely on the debug output channel from `mGBA` to get a message
into the real world. There's macros setup for each message severity, and they
all accept a format string and arguments, like how `println` works. The catch is
that a given message is capped at a length of 255 bytes, and it should probably
be ASCII only.

In the case of the `fatal` message level, it also halts the emulator.

Of course, if the program is run on real hardware then the `fatal` message won't
stop the program, so we still need the infinite loop there too.

(not that this program _can_ panic, but `rustc` doesn't know that so it demands
we have a `panic_handler`)

```rust
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
  // This kills the emulation with a message if we're running within mGBA.
  fatal!("{}", info);
  // If we're _not_ running within mGBA then we still need to not return, so
  // loop forever doing nothing.
  loop {}
}
```

### Waiting Around

Like I talked about before, sometimes we need to wait around a bit for the right
moment to start doing work. However, we don't know how to do the good version of
waiting for VBlank and VDraw to start, so we'll use the really bad version of it
for now.

```rust
/// Performs a busy loop until VBlank starts.
///
/// This is very inefficient, and please keep following the lessons until we
/// cover how interrupts work!
pub fn spin_until_vblank() {
  while VCOUNT.read() < VBLANK_SCANLINE {}
}

/// Performs a busy loop until VDraw starts.
///
/// This is very inefficient, and please keep following the lessons until we
/// cover how interrupts work!
pub fn spin_until_vdraw() {
  while VCOUNT.read() >= VBLANK_SCANLINE {}
}
```

### Setup in `main`

In main we set the display control value we want and declare a few variables
we're going to use in our primary loop.

```rust
#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
  const SETTING: DisplayControlSetting =
    DisplayControlSetting::new().with_mode(DisplayMode::Mode3).with_bg2(true);
  DISPCNT.write(SETTING);

  let mut px = Mode3::WIDTH / 2;
  let mut py = Mode3::HEIGHT / 2;
  let mut color = Color::from_rgb(31, 0, 0);
```

### Stuff During VDraw

When a frame starts we want to read the keys, then adjust as much of the game
state as we can without touching VRAM.

Once we're ready, we do our spin loop until VBlank starts.

In this case, we're going to adjust `px` and `py` depending on the arrow pad
input, and also we'll cycle around the color depending on L and R being pressed.

```rust
  loop {
    // read our keys for this frame
    let this_frame_keys = read_key_input();

    // adjust game state and wait for vblank
    px = px.wrapping_add(2 * this_frame_keys.x_tribool() as usize);
    py = py.wrapping_add(2 * this_frame_keys.y_tribool() as usize);
    if this_frame_keys.l() {
      color = Color(color.0.rotate_left(5));
    }
    if this_frame_keys.r() {
      color = Color(color.0.rotate_right(5));
    }

    // now we wait
    spin_until_vblank();
```

### Stuff During VBlank

When VBlank starts we want want to update video memory to display the new
frame's situation.

In our case, we're going to paint a little square of the current color, but also
if you go off the map it resets the screen.

At the end, we spin until VDraw starts so we can do the whole thing again.

```rust
    // draw the new game and wait until the next frame starts.
    if px >= Mode3::WIDTH || py >= Mode3::HEIGHT {
      // out of bounds, reset the screen and position.
      Mode3::dma_clear_to(Color::from_rgb(0, 0, 0));
      px = Mode3::WIDTH / 2;
      py = Mode3::HEIGHT / 2;
    } else {
      // draw the new part of the line
      Mode3::write(px, py, color);
      Mode3::write(px, py + 1, color);
      Mode3::write(px + 1, py, color);
      Mode3::write(px + 1, py + 1, color);
    }

    // now we wait again
    spin_until_vdraw();
  }
}
```
