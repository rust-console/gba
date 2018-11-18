# light_cycle

Now let's make a game of "light_cycle" with our new knowledge.

## Gameplay

`light_cycle` is pretty simple, and very obvious if you've ever seen Tron. The
player moves around the screen with a trail left behind them. They die if they
go off the screen or if they touch their own trail.

## Operations

We need some better drawing operations this time around.

```rust
pub unsafe fn mode3_clear_screen(color: u16) {
  let color = color as u32;
  let bulk_color = color << 16 | color;
  let mut ptr = VolatilePtr(VRAM as *mut u32);
  for _ in 0..SCREEN_HEIGHT {
    for _ in 0..(SCREEN_WIDTH / 2) {
      ptr.write(bulk_color);
      ptr = ptr.offset(1);
    }
  }
}

pub unsafe fn mode3_draw_pixel(col: isize, row: isize, color: u16) {
  VolatilePtr(VRAM as *mut u16).offset(col + row * SCREEN_WIDTH).write(color);
}

pub unsafe fn mode3_read_pixel(col: isize, row: isize) -> u16 {
  VolatilePtr(VRAM as *mut u16).offset(col + row * SCREEN_WIDTH).read()
}
```

The draw pixel and read pixel are both pretty obvious. What's new is the clear
screen operation. It changes the `u16` color into a `u32` and then packs the
value in twice. Then we write out `u32` values the whole way through screen
memory. This means we have to do less write operations overall, and so the
screen clear is twice as fast.

Now we just have to fill in the main function:

```rust
#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
  unsafe {
    DISPCNT.write(MODE3 | BG2);
  }

  let mut px = SCREEN_WIDTH / 2;
  let mut py = SCREEN_HEIGHT / 2;
  let mut color = rgb16(31, 0, 0);

  loop {
    // read the input for this frame
    let this_frame_keys = key_input();

    // adjust game state and wait for vblank
    px += 2 * this_frame_keys.column_direction() as isize;
    py += 2 * this_frame_keys.row_direction() as isize;
    wait_until_vblank();

    // draw the new game and wait until the next frame starts.
    unsafe {
      if px < 0 || py < 0 || px == SCREEN_WIDTH || py == SCREEN_HEIGHT {
        // out of bounds, reset the screen and position.
        mode3_clear_screen(0);
        color = color.rotate_left(5);
        px = SCREEN_WIDTH / 2;
        py = SCREEN_HEIGHT / 2;
      } else {
        let color_here = mode3_read_pixel(px, py);
        if color_here != 0 {
          // crashed into our own line, reset the screen
          mode3_clear_screen(0);
          color = color.rotate_left(5);
        } else {
          // draw the new part of the line
          mode3_draw_pixel(px, py, color);
          mode3_draw_pixel(px, py + 1, color);
          mode3_draw_pixel(px + 1, py, color);
          mode3_draw_pixel(px + 1, py + 1, color);
        }
      }
    }
    wait_until_vdraw();
  }
}
```

Oh that's a lot more than before!

First we set Mode 3 and Background 2, we know about that.

Then we're going to store the player's x and y, along with a color value for
their light cycle. Then we enter the core loop.

We read the keys for input, and then do as much as we can without touching video
memory. Since we're using video memory as the place to store the player's light
trail, we can't do much, we just update their position and wait for VBlank to
start. The player will be a 2x2 square, so the arrows will move you 2 pixels per
frame.

Once we're in VBlank we check to see what kind of drawing we're doing. If the
player has gone out of bounds, we clear the screen, rotate their color, and then
reset their position. Why rotate the color? Just because it's fun to have
different colors.

Next, if the player is in bounds we read the video memory for their position. If
it's not black that means we've been here before and the player has crashed into
their own line. In this case, we reset the game without moving them to a new
location.

Finally, if the player is in bounds and they haven't crashed, we write their
color into memory at this position.

Regardless of how it worked out, we hold here until vdraw starts before going to
the next loop. That's all there is to it.

## The gba crate doesn't quite work like this

Once again, as with the `hello1` and `hello2` examples, the `gba` crate covers
much of this same ground as our example here, but in slightly different ways.

Better organization and abstractions are usually only realized once you've used
more of the whole thing you're trying to work with. If we want to have a crate
where the whole thing is well integrated with itself, then the examples would
also end up having to explain about things we haven't really touched on much
yet. It becomes a lot harder to teach.

So, going forward, we will continue to teach concepts and build examples that
don't directly depend on the `gba` crate. This allows the crate to freely grow
without all the past examples becoming a great inertia upon it.