# Hello, Magic

So we know all the steps to build our source, we just need some source.

We're beginners, so we'll start small. With normal programming there's usually a
console available, so the minimal program prints "Hello, world" to the terminal.
On a GBA we don't have a terminal and standard out and all that, so the minimal
program draws a red, blue, and green dot to the screen.

At the lowest level of device programming, it's all [Magic
Numbers](https://en.wikipedia.org/wiki/Magic_number_(programming)). You write
special values to special places and then the hardware does something. A clear
API makes every magic number and magic location easy to understand. A clear _and
good_ API also prevents you from using the wrong magic number in the wrong place
and causing problems for yourself.

This is the minimal example to just test that our build system is all set, so
just this once we'll go _full_ magic number crazy town, for fun. Ready? Here
goes:

`hello_magic.rs`:

```rust
#![no_std]
#![feature(start)]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  loop {}
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
  unsafe {
    (0x400_0000 as *mut u16).write_volatile(0x0403);
    (0x600_0000 as *mut u16).offset(120 + 80 * 240).write_volatile(0x001F);
    (0x600_0000 as *mut u16).offset(136 + 80 * 240).write_volatile(0x03E0);
    (0x600_0000 as *mut u16).offset(120 + 96 * 240).write_volatile(0x7C00);
    loop {}
  }
}
```

Throw that into your project skeleton, build the program, and give it a run. You
should see a red, green, and blue dot close-ish to the middle of the screen. If
you don't, something _already_ went wrong. Double check things, phone a friend,
write your senators, try asking `Lokathor` or `Ketsuban` on the [Rust Community
Discord](https://discordapp.com/invite/aVESxV8), until you're eventually able to
get your three dots going.

Of course, I'm sure you want to know why those numbers are the numbers to use.
Well that's what the whole rest of the book is about!
