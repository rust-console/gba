# hello2

Okay so let's have a look again:

`hello1`

```rust
#![feature(start)]
#![no_std]

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  loop {}
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
  unsafe {
    (0x04000000 as *mut u16).write_volatile(0x0403);
    (0x06000000 as *mut u16).offset(120 + 80 * 240).write_volatile(0x001F);
    (0x06000000 as *mut u16).offset(136 + 80 * 240).write_volatile(0x03E0);
    (0x06000000 as *mut u16).offset(120 + 96 * 240).write_volatile(0x7C00);
    loop {}
  }
}
```

Now let's clean this up so that it's clearer what's going on.

First we'll label that display control stuff:

```rust
pub const DISPCNT: *mut u16 = 0x04000000 as *mut u16;
pub const MODE3: u16 = 3;
pub const BG2: u16 = 0b100_0000_0000;
```

Next we make some const values for the actual pixel drawing

```rust
pub const VRAM: usize = 0x06000000;
pub const SCREEN_WIDTH: isize = 240;
```

And then we want a small helper function for putting together a color value.

Happily, this one can even be declared as a const function. At the time of
writing, we've got the "minimal const fn" support in nightly. It really is quite
limited, but I'm happy to let rustc and LLVM pre-compute as much as they can
when it comes to the GBA's tiny CPU.

```rust
pub const fn rgb16(red: u16, green: u16, blue: u16) -> u16 {
  blue << 10 | green << 5 | red
}
```

Finally, we'll make a function for drawing a pixel in Mode 3. Even though it's
just a one-liner, having the "important parts" be labeled as function arguments
usually helps you think about it a lot better.

```rust
pub unsafe fn mode3_pixel(col: isize, row: isize, color: u16) {
  (VRAM as *mut u16).offset(col + row * SCREEN_WIDTH).write_volatile(color);
}
```

So now we've got this:

`hello2`

```rust
#![feature(start)]
#![no_std]

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  loop {}
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
  unsafe {
    DISPCNT.write_volatile(MODE3 | BG2);
    mode3_pixel(120, 80, rgb16(31, 0, 0));
    mode3_pixel(136, 80, rgb16(0, 31, 0));
    mode3_pixel(120, 96, rgb16(0, 0, 31));
    loop {}
  }
}

pub const DISPCNT: *mut u16 = 0x04000000 as *mut u16;
pub const MODE3: u16 = 3;
pub const BG2: u16 = 0b100_0000_0000;

pub const VRAM: usize = 0x06000000;
pub const SCREEN_WIDTH: isize = 240;

pub const fn rgb16(red: u16, green: u16, blue: u16) -> u16 {
  blue << 10 | green << 5 | red
}

pub unsafe fn mode3_pixel(col: isize, row: isize, color: u16) {
  (VRAM as *mut u16).offset(col + row * SCREEN_WIDTH).write_volatile(color);
}
```

Exact same program that we started with, but much easier to read.

Of course, in the full `gba` crate that this book is a part of we have these and
other elements all labeled and sorted out for you (not identically, but
similarly). Still, for educational purposes it's often best to do it yourself at
least once.
